use scraper::{Html,ElementRef,Selector};

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
