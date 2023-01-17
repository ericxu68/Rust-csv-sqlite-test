use csv;
use std::error::Error;
use std::cell::Cell;
use crate::Input;
use chrono::prelude::*;
use crate::PrayTime;
use crate::Hold;
use crate::TimeLimit;



fn parse_time(time:&str)->Result<DateTime<FixedOffset>,Box<dyn Error>> {
    Ok(DateTime::parse_from_str(time, "%d-%m-%Y %H:%M:%S %z")?)
}
fn parse_limit(time:&str)->u32{
    let parsed = time.split(":").map(|e|e.parse::<u32>().unwrap()).collect::<Vec<_>>();
    parsed[0]*60+parsed[1]
}
pub fn new_hold()->Hold{
    Hold { holder: Vec::new() }
}

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
impl Hold{
    fn report_in(&mut self,pin:String,name:String
        ,date:(u32,u32,u32),pray:PrayTime,db_date:String){
        let iter = &mut self.holder.iter().filter(|&e|&e.pin==&pin);
        let mut cell:Vec<crate::PrayHold>;
        if iter.clone().count()==0{
            //create new user if no user yet
            cell = vec![crate::PrayHold{
                pray,date,db_date
            }];
            self.holder.push(crate::Holder{
                name,pin,
                pray:Cell::new(cell)
            })
        }else{
            cell = iter.clone().next().unwrap().pray.take();
            if &cell.iter().filter(|&e|&e.date==&date&&e.pray==pray.clone()).count() == &0{
                //only add record if at same day no double praytime
                cell.push(crate::PrayHold{
                    pray,date,db_date
                });
            }
            iter.next().unwrap().pray.set(cell)
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

// pub fn csv2database(path: &str,tl:&TimeLimit) -> Result<(),Box<dyn Error>> {
//     let mut reader = csv::Reader::from_path(path)?;
//     let mut err_count:usize = 0;
//     for i in reader.records(){
//         //first error read check
//         if i.is_err(){
//             err_count += 1;
//             continue;
//         }
//         let record = i.unwrap();
//         //second error parse check
//         let deserialize:Result<Input,csv::Error> = record.deserialize(None);
//         if deserialize.is_err(){
//             err_count += 1;
//             continue;
//         }
//         let mut out:Hold = Vec::new();
//     }
//     Ok(())
// }

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
    #[test]
    fn test_hold() {
        let mut hold = new_hold();
        hold.report_in("123".to_string(), "idk".to_string(), (1,1,1), PrayTime::Duhur,"somestring".to_string());
        assert_eq!(hold.holder[0].pin,"123".to_string());
        assert_eq!(hold.holder[0].pray.get_mut()[0].pray,PrayTime::Duhur );
    }
}
