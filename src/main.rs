struct Rubiks{
	front: [i32; 9], 
	right: [i32; 9],
	back: [i32; 9],
	left: [i32; 9],
	top: [i32; 9],
	bottom: [i32; 9]
}

fn main() {
    let mut rubiks = Rubiks {
    	front: [0,1,2,3,4,5,6,7,8], 
		right: [0,1,2,3,4,5,6,7,8],
		back: [0,1,2,3,4,5,6,7,8],
		left: [0,1,2,3,4,5,6,7,8],
		top: [0,1,2,3,4,5,6,7,8],
		bottom: [0,1,2,3,4,5,6,7,8]
	};
	//let mut horizontal_faces = [rubiks.front, rubiks.right, rubiks.back, rubiks.left];
    //let mut vertical_faces = [rubiks.front, rubiks.bottom, rubiks.back, rubiks.top];

    rubiks.top = rotate_face_clockwise([0,1,2,3,4,5,6,7,8]);

    println!("{:?}", rubiks.top);

}

/// Roates a rubiks cube face clockwise.
fn rotate_face_clockwise (face: [i32; 9]) -> [i32; 9] {
	[face[0], face[3], face[6],
	face[1], face[4], face[7],
	face[2], face[5], face[8]]
}

/// Roates a rubiks cube face anticlockwise.
fn rotate_face_anticlockwise (face: [i32; 9]) -> [i32; 9]{
	[face[8], face[5], face[2],
	face[7], face[4], face[1],
	face[6], face[3], face[0]]
}
