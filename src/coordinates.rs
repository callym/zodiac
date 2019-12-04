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
    // http://www.stjarnhimlen.se/comp/tutorial.html
    let lat = z.atan2((x.powi(2) + y.powi(2)).sqrt()).to_degrees();

    Self {
      lat: lat,
      lon: ra,
      dist: r,
    }
  }
}

impl From<Coordinates> for RectangularCoordinates {
  fn from(coord: Coordinates) -> Self {
    let lon = coord.lon.to_radians();
    let lat = coord.lat.to_radians();

    Self {
      x: coord.dist * lon.cos() * lat.cos(),
      y: coord.dist * lon.sin() * lat.cos(),
      z: coord.dist * lat.sin()
    }
  }
}
