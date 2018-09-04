#[derive(PartialEq, Debug)]
struct Rubiks {
    front: [i32; 9],
    right: [i32; 9],
    back: [i32; 9],
    left: [i32; 9],
    top: [i32; 9], // folded out this goes down from the top right
    bottom: [i32; 9],
}

fn main() {
    let mut rubiks = Rubiks {
        front: [0i32; 9],
        right: [1i32; 9],
        back: [2i32; 9],
        left: [3i32; 9],
        top: [4i32; 9],
        bottom: [5i32; 9],
    };
    //let mut horizontal_faces = [rubiks.front, rubiks.right, rubiks.back, rubiks.left];
    //let mut vertical_faces = [rubiks.front, rubiks.bottom, rubiks.back, rubiks.top];
    rubiks.top = rotate_face_clockwise([0, 1, 2, 3, 4, 5, 6, 7, 8]);
    rotate_right_clockwise(rubiks);
}

fn transpose(face: [i32; 9]) -> [i32; 9] {
    [
        face[8], face[5], face[2], face[7], face[4], face[1], face[6], face[3], face[0],
    ]
}

fn flip_horz(face: [i32; 9]) -> [i32; 9] {
    [
        face[2], face[1], face[0], face[5], face[4], face[3], face[8], face[7], face[6],
    ]
}

fn flip_vert(face: [i32; 9]) -> [i32; 9] {
    [
        face[6], face[7], face[8], face[3], face[4], face[5], face[0], face[1], face[2],
    ]
}

/// Roates a rubiks cube face clockwise.
fn rotate_face_clockwise(face: [i32; 9]) -> [i32; 9] {
    [
        face[2], face[5], face[8], face[1], face[4], face[7], face[0], face[3], face[6],
    ]
}

/// Roates a rubiks cube face anticlockwise.
fn rotate_face_anticlockwise(face: [i32; 9]) -> [i32; 9] {
    [
        face[6], face[3], face[0], face[7], face[4], face[1], face[8], face[5], face[2],
    ]
}

fn rotate_right_clockwise(rubiks: Rubiks) -> Rubiks {
    let mut my_rubiks = rubiks;
    my_rubiks.right = rotate_face_clockwise(my_rubiks.right);
    //my_rubiks.top = flip_vert(flip_horz(my_rubiks.top));

    let front_values = my_rubiks.front;
    my_rubiks.front[2] = my_rubiks.bottom[2];
    my_rubiks.front[5] = my_rubiks.bottom[5];
    my_rubiks.front[8] = my_rubiks.bottom[8];

    my_rubiks.bottom[2] = my_rubiks.back[6];
    my_rubiks.bottom[5] = my_rubiks.back[3];
    my_rubiks.bottom[8] = my_rubiks.back[0];

    my_rubiks.back[0] = my_rubiks.top[8];
    my_rubiks.back[3] = my_rubiks.top[5];
    my_rubiks.back[6] = my_rubiks.top[2];

    my_rubiks.top[2] = front_values[2];
    my_rubiks.top[5] = front_values[5];
    my_rubiks.top[8] = front_values[8];
    //my_rubiks.top = flip_horz(flip_vert(my_rubiks.top));

    my_rubiks
}

fn f(rubiks: Rubiks) -> Rubiks {
    let mut my_rubiks = Rubiks {
        front: rubiks.left,
        right: rubiks.front,
        back: rubiks.right,
        left: rubiks.back,
        top: flip_vert(transpose(rubiks.top)),
        bottom: transpose(flip_vert(rubiks.bottom)),
    };

    my_rubiks = r(my_rubiks);
    my_rubiks = Rubiks {
        front: my_rubiks.right,
        right: my_rubiks.back,
        back: my_rubiks.left,
        left: my_rubiks.front,
        top: transpose(flip_vert(my_rubiks.top)),
        bottom: flip_vert(transpose(my_rubiks.bottom)),
    };
    my_rubiks
}

