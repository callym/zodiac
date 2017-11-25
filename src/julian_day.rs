use chrono::{ Datelike, Date, DateTime, FixedOffset };

#[derive(Copy, Clone, Debug)]
pub struct JulianDay(f64);

impl From<Date<FixedOffset>> for JulianDay {
	fn from(date: Date<FixedOffset>) -> Self {
		let datetime = date.and_hms(0, 0, 0);
		datetime.into()
	}
}

impl From<DateTime<FixedOffset>> for JulianDay {
	fn from(datetime: DateTime<FixedOffset>) -> Self {
		let (year, month, day) = {
			let mut year = datetime.year();
			let mut month = datetime.month();
			let day = datetime.day();

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

		let jd = c + day + e + f - 1524.0;

		JulianDay(jd)
	}
}

impl Into<f64> for JulianDay {
	fn into(self) -> f64 {
		self.0
	}
}
