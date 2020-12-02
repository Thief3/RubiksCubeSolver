pub mod cubie_cube;
pub mod face_cube;
pub mod coord_cube;

// 3^7 possible corner orientations
pub const TWIST: usize = 2187;
// 2^11 possible edge flips
pub const FLIP: usize = 2048;
// 12C4 possible positions of FR, FL, BL, BR
pub const UDSLICE: usize = 495;
// 4! possible permutations of FR, FL, BL, BR
pub const EDGE4: usize = 24;
// 8! possible permutations of UR, UF, UL, UB, DR, DF, DL, DB in phase two
pub const EDGE8: usize = 40320;
// 8! possible permutations of the corners
pub const CORNER: usize = 40320;
// 12! possible permutations of all edges
pub const EDGE: usize = 479001600;
// 6*3 possible moves
pub const MOVES: usize = 18;

pub fn r_cast(x: isize, y: usize) -> usize{
    if x < 0 {
        return (y as isize + x) as usize;
    }
    
    return x as usize
}

pub fn emod(a: isize, b: usize) -> isize {
    let c: isize = b as isize;
    ((a % c) + c) % c
}
