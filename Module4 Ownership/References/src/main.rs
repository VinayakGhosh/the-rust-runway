fn main() {

    // -----------References Rule-----------
    // 1. At any given time, you can have either one mutable reference or any number of immutable reference.
    // 2. References must be valid (the data they point to must be valid)



    // References don't take ownership of the underlying value

    // Passing in references as function parameters is called borrowing

    let s = String::from("Hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

    // if you want to pass mutable referenes
    let mut s1 = String::from("Hello");
    change(& mut s1)
}

fn calculate_length(s: &String)->usize{         // references are immutable by default
    let length = s.len();
    length
}

fn change(s: & mut String){                     
    s.push_str(", world");
    println!("s is {}", s)
}
