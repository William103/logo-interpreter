#![allow(dead_code)]

use sdl2::render::Canvas;

use crate::parser::{Block, Command};

pub struct Turtle {
    x: f64,
    y: f64,
    angle: f64,
    pen_up: bool,
}

impl Turtle {
    pub fn new(x: f64, y: f64) -> Turtle {
        Turtle {
            x,
            y,
            angle: 0.0,
            pen_up: false,
        }
    }
}

pub fn execute_block<T: sdl2::render::RenderTarget>(
    block: &Block,
    turtle: &mut Turtle,
    canvas: &mut Canvas<T>,
) -> Result<(), String> {
    for command in &block.commands {
        execute_command(&command, turtle, canvas)?;
    }
    Ok(())
}

fn execute_command<T: sdl2::render::RenderTarget>(
    command: &Command,
    turtle: &mut Turtle,
    canvas: &mut Canvas<T>,
) -> Result<(), String> {
    match command.name.as_str() {
        "forward" | "fd" => {
            if let Some(dist) = command.argument {
                let old_x = turtle.x;
                let old_y = turtle.y;
                turtle.x += turtle.angle.cos() * dist as f64;
                turtle.y += turtle.angle.sin() * dist as f64;

                canvas.draw_line(
                    (old_x.round() as i32, old_y.round() as i32),
                    (turtle.x.round() as i32, turtle.y.round() as i32),
                )
            } else {
                Err("Expected argument".to_string())
            }
        }
        "backward" | "bk" => {
            if let Some(dist) = command.argument {
                let old_x = turtle.x;
                let old_y = turtle.y;
                turtle.x -= turtle.angle.cos() * dist as f64;
                turtle.y -= turtle.angle.sin() * dist as f64;

                canvas.draw_line(
                    (old_x.round() as i32, old_y.round() as i32),
                    (turtle.x.round() as i32, turtle.y.round() as i32),
                )
            } else {
                Err("Expected argument".to_string())
            }
        }
        "left" | "lt" => {
            if let Some(amount) = command.argument {
                turtle.angle += amount as f64 / 180.0 * std::f64::consts::PI;
                Ok(())
            } else {
                Err("Expected argument".to_string())
            }
        }
        "right" | "rt" => {
            if let Some(amount) = command.argument {
                turtle.angle -= amount as f64 / 180.0 * std::f64::consts::PI;
                Ok(())
            } else {
                Err("Expected argument".to_string())
            }
        }
        "penup" | "pu" => {
            if let Some(_) = command.argument {
                Err("Unexpected argument".to_string())
            } else {
                turtle.pen_up = true;
                Ok(())
            }
        }
        "pendown" | "pd" => {
            if let Some(_) = command.argument {
                Err("Unexpected argument".to_string())
            } else {
                turtle.pen_up = false;
                Ok(())
            }
        }
        "repeat" | "rp" => {
            if let Some(n) = command.argument {
                if let Some(block) = &command.block {
                    for _ in 0..n {
                        execute_block(block, turtle, canvas)?;
                    }
                    Ok(())
                } else {
                    Err("Expected block".to_string())
                }
            } else {
                Err("Expected argument".to_string())
            }
        }
        _ => Err("Unknown command".to_string())
    }
}
