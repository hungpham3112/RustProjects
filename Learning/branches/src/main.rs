fn main() {
    let x = 5;
    let statement = if x < 4 {
        println!("Condition is {}", x < 4);
    } else {
        println!("Condition is {}", !(x < 4));
    };
    println!("statement is: {:#?}", statement);
    {
        let arr = [1, 2, 3, 4, 5];
        for (idx, &element) in arr.iter().enumerate() {
            println!("index of {:#?} in arr is: {:#?}", element, idx);
        }
    }
    {
        let arr = [1, 2, 3, 4, 5];
        let mut index = 0;
        while index < arr.len() {
            println!("arr[{index}] is: {}", arr[index]);
            index += 1;
        }
    }
    {
        for i in (1..4).rev() {
            println!("{i}");
        }
    }
    {
        let temparature = -32.0;
        println!(
            "temparature in F is: {}, in C is: {}",
            temparature,
            fah2cel(temparature)
        );
    }
    {
        let num = 10;
        println!("The fibonancci number {num}th is: {}", fibonancci(num));
    }
    {


    }
}

fn fah2cel(num: f32) -> f32 {
    return (num - 32.0) * 5.0 / 9.0;
}

fn fibonancci(index: i32) -> i32 {
    let (mut num1, mut num2) = (0, 1);
    for _ in 1..(index - 1) {
        (num1, num2) = (num2, num1 + num2)
    }
    return num2;
}
