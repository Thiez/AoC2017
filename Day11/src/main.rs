
extern crate load_input;

fn main() {
    let input = load_input::load_input();
    let (mut n, mut e) = (0, 0);
    let mut max_dst = calculate_distance(n, e);
    for dir in input.split(',').map(str::trim) {
        let deltas = match dir {
            "n" => (2, 0),
            "ne" => (1, 1),
            "se" => (-1, 1),
            "s" => (-2, 0),
            "sw" => (-1, -1),
            "nw" => (1, -1),
            _ => panic!("Bad direction: {}", dir)
        };
        n += deltas.0;
        e += deltas.1;
        max_dst = std::cmp::max(max_dst, calculate_distance(n, e));
    }

    println!("Distance at the end: {}", calculate_distance(n, e));
    println!("Maximum distance: {}", max_dst);
}

fn calculate_distance(north: i64, east: i64) -> u64 {
    let north = i64::abs(north) as u64;
    let east = i64::abs(east) as u64;
    (north - std::cmp::min(east, north)) / 2 + east
}