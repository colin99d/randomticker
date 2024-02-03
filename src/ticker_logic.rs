use crate::generate_random::{get_timestamp, Random};
use crate::statics::{LETTERS, TICKERS};

fn generate_ticker(generator: &Random) -> String {
    (0..5)
        .map(|_| generator.rnd_choice(&LETTERS))
        .filter(|&value| value != ' ')
        .collect()
}

pub fn get_valid_ticker() -> String {
    let seed = get_timestamp();
    let random = Random::new(seed);
    loop {
        let ticker = generate_ticker(&random);
        if TICKERS.contains(&ticker.as_str()) {
            return ticker;
        }
    }
}
