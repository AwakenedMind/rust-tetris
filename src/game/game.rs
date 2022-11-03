use std::{collections::HashSet, mem, vec};

use crate::tetromino::tetromino::{Pos, Tetromino};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}
/** Tetris Game Model */
#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    cur_tetromino: Tetromino,
    board_tetrominoes: Vec<Tetromino>,
    lost: bool,
}

impl Tetris {
    /** Create a new Tetris Instance */
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            /** Ensure width and height are always greater than 0 */
            width: width as i32,
            height: height as i32,
            cur_tetromino: &Tetromino::random_tetromino() + Pos((width as i32 - 1) / 2, 0),
            board_tetrominoes: vec![],
            lost: false,
        }
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let height = self.height;
        let width = self.width;

        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x, y)))
    }

    pub fn get(&self, pos: Pos) -> Option<&'static str> {
        if self.cur_tetromino.has_position(pos) {
            Some(self.cur_tetromino.typ())
        } else {
            self.board_tetrominoes
                .iter()
                .find(|t| t.has_position(pos))
                .map(|t| t.typ())
        }
    }

    pub fn is_line_full(&self, y: i32) -> bool {
        self.board_tetrominoes
            .iter()
            .flat_map(|t| t.iter_positions())
            .filter(|pos| pos.1 == y)
            .collect::<HashSet<_>>()
            .len() as i32
            == self.width
    }

    fn remove_line(&mut self, y: i32) {
        for t in self.board_tetrominoes.iter_mut() {
            t.remove_line(y)
        }
    }

    fn remove_full_lines(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y)
            }
        }
    }

    /** tick will move the current tetromino down by add 1 on the y position */
    pub fn tick(&mut self) {
        if self.lost {
            return;
        }

        let translated_tetromino = &self.cur_tetromino + Pos(0, 1);

        if self.is_out_of_bounds(&translated_tetromino) || self.has_collided(&translated_tetromino)
        {
            // now we transition the current shape onto the board tetrominoes and generate a new cur tetromino
            let new_board_tetromino = mem::replace(
                &mut self.cur_tetromino,
                &Tetromino::random_tetromino() + Pos(self.width / 2, 0),
            );
            self.board_tetrominoes.push(new_board_tetromino);
            self.remove_full_lines();

            // check if the newly generated tetromino collided
            if self.has_collided(&self.cur_tetromino) {
                self.lost = true
            }
        } else {
            self.cur_tetromino = translated_tetromino;
        }
    }

    /** checks whether the the tetromino is out of bounds on the x pos  */
    pub fn is_out_of_bounds(&self, t: &Tetromino) -> bool {
        !t.iter_positions()
            .all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 < pos.1 && pos.1 < self.height)
    }

    /** tetrominoes will permanently sit on the bottom of the board so we need to check if current tetromino has collided with any of them */
    pub fn has_collided(&self, t: &Tetromino) -> bool {
        self.board_tetrominoes
            .iter()
            .any(|boardT| boardT.collides_with(t))
    }

    pub fn shift_tetromino(&mut self, d: Direction) {
        let translated_tetromino = &self.cur_tetromino
            + match d {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };

        if !self.is_out_of_bounds(&translated_tetromino)
            && !self.has_collided(&translated_tetromino)
        {
            self.cur_tetromino = translated_tetromino;
        }
    }

    /** Calcuates if the current tetromino would collide if it was rotated */
    pub fn rotated_tetromino(&mut self) {
        let rotated_tetromino = self.cur_tetromino.rotated();

        if !self.is_out_of_bounds(&rotated_tetromino) && !self.has_collided(&rotated_tetromino) {
            self.cur_tetromino = rotated_tetromino;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;

    #[test]
    fn test() {
        let mut game = Tetris::new(10, 30);
        Tetris::tick(&mut game);
        Tetris::tick(&mut game);
        Tetris::tick(&mut game);

        println!("{:#?}", game)
    }
}
