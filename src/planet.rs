use vsop87::{ vsop87c, RectangularCoordinates };

use ::{ Coordinates, JulianDay, Placement, Sign };

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

	pub fn get_sign<T>(&self, date: T) -> Sign where T: Into<JulianDay> {
		let coordinates = self.coordinates(date.into());
		Sign::from_coordinates(&coordinates)
	}

	pub fn get_placement<T>(&self, date: T) -> Placement where T: Into<JulianDay> + Clone {
		Placement::new(*self, date)
	}

	pub fn coordinates<T>(&self, date: T) -> Coordinates where T: Into<JulianDay> {
		let julian_day: JulianDay = date.into();

		let earth = vsop87c::earth(julian_day.into());
		let planet = match *self {
			Planet::Sun => RectangularCoordinates { x: 0.0, y: 0.0, z: 0.0 },
			Planet::Moon => return moon(julian_day.into()).into(),
			Planet::Mercury => vsop87c::mercury(julian_day.into()),
			Planet::Venus => vsop87c::venus(julian_day.into()),
			Planet::Mars => vsop87c::mars(julian_day.into()),
			Planet::Jupiter => vsop87c::jupiter(julian_day.into()),
			Planet::Saturn => vsop87c::saturn(julian_day.into()),
			Planet::Uranus => vsop87c::uranus(julian_day.into()),
			Planet::Neptune => vsop87c::neptune(julian_day.into()),
			Planet::Pluto => pluto(julian_day.into()),
		};

		let geocentric = RectangularCoordinates {
			x: planet.x - earth.x,
			y: planet.y - earth.y,
			z: planet.z - earth.z,
		};

		geocentric.into()
	}
}

// http://www.stjarnhimlen.se/comp/tutorial.html
#[allow(non_snake_case)]
fn moon(julian_day: f64) -> RectangularCoordinates {
	let julian_day = julian_day - 2451543.5;

	fn range(n: f64) -> f64 {
		let mut n = n;

		if n > 0.0 && n < 360.0 {
			return n;
		}

		if n < 0.0 {
			while n < 0.0 {
				n += 360.0;
			}
			return n;
		}

		if n > 360.0 {
			while n > 360.0 {
				n -= 360.0;
			}
			return n;
		}

		n
	}

	// Sun constants
	// mean longitude
	let Ls = (282.9404 + 4.70935_e-5 * julian_day) + (356.0470 + 0.9856002585 * julian_day);
	// mean anomaly
	let Ms = 356.0470 + 0.9856002585 * julian_day;

	let Ls = range(Ls);
	let Ms = range(Ms);

	// Moon constants
	// long asc. node
	let N = 125.1228 - 0.0529538083 * julian_day;
	// inclination
	let i: f64 = 5.1454;
	// arg. of perigee
	let w = 318.0634 + 0.1643573223  * julian_day;
	// mean distance
	let a = 60.2666;
	// eccentricity
	let e = 0.054900;
	// mean anomaly
	let Mm = 115.3654 + 13.0649929509 * julian_day;
	// mean longitude
	let Lm = range(N) + range(w) + range(Mm);
	// mean elongation
	let D = range(Lm) - range(Ls);
	// argument of latitude
	let F = range(Lm) - range(N);

	let Ms: f64 = range(Ms).to_radians();
	let N: f64 = range(N).to_radians();
	let i: f64 = range(i).to_radians();
	let w: f64 = range(w).to_radians();
	let Mm: f64 = range(Mm).to_radians();
	let D: f64 = range(D).to_radians();
	let F: f64 = range(F).to_radians();

	// pertubations in longitude
	let Plon: f64 =   (-1.274 as f64).to_radians() * (Mm - 2.0 * D).sin()
					+ (0.658 as f64).to_radians() * (2.0 * D).sin()
					+ (-0.186 as f64).to_radians() * Ms.sin()
					+ (-0.059 as f64).to_radians() * (2.0 *Mm - 2.0 * D).sin()
					+ (-0.057 as f64).to_radians() * (Mm - 2.0 * D + Ms).sin()
					+ (0.053 as f64).to_radians() * (Mm + 2.0 * D).sin()
					+ (0.046 as f64).to_radians() * (2.0 * D - Ms).sin()
					+ (0.041 as f64).to_radians() * (Mm - Ms).sin()
					+ (-0.035 as f64).to_radians() * D.sin()
					+ (-0.031 as f64).to_radians() * (Mm + Ms).sin()
					+ (-0.015 as f64).to_radians() * (2.0 * F - 2.0 * D).sin()
					+ (0.011 as f64).to_radians() * (Mm - 4.0 * D).sin();

	// pertubations in latitude
	let Plat: f64 =   (-0.173 as f64).to_radians() * (F - 2.0 * D).sin()
					+ (-0.055 as f64).to_radians() * (Mm - F - 2.0 * D).sin()
					+ (-0.046 as f64).to_radians() * (Mm + F - 2.0 * D).sin()
					+ (0.033 as f64).to_radians() * (F + 2.0 * D).sin()
					+ (0.017 as f64).to_radians() * (2.0 * Mm + F).sin();

	// pertubations in lunar distance
	let Pdist: f64 =  (-0.58 as f64) * (Mm - (2.0 * D)).cos()
					+ (-0.46 as f64) * (2.0 * D).cos();

	// eccentric anomaly
	let mut E0 = Mm + (1.0 as f64) * e * Mm.sin() * (1.0 + e + Mm.cos());
	let mut E1 = 0.0;
	while (E0 - E1).abs() > 0.005 {
		E1 = E0 - (E0 - (1.0 as f64) * e * E0.sin() - Mm) / (1.0 - e * E0.cos());
		E0 = E1;
	}
	let E = E1;

	let x = a * (E.cos() - e);
	let y = a * (1.0 - e.powi(2)).sqrt() * E.sin();

	let r = (x.powi(2) + y.powi(2)).sqrt();
	let v = y.atan2(x);
	let v = range(v.to_degrees()).to_radians();

	let x = r * (N.cos() * (v + w).cos() - N.sin() * (v + w).sin() * i.cos());
	let y = r * (N.sin() * (v + w).cos() + N.cos() * (v + w).sin() * i.cos());
	let z = r * (v + w).sin() * i.sin();

	let rc = RectangularCoordinates {
		x: x,
		y: y,
		z: z,
	};

	let mut coord: Coordinates = rc.into();
	println!("{:?}", Pdist);
	coord.lon += Plon.to_degrees();
	coord.lat += Plat.to_degrees();
	coord.dist += Pdist;
	coord.dist *= 4.2587504555972E-5;

	coord.into()
}

