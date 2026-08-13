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
    println!("SUBSCRIBE_COUNT is: {}", SUBSCRIBE_COUNT);


    // Compound types

    // Tuple
    let tup = ("Let's get rusty!", 100_100);   //tuple
    // two ways to get value out of tuple:
    // 1) destructuring
    let(_channel, _subcount) = tup;

    // 2) dot notation
    let _sub_count = tup.1;

    // Array
    let error_codes = [200, 404, 500];
    let _not_found = error_codes[1];

    let _byte = [0,8];

    let sum = sum_numbers(10,20);
    println!("sum is {}", sum);


    // control flow
    let condition = true;
    let number = if condition {5} else {6};
    println!("number: {}", number);

    // Return values from loop
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter ==5{
            break counter;
        }
    };
    println!("result is: {}", result)

}


fn sum_numbers(x: i32, y: i32)-> i32{
    println!("x is: {}", x);
    println!("y is: {}", y);
    return x+y;
}
