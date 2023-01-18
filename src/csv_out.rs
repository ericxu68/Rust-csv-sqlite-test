use crate::Hold;
use crate::Holder;
use crate::PrayTime;
use chrono::prelude::*;
use csv;
use std::error::Error;
use std::time::Instant;


impl Hold {
    pub fn get_range(&self)->(String,String){
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
    pub fn get_machine(&self)->Vec<String>{
        let mut out:Vec<String> = Vec::new();
        for i in self.holder.iter().map(|e|e.pray.borrow()){
            for j in i.iter().map(|e|e.machine.clone()){
                if !out.contains(&j){
                    out.push(j)
                }
            }
        }
        out
    }
    fn ordered(&mut self,machine:&str){
        self.holder.sort_by(|a,b|b.total_only(machine).cmp(&a.total_only(machine)))
    }
    pub fn direct_csv(&mut self,machine:&str,path:&str)->Result<(),Box<dyn Error>>{
        let now = Instant::now();
        let total = self.holder.len();
        println!("--------------------- Start -------------------------");
        let mut writer = csv::Writer::from_path(path)?;
        writer.write_record(&["name", "pin","perusahaan",
            "devisi", "duhur", "asyar", "maghrib","isya",
            "subuh","tahajud","total"])?;
        self.ordered(machine);
        println!("ordered {total} list");
        print!("\n");
        for i in &self.holder{
            print!("writing {} data",i.name);
            print!("\r");
            let tot = i.sholat_parser(machine);
            let pin = i.pin_parser();
            writer.write_record(&[
                i.name.as_str(),
                i.pin.as_str(),
                pin.0.as_str(),
                pin.1.as_str(),
                &tot.duhur.to_string(),
                &tot.asyar.to_string(),
                &tot.maghrib.to_string(),
                &tot.isya.to_string(),
                &tot.subuh.to_string(),
                &tot.tahajud.to_string(),
                &tot.total.to_string()
            ])?;
        }
        println!("-----------------------------------------------------");
        println!("*********************  FINISHED *********************");
        println!("-----------------------------------------------------");
        println!("Processing speed = {:.2?}",now.elapsed());
        println!("writen data with machine = {} on {}",machine,path);
        Ok(())
    }
}

#[derive(Debug)]
struct PrayCount {
    total: usize,
    duhur:usize,
    asyar:usize,
    maghrib:usize,
    isya:usize,
    subuh:usize,
    tahajud:usize
}


impl Holder {
    fn pin_parser(&self)->(String,String){
        //company-devision
        let mut pin = self.pin.to_owned();
        if pin.len() < 9 || pin.len() >10{
        return ("Invalid".to_owned(),"Invalid".to_owned());
        }else if pin.len() ==9{
            pin = ["0",pin.as_str()].concat()
        }
        let pr = &pin[0..2].parse::<u16>().unwrap_or(99);
        let dv = &pin[2..4].parse::<u16>().unwrap_or(99);
        (company(*pr),devisi(*dv))
    }
    fn sholat_parser(&self,machine:&str)->PrayCount{
        let raw = self.pray.borrow();
        let pray = raw.iter().filter(|e|&e.machine==machine).collect::<Vec<_>>();
        PrayCount {
            total: pray.len(),
            duhur: pray.iter().filter(|e|e.pray==PrayTime::Duhur).count(),
            asyar: pray.iter().filter(|e|e.pray==PrayTime::Asyar).count(),
            maghrib: pray.iter().filter(|e|e.pray==PrayTime::Maghrib).count(),
            isya: pray.iter().filter(|e|e.pray==PrayTime::Isya).count(),
            subuh: pray.iter().filter(|e|e.pray==PrayTime::Subuh).count(),
            tahajud: pray.iter().filter(|e|e.pray==PrayTime::Tahajud).count(),
        }
    }
    fn total_only(&self,machine:&str)->usize{
        let raw = self.pray.borrow();
        let pray = raw.iter().filter(|e|&e.machine==machine).collect::<Vec<_>>();
        pray.len()
    }
}
fn company(pin:u16)->String{
    match pin {
        1=>"PT. POLOWIJO GOSARI INDONESIA  ( PGI & PG )".to_string(),
        2=>"PT. PUPUK KARYA POLOWIJO  ( PKP )".to_string(),
        3=> "PT. GUNUNG EMAS PUTIH  ( GEP )".to_string(),
        4=> "PT. SARI GUNUNG POLOWIJO  ( SGP )".to_string(),
        5=>"PT. POLOWIJO GRAHA NIAGA  ( PGN )".to_string(),
        6=>"PT. GALASARI GUNUNG SEJAHTERA  ( GGS )".to_string(),
        7=>"PT. GALASARI AGRO NIAGA SEJAHTERA  ( GANIS )".to_string(),
        8=>"PT. DIPO INVESTAMA INDONESIA".to_string(),
        9=>"PT. BUMI SARI TEKNOLOGI".to_string(),
        10=>"PT. INTISARI MEDIA NUSANTARA".to_string(),
        11=>"YAYASAN HARFIN GOSARI".to_string(),
        12=>"MASJID AKBAR MOED'HAR ARIFIN".to_string(),
        13=>"HARFIN MART".to_string(),
        14=>"PT. MAGNESIUM GOSARI INTERNASIONAL  ( MGI )".to_string(),
        15=>"GUEST HOUSE".to_string(),
        _=>"UNIDENTIFIED".to_string()
    }
}
fn devisi(pin:u16)->String{
    match pin {
         1=>"BOD / BOC / STAF AHLI".to_string(),
         2=>"FAT".to_string(),
         3=>"HRGA / LEGAL / SAFETY / RUMAH TANGGA".to_string(),
         4=>"HUMAS / IT / SEKRETARIAT / PA".to_string(),
         5=>"SECURITY / WAKER".to_string(),
         6=>"DRIVER OPERASIONAL / P5".to_string(),
         7=>"PRODUKSI / OPERASIONAL / CHEKEER".to_string(),
         8=>"MAINTENANCE / MEKANIK / WELDER / FUELMAN".to_string(),
         9=>"CLEANING SERVICE / HOUSE KEEPING".to_string(),
         10=>"OPERATOR ALAT BERAT".to_string(),
         11=>"CUSTOMER SERVICE".to_string(),
         12=>"PPIC / PROCUREMENT".to_string(),
         13=>"LABORATORIUM / QA".to_string(),
         14=>"RISET & DEVELOPMENT".to_string(),
         15=>"MARKETING".to_string(),
         16=>"KANTOR JAKARTA".to_string(),
         17=>"HARIAN".to_string(),
         18=>"PRAMUNIAGA".to_string(),
         _=>"UNIDENTIFIED".to_string()
    }
}
#[cfg(test)]
mod testing {
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
    #[test]
    fn test_machine() {
        let mut hold = crate::csv_in::new_hold();
        let date = parse_time("18-10-2022 15:02:45 +07:00").unwrap();
        hold.report_in("123".to_string(), "idk".to_string(), (1,1,1), PrayTime::Duhur,date.to_owned(),"term".to_owned());
        assert_eq!(hold.get_machine(),vec!["term".to_owned()]);
    }
    #[test]
    fn test_pin() {
        let mut hold = crate::csv_in::new_hold();
        let date = parse_time("18-10-2022 15:02:45 +07:00").unwrap();
        hold.report_in("123".to_string(), "idk".to_string(), (1,1,1), PrayTime::Duhur,date.to_owned(),"term".to_owned());
        let pin = hold.holder[0].pin_parser();
        assert_eq!(pin.0,"Invalid".to_owned())
    }
}
