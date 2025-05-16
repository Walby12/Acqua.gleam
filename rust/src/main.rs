extern crate sdl2;

use std::fs;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use serde_json::Value;

fn exec_json(data: String) {
    let v: Value = serde_json::from_str(&data).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

   
    if let Some(func) = v["func"].as_str() {
        match func {
            "init_window" => {
                let win_name = v["name"].as_str().unwrap_or("gleam sdl2");
                let rgb_array = v["color"].as_array().expect("Expected color to be an array");

                let r = rgb_array.get(0).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                let g = rgb_array.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                let b = rgb_array.get(2).and_then(|v| v.as_u64()).unwrap_or(0) as u8;

                let window: Window = video_subsystem
                    .window(win_name, 800, 600)
                    .position_centered()
                    .build()
                    .expect("Failed to create window");

                let mut canvas: Canvas<Window> = window
                    .into_canvas()
                    .build()
                    .expect("Failed to create canvas");

                canvas.set_draw_color(Color::RGB(r,g,b));
                canvas.clear();
                canvas.present();
            }

            _ => {
                eprintln!("Unknown function: {}", func);
            }
        }
    } else {
        eprintln!("Missing 'func' field in JSON");
    }
    
}

fn read_json_file() -> String {
    let data = fs::read_to_string("../main.json").unwrap();
    data
}

pub fn main() {
    let data = read_json_file();
    exec_json(data);


}
