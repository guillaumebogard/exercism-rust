// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
  seconds: u64,
}

impl From<u64> for Duration {
  fn from(s: u64) -> Self {
    Self { seconds: s }
  }
}

const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

pub trait Planet {
  const ORBITAL_PERIOD_RELATIVE_TO_EARTH: f64;

  fn years_during(d: &Duration) -> f64 {
    d.seconds as f64 / EARTH_YEAR_IN_SECONDS / Self::ORBITAL_PERIOD_RELATIVE_TO_EARTH
  }
}

macro_rules! impl_planet {
  ($name:ident, $orbital_period:expr) => {
    pub struct $name;

    impl Planet for $name {
      const ORBITAL_PERIOD_RELATIVE_TO_EARTH: f64 = $orbital_period;
    }
  };
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);
