use vsop87::RectangularCoordinates;

#[derive(Clone, Debug)]
pub struct Coordinates {
	pub lat: f64,
	pub lon: f64,
	pub dist: f64,
}

impl From<RectangularCoordinates> for Coordinates {
	fn from(coord: RectangularCoordinates) -> Self {
		let x = coord.x;
		let y = coord.y;
		let z = coord.z;

		let r = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
		let ra = y.atan2(x).to_degrees();
		let ra = if ra >= 0.0 { ra } else { ra + 360.0 };
		let lat = (z / r).asin().to_degrees();

		Self {
			lat: lat,
			lon: ra,
			dist: r,
		}
	}
}
