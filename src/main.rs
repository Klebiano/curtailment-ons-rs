use scraper::{Html, Selector};
use reqwest::{header, Client};
use std::collections::HashMap;
use std::env;
use regex::{Regex};
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use polars::prelude::*;
use std::fs::{File, create_dir_all};

const URL: &str = "https://dados.ons.org.br/dataset/restricao_coff_eolica_usi";

fn save_parquet_file(mut df: DataFrame, start_date: NaiveDate, end_date: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving parquet file...");
    let output_dir = "output";
    create_dir_all(output_dir)?;

    let file_path = format!("{}/curtailment_{}_{}.parquet", output_dir, start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d"));
    let file = File::create(file_path)?;

    ParquetWriter::new(file).finish(&mut df)?;
    Ok(())
}

async fn get_data_file(links_hash: Vec<String>) -> Result<DataFrame, Box<dyn std::error::Error>>{
    println!("Getting data files into memory...");
    let scan_args = ScanArgsParquet {
        allow_missing_columns: true,
        ..ScanArgsParquet::default()
    };

    let cols_to_cast = vec![
        "val_geracao", "val_geracaolimitada", "val_disponibilidade",
        "val_geracaoreferencia", "val_geracaoreferenciafinal"
    ];

    let lazy_frames: Vec<LazyFrame> = links_hash
        .iter()
        .map(|url| {
            let mut lf = LazyFrame::scan_parquet(PlRefPath::new(url.as_str()), scan_args.clone()).expect("error scanning the URL");

            lf = lf.with_column(lit(NULL).cast(DataType::String).alias("dsc_restricao"));

            for col_name in &cols_to_cast {
                lf = lf.with_column(col(*col_name).cast(DataType::String).alias(*col_name));
            }
            lf
        })
        .collect();

    let mut combined_lf = concat(lazy_frames, UnionArgs::default())?;

    for col_name in cols_to_cast {
        combined_lf = combined_lf.with_column(
            col(col_name)
                .str()
                .replace_all(lit(","), lit("."), true)
                .cast(DataType::Float64)
                .alias(col_name),
        );
    }

    let df = combined_lf.collect()?;
    // println!("{:?}", df);
    Ok(df)
}


async fn get_ons_curtailment_links(start_date: NaiveDate, end_date: NaiveDate) -> Result<HashMap<NaiveDate, String>, Box<dyn std::error::Error>> {
    println!("Getting ONS links...");
    let mut headers = header::HeaderMap::new();
    let mut ons_links: HashMap<NaiveDate, String> = HashMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:147.0) Gecko/20100101 Firefox/147.0".parse().unwrap());
    
    let client = Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .build()?;

    let document = client.get(URL)
    .headers(headers)
    .send().await?
    .text().await?;
    // println!("{:?}", document);

    let document = Html::parse_document(document.as_str());
    // Selector CSS para obter apenas as tags `âncoras` com arquivos `.parquet` do html recebido
    let selector = Selector::parse("li > a.resource-url-analytics[href*='.parquet'")?;

    let filtered_elements = document.select(&selector);

    for elem in filtered_elements {
        let href = elem.value().attr("href");

        if let Some(valid_href) = href {
            // println!("{}", valid_href);
            let re = Regex::new(r"(\d{4})_(\d{2}).parquet").expect("invalid parttern. .parquet file should end with '%Y_%m.parquet'");

            if let Some(captures) = re.captures(valid_href) {
                
                let year = captures.get(1).expect("Year input format not valid!").as_str().parse::<i32>().expect("invalid year parsing!");
                let month = captures.get(2).expect("Month input format not valid!").as_str().parse::<u32>().expect("invalid month parsing!");

                let specific_date = NaiveDate::parse_from_str(&format!("{}-{}-01", year, month), "%Y-%m-%d").expect("invalid ONS filename parsing format, should be '%Y-%m'");

                if specific_date >= start_date && specific_date <= end_date {
                    ons_links.insert(specific_date, valid_href.to_string());
                }
            }
        }
    }
    println!("{:?}", ons_links);
    Ok(ons_links)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let start_date: NaiveDate = if args.len() > 1 {
        let date_str = &args[1];
        let naive_date = NaiveDate::parse_from_str(&format!("{}-01", date_str), "%Y-%m-%d").expect("invalid start date parsing format, should be '%Y-%m'");
        naive_date
    } else {
        let now: DateTime<Utc> = Utc::now();
        now.with_day(1).unwrap().naive_utc().into()
    };
    
    let end_date: NaiveDate = if args.len() > 2 {
        let date_str = &args[2];
        let naive_date = NaiveDate::parse_from_str(&format!("{}-01", date_str), "%Y-%m-%d").expect("invalid end date parsing format, should be '%Y-%m'");
        naive_date
    } else {
        let now: DateTime<Utc> = Utc::now();
        now.naive_utc().into()
    };

    println!("Detected start_date: {:?}", start_date);
    println!("Detected end_date  : {:?}", end_date);

    let ons_curtailment_links: HashMap<NaiveDate, String> = get_ons_curtailment_links(start_date, end_date).await?;
    let final_df =  get_data_file(ons_curtailment_links.into_values().collect()).await?;
    save_parquet_file(final_df, start_date, end_date)?;
    println!("Done!");
    Ok(())
}
