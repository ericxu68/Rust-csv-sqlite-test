use csv;
use std::error::Error;
use std::cell::RefCell;
use std::time::Instant;
use crate::Input;
use chrono::prelude::*;
use crate::PrayTime;
use crate::Hold;
use crate::TimeLimit;



pub fn parse_time(time:&str)->Result<DateTime<FixedOffset>,Box<dyn Error>> {
    //change string to datetime
    Ok(DateTime::parse_from_str(time, "%d-%m-%Y %H:%M:%S %z")?)
}
fn parse_limit(time:&str)->u32{
    //change string time to minute integer
    let mut parsed = time.split(":").map(|e|e.parse::<u32>().unwrap());
    parsed.next().unwrap()*60+parsed.next().unwrap()
}
fn date2tuple(time:&str)->Result<(u32,u32,u32),Box<dyn Error>>{
    // day-month-year
    let input = parse_time([time," +07:00"].concat().as_str())?;
    Ok((input.day() as u32,input.month() as u32,input.year() as u32))
}
pub fn new_hold()->Hold{
    //init new struct hold
    Hold { holder: Vec::new() }
}

impl TimeLimit{
    fn validator(&self,time:&str)->Option<PrayTime>{
        //get enum pray time from input
        let parsed;
        match parse_time(&[time," +07:00"].concat()){
            Ok(d)=> parsed = d,
            Err(_)=>return None
        };
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
        //get praytime name
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
    pub fn report_in(&mut self,pin:String,name:String,date:(u32,u32,u32)
        ,pray:PrayTime,db_date:DateTime<FixedOffset>,machine:String)->bool{
        //add valid input record methode
        let iter = &mut self.holder.iter().filter(|&e|&e.pin==&pin);
        let mut cell:Vec<crate::PrayHold>;
        if iter.clone().count()==0{
            //create new user if no user yet
            cell = vec![crate::PrayHold{pray,date,db_date,machine}];
            self.holder.push(crate::Holder{
                name,pin,
                pray:RefCell::new(cell)
            });
        }else{
            cell = iter.clone().next().unwrap().pray.take();
            if &cell.iter().filter(|&e|&e.date==&date&&e.pray==pray.clone()).count() == &0{
                //only add record if at same day no double praytime
                cell.push(crate::PrayHold{pray,date,db_date,machine});
                iter.next().unwrap().pray.replace(cell);
            }else {
                iter.next().unwrap().pray.replace(cell);
                return false;
            }
        }
        true
    }
}
impl Input{
    fn valid_out(&self,time:&TimeLimit)->Option<crate::CSVOUT>{
        match date2tuple(&self.date_full) {
            Ok(d) =>{
                match time.validator(&self.date_full) {
                    Some(t) => Some(crate::CSVOUT{
                        name:self.name.to_owned(),
                        pin:self.pin.to_owned(),
                        pray:t,
                        db_date:parse_time([self.date_full.as_str()," +07:00"].concat().as_str()).unwrap(),
                        date:d,
                        machine:self.machine.to_owned()
                    }),
                    None => None,
                }
            },
            Err(_) => None,
        }
    }
}
fn input_csv(path: &str) -> Result<(),Box<dyn Error>> {
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

pub fn csv2database(path: &str,tl:&TimeLimit) -> Result<Hold,Box<dyn Error>> {
    let now = Instant::now();
    let mut reader = csv::Reader::from_path(path)?;
    let mut err_count:usize = 0;
    let mut valid_count:usize = 0;
    let mut invalid_count:usize = 0;
    let mut data_count:usize = 0;
    let mut double_count = 0;
    let mut hold = new_hold();
    for i in reader.records(){
        data_count += 1;
        //first error read check
        if i.is_err(){
            err_count += 1;
            continue;
        }
        let record = i.unwrap();
        //second error parse check
        let deserialize:Result<Input,csv::Error> = record.deserialize(None);
        if deserialize.is_err(){
            err_count += 1;
            continue;
        }
        match deserialize?.valid_out(tl){
            Some(t)=>{
                match hold.report_in(t.pin, t.name.to_owned(), t.date, t.pray, t.db_date, t.machine){
                    true=>valid_count += 1,
                    false=>double_count += 1
                };
                println!("parsed {} data",t.name);
            }
            None=> invalid_count += 1
        };
    }
    println!("-----------------------------------------------------");
    println!("*********************  FINISHED *********************");
    println!("-----------------------------------------------------");
    println!("Processing speed = {:.2?}",now.elapsed());
    println!("Total Data Processed = {data_count}");
    println!("Error Count = {err_count}");
    println!("Invalid Data Count = {invalid_count}");
    println!("Double Data Count = {double_count}");
    println!("Valid Data Count = {valid_count}");
    Ok(hold)
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
        assert_eq!(parsed.hour(),15);
        assert_eq!(parsed.minute(),2);
        assert_eq!(parsed.day(),18);
        assert_eq!(parsed.month(),10);
        assert_eq!(parsed.year(),2022)
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
        assert_eq!(pray.validator("18-10-2022 15:02:45"),Some(PrayTime::Asyar));
        assert_eq!(pray.validator("18-10-2022 15:42:15"),None);
        assert_eq!(pray.validator("18-10-2022 03:42:45"),None);
        assert_eq!(pray.validator("18-10-2022 03:30:45"),Some(PrayTime::Tahajud));
    }
    #[test]
    fn test_name() {
        assert_eq!(PrayTime::Duhur.get_name(),"duhur")
    }
    #[test]
    fn test_hold() {
        let mut hold = new_hold();
        let date = parse_time("18-10-2022 15:02:45 +07:00").unwrap();
        hold.report_in("123".to_string(), "idk".to_string(), (1,1,1), PrayTime::Duhur,date.to_owned(),"term".to_owned());
        assert_eq!(hold.holder[0].pin,"123".to_string());
        assert_eq!(hold.holder[0].pray.get_mut()[0].pray,PrayTime::Duhur );
        hold.report_in("123".to_string(), "hmm".to_string(), (1,1,1), PrayTime::Asyar, date.to_owned(),"term".to_owned());
        assert_eq!(hold.holder[0].pray.get_mut().len(),2 );
        hold.report_in("123".to_string(), "hmm".to_string(), (1,1,1), PrayTime::Asyar, date.to_owned(),"term".to_owned());
        assert_eq!(hold.holder[0].pray.get_mut().len(),2 );
        hold.report_in("123".to_string(), "hmm".to_string(), (1,1,2), PrayTime::Asyar, date.to_owned(),"term".to_owned());
        assert_eq!(hold.holder[0].pray.get_mut().len(),3 );
    }
}
