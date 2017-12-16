extern crate knot_hash;
extern crate load_input;

fn main() {
    let input = load_input::load_input();

    let mut bits = Vec::new();
    for n in 0..128 {
        let hash = knot_hash::knot_hash(format!("{}-{}", input, n).as_bytes());
        for c in hash.chars() {
            let num = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' => 10,
                'b' => 11,
                'c' => 12,
                'd' => 13,
                'e' => 14,
                'f' => 15,
                _ => panic!("bad input")
            };
            for i in 0..4 {
                bits.push(if num & (8 >> i) != 0 { -1 } else { -2 })
            }
        }
    }

    let used = bits.iter().filter(|&&n| n != -2).count();
    println!("Num blocks: {}", used);

    let mut to_visit = Vec::new();
    for i in 0..bits.len() {
        if bits[i] == -1 {
            to_visit.push(i);
        }


        while let Some(current) = to_visit.pop() {
            bits[current] = i as i32;
            let mut neighbour_buffer = [0; 4];
            for &mut neighbour in neighbours(current, &mut neighbour_buffer) {
                if bits[neighbour] == -1 {
                    to_visit.push(neighbour);
                }
            }
        }
    }

    let groups = bits.iter().filter(|&&n|n >= 0).collect::<::std::collections::HashSet<_>>();
    println!("Num groups: {}", groups.len());
}

fn neighbours<'a>(pos: usize, buffer: &'a mut [usize; 4]) -> &'a mut [usize] {
    let x = pos % 128;
    let y = pos / 128;
    let mut i = 0;
    if 0 < x { buffer[i] = pos - 1; i += 1; }
    if x < 127 { buffer[i] = pos + 1; i += 1; }
    if 0 < y { buffer[i] = pos - 128; i += 1; }
    if y < 127 { buffer[i] = pos + 128; i += 1; }
    &mut buffer[0..i]
}
