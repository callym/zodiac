use std::collections::BTreeMap;

use crate::{ JulianDay, Planet, Placement, Sign };
use crate::Planet::*;

#[derive(Debug, Clone)]
pub struct Chart {
	chart: BTreeMap<Planet, Placement>,
}

impl Chart {
	pub fn new<T>(date: T) -> Self  where T: Into<JulianDay> {
		let jd: JulianDay = date.into();

		let mut chart = BTreeMap::new();

		chart.insert(Sun, Sun.get_placement(jd));
		chart.insert(Moon, Moon.get_placement(jd));
		chart.insert(Mercury, Mercury.get_placement(jd));
		chart.insert(Venus, Venus.get_placement(jd));
		chart.insert(Mars, Mars.get_placement(jd));
		chart.insert(Jupiter, Jupiter.get_placement(jd));
		chart.insert(Saturn, Saturn.get_placement(jd));
		chart.insert(Uranus, Uranus.get_placement(jd));
		chart.insert(Neptune, Neptune.get_placement(jd));
		chart.insert(Pluto, Pluto.get_placement(jd));

		Self { chart }
	}

	pub fn get_placement(&self, p: Planet) -> Placement {
		*self.chart.get(&p).unwrap()
	}

	pub fn get_sign(&self, p: Planet) -> Sign {
		self.chart.get(&p).unwrap().sign
	}

	pub fn iter_signs<'a>(&'a self) -> impl Iterator<Item = (Planet, Sign)> + 'a {
		self.chart.iter().map(|(k, v)| (*k, v.sign))
	}
}

impl AsRef<BTreeMap<Planet, Placement>> for Chart {
    fn as_ref(&self) -> &BTreeMap<Planet, Placement> {
        &self.chart
    }
}
