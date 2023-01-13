use csv;
use std::error::Error;
use crate::Input;

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
            Err(_) => "fail",
        };
        assert_eq!(res, "success");
    }
}
