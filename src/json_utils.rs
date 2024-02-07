use serde::{Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use crate::scene::{Scene};
use crate::color::{string_to_hex_int};
use crate::material::{MatType};
use crate::vector::Vector;
use crate::camera::Camera;
use crate::object_builder::{plane_builder, sphere_builder, sphere_light_builder};

// Structures for serde deserialization
#[derive(Debug, Deserialize, Clone)]
struct JsonMaterial
{
    #[serde(rename = "type")]
    material_type : String,
    color : String
}

#[derive(Debug, Deserialize, Clone)]
struct JsonResolution
{
    width : u32,
    height : u32
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
enum JsonObject 
{
    #[serde(rename = "plane")]
    JsonPlane 
    {
        normal : Option<[f64; 3]>,
        d : Option<f64>,
        material : JsonMaterial
    },
    
    #[serde(rename = "sphere")]
    JsonSphere
    {
        center : Option<[f64; 3]>,
        radius : Option<f64>,
        material: JsonMaterial
    },
    
    #[serde(rename = "light")]
    JsonLight
    {
        center : Option<[f64; 3]>,
        radius : Option<f64>,
        emission: Option<f64>
    },

    #[serde(rename = "camera")]
    JsonCamera
    {
        look_from : Option<[f64; 3]>,
        look_at : Option<[f64; 3]>,
        vup : Option<[f64; 3]>,
        fov : Option<f64>,
        resolution : JsonResolution
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    objects: Vec<JsonObject>,
}

fn scene_add_planes(object_map : &HashMap<String, Vec<JsonObject>>, mut scene : Scene) -> Scene
{
    if let Some(objects) =  object_map.get("plane")
    {
        for object in objects 
        {
            if let JsonObject::JsonPlane 
            {
                normal,
                d,
                material
            } = object
            {
                let unwrapped_normal = normal.unwrap();
                let unwrapped_d = d.unwrap();
                let unwrapped_mat_type = material.material_type.clone();
                let unwrapped_color = material.color.clone();

                let mut plane_normal = Vector::new();
                plane_normal.set_vector(unwrapped_normal[0], unwrapped_normal[1], unwrapped_normal[2]);

                let plane_mat_type: MatType = match unwrapped_mat_type.as_str() {
                    "diffuse" => MatType::Diffuse,
                    "specular" => MatType::Specular,
                    _ => MatType::Refractive
                };

                let plane = plane_builder(plane_normal, unwrapped_d, plane_mat_type , string_to_hex_int(&unwrapped_color));

                scene.add(plane);
            }
        }
    }

    return scene
}

fn scene_add_spheres(object_map : &HashMap<String, Vec<JsonObject>>, mut scene : Scene) -> Scene
{
    if let Some(objects) =  object_map.get("sphere") 
    {
        for object in objects 
        {
            if let JsonObject::JsonSphere 
            {
                center,
                radius,
                material
            } = object
            {
                let unwrapped_center = center.unwrap();
                let unwrapped_radius = radius.unwrap();
                let unwrapped_mat_type = material.material_type.clone();
                let unwrapped_color = material.color.clone();

                let mut sphere_center = Vector::new();
                sphere_center.set_vector(unwrapped_center[0], unwrapped_center[1], unwrapped_center[2]);

                let plane_mat_type: MatType = match unwrapped_mat_type.as_str() {
                    "diffuse" => MatType::Diffuse,
                    "specular" => MatType::Specular,
                    _ => MatType::Refractive,
                };

                let plane = sphere_builder(sphere_center, unwrapped_radius, plane_mat_type , string_to_hex_int(&unwrapped_color));

                scene.add(plane);
            }
        }
    }

    return scene
}

fn create_scene_and_add_camera(object_map : &HashMap<String, Vec<JsonObject>>) -> Scene
{
    let unwrapped_look_from;
    let unwrapped_look_at;
    let unwrapped_vup;
    let mut unwrapped_fov : f64 = 0.0;
    let mut unwrapped_width : u32 = 0;
    let mut unwrapped_height : u32 = 0;

    let mut camera_look_from = Vector::new();
    let mut camera_look_at = Vector::new();
    let mut camera_vup = Vector::new();

    if let Some(objects) =  object_map.get("camera")
    {
        if let Some(camera) = objects.first()
        {
            if let JsonObject::JsonCamera 
            {
                look_from,
                look_at,
                vup,
                fov,
                resolution,
            } = camera
            {
                unwrapped_look_from = look_from.unwrap();
                unwrapped_look_at = look_at.unwrap();
                unwrapped_vup = vup.unwrap();
                unwrapped_fov = fov.unwrap();
                unwrapped_width = resolution.width;
                unwrapped_height = resolution.height;

                camera_look_from.set_vector(unwrapped_look_from[0], unwrapped_look_from[1], unwrapped_look_from[2]);
                camera_look_at.set_vector(unwrapped_look_at[0], unwrapped_look_at[1], unwrapped_look_at[2]);
                camera_vup.set_vector(unwrapped_vup[0], unwrapped_vup[1], unwrapped_vup[2]);
            }
        }
    }
    let cam = Camera::new(camera_look_at, camera_look_from, camera_vup, unwrapped_fov, unwrapped_width, unwrapped_height);
    let scene = Scene{ objects: vec![], camera : cam };

    return scene
}

fn scene_add_lights(object_map : &HashMap<String, Vec<JsonObject>>, mut scene : Scene) -> Scene
{
    if let Some(objects) =  object_map.get("light") {
        for object in objects 
        {
            if let JsonObject::JsonLight 
            {
                center,
                radius,
                emission
            } = object
            {
                let unwrapped_center = center.unwrap();
                let unwrapped_radius = radius.unwrap();
                let unwrapped_emission = emission.unwrap();

                let mut light_center = Vector::new();
                light_center.set_vector(unwrapped_center[0], unwrapped_center[1], unwrapped_center[2]);

                let light = sphere_light_builder(light_center, unwrapped_radius, unwrapped_emission);

                scene.add(light);
            }
        }
    }
    
    return scene
}

fn scene_builder(object_map : HashMap<String, Vec<JsonObject>>) -> Scene
{
    //let mut scene = Scene{ objects: vec![] , camera : Camera};
    let mut scene = create_scene_and_add_camera(&object_map);
    scene = scene_add_planes(&object_map, scene);
    scene = scene_add_spheres(&object_map, scene);
    scene = scene_add_lights(&object_map, scene);
    
    return scene
}

pub fn get_scenes_folder() -> String
{
    let mut path_to_exe = match  std::env::current_exe()
    {
        Ok(path) =>
        {
            path
        }
        Err(_) => 
        {
            panic!("Error occured while getting path!");
        }
    };
    path_to_exe.pop();
    let path_string = path_to_exe.to_str().unwrap();

    //println!("{}", path_string);

    
    let scenes_path : String;
    if cfg!(target_os = "windows")
    { scenes_path = format!("{}{}", path_string, "\\scenes\\"); }
    else
    { scenes_path = format!("{}{}", path_string, "/scenes/"); };
    
    //let scenes_path =format!("{}{}", path_string, "\\scenes\\" );

    //println!("{}", scenes_path);

    return scenes_path;
}

fn deserialize_json(scene_name : &String) -> Result<HashMap<String, Vec<JsonObject>>, Box<dyn std::error::Error>>
{
    let scenes_path = get_scenes_folder();
    let file_path = format!("{}{}", scenes_path, scene_name);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader).map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    let mut object_map: HashMap<String, Vec<JsonObject>> = HashMap::new();

    for object in config.objects {
        match &object {
            JsonObject::JsonPlane { material, .. } => 
            {
                object_map
                    .entry("plane".to_string())
                    .or_insert(Vec::new())
                    .push(object.clone());
                object_map
                    .entry(material.material_type.clone())
                    .or_insert(Vec::new())
                    .push(object);
            }
            JsonObject::JsonSphere { material, .. } => 
            {
                object_map
                    .entry("sphere".to_string())
                    .or_insert(Vec::new())
                    .push(object.clone());
                object_map
                    .entry(material.material_type.clone())
                    .or_insert(Vec::new())
                    .push(object);
            }
            JsonObject::JsonLight { .. } => 
            {
                object_map
                    .entry("light".to_string())
                    .or_insert(Vec::new())
                    .push(object);
            }
            JsonObject::JsonCamera { .. } => 
            {
                object_map
                    .entry("camera".to_string())
                    .or_insert(Vec::new())
                    .push(object);
            }
        }
    }
    
    Ok(object_map)
}

pub fn get_scene_from_json(scene_name : &String) -> Scene
{
    let object_map_result = deserialize_json(scene_name);
    let object_map = object_map_result.unwrap();

    scene_builder(object_map)
}