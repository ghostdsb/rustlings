// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(String),
    Move {x: i32, y: i32},
    ChangeColor {r: u32, g: u32, b: u32},
}

fn main() {
    println!("{:?}", Message::Quit(String::from("Quitting")));
    println!("{:?}", Message::Echo(String::from("Echo")));
    println!("{:?}", Message::Move{x: 10, y:12});
    println!("{:?}", Message::ChangeColor{r: 0, g:0, b: 255});
}
