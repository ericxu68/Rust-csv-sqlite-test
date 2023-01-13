use serde::Deserialize;

pub mod csv_in;

#[derive(Debug,Deserialize,Clone,PartialEq)]
struct Input<'a>{
    date_full: &'a str,
    date: &'a str,
    clock: &'a str,
    pin: &'a str,
    nip: &'a str,
    name: &'a str,
    occupation: Option<&'a str>,
    departement: Option<&'a str>,
    office : Option<&'a str>,
    verivication : &'a str,
    io :&'a str,
    workcode : &'a str,
    sn :&'a str,
    machine: &'a str
}

fn main() {
    let res = csv_in::input_csv("./26 Sep - 20 Okt 22.csv");
    if res.is_ok(){
        println!("success");
    }else {
        println!("{}",res.unwrap_err());
    }
}
