#![feature(const_fn)]

pub struct Sudocu {
    map: [u8; 81],
    side_length: usize,
    backtrack_step: u8,
    last_backtrack_index: u8,
    pub indexes_map: [([usize; 9], [usize; 9], [usize; 9]); 81],
}

impl Sudocu {
    pub fn new() -> Sudocu {
        let mut s = Sudocu {
            map: [0; 81],
            side_length: 9,
            backtrack_step: 5,
            last_backtrack_index: 0,
            indexes_map: [([0; 9], [0; 9], [0;9]); 81],
        };
        s.calculate_indexes_map();
        s
    }

    pub fn calculate_indexes_map(&mut self) {
        for i in 0..self.map.len() {
            let mut start_pos: usize;

            // calculate row indexes
            start_pos = (i / self.side_length) * self.side_length;

            for b in 0..self.side_length {
                self.indexes_map[i].0[b] = start_pos;
                start_pos += 1;
            }
        }
    }
}