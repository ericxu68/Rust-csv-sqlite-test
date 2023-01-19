#![allow(dead_code)]
use chrono::FixedOffset;
use serde::Deserialize;
use std::cell::RefCell;

pub mod csv_in;
pub mod sql_init;
pub mod csv_out;
pub mod html_in;

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

#[derive(Debug,PartialEq, Clone,Eq,PartialOrd, Ord)]
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

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Hold {
    holder: Vec<Holder>
}
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct Holder {
    pin: String,
    name: String,
    pray: RefCell<Vec<PrayHold>>
}
#[derive(Debug,PartialEq,Eq,PartialOrd, Ord)]
pub struct PrayHold {
    date: (u32,u32,u32),
    pray:PrayTime,
    db_date:chrono::DateTime<FixedOffset>,
    machine:String
}
#[derive(Debug)]
struct CSVOUT {
    pin: String,
    name: String,
    date: (u32,u32,u32),
    pray: PrayTime,
    db_date: chrono::DateTime<FixedOffset>,
    machine: String
}

static mut CACHE:Hold=Hold{holder:Vec::new()};

fn main() {
    // let pray:TimeLimit = TimeLimit { 
    //     duhur_s: "11:00".to_owned(),
    //     duhur_f: "13:30".to_owned(),
    //     asyar_s: "14:00".to_owned(),
    //     asyar_f: "16:30".to_owned(),
    //     maghrib_s: "18:00".to_owned(),
    //     maghrib_f: "18:30".to_owned(),
    //     isya_s: "18:31".to_owned(),
    //     isya_f: "19:30".to_owned(),
    //     subuh_s: "03:30".to_owned(),
    //     subuh_f: "05:30".to_owned(),
    //     tahajud_s: "02:00".to_owned(),
    //     tahajud_f: "03:20".to_owned()
    // };
    // let hold = csv_in::csv2database("./26 Sep - 20 Okt 22.csv",&pray);
    // if hold.is_ok(){
    //     unsafe{
    //         CACHE = hold.unwrap();
    //         let machine = CACHE.get_machine();
    //         println!("{:?}",machine);
    //         CACHE.direct_csv(machine[0].as_str(), "./out.csv").unwrap();
    //     }
    // }
    let idk = html_in::find_table(std::fs::read_to_string("./26 Sep - 20 Okt 22.html").unwrap().as_str());
    for i in idk.unwrap(){
        for j in i{
            println!("{}",j)
        }
    }
}
