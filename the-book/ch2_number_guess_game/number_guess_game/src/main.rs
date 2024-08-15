use rand::Rng;

static LOWER_RANGE: u32 = 1;
static HIGHER_RANGE: u32 = 10;

#[derive(Debug)]
struct Game {
    number: u32,
    attempts: usize,
    guesses: Vec<u32>,
}
// todo: Think if Vec<> in below code can be used with just refs, and if there is unneccessary clonin happening as of now.
fn main() {
    println!(
        "Welcome (🙏) to\nNumber Guessing Game!\n🎉\n\tTry guessing the number between {} & {}",
        LOWER_RANGE, HIGHER_RANGE
    );
    let mut game_history: Vec<Game> = vec![];
    loop {
        let game_number = game_history.len() + 1;
        println!("Starting Game #{}:", game_number);
        let secret_number = rand::thread_rng().gen_range(LOWER_RANGE..=HIGHER_RANGE);
        let mut guesses: Vec<u32> = vec![];
        loop {
            // Increment attempt number:
            let attempts = guesses.len() + 1;

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
            guesses.push(guessed);
            println!("You guessed: {guessed}");

            // Compare numbers:
            match guessed.cmp(&secret_number) {
                std::cmp::Ordering::Less => println!("🤏 Too small. Try bigger!"),
                std::cmp::Ordering::Greater => println!("😲 Too big. Try smaller!"),
                std::cmp::Ordering::Equal => {
                    println!("🎯 Bingo! You guessed {secret_number} in {attempts} attempts!");
                    game_history.push(Game {
                        number: secret_number,
                        attempts,
                        guesses,
                    });
                    println!("\nGame History: * * * * * *\n\tnumber\tattempts\tguesses");
                    game_history.iter().for_each(|game| {
                        println!("\t{}\t{}\t{:?}", game.number, game.attempts, game.guesses);
                    });

                    break;
                }
            }
        }
    }
}
