use chrono::Datelike;
use vsop87::{ vsop87c, RectangularCoordinates };

use ::{ Coordinates, JulianDay, Sign };

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Planet {
	Sun,
	Moon,
	Mercury,
	Venus,
	Mars,
	Jupiter,
	Saturn,
	Uranus,
	Neptune,
	Pluto,
}

impl Planet {
	pub fn emoji(&self) -> &'static str {
		match *self {
			Planet::Sun => "☉",
			Planet::Moon => "☽",
			Planet::Mercury => "☿",
			Planet::Venus => "♀",
			Planet::Mars => "♂",
			Planet::Jupiter => "♃",
			Planet::Saturn => "♄",
			Planet::Uranus => "⛢",
			Planet::Neptune => "♆",
			Planet::Pluto => "♇",
		}
	}

	pub fn get_sign<T>(&self, date: T) -> Sign where T: Datelike {
		let coordinates = self.coordinates(date.into());
		Sign::from_coordinates(&coordinates)
	}

	pub fn coordinates(&self, julian_day: JulianDay) -> Coordinates {
		let earth = vsop87c::earth(julian_day.into());
		let planet = match *self {
			Planet::Sun => RectangularCoordinates { x: 0.0, y: 0.0, z: 0.0 },
			Planet::Moon => panic!(),
			Planet::Mercury => vsop87c::mercury(julian_day.into()),
			Planet::Venus => vsop87c::venus(julian_day.into()),
			Planet::Mars => vsop87c::mars(julian_day.into()),
			Planet::Jupiter => vsop87c::jupiter(julian_day.into()),
			Planet::Saturn => vsop87c::saturn(julian_day.into()),
			Planet::Uranus => vsop87c::uranus(julian_day.into()),
			Planet::Neptune => vsop87c::neptune(julian_day.into()),
			Planet::Pluto => panic!(),
		};

		let geocentric = RectangularCoordinates {
			x: planet.x - earth.x,
			y: planet.y - earth.y,
			z: planet.z - earth.z,
		};

		geocentric.into()
	}
}
