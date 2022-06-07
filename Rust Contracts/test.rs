let s = "hello";

{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}





let s = String::from("hello");

let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`





{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}  