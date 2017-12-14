use ::Coordinates;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
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
	pub fn emoji(&self) -> &'static str {
		match *self {
			Sign::Aries => "♈",
			Sign::Taurus => "♉",
			Sign::Gemini => "♊",
			Sign::Cancer => "♋",
			Sign::Leo => "♌",
			Sign::Virgo => "♍",
			Sign::Libra => "♎",
			Sign::Scorpio => "♏",
			Sign::Sagittarius => "♐",
			Sign::Capricorn => "♑",
			Sign::Aquarius => "♒",
			Sign::Pisces => "♓",
		}
	}

	pub fn from_coordinates(coord: &Coordinates) -> Self {
		match coord.lon as u32 {
			0 ... 29 => Sign::Aries,
			30 ... 59 => Sign::Taurus,
			60 ... 89 => Sign::Gemini,
			90 ... 119 => Sign::Cancer,
			120 ... 149 => Sign::Leo,
			150 ... 179 => Sign::Virgo,
			180 ... 209 => Sign::Libra,
			210 ... 239 => Sign::Scorpio,
			240 ... 269 => Sign::Sagittarius,
			270 ... 299 => Sign::Capricorn,
			300 ... 329 => Sign::Aquarius,
			330 ... 359 => Sign::Pisces,
			_ => panic!(),
		}
	}
}
