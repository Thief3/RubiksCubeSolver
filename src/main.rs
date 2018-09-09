//mod notation;

#[derive(PartialEq, Debug)]
pub struct Cube {
    corner_orientation: i32,
    edge_orientation: i32,
    corner_permutation: i32,
    edge_permutation: i32,
    ud_slice: i32,
    corners: [CornerCubie; 8],
}

impl Cube {
    fn new() -> Cube {
        Cube {
            corner_orientation: 0,
            edge_orientation: 0,
            corner_permutation: 0,
            edge_permutation: 0,
            ud_slice: 0,
            corners: [
                CornerCubie::new(Corner::URF),
                CornerCubie::new(Corner::UFL),
                CornerCubie::new(Corner::ULB),
                CornerCubie::new(Corner::UBR),
                CornerCubie::new(Corner::DFR),
                CornerCubie::new(Corner::DLF),
                CornerCubie::new(Corner::DBL),
                CornerCubie::new(Corner::DRB),
            ],
        }
    }

    fn f(&mut self) {
        for i in 0..7 {
            self.corners[i].f();
        }
    }
    fn b(&mut self) {
        for i in 0..7 {
            self.corners[i].b();
        }
    }
    fn l(&mut self) {
        for i in 0..7 {
            self.corners[i].l();
        }
    }
    fn r(&mut self) {
        for i in 0..7 {
            self.corners[i].r();
        }
    }
    fn u(&mut self) {
        for i in 0..7 {
            self.corners[i].u();
        }
    }
    fn d(&mut self) {
        for i in 0..7 {
            self.corners[i].d();
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CornerCubie {
    orientation: i32,
    coordinate: Corner,
}

impl CornerCubie {
    fn new(c: Corner) -> CornerCubie {
        let a = CornerCubie {
            orientation: 0,
            coordinate: c,
        };

        a
    }

    fn movement(&mut self, corners: &[Corner; 8], orientation_change: [i32; 8]) {
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

    fn f(&mut self) {
        self.movement(&F_CORNER_TRANSFORM, F_CORNER_ORIENTATION_TRANSFORM)
    }
    fn b(&mut self) {
        self.movement(&B_CORNER_TRANSFORM, B_CORNER_ORIENTATION_TRANSFORM)
    }
    fn r(&mut self) {
        self.movement(&R_CORNER_TRANSFORM, R_CORNER_ORIENTATION_TRANSFORM)
    }
    fn l(&mut self) {
        self.movement(&L_CORNER_TRANSFORM, L_CORNER_ORIENTATION_TRANSFORM)
    }
    fn u(&mut self) {
        self.movement(&U_CORNER_TRANSFORM, U_CORNER_ORIENTATION_TRANSFORM)
    }
    fn d(&mut self) {
        self.movement(&D_CORNER_TRANSFORM, D_CORNER_ORIENTATION_TRANSFORM)
    }
}

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
    Corner::DRB,
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

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum Corner {
    URF = 1,
    UFL,
    ULB,
    UBR,
    DFR,
    DLF,
    DBL,
    DRB,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Edge {
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

fn main() {
    let mut c = Cube::new();
    c.d();
    c.d();
    c.d();
    c.d();
    println!("{:?}", c);
}
