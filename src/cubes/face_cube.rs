pub struct FaceCube {
    f: []
}

impl FaceCube {
    /// Make a new FaceCube from a Vec<char>
    pub fn new(cube_array: Vec<char>){
        if cube_array.len > 54 {
            panic!("You're passing in too large a cube!!");
        }
        else if cube_array.len < 54 {
            panic!("You're passing in too small a cube!!");
        }

        for i in 0..cube_array.len() {
            f[i] = cube_array[i].get_facelets();
        }
    }

    /// Make a new default state FaceCube.
    pub fn reset() -> FaceCube{
        FaceCube::new("UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB".chars().collect)
    }

    pub fn to_string(&self) -> String{
        self.f.join()
    }
}
