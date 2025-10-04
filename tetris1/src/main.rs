use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;

    println!("Press any key to see its code (Press 'q' to quit)");
    
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