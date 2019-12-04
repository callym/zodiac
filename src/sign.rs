use std::ops::Range;
use serde::{ Serialize, Deserialize };

use crate::Coordinates;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

  pub fn range(&self) -> Range<u32> {
    match *self {
      Sign::Aries => 0 .. 30,
      Sign::Taurus => 30 .. 60,
      Sign::Gemini => 60 .. 90,
      Sign::Cancer => 90 .. 120,
      Sign::Leo => 120 .. 150,
      Sign::Virgo => 150 .. 180,
      Sign::Libra => 180 .. 210,
      Sign::Scorpio => 210 .. 240,
      Sign::Sagittarius => 240 .. 270,
      Sign::Capricorn => 270 .. 300,
      Sign::Aquarius => 300 .. 330,
      Sign::Pisces => 330 .. 360,
    }
  }

  pub fn from_coordinates(coord: &Coordinates) -> Self {
    match &(coord.lon as u32) {
      i if Sign::Aries.range().contains(i) => Sign::Aries,
      i if Sign::Taurus.range().contains(i) => Sign::Taurus,
      i if Sign::Gemini.range().contains(i) => Sign::Gemini,
      i if Sign::Cancer.range().contains(i) => Sign::Cancer,
      i if Sign::Leo.range().contains(i) => Sign::Leo,
      i if Sign::Virgo.range().contains(i) => Sign::Virgo,
      i if Sign::Libra.range().contains(i) => Sign::Libra,
      i if Sign::Scorpio.range().contains(i) => Sign::Scorpio,
      i if Sign::Sagittarius.range().contains(i) => Sign::Sagittarius,
      i if Sign::Capricorn.range().contains(i) => Sign::Capricorn,
      i if Sign::Aquarius.range().contains(i) => Sign::Aquarius,
      i if Sign::Pisces.range().contains(i) => Sign::Pisces,
      _ => panic!(),
    }
  }
}
