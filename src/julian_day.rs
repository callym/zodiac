use chrono::{ Datelike, Timelike };

#[derive(Copy, Clone, Debug)]
pub struct JulianDay(f64);

impl JulianDay {
  pub fn raw(raw: f64) -> Self {
    JulianDay(raw)
  }
}

fn jd<T: Datelike>(date: &T) -> f64 {
  let (year, month, day) = {
    let mut year = date.year();
    let mut month = date.month();
    let day = date.day();

    if month == 1 || month == 2 {
      year -= 1;
      month += 12;
    }
    (year as f64, month as f64, day as f64)
  };

  let a = (year / 100.0) as i32 as f64;
  let b = (a / 4.0) as i32 as f64;
  let c = 2.0 - a + b;
  let e = (365.25 * (year + 4716.0)) as i32 as f64;
  let f = (30.6001 * (month + 1.0)) as i32 as f64;

  c + day + e + f - 1524.5
}

impl<T> From<T> for JulianDay where T: Datelike {
  default fn from(date: T) -> Self {
    JulianDay(jd(&date))
  }
}

impl<T> From<T> for JulianDay where T: Datelike + Timelike {
  fn from(date: T) -> Self {
    let jd = jd(&date);

    let total_secs = 60.0 * 60.0 * 24.0;
    let secs = date.num_seconds_from_midnight() as f64;

    let percent = secs / total_secs;

    JulianDay(jd + percent)
  }
}

impl Into<f64> for JulianDay {
  fn into(self) -> f64 {
    self.0
  }
}
