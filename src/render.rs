use rayon::prelude::*;
use std::time::Instant;
use std::io::{self};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::{Color, Image, MatType, random, Vector};
use crate::vector::orthonormal_sys;
use crossterm::{execute, cursor, terminal};

const REFRACTION_INDEX : f64 = 1.52;

// Code for both sampling methods based on www.rorydriscoll.com/2009/01/07/better-sampling/

#[allow(dead_code)]
fn uniform_hemisphere_sample(u1 : f64, u2 : f64) -> Vector
{
    let r = (1.0 - u1 * u1).sqrt();
    let phi = 2.0 * std::f64::consts::PI * u2;
    
    Vector
    {
        x: phi.cos() * r,
        y: phi.sin() * r,
        z: u1
    }
}

pub fn cosine_weighted_hemisphere_sample(u1 : f64, u2 : f64) -> Vector
{
    let r = u1.sqrt();
    let theta = 2.0 * std::f64::consts::PI * u2;
    let x = r * theta.cos();
    let y = r * theta.sin();

    
    Vector
    {
        x: x,
        y: y,
        z: (1.0 - u1).max(0.0).sqrt()
    }
}

fn diffuse_brdf(surface_normal : Vector, ray : &mut Ray) -> f64
{
    let mut rot_x = Vector::new();
    let mut rot_y = Vector::new();

    orthonormal_sys(surface_normal, &mut rot_x, &mut rot_y);
    
    let u1 = random::gen_num();
    let u2 = random::gen_num();

    // Sample random direction on a hemisphere
    let sampled_dir = cosine_weighted_hemisphere_sample(u1, u2);

    let mut rot_dir = Vector::new();
    
    // Rotate the sampled direction 
    rot_dir.x = Vector::new().set_vector(rot_x.x, rot_y.x, surface_normal.x).dot_product(sampled_dir);
    rot_dir.y = Vector::new().set_vector(rot_x.y, rot_y.y, surface_normal.y).dot_product(sampled_dir);
    rot_dir.z = Vector::new().set_vector(rot_x.z, rot_y.z, surface_normal.z).dot_product(sampled_dir);

    ray.dest = rot_dir;

    let cos_theta = ray.dest.dot_product(surface_normal);

    return  cos_theta;
}

fn specular_brdf(surface_normal : Vector, ray : &mut Ray)
{
    let cos_theta = ray.dest.dot_product(surface_normal);
    
    ray.dest = ray.dest - surface_normal * (cos_theta * 2.0);
    ray.dest.normalize();
}

fn refractive_brdf(mut surface_normal : Vector, ray : &mut Ray)
{
    let mut n = REFRACTION_INDEX;
    let mut r0 = (1.0 - n) / (1.0 + n);

    r0 = r0 * r0;

    if surface_normal.dot_product(ray.dest) > 0.0
    {
        surface_normal = surface_normal * -1.0;
        n = 1.0 / n;
    }

    n = 1.0 / n;
    
    let cos_theta1 = surface_normal.dot_product(ray.dest) * -1.0;
    let cos_theta2 = 1.0 - n * n * (1.0 - cos_theta1 * cos_theta1);
    
    // Refraction probabilaty via Shlick's approximation
    let r_prob = r0 + (1.0 - r0) * (1.0 - cos_theta1).powf(5.0);

    // Refract
    if cos_theta2 > 0.0 && random::gen_num() > r_prob
    {
        // Snell's law
        ray.dest = (ray.dest * n) + (surface_normal * (n * cos_theta1 - cos_theta2.sqrt()));
        ray.dest.normalize();
    }

    // Reflect
    else
    {
        ray.dest = ray.dest + surface_normal * (cos_theta1 * 2.0);
        ray.dest.normalize();
    }
}

