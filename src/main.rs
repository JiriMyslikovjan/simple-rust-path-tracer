use crate::color::Color;
use crate::image::Image;
use crate::material::MatType;
use crate::vector::Vector;
extern crate rand;
use crate::user_input::main_loop;

mod vector;
mod ray;
mod color;
mod material;
mod object;
mod image;
mod render;
mod scene;
mod random;
mod camera;
mod user_input;
mod json_utils;
mod object_builder;

fn main()
{
     main_loop();
}