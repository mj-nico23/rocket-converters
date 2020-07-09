use num::{Float, FromPrimitive};

pub struct Length<T>
where
    T: Float + FromPrimitive,
{
    value: T,
    unit: Unit,
}

#[derive(PartialEq)]
pub enum Unit {
    Micrometer,
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
}

impl<T> Length<T>
where
    T: Float + FromPrimitive,
{
    pub fn new(value: T, unit: Unit) -> Length<T> {
        Length { value, unit }
    }

    pub fn convert_to(&self, to: Unit) -> T {
        match to {
            Unit::Micrometer => self.convert_to_micrometer(),
            Unit::Millimeter => self.convert_to_kilometer(),
            Unit::Centimeter => self.convert_to_kilometer(),
            Unit::Meter => self.convert_to_meter(),
            Unit::Kilometer => self.convert_to_kilometer(),
            Unit::Inch => self.convert_to_kilometer(),
            Unit::Foot => self.convert_to_foot(),
            Unit::Yard => self.convert_to_kilometer(),
            Unit::Mile => self.convert_to_kilometer(),
            Unit::NauticalMile => self.convert_to_kilometer(),
        }
    }

    pub fn convert_to_micrometer(&self) -> T {
        match &self.unit {
            Unit::Micrometer => self.value,
            Unit::Millimeter => self.value * T::from_u32(1000).unwrap(),
            Unit::Centimeter => self.value * T::from_i32(10000).unwrap(),
            Unit::Meter => self.value * T::from_f32(1e+6).unwrap(),
            Unit::Kilometer => self.value * T::from_f64(1e+9).unwrap(),
            Unit::Inch => self.value * T::from_u32(25400).unwrap(),
            Unit::Foot => self.value * T::from_u32(304800).unwrap(),
            Unit::Yard => self.value * T::from_u32(914400).unwrap(),
            Unit::Mile => self.value * T::from_f64(1.609e+9).unwrap(),
            Unit::NauticalMile => self.value * T::from_f64(1.852e+9).unwrap(),
        }
    }

    pub fn convert_to_milimeter(&self) -> T {
        match &self.unit {
            Unit::Millimeter => self.value,
            Unit::Micrometer => self.value / T::from_u32(1000).unwrap(),
            Unit::Centimeter => self.value * T::from_u8(10).unwrap(),
            Unit::Meter => self.value * T::from_u16(1000).unwrap(),
            Unit::Kilometer => self.value * T::from_f32(1e+6).unwrap(),
            Unit::Inch => self.value * T::from_f32(25.4).unwrap(),
            Unit::Foot => self.value * T::from_u16(305).unwrap(),
            Unit::Yard => self.value * T::from_u16(914).unwrap(),
            Unit::Mile => self.value * T::from_f32(1.609e+6).unwrap(),
            Unit::NauticalMile => self.value * T::from_f64(1.852e+6).unwrap(),
        }
    }

    pub fn convert_to_centimeter(&self) -> T {
        match &self.unit {
            Unit::Centimeter => self.value,
            Unit::Micrometer => self.value / T::from_u32(10000).unwrap(),
            Unit::Millimeter => self.value / T::from_u8(10).unwrap(),
            Unit::Meter => self.value * T::from_u16(100).unwrap(),
            Unit::Kilometer => self.value * T::from_u32(100000).unwrap(),
            Unit::Inch => self.value * T::from_f32(2.54).unwrap(),
            Unit::Foot => self.value * T::from_f32(30.48).unwrap(),
            Unit::Yard => self.value * T::from_f32(91.44).unwrap(),
            Unit::Mile => self.value * T::from_u32(1160934).unwrap(),
            Unit::NauticalMile => self.value * T::from_u32(185200).unwrap(),
        }
    }

    pub fn convert_to_meter(&self) -> T {
        match &self.unit {
            Unit::Meter => self.value,
            Unit::Micrometer => self.value * T::from_f32(1e+6).unwrap(),
            Unit::Millimeter => self.value * T::from_i32(1000).unwrap(),
            Unit::Centimeter => self.value * T::from_i32(100).unwrap(),
            Unit::Kilometer => self.value / T::from_i32(1000).unwrap(),
            Unit::Inch => self.value * T::from_f32(39.37).unwrap(),
            Unit::Foot => self.value * T::from_f32(3.281).unwrap(),
            Unit::Yard => self.value * T::from_f32(1.094).unwrap(),
            Unit::Mile => self.value / T::from_i32(1609).unwrap(),
            Unit::NauticalMile => self.value / T::from_i32(1852).unwrap(),
        }
    }

    fn convert_to_kilometer(&self) -> T {
        match &self.unit {
            Unit::Kilometer => self.value,
            Unit::Meter => self.value / T::from_u16(1000).unwrap(),
            Unit::Micrometer => self.value * T::from_f32(1e+6).unwrap(),
            Unit::Millimeter => self.value * T::from_i32(1000).unwrap(),
            Unit::Centimeter => self.value * T::from_i32(100).unwrap(),
            Unit::Inch => self.value * T::from_f32(39.37).unwrap(),
            Unit::Foot => self.value * T::from_f32(3.281).unwrap(),
            Unit::Yard => self.value * T::from_f32(1.094).unwrap(),
            Unit::Mile => self.value / T::from_i32(1609).unwrap(),
            Unit::NauticalMile => self.value / T::from_i32(1852).unwrap(),
        }
    }

    fn convert_to_foot(&self) -> T {
        match &self.unit {
            Unit::Foot => self.value,
            Unit::Micrometer => self.value / T::from_f64(1e+9).unwrap(),
            Unit::Millimeter => self.value / T::from_f32(1e+6).unwrap(),
            Unit::Centimeter => self.value / T::from_u32(100000).unwrap(),
            Unit::Meter => self.value * T::from_f32(3.281).unwrap(),
            Unit::Kilometer => self.value * T::from_u16(3281).unwrap(),
            Unit::Inch => self.value / T::from_u16(39370).unwrap(),
            Unit::Yard => self.value / T::from_u16(1094).unwrap(),
            Unit::Mile => self.value * T::from_f32(1.609).unwrap(),
            Unit::NauticalMile => self.value / T::from_f32(1.852).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_convert_to_micrometer() {
        let c: f32 = 1.0;
        assert_eq!(1.0, Length::new(c, Unit::Micrometer).convert_to_micrometer());
        assert_eq!(1000.0, Length::new(c, Unit::Millimeter).convert_to_micrometer());
        assert_eq!(10000.0, Length::new(c, Unit::Centimeter).convert_to_micrometer());
        assert_eq!(1e+6, Length::new(c, Unit::Meter).convert_to_micrometer());
        assert_eq!(1e+9, Length::new(c, Unit::Kilometer).convert_to_micrometer());
        assert_eq!(25400.0, Length::new(c, Unit::Inch).convert_to_micrometer());
        assert_eq!(304800.0, Length::new(c, Unit::Foot).convert_to_micrometer());
        assert_eq!(914400.0, Length::new(c, Unit::Yard).convert_to_micrometer());
        assert_eq!(1.609e+9, Length::new(c, Unit::Mile).convert_to_micrometer());
        assert_eq!(1.852e+9, Length::new(c, Unit::NauticalMile).convert_to_micrometer());
    }

    #[test]
    fn meter_to_foot() {
        assert_eq!(
            3.281,
            Length::new(1.0 as f32, Unit::Meter).convert_to(Unit::Foot)
        );
    }

    #[test]
    fn convert_to_kilometer() {
        assert_eq!(0.001, Length::new(1.0, Unit::Meter).convert_to_kilometer());
    }

    #[test]
    fn convert_to_foot() {
        assert_eq!(
            3.281,
            Length::new(1.0 as f32, Unit::Meter).convert_to_foot()
        );
    }
}
