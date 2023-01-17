#![allow(dead_code)]
use serde::Deserialize;
use std::cell::Cell;

pub mod csv_in;
pub mod sql_init;

#[derive(Debug,Deserialize,Clone,PartialEq)]
pub struct Input{
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

#[derive(Debug,PartialEq, Clone)]
pub enum PrayTime {
    Duhur,
    Asyar,
    Maghrib,
    Isya,
    Subuh,
    Tahajud
}

#[derive(Debug)]
pub struct TimeLimit {
    duhur_s: String,
    duhur_f: String,
    asyar_s: String,
    asyar_f: String,
    maghrib_s: String,
    maghrib_f: String,
    isya_s: String,
    isya_f: String,
    subuh_s: String,
    subuh_f: String,
    tahajud_s: String,
    tahajud_f: String,
}

pub struct Hold {
    holder: Vec<Holder>
}
pub struct Holder {
    pin: String,
    name: String,
    pray: Cell<Vec<PrayHold>>
}
#[derive(Debug,PartialEq)]
pub struct PrayHold {
    date: (u32,u32,u32),
    pray:PrayTime,
    db_date:String
}
fn main() {
    let res = csv_in::input_csv("./26 Sep - 20 Okt 22.csv");
    if res.is_ok(){
        println!("success");
    }else {
        println!("{}",res.unwrap_err());
    }
}
