use chrono::{TimeZone, Utc};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn rnd(&self, low: f64, high: f64) -> f64 {
        self.rng.borrow_mut().gen_range(low..high)
    }

    pub fn rnd_choice<'a, T: Clone>(&'a self, slice: &'a [T]) -> T {
        let mut rng = self.rng.borrow_mut();
        slice.choose(&mut *rng).unwrap().clone()
    }
}

pub fn num_days() -> i64 {
    let now = Utc::now();
    let epoch = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    let duration_since_epoch = now.signed_duration_since(epoch);
    duration_since_epoch.num_days()
}
