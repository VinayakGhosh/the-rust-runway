fn main() {
    // Mutable variables
    let mut x = 5;
    println!("Value of x is {}", x);

    x=6;
    println!("Value of x is {}", x);

    // shadowing
    let y = 6;
    println!("Value of Y = {}", y);
    let y = "six";
    println!("Value of Y = {}", y);


    const SUBSCRIBE_COUNT: u32 = 100000;
    println!("SUBSCRIBE_COUNT is: {}", SUBSCRIBE_COUNT)

}
