#[derive(PartialEq)]
#[derive(Debug)]
struct Rubiks{
	front: [i32; 9], 
	right: [i32; 9],
	back: [i32; 9],
	left: [i32; 9],
	top: [i32; 9], // folded out this goes down from the top right
	bottom: [i32; 9]
}

fn main() {
    let mut rubiks = Rubiks {
    	front: [0i32 ; 9], 
		right: [1i32 ; 9],
		back: [2i32 ; 9],
		left: [3i32 ; 9],
		top: [4i32 ; 9],
		bottom: [5i32 ; 9]
	};
	//let mut horizontal_faces = [rubiks.front, rubiks.right, rubiks.back, rubiks.left];
    //let mut vertical_faces = [rubiks.front, rubiks.bottom, rubiks.back, rubiks.top];

    rubiks.top = rotate_face_clockwise([0,1,2,3,4,5,6,7,8]);

   	rotate_right_clockwise(rubiks);

}

fn flip_horz(face: [i32; 9]) -> [i32; 9] {
	[face[2], face[1], face[0],
	face[5], face[4], face[3],
	face[8], face[7], face[6]]
}

fn flip_vert(face: [i32; 9]) -> [i32; 9] {
	[face[6], face[7], face[8],
	face[3], face[4], face[5],
	face[0], face[1], face[2]]
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

fn rotate_right_clockwise(rubiks: Rubiks) -> Rubiks{
	let mut my_rubiks = rubiks;
	my_rubiks.right = rotate_face_clockwise (my_rubiks.right);
	//my_rubiks.top = flip_vert(flip_horz(my_rubiks.top));
	
	let front_values = my_rubiks.front;

	my_rubiks.front[2] = my_rubiks.bottom[2]; 
	my_rubiks.front[5] = my_rubiks.bottom[5]; 
	my_rubiks.front[8] = my_rubiks.bottom[8]; 

	my_rubiks.bottom[2] = my_rubiks.back[2]; 
	my_rubiks.bottom[5] = my_rubiks.back[5]; 
	my_rubiks.bottom[8] = my_rubiks.back[8]; 

	my_rubiks.back[2] = my_rubiks.top[2]; 
	my_rubiks.back[5] = my_rubiks.top[5]; 
	my_rubiks.back[8] = my_rubiks.top[8]; 

	my_rubiks.top[2] = front_values[2];
	my_rubiks.top[5] = front_values[5];
	my_rubiks.top[8] = front_values[8];

	//my_rubiks.top = flip_horz(flip_vert(my_rubiks.top));
	my_rubiks
} 

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rotate_face_clockwise() {
		assert_eq!(
			[0, 3, 6, 1, 4, 7, 2, 5, 8],
			rotate_face_clockwise([0, 1, 2, 3, 4, 5, 6, 7, 8]));
	}

	#[test]
	fn test_rotate_face_anticlockwise() {
		assert_eq!(
			[8, 5, 2, 7, 4, 1, 6, 3, 0],
			rotate_face_anticlockwise([0, 1, 2, 3, 4, 5, 6, 7, 8]));
	}

	#[test]
	fn test_flip_horz(){
		assert_eq!(
			[2,1,0,5,4,3,8,7,6],
			flip_horz([0,1,2,3,4,5,6,7,8]));
	}

	#[test]
	fn test_flip_vert(){
		assert_eq!(
			[6,7,8,3,4,5,0,1,2],
			flip_vert([0,1,2,3,4,5,6,7,8]))
	}

	#[test]
	fn test_double_flip(){
		assert_eq!(
			[8,7,6,5,4,3,2,1,0],
			flip_vert(flip_horz([0,1,2,3,4,5,6,7,8])))
	}

	#[test]
	fn test_rotate_right_clockwise(){
		let rubiks = Rubiks {
    		front: [0i32 ; 9], 
			right: [0,1,2,3,4,5,6,7,8],
			back: [2i32 ; 9],
			left: [3i32 ; 9],
			top: [4i32 ; 9],
			bottom: [5i32 ; 9]
		};

		let test_rubiks = Rubiks {
			front: [0,0,5,0,0,5,0,0,5],
			right: [0,3,6,1,4,7,2,5,8],
			back: [2,2,4,2,2,4,2,2,4],
			left: [3i32 ; 9],
			top: [4,4,0,4,4,0,4,4,0],
			bottom: [5,5,2,5,5,2,5,5,2]
		};

		assert_eq!(
			test_rubiks,
			rotate_right_clockwise(rubiks))
	}
}