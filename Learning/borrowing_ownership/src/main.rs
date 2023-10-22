fn main() {
    let s: String = String::from("hello");
    take_ownership(s.clone());
    println!("{s}");

    let x = 5;
    make_copy(x);
    println!("{x}");

    let i_am_string = give_ownership();
    println!("{i_am_string}");
    println!("{i_am_string}");

    println!("{:#?}", take_and_give_back(i_am_string.clone()));

    let len = count_letter(&i_am_string);
    println!("String `{i_am_string}` has {len} letter(s)");
    println!("String `{i_am_string}` has {len} letter(s)");
    {
        let mut string = String::from("hello");
        change(&mut string);
        println!("{string}");
        change(&mut string);
        println!("{string}");
        let ref_string = no_dangle(&string);
        println!("{ref_string}");
    }
    {
        let s = String::from("hello");
        let reference_to_s = no_dangle(&s);
        println!("{reference_to_s}");
    }
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
