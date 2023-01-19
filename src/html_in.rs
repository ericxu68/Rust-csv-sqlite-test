use scraper::{Html,ElementRef,Selector};
use crate::csv_in::{new_hold,parse_time,date2tuple};
use crate::{Hold,TimeLimit,CSVOUT};
use std::fs::read_to_string;
use std::error::Error;
use std::time::Instant;



fn get_same_csv_out(tl:&TimeLimit,date:&str,name:String,pin:String,machine:String)->Option<CSVOUT>{
    match date2tuple(date){
        Ok(d)=>{
            match tl.validator(date){
                Some(t)=> Some(CSVOUT { 
                    pin,
                    name, 
                    date: d, 
                    pray: t, 
                    db_date: parse_time([date," +07:00"].concat().as_str()).unwrap(), 
                    machine
                }),
                None => None
            }
        }
        Err(_)=>None
    }
}


pub fn find_table(http_data: &str) -> Option<Vec<Vec<String>>> {
    let css = |selector| Selector::parse(selector).unwrap();
    let get_unwrap_cells = |row:ElementRef,selector,wraper|{
        row.select(&css(selector)).map(|wrap|wrap.select(&css(wraper)).next().unwrap()
        .inner_html().trim().to_string()).collect()
    };
    let html = Html::parse_fragment(http_data);
    let table_out = html.select(&css("table")).next()?;
    let tbody = css("tbody");
    let table = table_out.select(&tbody).next()?;
    let tr = css("tr");
    let mut rows = table.select(&tr);
    rows.next();
    rows.next();
    let rows: Vec<_> = rows.map(|row| get_unwrap_cells(row, "td","font")).collect();
    Some(rows)
}
pub fn html2hold(path:&str,tl:&TimeLimit)->Result<Hold,Box<dyn Error>>{
    let now = Instant::now();
    println!("************************ START *************************");
    let html_data = read_to_string(path)?;
        let css = |selector| Selector::parse(selector).unwrap();
    let get_unwrap_cells = |row:ElementRef,selector,wraper|{
        row.select(&css(selector)).map(|wrap|wrap.select(&css(wraper)).next().unwrap()
        .inner_html().trim().to_string()).collect()
    };
    let mut err_count:usize = 0;
    let mut invalid_count:usize = 0;
    let mut data_count:usize = 0;
    let mut double_count:usize = 0;
    let mut valid_count:usize = 0;
    let mut hold = new_hold();
    let html = Html::parse_fragment(&html_data);
    let table_out = html.select(&css("table")).next().unwrap();
    let tbody = css("tbody");
    let table = table_out.select(&tbody).next().unwrap();
    let tr = css("tr");
    let mut rows = table.select(&tr);
    rows.next();
    rows.next();
    for row in rows{
        data_count += 1;
        let cell:Vec<_> = get_unwrap_cells(row,"td","font");
        if cell.len() != 14 {
            err_count += 1;
            continue;
        }
        match get_same_csv_out(tl, &cell[0], cell[5].to_owned(), cell[3].to_owned(), cell[13].to_owned()){
            Some(d)=>{
                match hold.report_in(d.pin, d.name.to_owned(), d.date, d.pray, d.db_date, d.machine){
                    true =>valid_count += 1,
                    false =>double_count += 1
                }
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
mod testing{
    use super::*;

    static HTMLTEST:&str = r#"<BODY BGCOLOR=#C0C0C0>
<TABLE BORDER=0 CELLSPACING=1 CELLPADDING=2 BGCOLOR=#C0C0C0>
<TR VALIGN="TOP" class="Band" BGCOLOR=#F0FBFF><TD NOWRAP COLSPAN=14 ALIGN="CENTER" HEIGHT=21>&nbsp;</TD></TR>
<TR VALIGN="TOP" class="Header" BGCOLOR=#F0FBFF><TD NOWRAP WIDTH=111 ALIGN="CENTER">Tanggal scan</TD><TD NOWRAP WIDTH=66 ALIGN="CENTER">Tanggal</TD><TD NOWRAP WIDTH=52 ALIGN="CENTER">Jam</TD><TD NOWRAP WIDTH=70 ALIGN="CENTER">PIN</TD><TD NOWRAP WIDTH=70 ALIGN="CENTER">NIP</TD><TD NOWRAP WIDTH=157 ALIGN="CENTER">Nama</TD><TD NOWRAP WIDTH=65 ALIGN="CENTER">Jabatan</TD><TD NOWRAP WIDTH=102 ALIGN="CENTER">Departemen</TD><TD NOWRAP WIDTH=79 ALIGN="CENTER">Kantor</TD><TD NOWRAP WIDTH=72 ALIGN="CENTER">Verifikasi</TD><TD NOWRAP WIDTH=43 ALIGN="CENTER">I/O</TD><TD NOWRAP WIDTH=77 ALIGN="CENTER">Workcode</TD><TD NOWRAP WIDTH=94 ALIGN="CENTER">SN</TD><TD NOWRAP WIDTH=84 ALIGN="CENTER">Mesin</TD></TR>
<TR><TD NOWRAP ALIGN="CENTER" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">18-10-2022 15:02:45</FONT></TD><TD NOWRAP ALIGN="CENTER" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">18-10-2022</FONT></TD><TD NOWRAP ALIGN="CENTER" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">15:02:45</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">3</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">1005</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">Mustasimul Hanun</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">&nbsp;</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">CL</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">Masjid</FONT></TD><TD NOWRAP ALIGN="RIGHT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">1</FONT></TD><TD NOWRAP ALIGN="RIGHT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">1</FONT></TD><TD NOWRAP ALIGN="RIGHT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">0</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">61629018250887</FONT></TD><TD NOWRAP ALIGN="LEFT" BGCOLOR=#FFFFFF><FONT STYLE="font-family: Arial; font-size: 8pt; color: #000000">Mesin 2</FONT></TD></TR></TABLE>
</BODY>"#;
    #[test]
    fn test_html() {
        let rows = find_table(&HTMLTEST);
        for i in &rows.unwrap(){
            assert_eq!(14,i.len());
        }
    }
}
