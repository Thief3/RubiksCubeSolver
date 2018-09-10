//mod notation;

mod physical;
mod solver;
mod utility;

fn main() {
    let mut c = physical::Cube::new();
    c.r();
    c.r();
    c.r();
    solver::search(&mut c, 12);
    //println!("{}", utility::factorial(0));
}
