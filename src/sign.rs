use ::Coordinates;

#[derive(Copy, Clone, Debug)]
pub enum Sign {
	Aries,
	Taurus,
	Gemini,
	Cancer,
	Leo,
	Virgo,
	Libra,
	Scorpio,
	Sagittarius,
	Capricorn,
	Aquarius,
	Pisces,
}

impl Sign {
	pub fn from_coordinates(coord: &Coordinates) -> Self {
		match coord.lon as u32 {
			0 ... 30 => Sign::Aries,
			31 ... 60 => Sign::Taurus,
			61 ... 90 => Sign::Gemini,
			91 ... 120 => Sign::Cancer,
			121 ... 150 => Sign::Leo,
			151 ... 180 => Sign::Virgo,
			181 ... 210 => Sign::Libra,
			211 ... 240 => Sign::Scorpio,
			241 ... 270 => Sign::Sagittarius,
			271 ... 300 => Sign::Capricorn,
			301 ... 330 => Sign::Aquarius,
			331 ... 360 => Sign::Pisces,
			_ => panic!(),
		}
	}
}
