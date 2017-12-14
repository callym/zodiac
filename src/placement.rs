use std::hash::{ Hash, Hasher };
use ::{ Degrees, JulianDay, Planet, Sign };

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Placement {
	pub degrees: Degrees,
	pub planet: Planet,
	pub sign: Sign,
}

impl Placement {
	pub fn new<T>(planet: Planet, date: T) -> Self where T: Into<JulianDay> + Clone {
		let coord = planet.coordinates(date.clone());
		let sign = planet.get_sign(date.clone());

		let lon = coord.lon - sign.range().start as f64;

		Self {
			degrees: Degrees::new(lon),
			planet: planet,
			sign: sign,
		}
	}
}

impl Hash for Placement {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        (&self.planet, &self.sign).hash(state);
    }
}

impl PartialEq for Placement {
    fn eq(&self, other: &Self) -> bool {
        (&self.planet, &self.sign) == (&other.planet, &other.sign)
    }
}
