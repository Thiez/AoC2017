extern crate load_input;

use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Reg(usize);

#[derive(Copy, Clone, Debug)]
enum RegOrInt {
    Reg(Reg),
    Int(i64)
}

#[derive(Copy, Clone, Debug)]
enum Command {
    Snd(RegOrInt),
    Set(Reg, RegOrInt),
    Add(Reg, RegOrInt),
    Mul(Reg, RegOrInt),
    Mod(Reg, RegOrInt),
    Rcv(Reg),
    Jgz(RegOrInt, RegOrInt)
}

#[derive(Copy, Clone)]
struct State<'a> {
    instructions: &'a [Command],
    ip: i64,
    registers: [i64; 26],
    last_played: i64
}

struct ProgramState {
    ip: i64,
    registers: [i64; 26],
    incoming: VecDeque<i64>,
    received: Vec<i64>,
    sends: u64
}

struct MultiState<'a> {
    instructions: &'a [Command],
    p0: ProgramState,
    p1: ProgramState
}

impl FromStr for Reg {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y"," z"]
            .iter()
            .position(|&e|e==s)
            .map(Reg)
            .ok_or(())
    }
}

impl FromStr for RegOrInt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        s.parse::<Reg>().map(RegOrInt::Reg).or(s.parse().map(RegOrInt::Int).map_err(|_|()))
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        fn combine<T, U>(x: Option<&str>, y: Option<&str>, f: fn(T, U) -> Command) -> Result<Command, ()> where
            T: FromStr<Err=()>,
            U: FromStr<Err=()>,
        {
            x.ok_or(()).and_then(FromStr::from_str).and_then(|x|y.ok_or(()).and_then(FromStr::from_str).map(|y|f(x,y)))
        }
        let mut chunks = s.trim().split(' ');
        let command = chunks.next().unwrap_or("empty_line");
        let reg = chunks.next();
        let roi = chunks.next();
        match command {
            "snd" => reg.ok_or(()).and_then(FromStr::from_str).map(Command::Snd),
            "set" => combine(reg, roi, Command::Set),
            "add" => combine(reg, roi, Command::Add),
            "mul" => combine(reg, roi, Command::Mul),
            "mod" => combine(reg, roi, Command::Mod),
            "rcv" => reg.ok_or(()).and_then(FromStr::from_str).map(Command::Rcv),
            "jgz" => combine(reg, roi, Command::Jgz),
            _ => Err(()),
        }
    }
}

impl<'a> State<'a> {
    fn execute_one(mut self) -> Option<Self> {
        fn get_value(state: &State, roi: RegOrInt) -> i64 {
            match roi {
                RegOrInt::Reg(y) => state.registers[y.0],
                RegOrInt::Int(y) => y
            }
        }
        if self.ip < 0 || self.instructions.len() <= (self.ip as usize) {
            None
        } else {
            let instr = self.instructions[self.ip as usize];
            self.ip += 1;
            match instr {
                Command::Snd(x) => self.last_played = get_value(&self, x),
                Command::Set(x, y) => self.registers[x.0] = get_value(&self, y),
                Command::Add(x, y) => self.registers[x.0] += get_value(&self, y),
                Command::Mul(x, y) => self.registers[x.0] *= get_value(&self, y),
                Command::Mod(x, y) => self.registers[x.0] %= get_value(&self, y),
                Command::Rcv(x) if self.registers[x.0] != 0 => {
                    println!("Recovered {}", self.last_played);
                    //self.registers[x.0] = self.last_played
                    return None;
                },
                Command::Jgz(x, y) if 0 < get_value(&self, x) => self.ip += get_value(&self, y) - 1,
                _ => (),
            }
            Some(self)
        }
    }
}

impl<'a> MultiState<'a> {
    fn execute_one(mut self) -> Result<Self, Self> {
        fn get_value(state: &ProgramState, roi: RegOrInt) -> i64 {
            match roi {
                RegOrInt::Reg(y) => state.registers[y.0],
                RegOrInt::Int(y) => y
            }
        }
        fn execute_single(instructions: &[Command], state: &mut ProgramState, channel_out: &mut VecDeque<i64>) -> bool {
            if state.ip < 0 || instructions.len() <= (state.ip as usize) {
                return false;
            }
            let instr = instructions[state.ip as usize];
            match instr {
                Command::Snd(x) => {
                    channel_out.push_back(get_value(&state ,x));
                    state.sends += 1;
                },
                Command::Set(x, y) => state.registers[x.0] = get_value(&state, y),
                Command::Add(x, y) => state.registers[x.0] += get_value(&state, y),
                Command::Mul(x, y) => state.registers[x.0] *= get_value(&state, y),
                Command::Mod(x, y) => state.registers[x.0] %= get_value(&state, y),
                Command::Rcv(x) => {
                    if let Some(rcv) = state.incoming.pop_front() {
                        state.received.push(rcv);
                        state.registers[x.0] = rcv;
                    } else {
                        return false;
                    }
                },
                Command::Jgz(x, y) if 0 < get_value(&state, x) => state.ip += get_value(&state, y) - 1,
                _ => (),
            }
            state.ip += 1;
            true
        }

        let progress0 = {
            let MultiState {
                instructions,
                ref mut p0,
                p1: ProgramState {
                    ref mut incoming, ..
                }} = self;
            execute_single(instructions, p0, incoming)
        };
        let progress1 = {
            let MultiState {
                instructions,
                p0: ProgramState {
                    ref mut incoming, ..
                },
                ref mut p1 } = self;
            execute_single(instructions, p1, incoming)
        };

        match (progress0, progress1) {
            (false, false) => {
                println!("Deadlock!");
                return Err(self)
            },
            (false, true) => println!("0 sleeping"),
            (true, false) => println!("1 sleeping"),
            _ => (),
        }
        Ok(self)
    }
}

fn main() {
    let input = load_input::load_input();
    let commands = input.lines().map(|s|match s.parse() { Ok(v) => v, _ => panic!("{}", s )}).collect::<Vec<_>>();
    println!("Got {} commands", commands.len());
    let mut state = State {
        instructions: &commands,
        ip: 0,
        registers: [0; 26],
        last_played: 0
    };
    while let Some(s) = state.execute_one() { state = s; }


    let mut state = MultiState {
        instructions: &commands,
        p0: ProgramState {
            ip: 0,
            registers: [0i64; 26],
            incoming: Default::default(),
            received: Default::default(),
            sends: 0
        },
        p1: ProgramState {
            ip: 0,
            registers: [0i64; 26],
            incoming: Default::default(),
            received: Default::default(),
            sends: 0
        }
    };
    let p = "p".parse::<Reg>().unwrap();
    state.p0.registers[p.0] = 0;
    state.p1.registers[p.0] = 1;
    loop {
        match state.execute_one() {
            Ok(s) => state = s,
            Err(s) => {
                println!("p1 sends: {}", s.p1.sends);
                println!("p0: {:?}", s.p0.registers);
                println!("p1: {:?}", s.p1.registers);
                break;
            }
        }
    }
}
