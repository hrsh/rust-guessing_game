// use rand::Rng;
// //use std::cmp::Ordering;

// fn main () {
//   let secret = rand::thread_rng().gen_range(1..101);

//   loop {
//     println!("what is your guess?");
//     let mut guess = String::new();

//     io::stdin()
//       .read_line(buf: &mut guess)
//       .expect("failed to read :(");

//     let guess: u32 = match guess.trim().parse() {
//       Ok(n) => n,
//       Err(_) => continue,
//     };

//     match guess.com(&secret) {
//       std::cmp::Ordering::Less => println!("small"),
//       std::cmp::Ordering::Greater => println!("big"),
//       std::cmp::Ordering::Equal => {
//         println!("wiiiiin");
//         break;
//       }
//     }
//   }
// }