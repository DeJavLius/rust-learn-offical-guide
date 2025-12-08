fn main() {
    // 정수형
    {
        let y = 5;
        let y = y + 1;

        {
            let y = y * 2;
            println!("The value of y in the inner scope is : {y}");
        }

        println!("The value of y is : {y}");
    }

    // 부동소수점
    {
        let x = 2.0;
        let y: f32 = 3.0;

        println!("The value of x, y in the inner scope is : f64 : {x}, f32 : {y}");
    }

    // 연산
    {
        let sum = 5 + 10;

        let difference = 95.5 - 4.3;

        let product = 4 * 30;

        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3;

        let remainder = 43 % 5;

        println!("sum :{sum}, diff : {difference}, product : {product}, quotient : {quotient}, trunc : {truncated}, remainder: {remainder}");
    }

    // 문자 char
    {
        let c = 'z';
        let z: char = 'Z';
        let heart_eyed_cat = '😻';

        println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");
    }

    // tuple
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;

        // tup.0 방식으로 출력 시 string 변환 안됨
        println!("The value of tup: (i32, f64, u8) : ({x}, {y}, {z})");

        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;

        println!("Value access by index: {five_hundred}, six_point_four: {six_point_four}, one: {one}");
    }

    // array
    {
        let a = [1, 2, 3, 4, 5];
        let b: [i32; 5] = [1, 2, 3, 4, 5];
        let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November","December"];
        let easy = [3; 5];

        let first = a[0];
        let second = b[1];

        // println!("The value of easy is: {0}", months[13]);
        println!("The value of a, b, months, easy, first, second is: {}, {}, {}, {}, {}, {}", a[0], b[1], months[3], easy[0], first, second);
    }
}