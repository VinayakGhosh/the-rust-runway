fn main() {
    // -----------Ownership Rules------------
    // 1) Each value in Rust has a variable that's called it's owner.
    // 2) There can only be one owner at a time.
    // 3) When the owner goes out of scope, the value will be dropped.



    // creating a scope
    {   // here s is not valid, it's not declared yet!
        let _s = "Hello";    // s is valid from this point forward
        // do stuff with s
    }   // this scope is over now, and s is no longer valid

    // Note that s above is a string literal and it get's directly stored in the binary and is fixed in size.

    // If we want a string dynamic in size and that we can mutate
    let _s = String::from("Hello");     // now this string is stored in the heap




    // -----------How variables and data interact-----------------
    let x = 5;
    let y = x;      // copying value

    let s1 = String::from("hello");
    // but to actually clone s1 into s3 we can do:
    let s3 = s1.clone();
    let s2 = s1;    // Move (not shallow copy)

    println!("s1 is: {}", s1);  //this line would give error as s1 is not valid anymore 
}
