use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Random {
    rng: Rc<RefCell<ChaCha8Rng>>,
}

impl Random {
    pub fn new(seed: u64) -> Random {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        Random {
            rng: Rc::new(RefCell::new(rng)),
        }
    }

    pub fn rnd_choice<'a, T: Clone>(&'a self, slice: &'a [T]) -> T {
        let mut rng = self.rng.borrow_mut();
        slice.choose(&mut *rng).unwrap().clone()
    }
}

pub fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since = start.duration_since(UNIX_EPOCH).unwrap();
    since.as_secs()
}