fn trace(scene : &Scene, mut ray: Ray, depth : i32, mut color: &mut Color)
{
    // Russian roulette 
    let mut rr_factor = 1.0;
    
    // Prevent early path termination
    if depth >= 5
    {
        let rr_stop = 0.1;

        // End recursion
        if random::gen_num() <= rr_stop
        { return; }
        
        rr_factor = 1.0 / (1.0 - rr_stop);
    }

    // Find intercestion
    let intersection = scene.intersect(ray);

    // End recursion if no intersection is found
    if ! intersection.found
    { return; }

    // Calculate hit point of a ray and an objecet
    let hit_point = ray.origin + ray.dest * intersection.distance;
    let surface_normal = intersection.object.body.normal(hit_point);

    // Hit point becomes new rays origin
    ray.origin = hit_point;

    let emission = intersection.object.body.get_material().emission;
    
    // Add emmission of an object to accumulated color 
    color.r = color.r + (emission * rr_factor);
    color.g = color.g + (emission * rr_factor);
    color.b = color.b + (emission * rr_factor);


    // Diffuse BRDF
    if intersection.object.body.get_material().mat_type == MatType::Diffuse
    {
        let cos_theta = diffuse_brdf(surface_normal, &mut ray);
        let mut tmp = Color::new_rgb(0.0,0.0,0.0);

        trace(scene, ray, depth + 1, &mut tmp);

        let object_color = intersection.object.body.get_material().color;
        tmp = tmp.mul_by_color(object_color);

        color.r = color.r + (tmp.r * cos_theta * rr_factor);
        color.g = color.g + (tmp.g * cos_theta * rr_factor);
        color.b = color.b + (tmp.b * cos_theta * rr_factor);
    }

    // Specular BRDF
    if intersection.object.body.get_material().mat_type == MatType::Specular
    {
        specular_brdf(surface_normal, &mut ray);

        let mut tmp = Color::new_rgb(0.0,0.0,0.0);

        trace(scene, ray, depth + 1, &mut tmp);

        color.r = color.r + tmp.r * rr_factor;
        color.g = color.g + tmp.g * rr_factor;
        color.b = color.b + tmp.b * rr_factor;
    }

    // Refraractive BRDF
    if intersection.object.body.get_material().mat_type == MatType::Refractive
    {
        refractive_brdf(surface_normal, &mut ray);

        let mut tmp = Color::new_rgb(0.0, 0.0, 0.0);

        trace(scene, ray, depth + 1, &mut tmp);

        color.r = color.r + (tmp.r * rr_factor);
        color.g = color.g + (tmp.g * rr_factor);
        color.b = color.b + (tmp.b * rr_factor);
    }
}

// Attempt at clearing the terminal on both platforms
fn clear_terminal()
{
    if let Err(err) = execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    {
        eprintln!("Clearing of terminal error {}", err);
    }
}

pub fn render(mut img: Image, image_name : &String, scene : &Scene, spp : u32)
{
    let benchmark = Instant::now();

    for row in 0..(img.height) as usize
    {
        println!("{}% Done", row as f64 / img.height as f64 * 100.0);
        
        clear_terminal();

        img.buffer[row].par_iter_mut().enumerate().for_each(|(pixel_num, pixel)|
        {
            for _i in 0..spp
            {
                let u = pixel_num as f64 + random::gen_num();
                let v = row as f64 + random::gen_num();
                let mut clr = Color::new_rgb(0.0, 0.0, 0.0);
                let ray = scene.camera.get_ray(u, v);

                trace(&scene, ray, 0, &mut clr);

                pixel.r = pixel.r + clr.r * (1.0 / spp as f64) * 0.25;
                pixel.g = pixel.g + clr.g * (1.0 / spp as f64) * 0.25;
                pixel.b = pixel.b + clr.b * (1.0 / spp as f64) * 0.25;
            }
        });
    }

    let elapsed = benchmark.elapsed();

    println!("Render took {} minutes and {} seconds", elapsed.as_secs() / 60, elapsed.as_secs() % 60);
    println!("Render finished");

    let result = img.write_to_ppm(&image_name);

    match result
    {
        Ok(_ret_val) => println!("Writing to file was successful"),
        Err(_ret_val) => panic!("An error occured while writing to file!")
    }

}