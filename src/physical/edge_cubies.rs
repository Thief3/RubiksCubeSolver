use std::cmp::Ordering;

/// A numbered enum of the edge pieces.
/// 
/// It is numbered to make ordered operations for permutation calculations,
/// easier to compute. The order is important.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Edge {
    UR = 1,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BL,
    BR,
}

#[derive(Debug, Eq, Copy, Clone)]
pub struct EdgeCubie{
    pub orientation: i32,
    pub coordinate: Edge,
    pub old_coordinate: Edge,
}

impl Ord for EdgeCubie{
    fn cmp(&self, other: &EdgeCubie) -> Ordering {
        (self.coordinate as i32).cmp(&(other.coordinate as i32))
    }
}

impl PartialOrd for EdgeCubie{
    fn partial_cmp(&self, other: &EdgeCubie) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EdgeCubie{
    fn eq(&self, other: &EdgeCubie) -> bool {
        (self.coordinate as i32) == (other.coordinate as i32)
    }
}

impl EdgeCubie {
    pub fn new(e: Edge)-> EdgeCubie{
        let a = EdgeCubie{
            orientation: 0,
            coordinate: e,
            old_coordinate: e,
        };

        a
    }

    fn movement(&mut self, edges: &[Edge; 12], orientation_change: &[i32; 12]){
        self.old_coordinate = self.coordinate;
        match self.coordinate {
            Edge::UR => {
                self.coordinate = edges[0];
                self.orientation = (self.orientation + orientation_change[0]) % 3;
            }
            Edge::UF => {
                self.coordinate = edges[1];
                self.orientation = (self.orientation + orientation_change[1]) % 3;
            }
            Edge::UL => {
                self.coordinate = edges[2];
                self.orientation = (self.orientation + orientation_change[2]) % 3;
            }
            Edge::UB => {
                self.coordinate = edges[3];
                self.orientation = (self.orientation + orientation_change[3]) % 3;
            }
            Edge::DR => {
                self.coordinate = edges[4];
                self.orientation = (self.orientation + orientation_change[4]) % 3;
            }
            Edge::DF => {
                self.coordinate = edges[5];
                self.orientation = (self.orientation + orientation_change[5]) % 3;
            }
            Edge::DL => {
                self.coordinate = edges[6];
                self.orientation = (self.orientation + orientation_change[6]) % 3;
            }
            Edge::DB => {
                self.coordinate = edges[7];
                self.orientation = (self.orientation + orientation_change[7]) % 3;
            }
            Edge::FR => {
                self.coordinate = edges[8];
                self.orientation = (self.orientation + orientation_change[8]) % 3;
            }
            Edge::FL => {
                self.coordinate = edges[9];
                self.orientation = (self.orientation + orientation_change[9]) % 3;
            }
            Edge::BL => {
                self.coordinate = edges[10];
                self.orientation = (self.orientation + orientation_change[10]) % 3;
            }
            Edge::BR => {
                self.coordinate = edges[11];
                self.orientation = (self.orientation + orientation_change[11]) % 3;
            }
        }
    }
        /// Typical rubiks cube movements.

    /// A forward clockwise movement.
    pub fn f(&mut self) {
        self.movement(&F_EDGE_TRANSFORM, &F_EDGE_ORIENTATION_TRANSFORM)
    }

    /// A back clockwise movement.
    pub fn b(&mut self) {
        self.movement(&B_EDGE_TRANSFORM, &B_EDGE_ORIENTATION_TRANSFORM)
    }

    /// A right clockwise movement.
    pub fn r(&mut self) {
        self.movement(&R_EDGE_TRANSFORM, &R_EDGE_ORIENTATION_TRANSFORM)
    }

    /// A left clockwise movement.
    pub fn l(&mut self) {
        self.movement(&L_EDGE_TRANSFORM, &L_EDGE_ORIENTATION_TRANSFORM)
    }

    /// A upper clockwise movement.
    pub fn u(&mut self) {
        self.movement(&U_EDGE_TRANSFORM, &U_EDGE_ORIENTATION_TRANSFORM)
    }

    /// A down clockwise movement.
    pub fn d(&mut self) {
        self.movement(&D_EDGE_TRANSFORM, &D_EDGE_ORIENTATION_TRANSFORM)
    }
}

/// ***************************************************************************
/// The variables used in the generic `movement` function above. These are
/// static as they'll be called a lot and there is no reason to create them
/// each time instead of referencing these values.
/// 
/// Obtained from (http://kociemba.org/math/CubeDefs.htm)
/// ***************************************************************************

static F_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR,
    Edge::FL,
    Edge::UL,
    Edge::UB,
    Edge::DR,
    Edge::FR,
    Edge::DL,
    Edge::DB,
    Edge::UF,
    Edge::DF,
    Edge::BL,
    Edge::BR,
];
static F_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0];

static B_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR, 
    Edge::UF,
    Edge::UL,
    Edge::BR,
    Edge::DR,
    Edge::DF,
    Edge::DL,
    Edge::BL,
    Edge::FR,
    Edge::FL,
    Edge::UB,
    Edge::DB,
];
static B_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1];

static R_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::FR, 
    Edge::UF,
    Edge::UL,
    Edge::UB,
    Edge::BR,
    Edge::DF,
    Edge::DL,
    Edge::DB,
    Edge::DR,
    Edge::FL,
    Edge::BL,
    Edge::UR,
];
static R_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

static L_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR, 
    Edge::UF,
    Edge::BL,
    Edge::UB,
    Edge::DR,
    Edge::DF,
    Edge::FL,
    Edge::DB,
    Edge::FR,
    Edge::UL,
    Edge::DL,
    Edge::BR,
];
static L_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

static U_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UB, 
    Edge::UR,
    Edge::UF,
    Edge::UL,
    Edge::DR,
    Edge::DF,
    Edge::DL,
    Edge::DB,
    Edge::FR,
    Edge::FL,
    Edge::BL,
    Edge::BR,
];
static U_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

static D_EDGE_TRANSFORM: [Edge; 12] = [
    Edge::UR, 
    Edge::UF,
    Edge::UL,
    Edge::UB,
    Edge::DF,
    Edge::DL,
    Edge::DB,
    Edge::DR,
    Edge::FR,
    Edge::FL,
    Edge::BL,
    Edge::BR,
];
static D_EDGE_ORIENTATION_TRANSFORM: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
