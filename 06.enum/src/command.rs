#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
     Quit,
     Move { x: i32, y: i32 },
     Say(String)
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Quit => println!("Quitting..."),
            Command::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Command::Say(message) => println!("Saying: {}", message),
        }
    }
}
