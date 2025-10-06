use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::process;

fn main() -> Result<(), std::io::Error> {

    
    println!("_________________________________\n\n              TETRIS\n_________________________________");
    println!("\n\n\n\nArrow keys <⌄> to Move");
    println!("Q to Quit <NEEDS FIXING>");
    println!("Press Space to start\n");
    enable_raw_mode()?;
        loop {
        if let Event::Key(even) = read()? {
            match even.code {
                KeyCode::Char(' ') => break,
                KeyCode::Char('q') => {
                    process::exit(0);
                },
                key => print!(""),
            }
        }
    }
    println!("Starting!");

    
    loop {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Left => {
					println!("<----Slide Left");
				},
                KeyCode::Right => {
					println!("Slide Right---->");
				},
                KeyCode::Down => {
					println!("Slide Down .... ");
				},
                KeyCode::Char(' ') => {
					println!("Select ");
				},
                KeyCode::Char('q') => break,
                key => println!("Key pressed: {:?}, Quiting", key),
            }
        }
    }
    
    disable_raw_mode()?;
    Ok(())
}