use std::{io::Write};
use crate::Color;

#[derive(Clone)]
pub struct Image
{
    pub width : u32,
    pub height : u32,
    pub buffer : Vec<Vec<Color>>
}

fn to_writable_byte(val : f64) -> u8
{
    let result = val.clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0 + 0.5;

    return result as u8;
}

impl Image
{
    pub fn new(w : u32, h: u32 ) -> Self
    {
        Image
        {
            width : w,
            height : h,
            buffer : vec![vec![Color::new_rgb(0.0, 0.0, 0.0); w as usize]; h as usize]
        }
    }

    fn to_writable_buff(self) -> Vec<u8>
    {
        let  buff_size = self.height * self.width * 3;
        let mut buffer = Vec::with_capacity(buff_size as usize);
        buffer.resize(buff_size as usize, 0);
        let mut buff_index = 0;

        for col in self.buffer.iter().rev()
        {
            for row in col.iter().rev()
            {
                buffer[buff_index] = to_writable_byte(row.r);
                buffer[buff_index + 1] = to_writable_byte(row.g);
                buffer[buff_index + 2] = to_writable_byte(row.b);

                buff_index += 3;
            }
        }

        return buffer;
    }

    pub fn write_to_ppm(self, file_name : &str) -> std::io::Result<()>
    {
        let file_path  = format!("{}{}", get_renders_folder(), file_name);

        let buffer = self.clone().to_writable_buff();
        
        let ppm_head = format!("P3\n{} {}\n{}\n", self.width, self.height, 255);
        
        let mut file = std::fs::File::create(file_path)?;
        file.write(ppm_head.as_bytes())?;
        let indent = 1;

        for byte in buffer
        {
            let byte_write = format!("{} ", byte);

            file.write(byte_write.as_bytes())?;

            if indent % self.width * 3 == 0
            {
                file.write("\n".as_bytes())?;
            }
        }

        Ok(())
    }
}

fn get_renders_folder() -> String
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

    let renders_path : String;
    
    if cfg!(target_os = "windows")
    { renders_path = format!("{}{}", path_string, "\\renders\\"); }
    
    else
    { renders_path = format!("{}{}", path_string, "/renders/"); };

    return renders_path;
}