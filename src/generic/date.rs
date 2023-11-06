use chrono::NaiveDate;

pub fn today_to_string (year: i32, month: u32, day: u32 ) -> String {
  let date: NaiveDate = NaiveDate::from_ymd_opt(year, month, day).unwrap();
  date.format("%m/%d/%Y").to_string()
}

#[cfg(test)]
mod tests {
  use super::today_to_string;

  #[test]
  fn test_today_to_string() {
    let response = today_to_string(2000, 1, 1);
    assert_eq!(response, "01/01/2000".to_string());
  }
}