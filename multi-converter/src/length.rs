use num::{Float, FromPrimitive};

/// Convert between Lengths units
/// This struct could be used with any Float implementation e.g. f32 or f6
///
/// ```
/// use multi_converter::length::{Length,LengthUnit};
/// 
/// // Create a Meter
/// let meter = Length::new(1.0, LengthUnit::Meter);
/// 
/// // Convert to Foot
/// let foot = meter.convert_to(LengthUnit::Foot);
/// 
/// println!("{} meter is {} feet", meter, foot);
/// ```
pub struct Length<T>
where
    T: Float + FromPrimitive,
{
    value: T,
    unit: LengthUnit,
}

#[derive(PartialEq)]
pub enum LengthUnit {
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

impl<T> std::fmt::Display for Length<T> where T: Float + FromPrimitive + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Length<T>
where
    T: Float + FromPrimitive,
{
    pub fn new(value: T, unit: LengthUnit) -> Length<T> {
        Length { value, unit }
    }

    pub fn convert_to(&self, to: LengthUnit) -> T {
        match to {
            LengthUnit::Micrometer => self.convert_to_micrometer(),
            LengthUnit::Millimeter => self.convert_to_milimeter(),
            LengthUnit::Centimeter => self.convert_to_centimeter(),
            LengthUnit::Meter => self.convert_to_meter(),
            LengthUnit::Kilometer => self.convert_to_kilometer(),
            LengthUnit::Inch => self.convert_to_inch(),
            LengthUnit::Foot => self.convert_to_foot(),
            LengthUnit::Yard => self.convert_to_yard(),
            LengthUnit::Mile => self.convert_to_mile(),
            LengthUnit::NauticalMile => self.convert_to_nautic_mile(),
        }
    }

    pub fn convert_to_micrometer(&self) -> T {
        match &self.unit {
            LengthUnit::Micrometer => self.value,
            LengthUnit::Millimeter => self.value * T::from_u32(1000).unwrap(),
            LengthUnit::Centimeter => self.value * T::from_i32(10000).unwrap(),
            LengthUnit::Meter => self.value * T::from_f32(1e+6).unwrap(),
            LengthUnit::Kilometer => self.value * T::from_f64(1e+9).unwrap(),
            LengthUnit::Inch => self.value * T::from_u32(25400).unwrap(),
            LengthUnit::Foot => self.value * T::from_u32(304800).unwrap(),
            LengthUnit::Yard => self.value * T::from_u32(914400).unwrap(),
            LengthUnit::Mile => self.value * T::from_f64(1.609e+9).unwrap(),
            LengthUnit::NauticalMile => self.value * T::from_f64(1.852e+9).unwrap(),
        }
    }

    pub fn convert_to_milimeter(&self) -> T {
        match &self.unit {
            LengthUnit::Millimeter => self.value,
            LengthUnit::Micrometer => self.value / T::from_u32(1000).unwrap(),
            LengthUnit::Centimeter => self.value * T::from_u8(10).unwrap(),
            LengthUnit::Meter => self.value * T::from_u16(1000).unwrap(),
            LengthUnit::Kilometer => self.value * T::from_f32(1e+6).unwrap(),
            LengthUnit::Inch => self.value * T::from_f32(25.4).unwrap(),
            LengthUnit::Foot => self.value * T::from_u16(305).unwrap(),
            LengthUnit::Yard => self.value * T::from_u16(914).unwrap(),
            LengthUnit::Mile => self.value * T::from_f32(1.609e+6).unwrap(),
            LengthUnit::NauticalMile => self.value * T::from_f64(1.852e+6).unwrap(),
        }
    }

    pub fn convert_to_centimeter(&self) -> T {
        match &self.unit {
            LengthUnit::Centimeter => self.value,
            LengthUnit::Micrometer => self.value / T::from_u32(10000).unwrap(),
            LengthUnit::Millimeter => self.value / T::from_u8(10).unwrap(),
            LengthUnit::Meter => self.value * T::from_u16(100).unwrap(),
            LengthUnit::Kilometer => self.value * T::from_u32(100000).unwrap(),
            LengthUnit::Inch => self.value * T::from_f32(2.54).unwrap(),
            LengthUnit::Foot => self.value * T::from_f32(30.48).unwrap(),
            LengthUnit::Yard => self.value * T::from_f32(91.44).unwrap(),
            LengthUnit::Mile => self.value * T::from_u32(1160934).unwrap(),
            LengthUnit::NauticalMile => self.value * T::from_u32(185200).unwrap(),
        }
    }

    pub fn convert_to_meter(&self) -> T {
        match &self.unit {
            LengthUnit::Meter => self.value,
            LengthUnit::Micrometer => self.value * T::from_f32(1e+6).unwrap(),
            LengthUnit::Millimeter => self.value * T::from_i32(1000).unwrap(),
            LengthUnit::Centimeter => self.value * T::from_i32(100).unwrap(),
            LengthUnit::Kilometer => self.value / T::from_i32(1000).unwrap(),
            LengthUnit::Inch => self.value * T::from_f32(39.37).unwrap(),
            LengthUnit::Foot => self.value * T::from_f32(3.281).unwrap(),
            LengthUnit::Yard => self.value * T::from_f32(1.094).unwrap(),
            LengthUnit::Mile => self.value / T::from_i32(1609).unwrap(),
            LengthUnit::NauticalMile => self.value / T::from_i32(1852).unwrap(),
        }
    }

    pub fn convert_to_kilometer(&self) -> T {
        match &self.unit {
            LengthUnit::Kilometer => self.value,
            LengthUnit::Meter => self.value / T::from_u16(1000).unwrap(),
            LengthUnit::Micrometer => self.value * T::from_f32(1e+6).unwrap(),
            LengthUnit::Millimeter => self.value * T::from_i32(1000).unwrap(),
            LengthUnit::Centimeter => self.value * T::from_i32(100).unwrap(),
            LengthUnit::Inch => self.value * T::from_f32(39.37).unwrap(),
            LengthUnit::Foot => self.value * T::from_f32(3.281).unwrap(),
            LengthUnit::Yard => self.value * T::from_f32(1.094).unwrap(),
            LengthUnit::Mile => self.value / T::from_i32(1609).unwrap(),
            LengthUnit::NauticalMile => self.value / T::from_i32(1852).unwrap(),
        }
    }

    pub fn convert_to_inch(&self) -> T {
        match &self.unit {
            LengthUnit::Inch => self.value,
            LengthUnit::Micrometer => self.value / T::from_u16(25400).unwrap(),
            LengthUnit::Millimeter => self.value / T::from_f32(25.4).unwrap(),
            LengthUnit::Centimeter => self.value / T::from_f32(2.54).unwrap(),
            LengthUnit::Meter => self.value * T::from_f32(39.37).unwrap(),
            LengthUnit::Kilometer => self.value * T::from_u16(39370).unwrap(),
            LengthUnit::Foot => self.value / T::from_u8(12).unwrap(),
            LengthUnit::Yard => self.value / T::from_u8(36).unwrap(),
            LengthUnit::Mile => self.value * T::from_u16(63360).unwrap(),
            LengthUnit::NauticalMile => self.value / T::from_u32(72913).unwrap(),
        }
    }

    pub fn convert_to_foot(&self) -> T {
        match &self.unit {
            LengthUnit::Foot => self.value,
            LengthUnit::Micrometer => self.value / T::from_u32(304800).unwrap(),
            LengthUnit::Millimeter => self.value / T::from_u16(305).unwrap(),
            LengthUnit::Centimeter => self.value / T::from_f32(30.48).unwrap(),
            LengthUnit::Meter => self.value * T::from_f32(3.281).unwrap(),
            LengthUnit::Kilometer => self.value * T::from_u16(3281).unwrap(),
            LengthUnit::Inch => self.value / T::from_u8(12).unwrap(),
            LengthUnit::Yard => self.value * T::from_u8(3).unwrap(),
            LengthUnit::Mile => self.value * T::from_u16(5280).unwrap(),
            LengthUnit::NauticalMile => self.value * T::from_u16(6076).unwrap(),
        }
    }
    pub fn convert_to_yard(&self) -> T {
        match &self.unit {
            LengthUnit::Yard => self.value,
            LengthUnit::Micrometer => self.value / T::from_u32(914400).unwrap(),
            LengthUnit::Millimeter => self.value / T::from_u16(914).unwrap(),
            LengthUnit::Centimeter => self.value / T::from_f32(30.48).unwrap(),
            LengthUnit::Meter => self.value * T::from_f32(3.281).unwrap(),
            LengthUnit::Kilometer => self.value * T::from_u16(3281).unwrap(),
            LengthUnit::Inch => self.value / T::from_u8(12).unwrap(),
            LengthUnit::Foot => self.value * T::from_u8(3).unwrap(),
            LengthUnit::Mile => self.value * T::from_u16(5280).unwrap(),
            LengthUnit::NauticalMile => self.value * T::from_u16(6076).unwrap(),
        }
    }

    pub fn convert_to_mile(&self) -> T {
        match &self.unit {
            LengthUnit::Yard => self.value,
            LengthUnit::Micrometer => self.value / T::from_u32(914400).unwrap(),
            LengthUnit::Millimeter => self.value / T::from_u16(914).unwrap(),
            LengthUnit::Centimeter => self.value / T::from_f32(91.44).unwrap(),
            LengthUnit::Meter => self.value * T::from_f32(1.094).unwrap(),
            LengthUnit::Kilometer => self.value * T::from_u16(1094).unwrap(),
            LengthUnit::Inch => self.value / T::from_u8(36).unwrap(),
            LengthUnit::Foot => self.value / T::from_u8(3).unwrap(),
            LengthUnit::Mile => self.value * T::from_u16(1760).unwrap(),
            LengthUnit::NauticalMile => self.value * T::from_u16(2025).unwrap(),
        }
    }

    pub fn convert_to_nautic_mile(&self) -> T {
        match &self.unit {
            LengthUnit::NauticalMile => self.value,
            LengthUnit::Micrometer => self.value / T::from_f32(1.852e+9).unwrap(),
            LengthUnit::Millimeter => self.value / T::from_f32(1.852e+6).unwrap(),
            LengthUnit::Centimeter => self.value / T::from_u32(185200).unwrap(),
            LengthUnit::Meter => self.value / T::from_u16(1852).unwrap(),
            LengthUnit::Kilometer => self.value / T::from_f32(1.852).unwrap(),
            LengthUnit::Inch => self.value / T::from_u32(72913).unwrap(),
            LengthUnit::Foot => self.value / T::from_u16(6076).unwrap(),
            LengthUnit::Yard => self.value / T::from_u16(2025).unwrap(),
            LengthUnit::Mile => self.value / T::from_f32(1.151).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_convert_to_micrometer() {
        let c: f32 = 1.0;
        assert_eq!(
            1.0,
            Length::new(c, LengthUnit::Micrometer).convert_to_micrometer()
        );
        assert_eq!(
            1000.0,
            Length::new(c, LengthUnit::Millimeter).convert_to_micrometer()
        );
        assert_eq!(
            10000.0,
            Length::new(c, LengthUnit::Centimeter).convert_to_micrometer()
        );
        assert_eq!(1e+6, Length::new(c, LengthUnit::Meter).convert_to_micrometer());
        assert_eq!(
            1e+9,
            Length::new(c, LengthUnit::Kilometer).convert_to_micrometer()
        );
        assert_eq!(25400.0, Length::new(c, LengthUnit::Inch).convert_to_micrometer());
        assert_eq!(304800.0, Length::new(c, LengthUnit::Foot).convert_to_micrometer());
        assert_eq!(914400.0, Length::new(c, LengthUnit::Yard).convert_to_micrometer());
        assert_eq!(1.609e+9, Length::new(c, LengthUnit::Mile).convert_to_micrometer());
        assert_eq!(
            1.852e+9,
            Length::new(c, LengthUnit::NauticalMile).convert_to_micrometer()
        );
    }

    #[test]
    fn meter_to_foot() {
        assert_eq!(
            3.281,
            Length::new(1.0 as f32, LengthUnit::Meter).convert_to(LengthUnit::Foot)
        );
    }

    #[test]
    fn convert_to_kilometer() {
        assert_eq!(0.001, Length::new(1.0, LengthUnit::Meter).convert_to_kilometer());
    }

    #[test]
    fn convert_to_foot() {
        assert_eq!(
            3.281,
            Length::new(1.0 as f32, LengthUnit::Meter).convert_to_foot()
        );
    }
}
