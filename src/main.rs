// Early attempt at an igisoro implementation
use std::rand;
use std::rand::Rng;

fn rand_int() -> u8 {
      rand::thread_rng().gen_range(1u8, 10)
}

fn random_initializer(n:u8) -> [[u8;8];2] {
    let mut example_state = [[n;8]; 2];
    for i in example_state.iter_mut() {
        for j in i.iter_mut(){
            *j = rand_int();
        }
    }
    example_state
}

fn smart_random_initializer(mut num_seeds:u8, x:uint, y:uint) -> [[u8;8];2] {

    let mut example_state = [[0;8]; 2 ]; //haven't figured out how to assign dimensions dynamically to an array - I suspect its not possible, for now work with x = 8, y = 2
    
    for i in range(0, num_seeds){
        let x = rand::thread_rng().gen_range(0, x);
        let y = rand::thread_rng().gen_range(0, y);
        example_state[y][x] += 1;
    }
    example_state
}



fn uniform_initalizer(n:u8) -> [[u8;8];2] {
    let mut example_state = [[n;8]; 2];
    example_state
}

fn display_state(a:&[[u8;8];2]){
    //Function to display state of a particular player
    let mut s = "".to_string();

    for j in a.iter(){
        for i in j.iter() {
            s = s + format!(" {} ", *i).as_slice();
        }
        s = s + "\n";
    }
    println!("{}", s);
}

fn play_move(player_state:&mut [[u8;8];2], clockwise:bool, mut current_row:uint, mut current_column:uint, num_seeds:int){
    
    //For this to work some tests are needed.

    // Assumptions:
    // 1. Movement is counter clockwise
    // 2. There are only two rows, but n columns
    
    // Proposed algorithm outline:
    // 1. Keep Track of current position (row, column) & moves left (seeds remaining)
    // 2. Based on position, pick next move: right, left, change_row. This can be some quite
    //    simple heuristics. E.g IF {row == TOP and column == 0} THEN move = change_row or IF {row == BOTTOM and column != max_column} THEN move = right.
    //    Could use an enum for this.
    // 3. Implement move, update new position, repeat
    
    // Variables assigned in this way:
    player_state[0][0] += 1;

    //let's try a traversal, moving from x to y in 3 counter-clockwise steps.
    // WARNING: The following implementation is broken, check out the proposed algorithm outline above
    let bool moving_right;
    let max_column = 7; // fix for now
    let mut num_seeds = num_seeds; // fix for now
    let mut subtractor = 0;
    for seed in range(0, num_seeds) {
        if current_column >= max_column {current_row = 1;subtractor += 1}
        println!("{}", player_state[current_row][current_column - subtractor]);
        current_column += 1;
    };
}

fn main() {
    let player_1_state = smart_random_initializer(8,8,2);
    display_state(&player_1_state);    
    let player_2_state = smart_random_initializer(8,8,2);
    display_state(&player_2_state);
    let mut player_3_state = smart_random_initializer(8,8,2);
    display_state(&player_3_state);
    play_move(&mut player_3_state, true, 0, 0, 3);
    display_state(&player_3_state);
}
