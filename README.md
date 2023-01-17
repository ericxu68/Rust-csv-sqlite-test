# What is this?
This Project is my personal training project for using Rust-lang

## My aim
- interface sqlite
- making clear IO from CSV
- Proper Unit test
- Proper Concurency Handling
- CLI Command

## Dependency
- SQLX
- CSV
- Tokio
- Chrono
- Serde

## Roadmap
* [x] Make Github Action Workflows
* [x] Using `Sqlx` for initialize Sqlite as driver
* [x] Making a new database if not exist automatically
* [x] Using `Sqlx` to query `.sql` file
* [x] Using `CSV` to read `.csv` file
* [x] Using `Serde` to Deserialize `.csv` Output
* [x] Using `Chrono` to get proper date format from `.csv`
* [ ] Using `Chrono` to filter serde's struct
* [ ] Using `sqlx` to write data on database with serde's sruct
* [ ] Using `Tokio` to get faster `sqlx` concurrent write queries
* [ ] Using `sqlx` to read database with dynamic CLI command as filter
* [ ] Using `Tokio` to get faster `sqlx` concurrent read queries
* [ ] Using `Tokio` to process huge array of sqlx out data for more readable format
* [ ] Using `CSV` to write `.csv` output data
