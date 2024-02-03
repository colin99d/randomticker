use crate::statics::{LETTERS, TICKERS};
use crate::generate_random::{Random, num_days}

fn generate_ticker(generator: &Random) -> String {
    let mut ticker = String::new();
    for i in range(5) {
        let value = generator.rnd_choice(&LETTERS)
        if value != ' ' {
            ticker.push_char(value);
        }
    }
    ticker
}

pub fn get_valid_ticker() -> String {
    let seed = num_days()
    let random = Random::new(seed)
    let i: usize = 0;
    loop {
        i += 1;
        let ticker = generate_ticker();
        if TICKERS.contains(tickr) {
            // On average we should have 544 tries
            println!("Tries: {i}");
            return ticker;
        }
    }
}
