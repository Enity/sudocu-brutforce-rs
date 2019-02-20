use rand::prelude::*;
use std::collections::HashSet;

struct Random {
    cache: HashSet<u8>,
    rng: ThreadRng,
}

impl Random {
    fn new() -> Random {
        Random {
            cache: HashSet::with_capacity(9),
            rng: thread_rng(),
        }
    }

    fn get_new(&mut self) -> Option<u8> {
        if self.cache.len() == 9 {
            None
        } else {
            let mut generated = self.rng.gen_range(1, 10);
            while !self.cache.insert(generated) {
                generated = self.rng.gen_range(1, 10);
            }
            Some(generated)
        }
    }

    fn reset(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_full() {
        let mut r = Random::new();
        for _ in 0..9 {
            r.get_new();
        }
        assert_eq!(None, r.get_new());
    }

    #[test]
    fn test_get_new_after_reset() {
        let mut r = Random::new();
        for _ in 0..9 {
            r.get_new();
        }
        r.reset();
        assert_ne!(None, r.get_new());
    }
}
