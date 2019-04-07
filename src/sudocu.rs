mod validator;

use validator::check_not_exists;

pub struct Sudocu {
    pub map: [u8; 81],
    side_length: usize,
    indexes_map: [([usize; 9], [usize; 9], [usize; 9]); 81],
}

impl Sudocu {
    pub fn new() -> Sudocu {
        let mut s = Sudocu {
            map: [0; 81],
            side_length: 9,
            indexes_map: [([0; 9], [0; 9], [0;9]); 81],
        };
        s.calculate_indexes_map();
        s
    }

    pub fn try_set(&mut self, index: usize, val: u8) -> bool {
        if !check_not_exists(val, &self.indexes_map[index].0, &self.map) {
            return false
        }
        if !check_not_exists(val, &self.indexes_map[index].1, &self.map) {
            return false
        }
        if !check_not_exists(val, &self.indexes_map[index].2, &self.map) {
            return false
        }

        self.map[index] = val;
        true
    }

    pub fn clean(&mut self, start_ind: usize) {
        for i in start_ind..self.map.len() {
            self.map[i] = 0;
        }
    }

    pub fn print(&self) {
        for (i, item) in self.map.iter().enumerate() {
            print!("{} ", item);
            if (i + 1) % self.side_length == 0 {
                println!();
            }
        }
        println!();
    }

    fn calculate_indexes_map(&mut self) {
        for i in 0..self.map.len() {
            let mut start_pos: usize;

            // calculate row indexes
            start_pos = (i / self.side_length) * self.side_length;
            for b in 0..self.side_length {
                self.indexes_map[i].0[b] = start_pos;
                start_pos += 1;
            }

             // calculcate column indexes
            start_pos = i % self.side_length;
            for b in 0..self.side_length {
                self.indexes_map[i].1[b] = start_pos;
                start_pos += self.side_length;
            }

            // calculate square indexes
            let x = i % self.side_length;
            let y = i / self.side_length;
            let square_side = 3usize;

            let x_startpos = x / square_side * square_side;
            let y_startpos = y / square_side * square_side;
            
            start_pos = x_startpos + self.side_length * y_startpos;

            for b in 0..self.side_length {
                self.indexes_map[i].2[b] = start_pos;
                start_pos += 1;
                if start_pos % square_side == 0 {
                    start_pos += square_side * 2;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexes_map() {
        let s = Sudocu::new();
        assert_eq!((
            [36,37,38,39,40,41,42,43,44],
            [5,14,23,32,41,50,59,68,77],
            [30,31,32,39,40,41,48,49,50],
        ), s.indexes_map[41])
    }
}
