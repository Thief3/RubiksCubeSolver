//mod notation;

mod physical;
mod utility;
mod solver;

fn main() {
    let mut c = physical::Cube::new();
    c.r();
    c.r();
    c.r();
    solver::phase_one::search(&mut c, 2);
    //println!("{}", utility::factorial(0));
}
