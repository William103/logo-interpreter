use logos::Logos;

mod tokenizer;
mod parser;
mod turtle;

use parser::parse;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tokenizer::Token;
use turtle::{Turtle, execute_block};
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("logo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    // let code = "fd 30 rt 120 fd 30 rt 120 fd 30";
    let code = "rp 36 [lt 10 pu fd 1 pd rp 120 [fd 2 rt 3]]";
    let mut lex = Token::lexer(code);
    let block = parse(&mut lex, false).expect("Error in parsing");
    let mut turtle = Turtle::new(400.0, 300.0);
 
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        execute_block(&block, &mut turtle, &mut canvas).expect("ERROR");
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
