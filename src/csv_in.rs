use csv;
use std::error::Error;
use crate::Input;
use chrono::prelude::*;


#[allow(dead_code)]
#[derive(Debug)]
struct TimeLimit {
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

#[allow(dead_code)]
#[derive(Debug,PartialEq, Eq)]
enum PrayTime {
    Duhur,
    Asyar,
    Maghrib,
    Isya,
    Subuh,
    Tahajud
}

#[allow(dead_code)]
#[derive(Debug,PartialEq)]
struct Hold {
    pin: String,
    date: (u32,u32,u32),
    pray:PrayTime,
    db_date:String
}

#[allow(dead_code)]
fn parse_time(time:&str)->Result<DateTime<FixedOffset>,Box<dyn Error>> {
    Ok(DateTime::parse_from_str(time, "%d-%m-%Y %H:%M:%S %z")?)
}
#[allow(dead_code)]
fn parse_limit(time:&str)->u32{
    let parsed = time.split(":").map(|e|e.parse::<u32>().unwrap()).collect::<Vec<_>>();
    parsed[0]*60+parsed[1]
}

#[allow(dead_code)]
impl TimeLimit{
    fn validator(&self,time:&str)->Option<PrayTime>{
        let parsed  = parse_time(&[time," +07:00"].concat()).unwrap();
        let minute = parsed.hour()*60 + parsed.minute();
        if minute>=parse_limit(&self.duhur_s)&& minute<=parse_limit(&self.duhur_f){
            return Some(PrayTime::Duhur);
        }else if minute>=parse_limit(&self.asyar_s)&& minute<=parse_limit(&self.asyar_f){
            return Some(PrayTime::Asyar);
        }else if minute>=parse_limit(&self.maghrib_s)&& minute<=parse_limit(&self.maghrib_f){
            return Some(PrayTime::Maghrib);
        }else if minute>=parse_limit(&self.isya_s)&& minute<=parse_limit(&self.isya_f){
            return Some(PrayTime::Isya);
        }else if minute>=parse_limit(&self.subuh_s)&& minute<=parse_limit(&self.subuh_f){
            return Some(PrayTime::Subuh);
        }else if minute>=parse_limit(&self.tahajud_s)&& minute<=parse_limit(&self.tahajud_f){
            return Some(PrayTime::Tahajud);
        }
        None
    }
}

#[allow(dead_code)]
impl PrayTime{
    fn get_name(&self)->String{
        match self {
            PrayTime::Duhur=>"duhur".to_owned(),
            PrayTime::Isya=>"isya".to_owned(),
            PrayTime::Asyar=>"asyar".to_owned(),
            PrayTime::Subuh=>"subuh".to_owned(),
            PrayTime::Maghrib=>"maghrib".to_owned(),
            PrayTime::Tahajud=>"tahajud".to_owned()
        }
    }
}

pub fn input_csv(path: &str) -> Result<(),Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    for i in reader.records(){
        if i.is_err(){
            println!("error");
            continue;
        }
        let record = i.unwrap();
        let deserialize:Input  = record.deserialize(None)?;
        println!("{:?}",deserialize);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let res = match input_csv("./26 Sep - 20 Okt 22.csv") {
            Ok(()) => "success",
            Err(err) => {
                println!("{err}");
                "fail"
            }
        };
        assert_eq!(res, "success");
    }
    #[test]
    fn test_parse() {
        let parsed = parse_time("18-10-2022 15:02:45 +07:00").unwrap();
        assert_eq!(parsed.hour(),15)
    }
    #[test]
    fn test_limit() {
        let parsed = parse_limit("02:30");
        assert_eq!(parsed,150);
    }
    #[test]
    fn test_validator() {
        let pray:TimeLimit = TimeLimit { 
            duhur_s: "12:00".to_owned(),
            duhur_f: "12:30".to_owned(),
            asyar_s: "15:00".to_owned(),
            asyar_f: "15:30".to_owned(),
            maghrib_s: "18:00".to_owned(),
            maghrib_f: "18:30".to_owned(),
            isya_s: "19:00".to_owned(),
            isya_f: "19:30".to_owned(),
            subuh_s: "04:00".to_owned(),
            subuh_f: "04:30".to_owned(),
            tahajud_s: "03:00".to_owned(),
            tahajud_f: "03:30".to_owned()
        };
        assert_eq!(pray.validator("18-10-2022 15:02:45"),Some(PrayTime::Asyar))
    }
    #[test]
    fn test_name() {
        assert_eq!(PrayTime::Duhur.get_name(),"duhur")
    }
}
