mod random;
mod sudocu;

use random::Random;
use sudocu::Sudocu;

fn main() {
    let mut r = Random::new();
    let mut s = Sudocu::new();

    let mut i = 0;
    while i < s.map.len() {
        let mut v = r.get_new().unwrap();

        while !s.try_set(i, v) {
            v = match r.get_new() {
                Some(v) => v,
                None => {
                    if i <= 5 {
                        i = 0;
                    } else {
                        i -= 5
                    }
                    s.clean(i, i + 5);
                    r.reset();
                    r.get_new().unwrap()
                }
            };
        }

        r.reset();
        i += 1;
    }

    s.print();
}
