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

fn main() {
    let player_1_state = smart_random_initializer(8,8,2);
    display_state(&player_1_state);    
    let player_2_state = smart_random_initializer(8,8,2);
    display_state(&player_2_state);    
}
