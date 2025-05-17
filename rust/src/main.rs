use std::fs;
use serde_json::Value;
use std::io::Write;

fn exec_json(data: String) {
    let v: Value = serde_json::from_str(&data).unwrap();

    if let Some(func) = v["func"].as_str() {
        match func {
            "fill" => {
                let pixels_array = v["pixels"].as_array().expect("ERROR: Expected pixels to be an array of u64");
                let pixels = pixels_array.get(0).and_then(|v| v.as_u64()).unwrap_or(0) as u64;
                let mut pix: Vec<u64> = vec![0; pixels.try_into().unwrap()];

                let width = v["width"].as_u64().expect("ERROR: Expected width to be an nuber");
                let height = v["height"].as_u64().expect("ERROR: Expected height to be a nuber");
                let color = v["color"].as_u64().expect("ERROR: Expected color to be a number");
                
                acqua_fill(&mut pix, width.try_into().unwrap(), height.try_into().unwrap(), color);
            }
            "fill to ppm" => {
                let pixels_array = v["pixels"].as_array().expect("ERROR: Expected pixels to be an array of u64");
                let pixels = pixels_array.get(0).and_then(|v| v.as_u64()).unwrap_or(0) as u64;
                let mut pix: Vec<u64> = vec![0; pixels.try_into().unwrap()];

                let width = v["width"].as_u64().expect("ERROR: Expected width to be an nuber");
                let height = v["height"].as_u64().expect("ERROR: Expected height to be a nuber");
                let color = v["color"].as_u64().expect("ERROR: Expected color to be a number");

                let file_path = v["file path"].as_str().expect("ERROR: Expected file path to be a string");
                
                acqua_fill(&mut pix, width.try_into().unwrap(), height.try_into().unwrap(), color);
                acqua_save_to_ppm(&pix, width.try_into().unwrap(), height.try_into().unwrap(), file_path);
            }
            _ => {
                println!("ERROR: unrecognized func name: {}", func);
            }
        }
    }
}

fn read_json_file() -> String {
    let data = fs::read_to_string("../main.json").unwrap();
    data
}

fn acqua_fill(pixels: &mut Vec<u64>, width: usize, height: usize, color: u64) {
   for i in 0..width*height {
        pixels[i] = color;
    }
}

fn acqua_save_to_ppm(pixels: &Vec<u64>, width: usize, height: usize, file_path: &str) {
    let mut file = fs::File::create(file_path).unwrap();
    let header = format!("P6\n{} {}\n255\n", width, height);
    file.write_all(header.as_bytes()).unwrap();

    let mut buffer = Vec::with_capacity(width * height * 3);

    for &pixel in pixels {
        let r = ((pixel >> 8*0) & 0xFF) as u8;
        let g = ((pixel >> 8) & 0xFF) as u8;
        let b = (pixel >> 8*2) as u8;
        buffer.extend_from_slice(&[r, g, b]);
    }

    file.write_all(&buffer).unwrap();
}

pub fn main() {
    let data = read_json_file();
    exec_json(data);
}
