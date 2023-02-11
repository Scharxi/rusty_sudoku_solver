use std::fmt::Formatter;
use std::slice::Iter;

/// The Sudoku Board
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    elements: [[i32; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_vec(vec: [[i32; WIDTH]; HEIGHT]) -> Self {
        Self { elements: vec }
    }

    pub fn iter(&self) -> Iter<'_, [i32; WIDTH]> {
        self.elements.iter()
    }

    pub fn find_empty(&self) -> Option<(usize, usize)> {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                if self.elements[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn valid(&self, pos: (usize, usize), num: i32) -> bool {
        for i in 0..WIDTH {
            if self.elements[i][pos.1] == num && (i, pos.1) != pos {
                return false;
            }
        }

        for j in 0..HEIGHT {
            if self.elements[pos.0][j] == num && (pos.0, j) != pos {
                return false;
            }
        }

        let start_i = pos.0 - pos.0 % 3;
        let start_j = pos.1 - pos.1 % 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.elements[start_i + i][start_j + j] == num && (start_i + i, start_j + j) != pos {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        let empty = self.find_empty();

        if empty.is_none() {
            return true;
        }

        let pos = empty.unwrap();
        for nums in 0..WIDTH {
            if self.valid(pos, (nums + 1) as i32) {
                self.elements[pos.0][pos.1] = (nums + 1) as i32;

                if self.solve() {
                    return true;
                }
                self.elements[pos.0][pos.1] = 0;
            }
        }
        false
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Board<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self {
            elements: [[0; WIDTH]; HEIGHT]
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> std::fmt::Display for Board<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                board_string.push_str(&format!("{}", self.elements[i][j].to_string() + " "));
                if (j + 1) % 3 == 0 && j != 0 && j + 1 != 9 {
                    board_string.push_str("| ");
                }

                if j == HEIGHT - 1 {
                    board_string.push('\n');
                }

                if j == HEIGHT - 1 && (i + 1) % 3 == 0 && i + 1 != WIDTH {
                    board_string.push_str("- - - - - - - - - - - \n");
                }
            }
        }
        writeln!(f, "{board_string}")
    }
}

