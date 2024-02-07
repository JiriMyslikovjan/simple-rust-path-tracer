use crate::Vector;
use crate::ray::*;

// Camera based on Peter Shirley's Ray Tracing In One Weekend book
pub struct Camera
{
    origin : Vector,
    horizontal : Vector,
    vertical : Vector,
    lower_left_corner : Vector,
    pub img_width : u32,
    pub img_height : u32
}

impl Camera
{
    pub fn new(look_at : Vector, look_from : Vector, vup : Vector, vertical_fov : f64, width : u32, height : u32) -> Self
    {

        let aspect_ratio = width as f64 / height as f64;
        let theta = std::f64::consts::PI / 180.0 * vertical_fov;

        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let mut cw = look_from - look_at;
        cw.normalize();
        let mut cu = vup.cross_product(cw);
        cu.normalize();
        let cv = cw.cross_product(cu);

        let _horizontal = cu * viewport_width;
        let _vertical = cv * viewport_height;
        let _lower_left_corner = look_from - _horizontal * (1.0 / 2.0) - _vertical * (1.0 / 2.0) - cw;


        Camera
        {
            origin: look_from,
            horizontal: _horizontal,
            vertical: _vertical,
            lower_left_corner : _lower_left_corner,
            img_width : width,
            img_height : height
        }
    }
    
    pub fn get_ray(&self, x : f64, y : f64) -> Ray
    {
        let u = (x as f64 + 0.5) / self.img_width as f64;
        let v = (y as f64 + 0.5) / self.img_height as f64;

        let mut new_ray = crate::ray::Ray::new();

        let new_ray_origin = self.origin;
        let new_ray_dest = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;

        new_ray.set_origin(new_ray_origin.x, new_ray_origin.y, new_ray_origin.z);
        new_ray.set_dest(new_ray_dest.x, new_ray_dest.y, new_ray_dest.z);

        return new_ray;
    }
}
