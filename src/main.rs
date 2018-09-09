//mod notation;

mod physical;
mod utility;

fn main() {
    let mut c = physical::Cube::new();
    c.r();
    c.calculate_corner_orientation();
    println!("{:?}", c.corner_orientation);
}
