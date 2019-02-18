use std::collections::HashSet;
use rand::prelude::*;

struct Random {
    cache: HashSet<u8>,
    rng: ThreadRng
}

impl Random {
    fn new() -> Random {
        Random {
            cache: HashSet::with_capacity(9),
            rng: thread_rng()
        }
    }

    fn get_new(&mut self) -> bool {
        let mut generated = self.rng.gen_range(1, 9);
        while self.cache.insert(generated) == false {
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new() {
        let mut r = Random::new();
        for _ in 0..8 {
            r.get_new();
        }
        dbg!(&r.cache);
        assert_eq!(false, r.get_new());
    }
}