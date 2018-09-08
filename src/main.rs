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

    fn f(&mut self) {
        match self.coordinate {
            Corner::URF => {
                self.coordinate = Corner::UFL;
                self.orientation = (self.orientation + 1) % 3;
            }
            Corner::UFL => {
                self.coordinate = Corner::DLF;
                self.orientation = (self.orientation + 2) % 3;
            }
            Corner::ULB => {
                self.coordinate = Corner::ULB;
                self.orientation = (self.orientation + 0) % 3;
            }
            Corner::UBR => {
                self.coordinate = Corner::UBR;
                self.orientation = (self.orientation + 0) % 3;
            }
            Corner::DFR => {
                self.coordinate = Corner::URF;
                self.orientation = (self.orientation + 2) % 3;
            }
            Corner::DLF => {
                self.coordinate = Corner::DFR;
                self.orientation = (self.orientation + 1) % 3;
            }
            Corner::DBL => {
                self.coordinate = Corner::DBL;
                self.orientation = (self.orientation + 0) % 3;
            }
            Corner::DRB => {
                self.coordinate = Corner::DRB;
                self.orientation = (self.orientation + 0) % 3;
            }
        };
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
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
    c.f();
    c.f();
    c.f();
    c.f();
    println!("{:?}", c);
}
