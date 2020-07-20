// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM kNOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(i32),
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(2));
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
