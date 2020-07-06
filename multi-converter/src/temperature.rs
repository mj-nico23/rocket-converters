pub struct Temperature{
    value: f32,
    unit: Unit
}

#[derive(PartialEq)]
pub enum Unit {
    Celsius,
    Faherenheit,
    Kelvin
}

impl Temperature {
    pub fn new(value: f32, unit: Unit) -> Temperature{
        Temperature {
            value,
            unit
        }
    }

    pub fn convert(&self, to: Unit) -> f32 {
        if to == self.unit {
            return self.value;
        } else {
            match (&self.unit, to) {
                (Unit::Celsius, Unit::Kelvin) => self.value + 273.15,
                (Unit::Celsius, Unit::Faherenheit) => (self.value * 1.8) + 32.0,
                (Unit::Faherenheit, Unit::Celsius) => (self.value - 32.0) * 0.55,
                (Unit::Faherenheit, Unit::Kelvin) => (self.value - 32.0) * 0.55 + 273.15,
                (Unit::Kelvin, Unit::Celsius) => self.value - 273.15,
                (Unit::Kelvin, Unit::Faherenheit) => (self.value - 273.15) * 1.8 + 32.0,
                _ => panic!("Unexpected unit convination")
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::Unit;
    
    #[test]
    fn celsius_to_faherenheit_it_works() {
        assert_eq!(Temperature::new(0.0, Unit::Celsius).convert(Unit::Faherenheit) , 32.0);
    }

    #[test]
    fn celsius_to_faherenheit_it_works_with_decimals() {
        assert_eq!(Temperature::new(12.0, Unit::Celsius).convert(Unit::Faherenheit) , 53.6);
    }

    #[test]
    fn celsius_to_kelvin_it_works() {
        assert_eq!(Temperature::new(0.0, Unit::Celsius).convert(Unit::Kelvin) , 273.15);
    }

    #[test]
    fn faherenheit_to_celsius_it_works() {
        assert_eq!(Temperature::new(32.0, Unit::Faherenheit).convert(Unit::Celsius), 0.0);
    }

    #[test]
    fn faherenheit_to_kelvin_it_works() {
        assert_eq!(Temperature::new(32.0, Unit::Faherenheit).convert(Unit::Kelvin), 273.15);
    }

    #[test]
    fn kelvin_to_celsius_it_works() {
        assert_eq!(Temperature::new(0.0, Unit::Kelvin).convert(Unit::Celsius), -273.15)
    }
    
    #[test]
    fn kelvin_to_faherenheit_it_works() {
        assert_eq!(Temperature::new(310.0, Unit::Kelvin).convert(Unit::Faherenheit), 98.33001)
    }
}