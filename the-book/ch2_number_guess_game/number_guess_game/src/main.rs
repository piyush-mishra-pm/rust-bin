use rand::Rng;

static LOWER_RANGE: u32 = 1;
static HIGHER_RANGE: u32 = 10;

#[derive(Debug)]
struct Game{
    number: u32,
    attempts: u32,
    guesses: Vec<u32>,
}

impl Game {
    fn new(number:u32, attempts:u32, guesses:Vec<u32>)->Self{
        Self{
            number,
            attempts,
            guesses,
        }
    }
}

fn main() {
    println!(
        "Welcome (🙏) to\nNumber Guessing Game!\n🎉\n\tTry guessing the number between {} & {}",
        LOWER_RANGE, HIGHER_RANGE
    );
    let mut game_history: Vec<Game> = vec![];
    let game_number = 0;
    loop {
        println!("Starting Game #{}:", game_number);
        let secret_number = rand::thread_rng().gen_range(LOWER_RANGE..=HIGHER_RANGE);
        let mut attempts: u32 = 0;
        let mut guesses: Vec<u32> = vec![];
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
            guesses.push(guessed);
            println!("You guessed: {guessed}");

            // Compare numbers:
            match guessed.cmp(&secret_number) {
                std::cmp::Ordering::Less => println!("🤏 Too small. Try bigger!"),
                std::cmp::Ordering::Greater => println!("😲 Too big. Try smaller!"),
                std::cmp::Ordering::Equal => {
                    println!("🎯 Bingo! You guessed {secret_number} in {attempts} attempts!");
                    game_history.push(Game::new(secret_number, attempts, guesses));
                    println!("\nGame History: * * * * * *\n\tnumber\tattempts\tguesses");
                    game_history.iter().for_each(|game| {
                        println!("\t{}\t{}\t{:?}",game.number, game.attempts, game.guesses);
                    });

                    break;
                }
            }
        }
    }
}
