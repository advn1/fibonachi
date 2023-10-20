use std::io;

fn main() {
    println!("Enter the nth Fibonacci number: ");

    let mut number = String::new();

     io::stdin()
         .read_line(&mut number)
         .expect("Failed to read line");

     let number: u32 = number
         .trim()
         .parse()
         .expect("Negative number");

     println!("{number}th fibonacci number is: {}",gen_fib_n(number));

}

 fn gen_fib_n(mut number: u32) -> u32 {
     let mut a = 0;
     let mut b  = 1;
     while number > 0 {
         (a,b) = (b, a+b);
         number -=1;
     }
     return a
 }