fn b(rubiks: Rubiks) -> Rubiks {
    let mut my_rubiks = Rubiks {
        front: rubiks.back,
        right: rubiks.left,
        back: rubiks.front,
        left: rubiks.right,
        top: flip_vert(flip_horz(rubiks.top)),
        bottom: flip_vert(flip_horz(rubiks.bottom)),
    };

    my_rubiks = f(my_rubiks);
    my_rubiks = Rubiks {
        front: my_rubiks.back,
        right: my_rubiks.left,
        back: my_rubiks.front,
        left: my_rubiks.right,
        top: flip_horz(flip_vert(my_rubiks.top)),
        bottom: flip_horz(flip_vert(my_rubiks.bottom)),
    };

    my_rubiks
}

/// Right Clockwise turn
fn r(rubiks: Rubiks) -> Rubiks {
    rotate_right_clockwise(rubiks)
}

/// Left Clockwise turn
fn l(rubiks: Rubiks) -> Rubiks {
    let mut my_rubiks = Rubiks {
        front: rubiks.back,
        right: rubiks.left,
        back: rubiks.front,
        left: rubiks.right,
        top: flip_vert(flip_horz(rubiks.top)),
        bottom: flip_vert(flip_horz(rubiks.bottom)),
    };
    my_rubiks = r(my_rubiks);
    my_rubiks = Rubiks {
        front: my_rubiks.back,
        right: my_rubiks.left,
        back: my_rubiks.front,
        left: my_rubiks.right,
        top: flip_horz(flip_vert(my_rubiks.top)),
        bottom: flip_horz(flip_vert(my_rubiks.bottom)),
    };

    my_rubiks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        assert_eq!(
            [8, 5, 2, 7, 4, 1, 6, 3, 0],
            transpose([0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[test]
    fn test_rotate_face_clockwise() {
        assert_eq!(
            [2, 5, 8, 1, 4, 7, 0, 3, 6],
            rotate_face_clockwise([0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[test]
    fn test_rotate_face_clockwise_two() {
        assert_eq!(
            [11, 14, 17, 10, 13, 16, 9, 12, 15],
            rotate_face_clockwise([9, 10, 11, 12, 13, 14, 15, 16, 17])
        );
    }

    #[test]
    fn test_rotate_face_anticlockwise() {
        assert_eq!(
            [6, 3, 0, 7, 4, 1, 8, 5, 2],
            rotate_face_anticlockwise([0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[test]
    fn test_flip_horz() {
        assert_eq!(
            [2, 1, 0, 5, 4, 3, 8, 7, 6],
            flip_horz([0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }

    #[test]
    fn test_flip_vert() {
        assert_eq!(
            [6, 7, 8, 3, 4, 5, 0, 1, 2],
            flip_vert([0, 1, 2, 3, 4, 5, 6, 7, 8])
        )
    }

    #[test]
    fn test_double_flip() {
        assert_eq!(
            [8, 7, 6, 5, 4, 3, 2, 1, 0],
            flip_vert(flip_horz([0, 1, 2, 3, 4, 5, 6, 7, 8]))
        )
    }

    #[test]
    fn test_rotate_right_clockwise() {
        let rubiks = Rubiks {
            front: [0i32; 9],
            right: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            back: [2i32; 9],
            left: [3i32; 9],
            top: [4i32; 9],
            bottom: [5i32; 9],
        };

        let test_rubiks = Rubiks {
            front: [0, 0, 5, 0, 0, 5, 0, 0, 5],
            right: [2, 5, 8, 1, 4, 7, 0, 3, 6],
            back: [4, 2, 2, 4, 2, 2, 4, 2, 2],
            left: [3i32; 9],
            top: [4, 4, 0, 4, 4, 0, 4, 4, 0],
            bottom: [5, 5, 2, 5, 5, 2, 5, 5, 2],
        };

        assert_eq!(test_rubiks, rotate_right_clockwise(rubiks))
    }

    #[test]
    fn test_rotate_right_clockwise_two() {
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        let test_rubiks = Rubiks {
            front: [0, 1, 47, 3, 4, 50, 6, 7, 53],
            right: [11, 14, 17, 10, 13, 16, 9, 12, 15],
            back: [44, 19, 20, 41, 22, 23, 38, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 2, 39, 40, 5, 42, 43, 8],
            bottom: [45, 46, 24, 48, 49, 21, 51, 52, 18],
        };

        assert_eq!(test_rubiks, rotate_right_clockwise(rubiks))
    }

    #[test]
    fn test_notation_r() {
        let rubiks = Rubiks {
            front: [0i32; 9],
            right: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            back: [2i32; 9],
            left: [3i32; 9],
            top: [4i32; 9],
            bottom: [5i32; 9],
        };

        let test_rubiks = Rubiks {
            front: [0, 0, 5, 0, 0, 5, 0, 0, 5],
            right: [2, 5, 8, 1, 4, 7, 0, 3, 6],
            back: [4, 2, 2, 4, 2, 2, 4, 2, 2],
            left: [3i32; 9],
            top: [4, 4, 0, 4, 4, 0, 4, 4, 0],
            bottom: [5, 5, 2, 5, 5, 2, 5, 5, 2],
        };

        assert_eq!(test_rubiks, r(rubiks))
    }

    #[test]
    fn test_notation_r_two() {
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        let test_rubiks = Rubiks {
            front: [0, 1, 47, 3, 4, 50, 6, 7, 53],
            right: [11, 14, 17, 10, 13, 16, 9, 12, 15],
            back: [44, 19, 20, 41, 22, 23, 38, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 2, 39, 40, 5, 42, 43, 8],
            bottom: [45, 46, 24, 48, 49, 21, 51, 52, 18],
        };

        assert_eq!(test_rubiks, r(rubiks))
    }

    #[test]
    fn test_notation_l() {
        let rubiks = Rubiks {
            front: [0i32; 9],
            right: [1i32; 9],
            back: [2i32; 9],
            left: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            top: [4i32; 9],
            bottom: [5i32; 9],
        };

        let test_rubiks = Rubiks {
            front: [4, 0, 0, 4, 0, 0, 4, 0, 0],
            right: [1i32; 9],
            back: [2, 2, 5, 2, 2, 5, 2, 2, 5],
            left: [2, 5, 8, 1, 4, 7, 0, 3, 6],
            top: [2, 4, 4, 2, 4, 4, 2, 4, 4],
            bottom: [0, 5, 5, 0, 5, 5, 0, 5, 5],
        };

        assert_eq!(test_rubiks, l(rubiks))
    }

    #[test]
    fn test_notation_l_two() {
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        let test_rubiks = Rubiks {
            front: [36, 1, 2, 39, 4, 5, 42, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 51, 21, 22, 48, 24, 25, 45],
            left: [29, 32, 35, 28, 31, 34, 27, 30, 33],
            top: [26, 37, 38, 23, 40, 41, 20, 43, 44],
            bottom: [0, 46, 47, 3, 49, 50, 6, 52, 53],
        };

        assert_eq!(test_rubiks, l(rubiks))
    }

    #[test]
    fn test_notation_f() {
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [1i32; 9],
            back: [2i32; 9],
            left: [3i32; 9],
            top: [4i32; 9],
            bottom: [5i32; 9],
        };

        let test_rubiks = Rubiks {
            front: [2, 5, 8, 1, 4, 7, 0, 3, 6],
            right: [4, 1, 1, 4, 1, 1, 4, 1, 1],
            back: [2i32; 9],
            left: [3, 3, 5, 3, 3, 5, 3, 3, 5],
            top: [3, 3, 3, 4, 4, 4, 4, 4, 4],
            bottom: [5, 5, 5, 5, 5, 5, 1, 1, 1],
        };

        assert_eq!(test_rubiks, f(rubiks))
    }

    #[test]
    fn test_notation_f_two() {
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        let test_rubiks = Rubiks {
            front: [2, 5, 8, 1, 4, 7, 0, 3, 6],
            right: [38, 10, 11, 37, 13, 14, 36, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 53, 30, 31, 52, 33, 34, 51],
            top: [29, 32, 35, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 9, 12, 15],
        };

        assert_eq!(test_rubiks, f(rubiks))
    }

    #[test]
    fn test_notation_b() {
        let rubiks = Rubiks {
            front: [0i32; 9],
            right: [1i32; 9],
            back: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            left: [3i32; 9],
            top: [4i32; 9],
            bottom: [5i32; 9],
        };

        let test_rubiks = Rubiks {
            front: [0i32; 9],
            right: [1, 1, 5, 1, 1, 5, 1, 1, 5],
            back: [2, 5, 8, 1, 4, 7, 0, 3, 6],
            left: [4, 3, 3, 4, 3, 3, 4, 3, 3],
            top: [4, 4, 4, 4, 4, 4, 1, 1, 1],
            bottom: [3, 3, 3, 5, 5, 5, 5, 5, 5],
        };

        assert_eq!(test_rubiks, b(rubiks))
    }

    #[test]
    fn test_notation_b_two() {
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        let test_rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 45, 12, 13, 46, 15, 16, 47],
            back: [20, 23, 26, 19, 22, 25, 18, 21, 24],
            left: [42, 28, 29, 43, 31, 32, 44, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 17, 14, 11],
            bottom: [33, 30, 27, 48, 49, 50, 51, 52, 53],
        };

        assert_eq!(test_rubiks, b(rubiks))
    }

    #[test]
    fn test_notation_d() {
        let rubiks = Rubiks {
            front: [0i32; 9],
            right: [1i32; 9],
            back: [2i32; 9],
            left: [3i32; 9],
            top: [4i32; 9],
            bottom: [0,1,2,3,4,5,6,7,8],
        };

        let test_rubiks = Rubiks{
            front: [3,3,3,0,0,0,0,0,0],
            right: [0,0,0,1,1,1,1,1,1],
            back: [1,1,1,2,2,2,2,2,2],
            left: [2,2,2,3,3,3,3,3,3],
            top: [4i32; 9],
            bottom: [2, 5, 8, 1, 4, 7, 0, 3, 6],
        };

        assert_eq!(test_rubiks, d(rubiks));
    }

    #[test]
    fn test_notation_d_two(){
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        let test_rubiks = Rubiks {
            front: [27,28,29,3,4,5,6,7,8],
            right: [0,1,2,12,13,14,15,16,17],
            back: [9,10,11,21,22,23,24,25,26],
            left: [18,19,20,30,31,32,33,34,35],
            top: [36, 37, 38, 39, 40, 41, 42,43,44],
            bottom: [47,50,53,46,49,52,45,48,51],
        };

        assert_eq!(test_rubiks, d(rubiks))
    }

    #[test]
    fn test_notation_u(){
        let rubiks = Rubiks {
            front: [0i32; 9],
            right: [1i32; 9],
            back: [2i32; 9],
            left: [3i32; 9],
            top: [0,1,2,3,4,5,6,7,8],
            bottom: [5i32;9],
        };

        let test_rubiks = Rubiks{
            front: [1,1,1,0,0,0,0,0,0],
            right: [2,2,2,1,1,1,1,1,1],
            back: [3,3,3,2,2,2,2,2,2],
            left: [0,0,0,3,3,3,3,3,3],
            top: [2, 5, 8, 1, 4, 7, 0, 3, 6],
            bottom: [5i32; 9],
        };

        assert_eq!(test_rubiks, u(rubiks));
    }

    #[test]
    fn test_notation_u_two(){
        let rubiks = Rubiks {
            front: [0, 1, 2, 3, 4, 5, 6, 7, 8],
            right: [9, 10, 11, 12, 13, 14, 15, 16, 17],
            back: [18, 19, 20, 21, 22, 23, 24, 25, 26],
            left: [27, 28, 29, 30, 31, 32, 33, 34, 35],
            top: [36, 37, 38, 39, 40, 41, 42, 43, 44],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        let test_rubiks = Rubiks {
            front: [3,10,11,3,4,5,6,7,8],
            right: [18,19,20,12,13,14,15,16,17],
            back: [27,28,29,21,22,23,24,25,26],
            left: [0,1,2,30,31,32,33,34,35],
            top: [38,41,44,37,40,43,36,39,42],
            bottom: [45, 46, 47, 48, 49, 50, 51, 52, 53],
        };

        assert_eq!(test_rubiks, u(rubiks))
    }
}
