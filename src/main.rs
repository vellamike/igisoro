// Early attempt at an igisoro implementation


use std::rand;
use std::rand::Rng;

fn rand_int() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen::<u8>()
}

fn random_initalizer(n:u8) -> [[u8;8];2] {
    let mut example_state = [[n;8]; 2];
    for i in example_state.iter_mut() {
        for j in i.iter_mut(){
            *j = rand_int();
        }
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
    let mut player_1_state = [[0;8]; 2];
    display_state(&player_1_state);

    let j = uniform_initalizer(2);
    display_state(&j);

    let p = random_initalizer(2);
    display_state(&p);

}
