use core::ops::arith{Add};
use std::{collections::HashSet, ops::Add};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Tetromino {
    /** The (x, y) position of the tetromino on the board */
    position: HashSet<Pos>,
    /** the (x, y) position of the tetromino for which when rotated will not change */
    anchor: Pos,
}

/** generates a new array of tetrominos given a position and anchor  */
macro_rules! impl_tetrominoes{
    // macro definition
    // $new:ident  - ident designator with the name new since we are creating new tetrominoes acts as the function name
    // $pos: expr  - evaluates to a value in this case will evaluate $pos and $anchor to produce a value
    // ,*          - repitition operator delimited by a comma
    // $()*        - will loop over new() for each element in the array
     ($($new:ident: [$($pos: expr),*] anchored at $anchor:expr; )*) => {
        $(
            pub fn $new() -> Self {
                Self {
                    position: [$($pos),*]
                        .into_iter()
                        .collect(),
                    anchor: $anchor,
                }
            }
        )*
    };
}

impl Tetromino {
    /* Stores the seven basic types of tetrominoes */
    impl_tetrominoes! {
      new_i: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)]  anchored at Pos(1, 0);
      new_o: [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)]  anchored at Pos(0, 0);
      new_t: [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)]  anchored at Pos(1, 0);
      new_j: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] anchored at Pos(0, 1);
      new_l: [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)]  anchored at Pos(0, 1);
      new_s: [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] anchored at Pos(0, 0);
      new_z: [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] anchored at Pos(0, 0);
    }

    pub fn random_tetromino() -> Self {
        let random = (rand::random::<f64>() * 7.0).floor() as u8;

        match random {
            0 => Self::new_i(),
            1 => Self::new_o(),
            2 => Self::new_t(),
            3 => Self::new_j(),
            4 => Self::new_l(),
            5 => Self::new_s(),
            6 => Self::new_z(),
            _ => unreachable!(),
        }
    }
}

impl Add for Shape {}
