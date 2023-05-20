fn main() {
    let integer: i32 = 45;
    let float: f64 = 2.6;
    let boolean: bool = false;
    let character: char = 'Z'; 

// COMPOUND TYPES 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring the variable 
    println!("The value of y is {y}");  
    
    let tup_1 = tup.0; 
    let tup_2 = tup.1; 
    println!("The 1st value of tup: {tup_1}, the 2nd value of tup: {tup_2}"); 

    let a = [1,2,3,4,5];
    
    let a: [i32; 5] = [1,2,3,4,5];
    for i in 0..5 {
        print!("{} ", a[i]);
    }
    println!("");

    let a = [3; 5];
    for i in 0..5 {
        print!("{} ", a[i]);
    }
    println!("");
}
