//! A module relating specifically to the corner pieces of the rubiks cube.
//!
//! Deals with movements and how they shift the coordinates and orientation
//! of the corner cube in question.

/// A numbered enum of the corner pieces.
///
/// It is numbered to make ordered operations for permutation calculations,
/// easier to compute. The order is important.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Corner {
    URF = 0,
    UFL,
    ULB,
    UBR,
    DFR,
    DLF,
    DBL,
    DRB,
}

/// The main CornerCubie struct.
///
/// # Variables
/// * `orientation` - A value of 0, 1, and 2, where 0 is the default
///     orientation, 1 a clockwise twist, and 2 an anti-clockwise twist.
/// * `coordinate` - A `Corner` that represents the cubes current position.
/// * `old_coordiante` - The `coordinate` that was last held before a move.
#[derive(Debug, PartialEq)]
pub struct CornerCubie {
    pub orientation: i32,
    pub coordinate: Corner,
    pub old_coordinate: Corner,
}

impl CornerCubie {
    /// Creates a new `CornerCubie` with coordinate c.
    ///
    /// # Parameters
    /// * `c` - The default `Corner` to set.
    /// # Return
    /// * `CornerCubie`
    pub fn new(c: Corner) -> CornerCubie {
        let a = CornerCubie {
            orientation: 0,
            coordinate: c,
            old_coordinate: c,
        };

        a
    }

    /// A generic movement function.
    ///
    /// # Parameters
    /// * `corners` - A reference of what each corner should become with this
    ///     movement.
    /// * `orientation_change` - An array of 8 `i32` types, each relating to
    ///     the additional orientation change.
    fn movement(&mut self, corners: &[Corner; 8], orientation_change: &[i32; 8]) {
        self.old_coordinate = self.coordinate;
        match self.coordinate {
            Corner::URF => {
                self.coordinate = corners[0];
                self.orientation = (self.orientation + orientation_change[0]) % 3;
            }
            Corner::UFL => {
                self.coordinate = corners[1];
                self.orientation = (self.orientation + orientation_change[1]) % 3;
            }
            Corner::ULB => {
                self.coordinate = corners[2];
                self.orientation = (self.orientation + orientation_change[2]) % 3;
            }
            Corner::UBR => {
                self.coordinate = corners[3];
                self.orientation = (self.orientation + orientation_change[3]) % 3;
            }
            Corner::DFR => {
                self.coordinate = corners[4];
                self.orientation = (self.orientation + orientation_change[4]) % 3;
            }
            Corner::DLF => {
                self.coordinate = corners[5];
                self.orientation = (self.orientation + orientation_change[5]) % 3;
            }
            Corner::DBL => {
                self.coordinate = corners[6];
                self.orientation = (self.orientation + orientation_change[6]) % 3;
            }
            Corner::DRB => {
                self.coordinate = corners[7];
                self.orientation = (self.orientation + orientation_change[7]) % 3;
            }
        };
    }

    /// Typical rubiks cube movements.

    /// A forward clockwise movement.
    pub fn f(&mut self) {
        self.movement(&F_CORNER_TRANSFORM, &F_CORNER_ORIENTATION_TRANSFORM)
    }

    /// A back clockwise movement.
    pub fn b(&mut self) {
        self.movement(&B_CORNER_TRANSFORM, &B_CORNER_ORIENTATION_TRANSFORM)
    }

    /// A right clockwise movement.
    pub fn r(&mut self) {
        self.movement(&R_CORNER_TRANSFORM, &R_CORNER_ORIENTATION_TRANSFORM)
    }

    /// A left clockwise movement.
    pub fn l(&mut self) {
        self.movement(&L_CORNER_TRANSFORM, &L_CORNER_ORIENTATION_TRANSFORM)
    }

    /// A upper clockwise movement.
    pub fn u(&mut self) {
        self.movement(&U_CORNER_TRANSFORM, &U_CORNER_ORIENTATION_TRANSFORM)
    }

    /// A down clockwise movement.
    pub fn d(&mut self) {
        self.movement(&D_CORNER_TRANSFORM, &D_CORNER_ORIENTATION_TRANSFORM)
    }
}

/// ***************************************************************************
/// The variables used in the generic `movement` function above. These are
/// static as they'll be called a lot and there is no reason to create them
/// each time instead of referencing these values.
///
/// Obtained from (http://kociemba.org/math/CubeDefs.htm)
/// ***************************************************************************

static F_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::UFL,
    Corner::DLF,
    Corner::ULB,
    Corner::UBR,
    Corner::URF,
    Corner::DFR,
    Corner::DBL,
    Corner::DRB,
];
static F_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [1, 2, 0, 0, 2, 1, 0, 0];

static B_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::UBR,
    Corner::DRB,
    Corner::DFR,
    Corner::DLF,
    Corner::ULB,
    Corner::DBL,
];
static B_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 0, 1, 2, 0, 0, 2, 1];

static R_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::DFR,
    Corner::UFL,
    Corner::ULB,
    Corner::URF,
    Corner::DRB,
    Corner::DLF,
    Corner::DBL,
    Corner::UBR,
];
static R_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [2, 0, 0, 1, 1, 0, 0, 2];

static L_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::URF,
    Corner::ULB,
    Corner::DBL,
    Corner::UBR,
    Corner::DFR,
    Corner::UFL,
    Corner::DLF,
    Corner::DRB,
];
static L_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 1, 2, 0, 0, 2, 1, 0];

static U_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::UBR,
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::DFR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
];
static U_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

static D_CORNER_TRANSFORM: [Corner; 8] = [
    Corner::URF,
    Corner::UFL,
    Corner::ULB,
    Corner::UBR,
    Corner::DLF,
    Corner::DBL,
    Corner::DRB,
    Corner::DFR,
];
static D_CORNER_ORIENTATION_TRANSFORM: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
