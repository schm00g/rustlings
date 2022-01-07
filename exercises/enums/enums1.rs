// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(String),
    Echo(String),
    Move(String),
    ChangeColor(String)
}


fn main() {
    let quit = Message::Quit(String::from("127.0.0.1"));
    let echo = Message::Quit(String::from("127.0.0.1"));
    let move_ = Message::Quit(String::from("127.0.0.1"));
    let change_color = Message::Quit(String::from("127.0.0.1"));

    println!("{:?}", quit);
    println!("{:?}", echo);
    println!("{:?}", move_);
    println!("{:?}", change_color);
}
