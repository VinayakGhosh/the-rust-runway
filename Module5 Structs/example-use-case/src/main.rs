use std::println;

#[derive(Debug)]
struct Rectange{
    width: u32,
    height: u32,
}

// method syntax
impl Rectange{
    fn area(&self) ->u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectange) -> bool{
        self.width > other.width && self.height>other.height
    }
}


fn main() {
    let rect=Rectange{
        width: 30,
        height: 50
    };
    let rect2 = Rectange{
        width:20,
        height:10
    };

    // println!("The area of the rectangle is {} square pixels.", area(&rect));

    println!("using the syntax method, area is {}", rect.area());

    println!("rect is {:#?}", rect);

    println!("rect can hold rect1: {}", rect.can_hold(&rect2));
    println!("{:?}", rect2);
}

// fn area(rectangle: &Rectange)->u32{
//     rectangle.width * rectangle.height
// }