use rand::Rng;

static LOWER_RANGE: u32 = 1;
static HIGHER_RANGE: u32 = 100;
// todo:
// 1. store record of past games. and the numbers.
fn main() {
    println!(
        "Welcome (🙏) to\nNumber Guessing Game!\n🎉\n\tTry guessing the number between {} & {}",
        LOWER_RANGE, HIGHER_RANGE
    );
    let secret_number = rand::thread_rng().gen_range(LOWER_RANGE..=HIGHER_RANGE);
    let mut attempts = 0;
    loop {
        // Increment attempt number:
        attempts += 1;
        
        // Receive guessed input from User:
        let mut guessed = String::new();
        println!("Your #{attempts} guess (numbers only):");
        std::io::stdin()
            .read_line(&mut guessed)
            .expect("❌ Error reading line!");

        let guessed: u32 = match guessed.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠ Only numeric inputs allowed!");
                continue;
            }
        };

        println!("You guessed: {guessed}");

        // Compare numbers:
        match guessed.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("🤏 Too small. Try bigger!"),
            std::cmp::Ordering::Greater => println!("😲 Too big. Try smaller!"),
            std::cmp::Ordering::Equal => {
                println!("🎯 Bingo! You guessed {secret_number} in {attempts} attempts!");
                break;
            }
        }
    }
}
