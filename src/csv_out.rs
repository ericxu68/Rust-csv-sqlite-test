use crate::Hold;
use chrono::prelude::*;


impl Hold {
    fn get_range(&self)->(String,String){
        //Start-Finish
        let mut start:u32=999999999;
        let mut finish:u32=0;
        let mut out_start:String=String::new();
        let mut out_finish:String=String::new();
        for i in self.holder.iter().map(|e|e.pray.borrow()){
            for j in i.iter().map(|e|e.db_date){
                let u32_year = j.year() as u32;
                let date =u32_year*372+j.month()*31+j.day();
                if date < start {
                    start = date;
                    out_start = j.to_string().split(" ").next().unwrap().to_owned();
                }
                if date > finish{
                    finish = date;
                    out_finish = j.to_string().split(" ").next().unwrap().to_owned();
                }
            }
        }
        (out_start,out_finish)
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::csv_in::parse_time;
    use crate::PrayTime;

    #[test]
    fn test_range() {
        let mut hold = crate::csv_in::new_hold();
        let date = parse_time("18-10-2022 15:02:45 +07:00").unwrap();
        hold.report_in("123".to_string(), "idk".to_string(), (1,1,1), PrayTime::Duhur,date.to_owned(),"term".to_owned());
        let idk = hold.get_range();
        assert_eq!(idk.1,"2022-10-18");
        assert_eq!(idk.0,"2022-10-18");
    }
}
