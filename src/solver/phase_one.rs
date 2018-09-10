use physical;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Moves{
    F1,
    B1,
    U1,
    D1,
    L1,
    R1,
    NONE
}

static PHASE_ONE_MOVE_LIST: [Moves; 6] = [Moves::F1,Moves::B1, Moves::U1, Moves::D1, Moves::L1, Moves::R1,];

pub fn search(rubiks: &mut physical::Cube, depth: i32){
    phase_one_search(rubiks, depth, Moves::NONE, &Vec::new());
}

fn phase_one_search(rubiks: &mut physical::Cube, depth: i32, last_movement: Moves, move_list: &Vec<Moves>) {
    
    let mut my_depth = depth;
    println!("Depth: {}", my_depth);
    while(!phase_one_subgoal(rubiks) && my_depth >0){
        println!("One more");
        for movement in PHASE_ONE_MOVE_LIST.iter() {
            let mut current_moves = move_list.clone();
            let mut c = rubiks.clone();
            println!("{:?}", *movement);
            do_move(&mut c, *movement);
            current_moves.push(*movement);
            println!("Corner: {}, \nEdge: {}, \nUDSlice: {}", c.corner_orientation, c.edge_orientation, c.ud_slice);
            if phase_one_subgoal(&mut c) {
                *rubiks = c;
                my_depth = 0;
                break;
            } else{
                println!("Last move: {:?}", *movement);
                phase_one_search(&mut c, my_depth - 1, Moves::NONE, &current_moves);
                my_depth = my_depth - 1;
            };
            println!("MoveList: {:?}", current_moves);
        }
    }
    println!("Found");
    
    //move_list
}

fn do_move(rubiks: &mut physical::Cube, movement: Moves){
    match movement{
        Moves::F1 => rubiks.f(),
        Moves::B1 => rubiks.b(),
        Moves::U1 => rubiks.u(),
        Moves::D1 => rubiks.d(),
        Moves::L1 => rubiks.l(),
        Moves::R1 => rubiks.r(),
        Moves::NONE => {},
    }
}

fn phase_one_subgoal(rubiks:&mut physical::Cube)->bool{
    (rubiks.corner_orientation == 0) &&
    (rubiks.edge_orientation == 0) &&
    (rubiks.ud_slice == 0)
}