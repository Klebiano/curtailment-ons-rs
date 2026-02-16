# curtailment-ons-rs (Português)

Esta é uma ferramenta de linha de comando para baixar dados de Restrição de Operação por Constrained-off de Usinas Eólicas do Operador Nacional do Sistema Elétrico (ONS) e salvá-los como um arquivo Parquet.

## Como usar

### Compilar e Executar

1.  **Clone o repositório:**
    ```bash
    git clone https://github.com/Klebiano/curtailment-ons-rs.git
    cd curtailment-ons-rs
    ```

2.  **Compile o projeto:**
    Compilar em modo release para criar um executável mais leve e otimizado.
    ```bash
    cargo build --release
    ```
    O executável estará localizado em `target/release/curtailment-ons-rs`.

3.  **Execute o programa:**
    Você pode executar o programa com datas de início e fim opcionais.

    ```bash
    ./target/release/curtailment-ons-rs [data_de_início] [data_de_fim]
    ```

- `data_de_início`: Opcional. A data de início no formato `AAAA-MM`. Se não for fornecida, o padrão é o primeiro dia do mês atual.
- `data_de_fim`: Opcional. A data de fim no formato `AAAA-MM`. Se não for fornecida, o padrão é a data atual.

### Exemplos

- Executar com as datas padrão (mês atual):
  ```bash
  ./target/release/curtailment-ons-rs
  ```

- Executar com um intervalo de datas específico:
  ```bash
  ./target/release/curtailment-ons-rs 2025-01 2026-02
  ```

O arquivo de saída será salvo no diretório `output`, com um nome de arquivo como `curtailment_AAAA-MM-DD_AAAA-MM-DD.parquet`.

---
# curtailment-ons-rs (English)

This is a command-line tool to download curtailment data from the Brazilian National System Operator (ONS) and save it as a Parquet file.

## How to use

### Build and Run

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Klebiano/curtailment-ons-rs.git
    cd curtailment-ons-rs
    ```

2.  **Build the project:**
    Building in release mode to create a lighter and optimized executable.
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/curtailment-ons-rs`.

3.  **Run the executable:**
    You can run the program with optional start and end dates.

    ```bash
    ./target/release/curtailment-ons-rs [start_date] [end_date]
    ```

- `start_date`: Optional. The start date in `YYYY-MM` format. If not provided, it defaults to the first day of the current month.
- `end_date`: Optional. The end date in `YYYY-MM` format. If not provided, it defaults to the current date.

### Examples

- Run with default dates (current month):
  ```bash
  ./target/release/curtailment-ons-rs
  ```

- Run with a specific date range:
  ```bash
  ./target/release/curtailment-ons-rs 2025-01 2026-02
  ```

The output will be saved in the `output` directory, with a filename like `curtailment_YYYY-MM-DD_YYYY-MM-DD.parquet`.
