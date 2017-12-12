use std::collections::{HashMap, HashSet, VecDeque};

extern crate load_input;

fn main() {
    let input = load_input::load_input();
    let mut edges = HashMap::new();
    for line in input.trim().split('\n').map(str::trim) {
        let mut iter = line.split("<->").map(str::trim);
        let from = iter.next().expect("vertex");
        let outgoing  = iter.next().expect("vertices").split(',').map(str::trim);
        for to in outgoing {
            edges.entry(from).or_insert_with(HashSet::new).insert(to);
            edges.entry(to).or_insert_with(HashSet::new).insert(from);
        }
    }
    
    let first_group = determine_group(&edges, "0");
    println!("Size of group \"0\": {}", first_group.len());

    let mut groups = 0;
    while let Some((&root,_)) = {edges.iter().next()} {
        groups += 1;
        let group = determine_group(&edges, root);
        for group_member in group {
            edges.remove(group_member);
        }
    }

    println!("There were {} groups in total.", groups);
}

fn determine_group<T: Copy + Eq + std::hash::Hash>(edges: &HashMap<T, HashSet<T>>, start:T) -> HashSet<T> {
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);
    while let Some(current) = to_visit.pop_front() {
        if seen.insert(current) {
            edges.get(&current).map(|out|to_visit.extend(out));
        }
    }
    seen
}
