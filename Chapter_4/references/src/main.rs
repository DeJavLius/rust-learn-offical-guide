fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);

        println!("s is {}", s);
    }

    {
        let mut s = String::from("hello");

        // let r1 = &mut s;
        // let r2 = &mut s;

        {
            let r1 = &mut s;

            println!("r1 is {}", r1);
        }

        let r2 = &mut s;

        println!("r2 is {}", r2);

        {
            let r1 = multiple_references(&mut s);
            let r2 = multiple_references(&mut s);

            println!("r1 is {}, r2 is {}", r1, r2);
        }

        {
            let r1 = &s;
            let r2 = &s;

            println!("non mutable variable are: r1 is {}, r2 is {}", r1, r2);
            let r3 = &mut s;

            println!("no more use of non mut var, can use mutable variable: r3 is {}", r3);
        }
    }

    {
        let reference_to_nothing = dangle();
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn multiple_references(another_var: &mut String) -> String {
    another_var.push_str(", world");
    another_var.to_string()
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}