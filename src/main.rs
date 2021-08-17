// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
    
//     let secret_number = rand::thread_rng().gen_range(1..11);
    
//     loop {
//         let mut guess = String::new();
//         println!("please enter a number!");
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("failed to read line :(");

//         //let guess: u32 = guess.trim().parse().expect("Enter a valid number");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("you guessed {} and secret number is {}", guess, secret_number);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("small"),
//             Ordering::Greater => println!("big"),
//             Ordering::Equal => {
//                 println!("winnnnnnnnnnnn");
//                 break;
//             }
//         }
//     }
// }

use std::io;
use rand::Rng;
//use std::cmp::Ordering;

fn main () {
  let secret = rand::thread_rng().gen_range(1..1001);

  let mut count: u32 = 0;

  loop {
    println!("what is your guess?");
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read :(");

    let guess: u32 = match guess.trim().parse() {
      Ok(n) => n,
      Err(_) => continue,
    };
    count = count + 1;
    match guess.cmp(&secret) {
      std::cmp::Ordering::Less => println!("small"),
      std::cmp::Ordering::Greater => println!("big"),
      std::cmp::Ordering::Equal => {
        println!("wiiiiin by {} times", count);
        break;
      }
    }
  }
}