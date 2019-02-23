use rand::prelude::*;

pub struct Random {
    cache: [u8; 9],
    current_index: usize,
    rng: ThreadRng,
}

impl Random {
    pub fn new() -> Random {
        Random {
            cache: [0; 9],
            current_index: 0,
            rng: thread_rng(),
        }
    }

    pub fn get_new(&mut self) -> Option<u8> {
        if self.current_index == 9 {
            None
        } else {
            let mut generated = self.rng.gen_range(1, 10);
            while !self.insert(generated) {
                generated = self.rng.gen_range(1, 10);
            }
            Some(generated)
        }
    }

    fn insert(&mut self, val: u8) -> bool {
        for item in self.cache.iter_mut() {
            if *item == val {
                return false
            }
        }
        self.cache[self.current_index] = val;
        self.current_index += 1;
        true
    }

    pub fn reset(&mut self) {
        self.current_index = 0;

        for item in self.cache.iter_mut() {
            *item = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_full() {
        let mut r = Random::new();
        for _ in 0..9 {
            assert_ne!(None, r.get_new());
        }
        r.cache.sort();
        assert_eq!(r.cache, [1,2,3,4,5,6,7,8,9]);
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
