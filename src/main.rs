use serde::Deserialize;

pub mod csv_in;
pub mod sql_init;



#[derive(Debug,Deserialize,Clone,PartialEq)]
struct Input{
    date_full: String,
    date: String,
    clock: String,
    pin: String,
    nip: String,
    name: String,
    occupation: Option<String>,
    departement: Option<String>,
    office : Option<String>,
    verivication : String,
    io :String,
    workcode : String,
    sn :String,
    machine: String
}

fn main() {
    let res = csv_in::input_csv("./26 Sep - 20 Okt 22.csv");
    if res.is_ok(){
        println!("success");
    }else {
        println!("{}",res.unwrap_err());
    }
}
