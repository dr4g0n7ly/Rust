use std::io;

fn main() {
    println!("");
    print!("Enter number: ");

    let mut num = String::new();

    io::stdin()
      .read_line(&mut num)
      .expect("Failed to read line");

    let num: i32 = num
      .trim()
      .parse()
      .expect("Invalid integer");

    println!("number entered: {num}");

    another_fn(num);

    let x = plus_one(num);
    println!("2nd functio output: {x}");
}

fn another_fn(x: i32) {
    println!("number + 1 is: {}", x+1);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
