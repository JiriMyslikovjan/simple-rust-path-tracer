use std::process::Command;
use std::io;
use std::fs;
use crate::json_utils::{get_scene_from_json, get_scenes_folder};
use crate::scene::Scene;
use crate::render::render;
use crate::image::Image;

const ILLEGAL_SYMBOLS: &[char] = &['\\', '/', ':', '*', '?', '"', '<', '>', '|', '.'];

fn get_gui_script_path() -> String
{
    let mut path_to_exe = match  std::env::current_exe()
    {
        Ok(path) =>
        { path }
        
        Err(_) => 
        { panic!("Error occured while getting path!"); }
    };

    path_to_exe.pop();
    let path_string = path_to_exe.to_str().unwrap();

    let script_path : String;
    if cfg!(target_os = "windows")
    { script_path = format!("{}{}", path_string, "\\scene_editor\\scene_editor.py"); }
    else
    { script_path = format!("{}{}", path_string, "/scene_editor/scene_editor.py"); };
    

    return script_path;
}

fn run_gui_script()
{
    let interpreter = 
    
    if cfg!(target_os = "windows")
    { "python.exe" }
    
    else
    { "python" };

    let script_path = get_gui_script_path();

    let command_output = Command::new(interpreter).args(&[script_path]).output();

    if ! command_output.unwrap().status.success()
    { println!("Opening scene editor failed.") }
}

pub fn get_scenes() -> Vec<String>
{
    let scenes_path = get_scenes_folder();
    let mut found_scenes : Vec<String >= Vec::new();

    let scenes : Vec<_> = fs::read_dir(scenes_path).expect("Reading from scenes folder failed.")
    .filter_map(|entry|
        {
            let entry = entry.expect("Reading entry from scenes folder failed");
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "json")
            { Some(path.file_name().unwrap().to_string_lossy().into_owned()) }
            else { None }
        }).collect();

        for scene in scenes
        { found_scenes.push(scene); }

        return found_scenes;
}

fn get_file_name() -> String
{
    let mut file_name = String::new();
    let mut correct = false;

    while ! correct
    {
        println!("Name your render(eg. frame):");
        
        io::stdin().read_line(&mut file_name).expect("Reading file name failed.");
        
        let trimmed_input = file_name.trim();

        if trimmed_input.chars().any(|c| ILLEGAL_SYMBOLS.contains(&c))
        { println!("Please refrain from using any illegal symbols"); }
        
        else 
        { correct = true }
    }
    
    let trimmed_file_name = file_name.trim_matches(|c| c == '\n' || c == '\r');
    let formatted_file_name = format!("{}{}", trimmed_file_name, ".ppm");

    return formatted_file_name
}

fn render_scene_dialog(scene : Scene)
{
    let mut correct = false;

    while ! correct
    {
        println!("Choose spp(samples per pixel):");
        
        let mut spp_choice = String::new();

        io::stdin().read_line(&mut spp_choice).expect("Reading scene choice failed.");
        let trimmed_input = spp_choice.trim();

        let parsed_input = trimmed_input.parse::<u32>();

        let file_name = get_file_name();

        match parsed_input
        {
            Ok(spp) => correct = render_scene(&scene, spp, file_name),
            Err(..) => println!("Incorrect scene choice input format.")
        };
    }
}

fn render_scene(scene : &Scene, spp : u32, file_name : String) -> bool
{
    let width = scene.camera.img_width;
    let height = scene.camera.img_height;
    let image = Image::new(width, height);
    let image_name = format!("{}", file_name);

    render(image,&image_name, scene, spp);

    return true;
}

fn scene_choice_correct(scene_num : i32, scenes : &Vec<String>) -> bool
{ return ! (scene_num > scenes.len() as i32 || scene_num < 1); }

fn load_and_render_scene()
{
    let mut end = false;
    let scenes = get_scenes();
    let mut choice_correct = false;

    if scenes.len() <= 0
    {
        println!("No scenes present in scenes folder");

        return;
    }

    while ! end 
    {
        println!("\nAvailible scenes:");
        
        for i in 0..scenes.len()
        { println!("{} {}", i + 1, scenes[i]); }

        println!("\nChoose scene: ");

        let mut scene_choice = String::new();

        io::stdin().read_line(&mut scene_choice).expect("Reading scene choice failed.");
        let trimmed_input = scene_choice.trim();

        let parsed_input = trimmed_input.parse::<i32>();

        match parsed_input
        {
            Ok(choice) => choice_correct = scene_choice_correct(choice , &scenes),
            Err(..) => println!("Incorrect input format.")
        };

        if choice_correct
        {
            let choice = parsed_input.unwrap() - 1;
            let scene_name = &scenes[choice as usize];

            let scene = get_scene_from_json(scene_name);
            render_scene_dialog(scene);

            end = true;
        }
    }
}

fn execute_choice(choice : i32) -> bool
{
    match choice
    {
        1 => run_gui_script(),
        2 => load_and_render_scene() ,
        3 => return true,
        _ => println!("Incorrect choice input.")
    }

    return  false;
}

pub fn main_loop()
{
    let mut end = false;
    
    while ! end
    {
        println!("1 Open scene editor\n2 Load and render scene\n3 Exit");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Reading user choice failed.");
        let trimmed_input = user_input.trim();

        match trimmed_input.parse::<i32>()
        {
            Ok(choice) => end = execute_choice(choice),
            Err(..) => println!("Incorrect input format.")
        };
    }
}
