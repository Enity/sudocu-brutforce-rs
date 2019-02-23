#![feature(test)]

mod random;
mod sudocu;

use random::Random;
use sudocu::Sudocu;

const INITIAL_BACKTRACK_STEP: usize = 2;

fn main() {
    let mut r = Random::new();
    let mut s = Sudocu::new();

    for _ in 0..100 {
        brut(&mut s, &mut r);
        s.print();
        s.clean(0);
    }
}

fn brut(s: &mut Sudocu, r: &mut Random) {
    let mut i = 0;
    let mut backtrack_step = INITIAL_BACKTRACK_STEP;
    let mut last_backtrack_index = 100;

    while i < s.map.len() {
        let mut v = r.get_new().unwrap();

        while !s.try_set(i, v) {
            v = match r.get_new() {
                Some(v) => v,
                None => {
                    if last_backtrack_index == i {
                        backtrack_step += 1;
                    }
                    last_backtrack_index = i;
                    if i <= backtrack_step {
                        i = 0;
                        backtrack_step = INITIAL_BACKTRACK_STEP;
                    } else {
                        i -= backtrack_step
                    }
                    s.clean(i);
                    r.reset();
                    r.get_new().unwrap()
                }
            };
        }

        r.reset();
        i += 1;
    }
}

extern crate test;

#[bench]
fn bench_brutforce(b: &mut test::Bencher) {
    let mut r = Random::new();
    let mut s = Sudocu::new();

    b.iter(|| {
        brut(&mut s, &mut r);
        s.clean(0);
    })
}
