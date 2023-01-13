use sqlx;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs::read_to_string;

pub async fn initialize(path: &str,script: &str) -> Result<(),sqlx::Error> {
    //create database if not exist
    if !std::path::Path::new(path).exists(){
        std::fs::File::create(path)?;
    }
    //make databas econnection pool
    let pool = SqlitePoolOptions::new().max_connections(5).connect(path).await?;
    //execute query file
    sqlx::query(&read_to_string(script).unwrap()).execute(&pool).await?;
    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_init() {
        let res = match initialize("./test.db", "./query/init.sql").await {
            Ok(()) => "success",
            Err(err) => {
                println!("{}",err);
                "failed on err"
            }
        };
        assert_eq!("success",res)
    }
}
