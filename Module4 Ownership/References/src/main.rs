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


    // Slices are a kind of reference that allows you to reference a contiguous sequence of elements in a collection rather than the whole collection. Slices let you work with a section of a collection rather than the whole collection. 

    let mut s = String::from("Hello world");
    let hello = &s[0..5];     // here 5 is exclusive
    let world = &s[6..11];    // here 11 is exclusive
}

fn calculate_length(s: &String)->usize{         // references are immutable by default
    let length = s.len();
    length
}

fn change(s: & mut String){                     
    s.push_str(", world");
    println!("s is {}", s)
}


