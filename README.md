# curtail-ons-rs (Português)

Esta é uma ferramenta de linha de comando para baixar dados de restrição de geração do Operador Nacional do Sistema Elétrico (ONS) e salvá-los como um arquivo Parquet.

## Como usar

Para usar esta ferramenta, você pode executar o programa com datas de início e fim opcionais.

```bash
cargo run -- [data_de_início] [data_de_fim]
```

- `data_de_início`: Opcional. A data de início no formato `AAAA-MM`. Se não for fornecida, o padrão é o primeiro dia do mês atual.
- `data_de_fim`: Opcional. A data de fim no formato `AAAA-MM`. Se não for fornecida, o padrão é a data atual.

### Exemplos

- Executar com as datas padrão (mês atual):
  ```bash
  cargo run
  ```

- Executar com um intervalo de datas específico:
  ```bash
  cargo run -- 2023-01 2023-03
  ```

O arquivo de saída será salvo no diretório `output`, com um nome de arquivo como `curtailment_AAAA-MM-DD_AAAA-MM-DD.parquet`.

---

# curtail-ons-rs (English)

This is a command-line tool to download curtailment data from the Brazilian National System Operator (ONS) and save it as a Parquet file.

## How to use

To use this tool, you can run the program with optional start and end dates.

```bash
cargo run -- [start_date] [end_date]
```

- `start_date`: Optional. The start date in `YYYY-MM` format. If not provided, it defaults to the first day of the current month.
- `end_date`: Optional. The end date in `YYYY-MM` format. If not provided, it defaults to the current date.

### Examples

- Run with default dates (current month):
  ```bash
  cargo run
  ```

- Run with a specific date range:
  ```bash
  cargo run -- 2023-01 2023-03
  ```

The output will be saved in the `output` directory, with a filename like `curtailment_YYYY-MM-DD_YYYY-MM-DD.parquet`.
