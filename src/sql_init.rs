use sqlx::{self, Pool, Sqlite, Row};
use sqlx::sqlite::SqlitePoolOptions;
use std::fs::read_to_string;
use std::path::Path;
use std::fs::File;
use sqlx::Error;

pub async fn initialize(path: &str,script: &str) -> Result<(),Error> {
    //create database if not exist
    if !Path::new(path).exists(){
        File::create(path)?;
    }
    //make databas econnection pool
    let pool = SqlitePoolOptions::new().max_connections(5).connect(path).await?;
    //execute query file
    sqlx::query(&read_to_string(script).unwrap()).execute(&pool).await?;
    Ok(())
}

pub async fn get_pool(path: &str) -> Result<Pool<Sqlite>,Error> {
    if !Path::new(path).exists(){
        File::create(path).unwrap();
    }
    SqlitePoolOptions::new().max_connections(10000).connect(path).await
}

pub async fn create_user(pool:&Pool<Sqlite>,name:&str,pin:&str)->Result<(),Error>{
    let query = format!("INSERT INTO user (pin,name) VALUES('{}','{}')",pin,name);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn get_all_user(pool:&Pool<Sqlite>)->Result<String,Error>{
    let rows = sqlx::query("SELECT * from user").fetch_all(pool).await?;
    let strings = rows.iter()
        .map(|e|format!("pin={}-name={}",e.get::<String,_>("pin"),e.get::<String,_>("name")))
        .collect::<Vec<String>>().join("\n");
    Ok(strings)
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

    #[tokio::test]
    async fn test_pool(){
        let res = match get_pool("./test.db").await{
            Ok(_)=>"success",
            Err(er)=>{
                println!("{er}");
                "fail"
            }
        };
        assert_eq!(res,"success")
    }
    // #[tokio::test]
    // async fn test_query(){
    //     test_init();
    //     let pool = get_pool("./test.db").await.unwrap();
    //     let res = match create_user(&pool, "hadziq", "123").await {
    //         Ok(()) => {
    //             match get_all_user(&pool).await{
    //                 Ok(r)=>{
    //                     println!("{r}");
    //                     "success"
    //                 }
    //                 Err(er)=>{
    //                     println!("{er}");
    //                     "fail"
    //                 }
    //             }
    //         },
    //         Err(er) => {
    //             println!("{er}");
    //             "fail"
    //         },
    //     };
    //     assert_eq!(res,"success")
    // }
}
