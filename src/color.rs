use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Color
{
    pub r : f64,
    pub g : f64,
    pub b : f64
}

impl Color
{
    // Converts hexadecimal values to RGB values
    pub fn new_hex(hex_code : i32) -> Self
    {
        Color
        {
            r : (((hex_code >> 16) & 0xFF) as f64 / 255.0),
            g : (((hex_code >> 8) & 0xFF) as f64 / 255.0),
            b : ((hex_code & 0xFF) as f64 / 255.0)
        }
    }

    pub fn new_rgb(_r : f64, _g : f64 , _b : f64) -> Self
    {
        Color
        {
            r : _r,
            g : _g,
            b : _b
        }
    }

    pub fn mul_by_color(&self, vec2 : Color) -> Color
    {
        Color
        {
            r : self.r * vec2.r,
            g : self.g * vec2.g,
            b : self.b * vec2.b
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, color2: Color) -> Color
    {
        Color
        {
            r : self.r + color2.r,
            g : self.g + color2.g,
            b : self.b + color2.b
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, constant : f64) -> Color {
        Color
        {
            r : self.r * constant,
            g : self.r * constant,
            b : self.r * constant
        }
    }
}

pub fn string_to_hex_int(color : &str ) -> i32
{
    let no_prefix = color.trim_start_matches("#");

    let result = i32::from_str_radix(no_prefix, 16);

    return  result.unwrap();
}