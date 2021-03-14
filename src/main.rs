extern crate sdl2; 

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use lib::{*};
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
        // .present_vsync()
        .accelerated()
        .build().unwrap();
    sdl_context.mouse().show_cursor(false);
    let (width, height) = canvas.output_size().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::ABGR8888, width as u32, height as u32)
        .unwrap();
    let mut whole_screen = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::ABGR8888, width as u32, height as u32)
        .unwrap();
    whole_screen.update( //fill?
        None, &vec![0; (width*height*4) as usize], (width*4) as usize
    ).unwrap();
    whole_screen.set_blend_mode(sdl2::render::BlendMode::None);
    texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    let mut start = std::time::Instant::now();
    let  mut last_printed:u64 = 0;
    let mut frames:u64 = 0;
    let mut factor = Factor::new(0xDEADBEAF_ABBADEAD_12345678_AABBCCDD);
    'running: loop {
        frames +=1;
        let rect_x = (frames % width as u64) as u32;
        let rect_y = (frames % height as u64) as u32;
        let rect_width = width-rect_x;
        let rect_height =  height-rect_y;
        texture.with_lock(
            None,
            |bytearray, _| {noise_fill(bytearray, & mut factor)}
        ).unwrap();
        whole_screen.with_lock(
                None,
                |bytearray, _| {fade_in_out(bytearray, frames)}
            ).unwrap();
        // if frames % 32 == 0{
        //     canvas.copy(&background, None, None).unwrap();
        // }

        canvas.copy(&whole_screen, None, None).unwrap();
        canvas.copy(&texture, None, sdl2::rect::Rect::new(rect_x as i32, rect_y as i32, rect_width, rect_height)).unwrap();
        
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
            println!("\nfps: {:.1}\n", fc as f64/dt);
        }

    }
}