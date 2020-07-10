use num::{Float, FromPrimitive};

/// This struct could be used with any Float implementation e.g. f32 or f64
pub struct Temperature<T>
where
    T: Float + FromPrimitive
{
    value: T,
    unit: TemperatureUnit,
}

#[derive(PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Faherenheit,
    Kelvin,
}

impl<T> std::fmt::Display for Temperature<T> where T: Float + FromPrimitive + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Temperature<T>
where
    T: Float + FromPrimitive
{
    pub fn new(value: T, unit: TemperatureUnit) -> Temperature<T>
    {
        Temperature { value, unit }
    }

    pub fn convert_to(&self, to: TemperatureUnit) -> T
    {
        match to {
            TemperatureUnit::Celsius => self.convert_to_celsius(),
            TemperatureUnit::Faherenheit => self.convert_to_faherenheit(),
            TemperatureUnit::Kelvin => self.convert_to_kelvin(),
        }
    }

    pub fn convert_to_celsius(&self) -> T
    {
        match &self.unit {
            TemperatureUnit::Celsius => self.value,
            TemperatureUnit::Faherenheit => (self.value - T::from_f32(32.0).unwrap()) * T::from_f32(0.55).unwrap(),
            TemperatureUnit::Kelvin => self.value - T::from_f32(273.15).unwrap(),
        }
    }

    pub fn convert_to_faherenheit(&self) -> T
    {
        match &self.unit {
            TemperatureUnit::Faherenheit => self.value,
            TemperatureUnit::Celsius => (self.value * T::from_f32(1.8).unwrap()) + T::from_f32(32.0).unwrap(),
            TemperatureUnit::Kelvin => (self.value - T::from_f32(273.15).unwrap()) * T::from_f32(1.8).unwrap() + T::from_f32(32.0).unwrap(),
        }
    }

    pub fn convert_to_kelvin(&self) -> T
    {
        match &self.unit {
            TemperatureUnit::Kelvin => self.value,
            TemperatureUnit::Faherenheit => (self.value - T::from_f32(32.0).unwrap()) * T::from_f32(0.55).unwrap() + T::from_f32(273.15).unwrap(),
            TemperatureUnit::Celsius => self.value + T::from_f32(273.15).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TemperatureUnit;
    use super::*;
    #[test]
    fn celsius_to_faherenheit_it_works() {
        assert_eq!(
            Temperature::new(0.0, TemperatureUnit::Celsius).convert_to_faherenheit(),
            32.0
        );
    }

    #[test]
    fn celsius_to_faherenheit_it_works_with_decimals_f32() {
        let c: f32 = 12.0;
        assert_eq!(
            Temperature::new(c, TemperatureUnit::Celsius).convert_to(TemperatureUnit::Faherenheit),
            53.6
        );
    }
    
    #[test]
    fn celsius_to_faherenheit_it_works_with_decimals_f64() {
        let c: f64 = 12.0;
        assert_eq!(
            Temperature::new(c, TemperatureUnit::Celsius).convert_to(TemperatureUnit::Faherenheit),
            53.59999942779541
        );
    }

    #[test]
    fn celsius_to_kelvin_it_works() {
        assert_eq!(
            Temperature::new(0.0 as f32, TemperatureUnit::Celsius).convert_to(TemperatureUnit::Kelvin),
            273.15
        );
    }

    #[test]
    fn faherenheit_to_celsius_it_works() {
        assert_eq!(
            Temperature::new(32.0, TemperatureUnit::Faherenheit).convert_to(TemperatureUnit::Celsius),
            0.0
        );
    }

    #[test]
    fn faherenheit_to_kelvin_it_works() {
        assert_eq!(
            Temperature::new(32.0 as f32, TemperatureUnit::Faherenheit).convert_to(TemperatureUnit::Kelvin),
            273.15
        );
    }

    #[test]
    fn kelvin_to_celsius_it_works() {
        assert_eq!(
            Temperature::new(0.0 as f32, TemperatureUnit::Kelvin).convert_to(TemperatureUnit::Celsius),
            -273.15
        )
    }
    #[test]
    fn kelvin_to_faherenheit_it_works() {
        assert_eq!(
            Temperature::new(310.0 as f32, TemperatureUnit::Kelvin).convert_to(TemperatureUnit::Faherenheit),
            98.33001
        )
    }
}
