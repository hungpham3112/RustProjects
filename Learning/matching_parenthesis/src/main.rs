/// Description
/// Given a string containing only characters (, ), [, ] {, }. Write a program that check whether the string is correct in expression.
/// Example:
/// ([]{()}()[]): correct
/// ([]{()]()[]): incorrect
/// Input
/// One line contains the string (the length of the string is less than or equal to 10^6)
/// Output
/// Write 1 if the sequence is correct, and write 0, otherwise
/// Example:
/// input
/// (()[][]{}){}{}[][]({[]()})
/// output
/// 1
use std::io;
fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let mut count = 0;
    for chr in string.chars() {
        match chr {
            '(' => count += 1,
            ')' => count -= 1,
            '[' => count += 1,
            ']' => count -= 1,
            '{' => count += 1,
            '}' => count -= 1,
            _ => continue,
        }
    }
    if count == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
