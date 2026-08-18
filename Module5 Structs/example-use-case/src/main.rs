use std::println;

struct Rectange{
    width: u32,
    height: u32,
}

fn main() {
    let rect=Rectange{
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect));
}

fn area(rectangle: &Rectange)->u32{
    rectangle.width * rectangle.height
}