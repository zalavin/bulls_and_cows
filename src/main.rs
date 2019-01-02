use std::io;
use rand::Rng;

fn main() {
    println!("Welcome!");

    // Generate a 4-digit number with different chars
    let mut vec = Vec::new();
    
    while vec.len() < 4 {
        let numb = rand::thread_rng().gen_range(0, 10);
        
        if !vec.contains(&numb) {
            vec.push(numb);
        }
    }

    let mut attempts = 1;
    
    loop {
        // Get a number
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
             .expect("Failed to read line");
             
         // Split number into chars
         let guess: Vec<u32> = guess
             .trim()
             .split("")
             .filter(|s| !s.is_empty())
             .map(|s| s.parse().unwrap())
             .collect();
        
        // Return if number matches
        if vec == guess {
            println!("You have won with {} attempts!", attempts);
            break;
        }

        // Find bulls and cows
        let mut bulls = 0;
        let mut cows = 0;
        let mut index = 0;

        for x in &guess {
            if vec[index] == guess[index] {
                bulls += 1;
            } else if vec.contains(&x) {
                cows += 1
            }
        
            index += 1;
        }
    
        // Display result
        println!("{} bulls and {} cows", bulls, cows);
    
        // Increment attempts
        attempts += 1;
    }
}
