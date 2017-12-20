extern crate load_input;

const PROGRAM_INIT : [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

fn get_number<It: Iterator<Item=char>>(it: &mut std::iter::Peekable<It>) -> u8 {
    let mut result = 0;
    loop {
        let add = match it.peek() {
            Some(&'0') => 0,
            Some(&'1') => 1,
            Some(&'2') => 2,
            Some(&'3') => 3,
            Some(&'4') => 4,
            Some(&'5') => 5,
            Some(&'6') => 6,
            Some(&'7') => 7,
            Some(&'8') => 8,
            Some(&'9') => 9,
            _ => break
        };
        it.next();
        result *= 10;
        result += add;
        //println!("Result = {}", result);
    }

    result
}

#[derive(Copy, Clone)]
struct StepCalculator {
    shuffles: [u8; 16],
    renames: [u8; 16]
}

fn calculate_after(sc: StepCalculator, steps: u64) -> [u8; 16] {
    let mut shuffles = PROGRAM_INIT;
    let mut renames = PROGRAM_INIT;

    for _ in 0..steps {
        for i in 0..16 {
            shuffles[i] = sc.shuffles[shuffles[i] as usize];
            renames[i] = sc.renames[renames[i] as usize];
        }
    }

    let mut result = shuffles;
    for i in 0..16 {
        result[i] = renames[result[i] as usize];
    }

    result
}

#[derive(Copy, Clone)]
enum Transformation {
    Shift(u8),
    Xchge(u8, u8),
    Rename(u8, u8)
}

fn collect_transformations(transformations: &[Transformation]) -> StepCalculator {
    let mut renames = PROGRAM_INIT;
    let mut shuffles = PROGRAM_INIT;
    for transformation in transformations {
        match *transformation {
            t@Transformation::Rename(_, _) => renames = process_step(renames, t),
            t => shuffles = process_step(shuffles, t),
        }
    }
    StepCalculator { shuffles, renames }
}

fn process_step(mut programs: [u8; 16], transformation: Transformation) -> [u8; 16] {
    match transformation {
        Transformation::Shift(s) => {
            let copy = programs;
            for n in 0..16 {
                programs[(n + (s as usize)) % 16] = copy[n];
            }
        },
        Transformation::Xchge(fst, snd) => {
            programs.swap(fst as usize, snd as usize);
        },
        Transformation::Rename(fst, snd) => {
            for pos in &mut programs[..] {
                if *pos == fst {
                    *pos = snd;
                } else if *pos == snd {
                    *pos = fst;
                }
            }
        }
    }
    programs
}

fn parse(input: &str) -> Vec<Transformation> {
    let mut result = Vec::new();
    let mut iter = input.chars().peekable();
    while let Some(c) = iter.next() {
        let t = match c {
            's' => {
                let number = get_number(&mut iter);
                Transformation::Shift(number)
            }
            'x' => {
                let fst = get_number(&mut iter);
                let slash = iter.next().expect("a slash");
                assert_eq!('/', slash);
                let snd = get_number(&mut iter);
                Transformation::Xchge(fst, snd)
            },
            'p' => {
                let fst = (iter.next().expect("a program") as u8) - ('a' as u8);
                let slash = iter.next().expect("a slash");
                assert_eq!('/', slash);
                let snd = (iter.next().expect("a program") as u8) - ('a' as u8);
                Transformation::Rename(fst, snd)
            },
            ',' => { continue },
            _ => panic!("Unexpected input: {}", c)
        };
        result.push(t);
    }
    result
}

fn main() {
    let input = load_input::load_input();
    let transformations = parse(&input);
    let sc = collect_transformations(&transformations);

    let programs = calculate_after(sc, 1);
    print!("After 1 step: ");
    for &program in &programs[..] { print!("{}", (program + ('a' as u8)) as char); }
    println!();

    let programs = calculate_after(sc, 1_000_000_000);
    print!("After 1_000_000_000 steps: ");
    for &program in &programs[..] { print!("{}", (program + ('a' as u8)) as char); }
    println!();
}
