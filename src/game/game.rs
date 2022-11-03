use crate::tetromino::tetromino::{Pos, Tetromino};
/** Tetris Game Model */
#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    cur_tetromino: Tetromino,
    board_tetrominoes: Vec<Tetromino>,
}

impl Tetris {
    /** Create a new Tetris Instance */
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            /** Ensure width and height are always greater than 0 */
            width: width as i32,
            height: height as i32,
            cur_tetromino: &Tetromino::random_tetromino() + Pos(((width as i32) / 2), 0),
            board_tetrominoes: vec![],
        }
    }
    /** tick will move the current tetromino dow by add 1 on the y position */
    pub fn tick(&mut self) {
        self.cur_tetromino = &self.cur_tetromino + Pos(0, 1)
    }

    /** checks whether the the tetromino is out of bounds on the x pos  */
    pub fn is_out_of_bounds(&self, t: Tetromino) -> bool {
        t.positions().all(|pos| 0 <= pos.0 && pos.0 < self.width)
    }

    /** tetrominoes will permanently sit on the bottom of the board so we need to check if current tetromino has collided with any of them */
    pub fn has_collided(&self, t: &Tetromino) -> bool {
        self.board_tetrominoes.iter().any(|boardT| todo!())
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
