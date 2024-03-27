fn return_uppercase(str: String) -> String {
    return str.to_uppercase();
}

fn first_word(string: &String) -> &str {
    let string_bytes = string.chars();
    for (idx, &value) in string_bytes.enumerate() {
        if value == ' ' {
            return &string[0..idx];
        }
    }
    &string[0..string.len()]
}

fn main() {
    // let s = String::from("hello");
    // take_ownership(s.clone());
    // println!("{s}");

    // let x = 5;
    // let _y = x;
    // make_copy(x);
    // println!("{x}");
    let str = "hello".to_string();
    let uppercase = return_uppercase(str);
    println!("{uppercase}");

    {
        //The slice type
        let mut long_string = String::from("This is a long sentence");
        let first_word = first_word(&long_string);
        println!("{first_word}");
        long_string.clear();
    }

    // let i_am_string = give_ownership();
    // println!("{i_am_string}");
    // println!("{i_am_string}");

    // println!("{:#?}", take_and_give_back(i_am_string.clone()));

    // let len = count_letter(&i_am_string);
    // println!("String `{i_am_string}` has {len} letter(s)");
    // println!("String `{i_am_string}` has {len} letter(s)");
    // {
    //     let mut string = String::from("hello");
    //     change(&mut string);
    //     println!("{string}");
    //     change(&mut string);
    //     println!("{string}");
    //     let ref_string = no_dangle(&string);
    //     println!("{ref_string}");
    // }
    // {
    //     let s = String::from("hello");
    //     let reference_to_s = no_dangle(&s);
    //     println!("{reference_to_s}");
    // }
    // {
    //     let a: i32 = 4;
    //     let b: i32 = 9;
    //     let c = add(a, b);
    //     println!("the sum of a and b is: {c}");
    // }
}

fn take_ownership(some_string: String) {
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn give_ownership() -> String {
    let x = String::from("string");
    return x;
}

fn take_and_give_back(string: String) -> String {
    string
}

fn count_letter(string: &String) -> usize {
    string.len()
}

fn change(string: &mut String) {
    string.push_str("flkjasd")
}

fn no_dangle(ref_s: &String) -> &String {
    ref_s
}

#[inline(always)]
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
