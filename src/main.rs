mod random;
mod sudocu;

use random::Random;
use sudocu::Sudocu;

const INITIAL_BACKTRACK_STEP: usize = 7;

fn main() {
    let mut r = Random::new();
    let mut s = Sudocu::new();

    let mut i = 0;
    let mut backtrack_step = INITIAL_BACKTRACK_STEP;

    while i < s.map.len() {
        let mut v = r.get_new().unwrap();

        while !s.try_set(i, v) {
            v = match r.get_new() {
                Some(v) => v,
                None => {
                    if i <= backtrack_step {
                        i = 0;
                    } else {
                        i -= backtrack_step
                    }
                    s.clean(i, i + backtrack_step);
                    backtrack_step += 2;
                    r.reset();
                    r.get_new().unwrap()
                }
            };
        }

        r.reset();
        backtrack_step = INITIAL_BACKTRACK_STEP;
        i += 1;
    }

    s.print();
}
