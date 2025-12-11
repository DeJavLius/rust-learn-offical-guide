fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    } // s drop

    {
        // integer is fixed type
        let x = 5;
        let y = x;

        println!("This is integer, push inside the stack x: {}, y: {}", x, y);

        let s1 = String::from("hello");
        let s2 = s1.clone();

        // wrong
        // println!("s1: {}", s1);
        println!("s1: {}, s2: {}", s1, s2);
    }

    {
        let s = String::from("hello");

        // takes_ownership got owner of s, and never came back
        takes_ownership(s);

        let x = 5;

        // makes_copy got x's copy
        makes_copy(x);

        // wrong code, owner is gone
        // println!("{}", s);
    }

    {
        // s1 is some_string
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        // s3 is a_string
        let s3 = takes_and_gives_back(s2);
        // s1, s3 are dropped

        println!("This is s1, s3: {}, {}", s1, s3);
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string = s address reference, because of owner changes s is dropped
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // x's copy drop
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string is s2
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}