#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

pub fn enum_one() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
enum MessageTwo {
    // TODO: Define the different variants used below.
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl MessageTwo {
    fn call(&self) {
        println!("{self:?}");
    }
}

pub fn enum_two() {
    let messages = [
        MessageTwo::Resize {
            width: 10,
            height: 30,
        },
        MessageTwo::Move(Point { x: 10, y: 15 }),
        MessageTwo::Echo(String::from("hello world")),
        MessageTwo::ChangeColor(200, 255, 255),
        MessageTwo::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

enum Message3 {
    // TODO: Define a few types of messages as used below.
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue)
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn process(&mut self, message: Message3) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        //
        // Resize,
        // Move,
        // Echo,
        // ChangeColor,
        // Quit,

        match message {
            Message3::Resize { width, height } => self.resize(width, height),
            Message3::ChangeColor(red, blue, green) => self.change_color(red, green, blue),
            Message3::Move(point) => self.move_position(point),
            Message3::Echo(string) => self.echo(string),
            Message3::Quit => self.quit(),
        }
    }
}

pub fn enum_three() {
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::new(),
        color: (0, 0, 0),
        quit: false,
    };

    let messages = [
        Message3::Resize {
            width: 100,
            height: 200,
        },
        Message3::Move(Point { x: 50, y: 75 }),
        Message3::Echo(String::from("Hello from enum_three!")),
        Message3::ChangeColor(255, 128, 64),
        Message3::Quit,
    ];

    for message in messages {
        state.process(message);
    }

    println!("Final state:");
    println!("  Size: {}x{}", state.width, state.height);
    println!("  Position: ({}, {})", state.position.x, state.position.y);
    println!("  Message: {}", state.message);
    println!("  Color: {:?}", state.color);
    println!("  Quit: {}", state.quit);
}
