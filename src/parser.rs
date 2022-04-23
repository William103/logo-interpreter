#![allow(dead_code)]

use crate::tokenizer::Token;
use logos::Lexer;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub block: Option<Block>,
    pub argument: Option<u64>,
}

#[derive(Debug)]
pub struct Block {
    pub commands: Vec<Command>,
}

impl Block {
    fn new() -> Block {
        Block {
            commands: Vec::new(),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let argument = self
            .argument
            .and_then(|n| Some(n.to_string()))
            .unwrap_or("".to_string());
        let block = self
            .block
            .as_ref()
            .and_then(|blk| Some(blk.to_string()))
            .unwrap_or("".to_string());
        write!(f, "{} {} {}", self.name, argument, block)
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for command in &self.commands {
            writeln!(f, "{}", command)?;
        }
        writeln!(f, "]")?;

        Ok(())
    }
}

pub fn parse(lex: &mut Lexer<Token>, in_block: bool) -> Result<Block, &'static str> {
    let mut block = Block::new();

    while let Some(token) = lex.next() {
        match token {
            Token::LBracket => return Err("Unexpected '['"),
            Token::RBracket => {
                if in_block {
                    return Ok(block);
                } else {
                    return Err("Unexpected ']'");
                }
            }
            Token::Number(_) => return Err("Unexpected number"),
            Token::Command(command) => {
                block.commands.push(handle_command(command, lex)?);
            }
            Token::Error => return Err("Invalid token!"),
        }
    }

    Ok(block)
}

fn handle_command(command: String, lex: &mut Lexer<Token>) -> Result<Command, &'static str> {
    match command.as_str() {
        "fd" | "forward" | "bk" | "backward" | "lt" | "left" | "rt" | "right" => {
            if let Some(Token::Number(arg)) = lex.next() {
                Ok(Command {
                    name: command,
                    block: None,
                    argument: Some(arg),
                })
            } else {
                Err("Expected number!")
            }
        }
        "pu" | "penup" | "pd" | "pendown" => Ok(Command {
            name: command,
            block: None,
            argument: None,
        }),
        "repeat" | "rp" => {
            if let Some(Token::Number(arg)) = lex.next() {
                if let Some(Token::LBracket) = lex.next() {
                    Ok(Command {
                        name: command,
                        block: Some(parse(lex, true)?),
                        argument: Some(arg),
                    })
                } else {
                    Err("Expected '['")
                }
            } else {
                Err("Expected number after repeat")
            }
        }
        _ => Err("Unknown command"),
    }
}
