use std::env;

fn main() {
    let arg = std::env::args()  /* returns iterator of String */
        .skip(1)
        .next()  /* advance iterator */
        .expect("should have one argument");
    println!("{}", arg.to_uppercase());
}
