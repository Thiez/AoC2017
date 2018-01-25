extern crate load_input;

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        let (dx, dy) = match rhs {
            Direction::Up => (0, !0),
            Direction::Down => (0, 1),
            Direction::Left => (!0, 0),
            Direction::Right => (1, 0)
        };
        Position {
            x: self.x.wrapping_add(dx),
            y: self.y.wrapping_add(dy)
        }
    }
}

#[derive(Copy, Clone)]
struct Map<'a>(&'a[&'a [u8]]);

impl<'a> Map<'a> {
    fn get(&self, p: Position) -> Option<u8> {
        self.0.get(p.y).and_then(|&row|row.get(p.x)).cloned()
    }
}

fn main() {
    let input = load_input::load_input();
    let mut position = {
        let x = input.chars().take_while(|&c|c != '\n').position(|c|c == '|').expect("A starting position.");
        let y = 0;
        Position { x, y }
    };
    let lines = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let map = Map(&lines);
    let mut seen = Vec::new();
    let mut direction = Direction::Down;
    let mut steps = 0;
    while let Some(c) = map.get(position) {
        match c {
            b'|' | b'-' => (),
            b'+' => {
                let (opt1, opt2) = match direction {
                    Direction::Up | Direction::Down => {
                        (Direction::Left, Direction::Right)
                    },
                    Direction::Left | Direction::Right => {
                        (Direction::Up, Direction::Down)
                    }
                };
                direction = match (opt1, map.get(position + opt1)) {
                    (Direction::Left, Some(c)) if c != b' ' && c != b'|' => opt1,
                    (Direction::Left, _) => opt2,
                    (Direction::Up, Some(c)) if c != b' ' && c != b'-' => opt1,
                    _ => opt2
                };
            },
            b' ' => break,
            c => seen.push(c)
        };
        steps += 1;
        position = position + direction;
    }

    println!("Steps: {}", steps);
    println!("Seen: {:?}", seen.into_iter().map(|b|b as char).collect::<String>());
}
