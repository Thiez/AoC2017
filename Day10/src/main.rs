extern crate knot_hash;
extern crate load_input;

fn main() {
    let input = load_input::load_input();
    let first_input = input
        .split(',')
        .map(str::trim)
        .flat_map(str::parse);
    
    let ks = knot_hash::KnotState::new()
        .perform_round(first_input);
    let first_hash = ks.list[0] as u16 * ks.list[1] as u16;
    
    let second_hash = knot_hash::knot_hash(input.as_bytes());

    println!("First answer: {}", first_hash);
    println!("Second answer: {}", &*second_hash);
}
