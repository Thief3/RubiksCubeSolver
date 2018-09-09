mod corner_cubies;

#[derive(PartialEq, Debug)]
pub struct Cube {
    corner_orientation: i32,
    edge_orientation: i32,
    corner_permutation: i32,
    edge_permutation: i32,
    ud_slice: i32,
    corners: [corner_cubies::CornerCubie; 8],
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            corner_orientation: 0,
            edge_orientation: 0,
            corner_permutation: 0,
            edge_permutation: 0,
            ud_slice: 0,
            corners: [
                corner_cubies::CornerCubie::new(corner_cubies::Corner::URF),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::UFL),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::ULB),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::UBR),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DFR),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DLF),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DBL),
                corner_cubies::CornerCubie::new(corner_cubies::Corner::DRB),
            ],
        }
    }

    pub fn f(&mut self) {
        for i in 0..7 {
            self.corners[i].f();
        }
    }
    pub fn b(&mut self) {
        for i in 0..7 {
            self.corners[i].b();
        }
    }
    pub fn l(&mut self) {
        for i in 0..7 {
            self.corners[i].l();
        }
    }
    pub fn r(&mut self) {
        for i in 0..7 {
            self.corners[i].r();
        }
    }
    pub fn u(&mut self) {
        for i in 0..7 {
            self.corners[i].u();
        }
    }
    pub fn d(&mut self) {
        for i in 0..7 {
            self.corners[i].d();
        }
    }
}