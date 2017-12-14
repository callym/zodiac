use std::fmt;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Degrees(f64);

impl Degrees {
	pub fn new(decimal: f64) -> Self {
		Degrees(decimal)
	}

	pub fn minutes(&self) -> (f64, f64) {
		let d = self.0.floor();
		let m_t = (self.0 - d) * 60.0;
		let m = m_t.round();

		(d, m)
	}
}

impl fmt::Display for Degrees {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (d, m) = self.minutes();

        write!(f, "{}° {}'", d, m)
    }
}

impl Into<f64> for Degrees {
	fn into(self) -> f64 {
		self.0
	}
}
