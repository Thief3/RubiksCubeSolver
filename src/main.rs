//mod notation;

mod physical;

fn main() {
    let mut c = physical::Cube::new();
    c.d();
    c.d();
    c.d();
    c.d();
    println!("{:?}", c);
}
