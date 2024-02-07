use crate::color::Color;

#[derive(PartialEq, Copy, Clone)]
pub enum MatType
{
    Diffuse,
    Specular,
    Refractive
}

#[derive(Copy, Clone)]
pub struct Material
{
    pub color : Color,
    pub mat_type : MatType,
    pub emission : f64
}

impl Material
{
    pub fn new(clr : i32, _mat_type : MatType, mat_emission : f64) -> Self
    {
        Material
        {
            color : Color::new_hex(clr),
            mat_type : _mat_type,
            emission : mat_emission
        }
    }
}