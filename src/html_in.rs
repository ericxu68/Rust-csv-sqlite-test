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

    #[test]
    fn test_html() {
        let input = std::fs::read_to_string("./26 Sep - 20 Okt 22.html").unwrap();
        let rows = find_table(&input);
        for i in &rows.unwrap(){
            assert_eq!(14,i.len());
        }
    }
}
