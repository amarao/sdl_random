extern crate sdl2; 

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use lib::{noise_flakes, fade_in_out};
use lib::Factor;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 noise demo",0, 0)
        .fullscreen_desktop()
        .borderless()
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        // .software()
        .build().unwrap();
    sdl_context.mouse().show_cursor(false);
    let (width, height) = canvas.output_size().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::ABGR8888, width as u32, height as u32)
        .unwrap();
    let mut background = texture_creator
        .create_texture_static(sdl2::pixels::PixelFormatEnum::ABGR8888, width as u32, height as u32)
        .unwrap();
    background.update(
        None, &vec![0; (width*height) as usize], (width*4) as usize
    ).unwrap();
    background.set_blend_mode(sdl2::render::BlendMode::Mod);
    texture.set_blend_mode(sdl2::render::BlendMode::Add);
    let mut start = std::time::Instant::now();
    let  mut last_printed:u64 = 0;
    let mut frames:u64 = 0;
    let mut factor = Factor::new(0xDEADBEAF_ABBADEAD_12345678_AABBCCDD);
    'running: loop {
        frames +=1;
        if frames % 4 == 0 {
            texture.with_lock(
                None,
                |bytearray, _| {noise_flakes(bytearray, & mut factor)}
            ).unwrap();
        }
        else{
            texture.with_lock(
                None,
                |bytearray, _| {fade_in_out(bytearray, frames)}
            ).unwrap();
        }
        let new_x = factor.next128() as u32 % width;
        let new_y = factor.next128() as u32 % height;
        let new_width = factor.next128() as u32 % (width-new_x);
        let new_height = factor.next128() as u32 % (height-new_y);
        canvas.copy(&background, None, None).unwrap();
        canvas.copy(&texture, None, sdl2::rect::Rect::new(new_x as i32, new_y as i32, new_width, new_height)).unwrap();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        // canvas.set_viewport(Some(sdl2::rect::Rect::new(new_x as i32, new_y as i32, new_width, new_height)));
        
        canvas.present();

        if start.elapsed() > std::time::Duration::new(1, 0){
            let dt = start.elapsed().as_secs_f64();
            let fc = frames - last_printed;
            last_printed = frames;
            start = std::time::Instant::now();
            println!("{:.1}", fc as f64/dt);
        }

    }
}