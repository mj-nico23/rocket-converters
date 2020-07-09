use num::{Float, FromPrimitive};

pub struct Temperature<T>
where
    T: Float + FromPrimitive
{
    value: T,
    unit: Unit,
}

#[derive(PartialEq)]
pub enum Unit {
    Celsius,
    Faherenheit,
    Kelvin,
}

impl<T> Temperature<T>
where
    T: Float + FromPrimitive
{
    pub fn new(value: T, unit: Unit) -> Temperature<T>
    {
        Temperature { value, unit }
    }

    pub fn convert_to(&self, to: Unit) -> T
    {
        match to {
            Unit::Celsius => self.convert_to_celsius(),
            Unit::Faherenheit => self.convert_to_faherenheit(),
            Unit::Kelvin => self.convert_to_kelvin(),
        }
    }

    pub fn convert_to_celsius(&self) -> T
    {
        match &self.unit {
            Unit::Celsius => self.value,
            Unit::Faherenheit => (self.value - T::from_f32(32.0).unwrap()) * T::from_f32(0.55).unwrap(),
            Unit::Kelvin => self.value - T::from_f32(273.15).unwrap(),
        }
    }

    pub fn convert_to_faherenheit(&self) -> T
    {
        match &self.unit {
            Unit::Faherenheit => self.value,
            Unit::Celsius => (self.value * T::from_f32(1.8).unwrap()) + T::from_f32(32.0).unwrap(),
            Unit::Kelvin => (self.value - T::from_f32(273.15).unwrap()) * T::from_f32(1.8).unwrap() + T::from_f32(32.0).unwrap(),
        }
    }

    pub fn convert_to_kelvin(&self) -> T
    {
        match &self.unit {
            Unit::Kelvin => self.value,
            Unit::Faherenheit => (self.value - T::from_f32(32.0).unwrap()) * T::from_f32(0.55).unwrap() + T::from_f32(273.15).unwrap(),
            Unit::Celsius => self.value + T::from_f32(273.15).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Unit;
    use super::*;
    #[test]
    fn celsius_to_faherenheit_it_works() {
        assert_eq!(
            Temperature::new(0.0, Unit::Celsius).convert_to_faherenheit(),
            32.0
        );
    }

    #[test]
    fn celsius_to_faherenheit_it_works_with_decimals_f32() {
        let c: f32 = 12.0;
        assert_eq!(
            Temperature::new(c, Unit::Celsius).convert_to(Unit::Faherenheit),
            53.6
        );
    }
    
    #[test]
    fn celsius_to_faherenheit_it_works_with_decimals_f64() {
        let c: f64 = 12.0;
        assert_eq!(
            Temperature::new(c, Unit::Celsius).convert_to(Unit::Faherenheit),
            53.59999942779541
        );
    }

    #[test]
    fn celsius_to_kelvin_it_works() {
        assert_eq!(
            Temperature::new(0.0 as f32, Unit::Celsius).convert_to(Unit::Kelvin),
            273.15
        );
    }

    #[test]
    fn faherenheit_to_celsius_it_works() {
        assert_eq!(
            Temperature::new(32.0, Unit::Faherenheit).convert_to(Unit::Celsius),
            0.0
        );
    }

    #[test]
    fn faherenheit_to_kelvin_it_works() {
        assert_eq!(
            Temperature::new(32.0 as f32, Unit::Faherenheit).convert_to(Unit::Kelvin),
            273.15
        );
    }

    #[test]
    fn kelvin_to_celsius_it_works() {
        assert_eq!(
            Temperature::new(0.0 as f32, Unit::Kelvin).convert_to(Unit::Celsius),
            -273.15
        )
    }
    #[test]
    fn kelvin_to_faherenheit_it_works() {
        assert_eq!(
            Temperature::new(310.0 as f32, Unit::Kelvin).convert_to(Unit::Faherenheit),
            98.33001
        )
    }
}
