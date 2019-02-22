mod random;
mod sudocu;

use random::Random;
use sudocu::Sudocu;

const INITIAL_BACKTRACK_STEP: usize = 7;

fn main() {
    let mut sudocus: Vec<Sudocu> = Vec::with_capacity(1000);

    for s in 0..100 {
        sudocus.push(Sudocu::new());
    }

    let mut r = Random::new();

    for s in sudocus.iter_mut() {
        brut(s, &mut r);
    }

    for s in sudocus.iter() {
        s.print();
    }
}

fn brut(s: &mut Sudocu, r: &mut Random) {
    let mut i = 0;
    let mut backtrack_step = INITIAL_BACKTRACK_STEP;

    while i < s.map.len() {
        let mut v = r.get_new().unwrap();

        while !s.try_set(i, v) {
            v = match r.get_new() {
                Some(v) => v,
                None => {
                    if backtrack_step > 40 {
                        s.clean(0);
                        i = 0;
                        r.reset();
                        backtrack_step = INITIAL_BACKTRACK_STEP;
                        r.get_new().unwrap()
                    } else {
                        if i <= backtrack_step { i = 0; }
                        else { i -= backtrack_step }
                        s.clean(i);
                        backtrack_step += 2;
                        r.reset();
                        r.get_new().unwrap()
                    }
                    
                }
            };
        }

        r.reset();
        i += 1;
    }
}
