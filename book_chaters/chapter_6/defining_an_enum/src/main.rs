// Enums are a way of defining data types in an different way than done with structs.
// Enums are build with the `enum` clause. Example, an IP address currently have the V4 and V6.
// We could represent the IP with an enum such as:
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


// it is possible to define a type for each variant. Each Enum variant is
// basically a string that can accept a type. For example, the next enum
// accepts a string as the type, this string defines its value for the variant.
// Each variant can hold whatever data type we like.
#[derive(Debug)]
enum IpAddrKindWithType {
    V4(String),
    V6(String)
}

// Enums can have variants with differnt types: strings, numeric types, structs.
#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quite,
    Move { x: i32, y:i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Just like structures, enums can have methods and functions in its  `impl`
// section.
impl Message {
    fn call(&self) -> u8 {
        println!("Call method used.");
        10
    }
}

fn main() {
    // To access the enum variants we must use the namespace accessor `::`
    let ip_1: IpAddrKind = IpAddrKind::V4;
    let ip_2: IpAddrKind = IpAddrKind::V6;
    let ip_3: IpAddrKindWithType = IpAddrKindWithType::V6(String::from("::1"));
    let ip_4: IpAddrKindWithType = IpAddrKindWithType::V4(String::from("127.0.0.1"));

    println!("Address type: {:#?}", ip_1);
    println!("Address type: {:#?}", ip_2);
    println!("Address type: {:#?}", ip_3);
    println!("Address type: {:#?}", ip_4);

    let message: Message = Message::Write(String::from("Hello world."));
    println!("Message struct: {:#?}", message.call());
}
