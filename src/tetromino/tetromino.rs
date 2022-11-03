use std::{collections::HashSet, ops::Add};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    /** The (x, y) positions of the tetromino on the board */
    positions: HashSet<Pos>,
    /** A single (x, y) position of the tetromino for which when rotated will not change */
    anchor: Pos,
    typ: &'static str,
}

/** generates a new array of tetrominos given a position and anchor  */
macro_rules! impl_tetrominoes{
    // macro definition
    // $new:ident  - ident designator with the name new since we are creating new tetrominoes acts as the function name
    // $pos: expr  - evaluates to a value in this case will evaluate $pos and $anchor to produce a value
    // ,*          - repitition operator delimited by a comma
    // $()*        - will loop over new() for each element in the array
     ($($new:ident $typ:literal: [$($pos: expr),*] @ $anchor:expr; )*) => {
        $(
            pub fn $new() -> Self {
                Self {
                    typ: $typ,
                    positions: [$($pos),*]
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
        new_i "🟦": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
        new_o "🟨": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
        new_t "🟫": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(1, 0);
        new_j "🟪": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
        new_l "🟧": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
        new_s "🟩": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
        new_z "🟥": [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
    }

    /**
     * Gets a copy of the current positions of the Tetromino
     * - elided lifetime
     **/
    pub fn positions(&self) -> impl Iterator<Item = Pos> + '_ {
        self.positions.iter().copied()
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        self.positions.iter().copied()
    }

    pub fn typ(&self) -> &'static str {
        self.typ
    }

    /** compares the current Tetromino pos with another (intersection of hashsets)  */
    pub fn collides_with(&self, other: &Tetromino) -> bool {
        self.positions.intersection(&other.positions).count() > 0
    }

    /**
     * Rotate a tetromino using R(x, y) = (x + yi) * i = xi + y*i^2 = -y + x^i = (-y, x)
     *
     * in order to rotate by 90degs without using a rotation matrix algorithm (linear algebra)
     * - we can use euclidean geometry to calculate the final positions of the tetromino instead which will account for theta + 90deg - theta = 90deg
     *   we use x to represent the real axis and y to represent an imaginary axis
     * - on a complex plane, a 90 deg rotation can be represented as
     *
     *  (a, b) represents the anchored position
     *  (xi, yi) represents the given point of the tetromino
     *
     *  1. We must subtract the tetromino's x position by the anchor x position, and subtract the tetromino's y position by the anchor y position
     *     (rx, ry) = R(xi - a, yi - b)
     *     = (-yi + b, xi - a)
     *
     *  2. Add anchor again to the rotated position  
     *     (rx + a, ry + b) = (-yi + b + a, xi - a + b)
     *
     */
    pub fn rotated(&self) -> Self {
        let Pos(a, b) = self.anchor;

        Self {
            typ: self.typ,
            positions: self
                .positions()
                .map(|Pos(x, y)| Pos(-y + b + a, x - a + b))
                .collect(),
            anchor: self.anchor,
        }
    }

    pub fn has_position(&self, pos: Pos) -> bool {
        self.positions.contains(&pos)
    }

    /** filter all positions that are not y add 1 since we are moving down */
    pub fn remove_line(&mut self, y: i32) {
        self.positions = self
            .positions
            .iter()
            .copied()
            .filter(|pos| pos.1 != y)
            .map(|pos| {
                if pos.1 >= y {
                    pos
                } else {
                    Pos(pos.0, pos.1 + 1)
                }
            })
            .collect()
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

/**
 * Tetrominoes start at (0,0) on the board
 * - we want to spawn teterominoes in the center of the board so they need to be translated using Add<T>
 * - we deference pos since we cannot add pos + &pos
 */
impl Add<Pos> for &Tetromino {
    type Output = Tetromino;

    /** Returns a new tetromino  */
    fn add(self, rhs: Pos) -> Self::Output {
        Tetromino {
            typ: self.typ,
            positions: self.positions.iter().map(|pos| *pos + rhs).collect(),
            anchor: self.anchor + rhs,
        }
    }
}