// http://www.stjarnhimlen.se/comp/ppcomp.html#14
#[allow(non_snake_case)]
fn pluto(julian_day: f64) -> RectangularCoordinates {
	let julian_day = julian_day - 2451543.5;

	let S = 50.03 + 0.033459652 * julian_day;
	let P = 238.95 + 0.003968789 * julian_day;

	let S = S.to_radians();
	let P = P.to_radians();

	let lon = (238.9508 as f64).to_radians() + (0.00400703 as f64).to_radians() * julian_day
			+ (-19.799 as f64).to_radians() * P.sin()
			+ (19.848 as f64).to_radians() * P.cos()
			+ (0.897 as f64).to_radians() * (2.0 * P).sin()
			+ (-4.956 as f64).to_radians() * (2.0 * P).cos()
			+ (0.610 as f64).to_radians() * (3.0 * P).sin()
			+ (1.211 as f64).to_radians() * (3.0 * P).cos()
			+ (-0.341 as f64).to_radians() * (4.0 * P).sin()
			+ (-0.190 as f64).to_radians() * (4.0 * P).cos()
			+ (0.128 as f64).to_radians() * (5.0 * P).sin()
			+ (-0.034 as f64).to_radians() * (5.0 * P).cos()
			+ (-0.038 as f64).to_radians() * (6.0 * P).sin()
			+ (0.031 as f64).to_radians() * (6.0 * P).cos()
			+ (0.020 as f64).to_radians() * (S - P).sin()
			+ (-0.010 as f64).to_radians() * (S - P).cos();

	let lat = (-3.9082 as f64).to_radians()
			+ (-5.453 as f64).to_radians() * P.sin()
			+ (-14.975 as f64).to_radians() * P.cos()
			+ (3.527 as f64).to_radians() * (2.0 * P).sin()
			+ (1.673 as f64).to_radians() * (2.0 * P).cos()
			+ (-1.051 as f64).to_radians() * (3.0 * P).sin()
			+ (0.328 as f64).to_radians() * (3.0 * P).cos()
			+ (0.179 as f64).to_radians() * (4.0 * P).sin()
			+ (-0.292 as f64).to_radians() * (4.0 * P).cos()
			+ (0.019 as f64).to_radians() * (5.0 * P).sin()
			+ (0.100 as f64).to_radians() * (5.0 * P).cos()
			+ (-0.031 as f64).to_radians() * (6.0 * P).sin()
			+ (-0.026 as f64).to_radians() * (6.0 * P).cos()
			+ (0.011 as f64).to_radians() * (S - P).cos();

	let dist = (40.72 as f64).to_radians()
			+ (6.68 as f64).to_radians() * P.sin()
			+ (6.90 as f64).to_radians() * P.cos()
			+ (-1.18 as f64).to_radians() * (2.0 * P).sin()
			+ (-0.03 as f64).to_radians() * (2.0 * P).cos()
			+ (0.15 as f64).to_radians() * (3.0 * P).sin()
			+ (-0.14 as f64).to_radians() * (3.0 * P).cos();

	let coord = Coordinates {
		lat: lat.to_degrees(),
		lon: lon.to_degrees(),
		dist: dist.to_degrees(),
	};

	coord.into()
}
