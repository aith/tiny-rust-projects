fn main() {
    println!({}, ret_str());
}

fn ret_str() -> String {
    let mut hello = String::from("Hello, ");

    hello.push('w');
    hello.push_str("orld!");
    hello
}
