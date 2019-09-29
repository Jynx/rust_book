/*
* Quit: No associated values
* Move: anonymous struct
* Write: String
* ChangeColor: 3 i32's
*/

/* 
* The advantage of this, is we can define a function that accepts "Message", and then pass Message::Quit, Message:Move, etc.
* as opposed to somehow accepting any of these struct's invidually. Message acts as a single type.
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do whatever
        // self would be "message" on line 36, for example so,
    }
}

// v4 has 4 associated u8's, v6 has a string,
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let message = Message::Write(String::from("hello"));
    message.call();

}

fn route(message: Message) {}
