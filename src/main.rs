mod sudocu;
mod random;

use sudocu::Sudocu;

fn main() {
    let mut s = Sudocu::new();
    dbg!(&s.indexes_map[10]);
}
