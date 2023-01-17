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
there is two path, one is using database and second use cache (unsafe)

### Common Route
* [x] Make Github Action Workflows
* [x] Using `CSV` to read `.csv` file
* [x] Using `Serde` to Deserialize `.csv` Output
* [x] Using `Chrono` to get proper date format from `.csv`
* [x] Using `Chrono` to filter serde's struct

### Database Route
* [x] Using `Sqlx` for initialize Sqlite as driver
* [x] Making a new database if not exist automatically
* [x] Using `Sqlx` to query `.sql` file
* [ ] Using `sqlx` to write data on database with serde's sruct
* [ ] Using `Tokio` to get faster `sqlx` concurrent write queries
* [ ] Using `sqlx` to read database with dynamic CLI command as filter
* [ ] Using `Tokio` to get faster `sqlx` concurrent read queries
* [ ] Using `Tokio` to process huge array of sqlx out data for more readable format
* [ ] Using `CSV` to write `.csv` output data
* [ ] banchmark all the process time

### Cache (unsafe) Route
* [x] safely initialize mutable static varable as cache
* [x] can modify and take ownership of cache to free its value
* [ ] using `CSV` to write `.csv` out from cache
* [ ] benchmark all the process time

