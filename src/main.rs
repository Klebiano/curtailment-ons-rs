use scraper::{Html, Selector};
use reqwest::{header, Client};
use std::{collections::HashMap, env};
use regex::{Regex};
use chrono::{TimeZone, Utc, DateTime};
use polars::prelude::*;

const URL: &str = "https://dados.ons.org.br/dataset/restricao_coff_eolica_usi";

async fn get_data_file(links_hash: Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
    let scan_args = ScanArgsParquet {
        allow_missing_columns: true,
        ..ScanArgsParquet::default()
    };

    let lazy_frames: Vec<LazyFrame> = links_hash
        .iter()
        .map(|url| {
            LazyFrame::scan_parquet(PlRefPath::new(url.as_str()), scan_args.clone())
                .expect("Falha ao iniciar scan do URL")
        })
        .collect();

    let combined_lf = concat(lazy_frames, UnionArgs::default())?;

    let df = combined_lf
        .collect()?;

    println!("{:?}", df);

    Ok(())
}


async fn get_ons_curtailment_links(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<HashMap<DateTime<Utc>, String>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    let mut ons_links: HashMap<DateTime<Utc>, String> = HashMap::new();
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

                let specific_date = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).single().expect("invalid datetime parsing!");

                if specific_date >= start_date && specific_date <= end_date {
                    ons_links.insert(specific_date, valid_href.to_string());
                }
            }
        }
    }
    
    Ok(ons_links)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // todo!("Adicionar filtros por CEG");
    let start_date: DateTime<Utc> = Utc.with_ymd_and_hms(2025, 5, 1, 0, 0, 0).single().expect("invalid start_date!");
    let end_date: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).single().expect("invalid start_date!");

    // let args: Vec<String> = env::args().collect();
    // println!("My path is {:?}.", args);

    let ons_curtailment_links: HashMap<DateTime<Utc>, String> = get_ons_curtailment_links(start_date, end_date).await?;

    get_data_file(ons_curtailment_links.into_values().collect()).await?;

    Ok(())
}


