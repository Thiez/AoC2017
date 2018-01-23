extern crate load_input;

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
    Snd(Reg),
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
                Command::Snd(x) => self.last_played = self.registers[x.0],
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
}
