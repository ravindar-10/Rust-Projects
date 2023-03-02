use std::io;
use rand::prelude::*;
use std::cmp::Ordering;
use colored::*;
fn main() {
    println!("Guess the Number");
    // let secret_number:i32=random();
    let mut incorrect_guess=0;
    loop{
    let secret_number:i32=thread_rng().gen_range(10..15);
    println!("Enter your Guess number:");
    let mut guess:String=String::new();
    io::stdin().read_line( &mut guess).expect("the number is not readable");
    let guess:i32=match guess.trim().parse() {
     Ok(num) => num,
     Err (_)=>{
         println!("{}","Please Enter a valid number".red());
         continue;
     },
    };
    println!("Your Guess number : {}",guess);
    println!("The Secret Number: {}",secret_number);
    // if random(){
    //     print!("it's random");
    // }
    match guess.cmp(&secret_number){
        Ordering::Greater => {println!("{}","Guess is too Big".red()); incorrect_guess+=1;},
        Ordering::Less => {println!("{}","Guess is too Small".red()); incorrect_guess+=1;},
        Ordering::Equal => {
            println!("{}","Congratulations You win !".green());
            break;
        }
    }
    }
    println!("{} {}","Incorect Guess:".blue(),incorrect_guess);
}
