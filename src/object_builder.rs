use crate::material::{Material, MatType};
use crate::object::{Object, Plane, Sphere};
use crate::vector::Vector;

pub fn sphere_builder(centre : Vector, radius : f64, material_type : MatType, hex_color : i32) -> Object 
{
    let material = Material::new(hex_color, material_type, 0.0);

    let sphere = Sphere::new(centre , radius, material);

    return Object{ body: Box::new(sphere) };
}

pub fn plane_builder(normal : Vector, d : f64, material_type : MatType, hex_color : i32) -> Object
{
    let material = Material::new(hex_color, material_type, 0.0);

    let plane = Plane::new(normal, d, material);

    return Object{ body : Box::new(plane) };    
}

pub fn sphere_light_builder(centre : Vector, radius : f64, emission : f64) -> Object
{
    let material = Material::new(0x000000, MatType::Diffuse, emission);

    let sphere = Sphere::new(centre, radius, material);

    return Object{ body: Box::new(sphere) };
}