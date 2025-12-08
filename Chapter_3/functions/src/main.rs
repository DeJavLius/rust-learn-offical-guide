fn main() {
    println!("Hello, world!");

    let y = {
        // 구문
        let x = 3;
        // 표현식
        x + 1
    };

    another_function(10);
    print_labeled_measurement(5, 'h');
    println!("evaluate y: {}", y);

    let x = five();
    println!("The value of x by last value five(): {}", x);

    let y = plus_one(x);
    println!("The value of y: {}", y);
}

fn another_function(x: i32) {
    println!("Another function's value: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // 에러 유발자
    x + 1//;
}