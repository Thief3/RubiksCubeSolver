use std::fmt;

enum Faces {
	FRONT = 0,
	RIGHT = 1,
	BACK = 2,
	LEFT = 3,
	TOP = 4,
	BOTTOM = 5,
}

fn main() {
    println!("{:?}", rotate_face_anticlockwise([0,1,2,3,4,5,6,7,8]));
    let mut horizontal_faces = [Faces::FRONT, Faces::RIGHT, Faces::BACK, Faces::LEFT];
    let mut vertical_faces = [Faces::FRONT, Faces::BOTTOM, Faces::BACK, Faces::TOP];
}

fn rotate_face_clockwise (face: [i32; 9]) -> [i32; 9] {
	[face[0], face[3], face[6],
	face[1], face[4], face[7],
	face[2], face[5], face[8]]
}

fn rotate_face_anticlockwise (face: [i32; 9]) -> [i32; 9]{
	[face[8], face[5], face[2],
	face[7], face[4], face[1],
	face[6], face[3], face[0]]
}
