extern crate load_input;

fn main() {
    let input = load_input::load_input();
    let mut depths = Vec::new();
    for line in input.split('\n').map(str::trim) {
        let mut data = line.split(':').map(str::trim).flat_map(<usize as ::std::str::FromStr>::from_str);
        let depth = data.next().expect("the layer");
        let range = data.next().expect("the range");
        while depths.len() < depth {
            depths.push(0);
        }

        depths.push(range);
    }

    println!("Got {} depths", depths.len());
    println!("Severity of trip at t=0: {}", calculate_severity(&depths, 0).expect("a severity"));

    let smallest_delay = (0..)
        .flat_map(|delay|if calculate_severity(&depths, delay) == None { Some(delay) } else { None })
        .next().expect("A winning strategy");
    println!("The smallest delay guaranteeing sneaky is {:?}", smallest_delay);
}

fn calculate_severity<'a, Depths: IntoIterator<Item=&'a usize>>(depths: Depths, offset: usize) -> Option<usize> {
    depths
        .into_iter()
        .cloned()
        .enumerate()
        .flat_map(|(n, range)|if range > 1 && (n + offset) % (range + range - 2) == 0 { Some(n * range) } else { None })
        .fold(None, |s,a|s.map(|sum|sum + a).or(Some(a)))
}
