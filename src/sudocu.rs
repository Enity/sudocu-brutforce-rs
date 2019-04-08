mod validator;

use validator::check_not_exists;

pub struct Sudocu {
    pub map: [u8; 81],
    side_length: usize,
    frozen_indexes: [bool; 81],
    indexes_map: [([usize; 9], [usize; 9], [usize; 9]); 81],
}

impl Sudocu {
    pub fn new() -> Sudocu {
        let mut s = Sudocu {
            map: [0; 81],
            side_length: 9,
            frozen_indexes: [false; 81],
            indexes_map: [([0; 9], [0; 9], [0;9]); 81],
        };
        s.calculate_indexes();
        s
    }

    pub fn fill(&mut self, data: &str) {
        for (i, c) in data.chars().enumerate() {
            let parsed = c.to_digit(10).expect("Invalid character") as u8;
            self.map[i] = parsed;

            if parsed != 0 {
                self.frozen_indexes[i] = true;
            }
        }
    }

    pub fn try_set(&mut self, index: usize, val: u8) -> bool {
        if self.frozen_indexes[index] {
            return true;
        }
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
            if !self.frozen_indexes[i] {
                self.map[i] = 0;
            }
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

    fn calculate_indexes(&mut self) {
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

    #[test]
    fn test_fill() {
        let mut s = Sudocu::new();
        s.fill("006430580405000000310500200060750910502000308039028040007005029000000807043072100");
        let expected_indexes: [u8; 81] = [0,0,6,4,3,0,5,8,0,4,0,5,0,0,0,0,0,0,3,1,0,5,0,0,2,0,0,0,6,0,7,5,0,9,1,0,5,0,2,0,0,0,3,0,8,0,3,9,0,2,8,0,4,0,0,0,7,0,0,5,0,2,9,0,0,0,0,0,0,8,0,7,0,4,3,0,7,2,1,0,0];
        assert_eq!(expected_indexes[..], s.map[..]);
        let expected_frozen = [false,false,true,true,true,false,true,true,false,true,false,true,false,false,false,false,false,false,true,true,false,true,false,false,true,false,false,false,true,false,true,true,false,true,true,false,true,false,true,false,false,false,true,false,true,false,true,true,false,true,true,false,true,false,false,false,true,false,false,true,false,true,true,false,false,false,false,false,false,true,false,true,false,true,true,false,true,true,true,false,false];
        assert_eq!(expected_frozen[..], s.frozen_indexes[..]);
    }
}
