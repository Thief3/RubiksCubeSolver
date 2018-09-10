//mod notation;

mod physical;
mod utility;

fn main() {
    let mut c = physical::Cube::new();
    c.calculate_ud_slice();
    println!("{}", c.ud_slice);
    //println!("{}", utility::factorial(0));
}
