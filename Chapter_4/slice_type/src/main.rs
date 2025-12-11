fn main() {
    // 문자열의 바이트에서 적합한 코드를 찾기 위한 번거로운 구현
    {
        let mut s = String::from("hello world");
        let f_word = first_word(&s);
        let s_word = second_word(&s);

        println!("First word of '{}' is {}", s, f_word);
        println!("Second word of '{}' is {}", s, s_word);

        // 가변 참조자 필요, 이후 불편 참조자로 생성한 값을 사용하면 오류
        s.clear();

        // error
        // println!("First word of '{}' is {}", s, f_word);
    }

    // 문자열 슬라이스
    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let hell = &s[..4];
        let world = &s[6..11];

        let len = s.len();

        let slice = &s[3..len];
        let slice_max = &s[3..];

        println!("hello, hell, world, slice, slice_max : {}, {}, {}, {}, {}", hello, hell, world, slice, slice_max);
    }

    // 문자열 슬라이스
    {
        let my_string = String::from("hello world");

        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        let word = first_word(&my_string);

        let my_string_literal = "hello world";

        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);
        let word = first_word(my_string_literal);

        println!("String literal and String slice: {}, {}, {}", my_string, my_string_literal, word);
    }

    // 숫자 슬라이스
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}