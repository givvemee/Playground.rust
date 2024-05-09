use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop {
        println!("Input your guess");
    
        
        // mut as mutable
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("fail case");
        
        let guess:u32 = match guess.trim().parse() {
            // parse 의 반환값은 Result, Result는 Ok 또는 Err 반환
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("작다"),
            Ordering::Greater => println!("크다"),
            Ordering::Equal => {
                println!("오 ㅋㅋ");
                break;
            },
        }

    }
}
