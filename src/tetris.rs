use crate::tetromino::Tetromino;

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
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            cur_tetromino: Tetromino::random_tetromino(),
            board_tetrominoes: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;

    #[test]
    fn test() {
        println!("{:#?}", Tetris::new(5, 10))
    }
}
