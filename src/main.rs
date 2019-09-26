
use std::convert::TryInto;
use std::io::{self, Read};


#[derive(Default)]
struct System {
    registers: Vec<u16>,
    memory: Vec<u16>,
    stack: Vec<u16>,
}

impl System {
    pub fn init() -> System {
        let mut state = System::default();
        state.memory = vec![0; 32768];
        state.registers = vec![0; 8];
        state
    }

    pub fn execute(&mut self) -> () {
        // println!("Running...");
        let mut idx: usize = 0;
        loop {
            // println!("DEBUG: idx: {}", idx);
            let instruction = self.memory[idx];
            // println!("DEBUG: instruction: {}", instruction);
            match instruction {
                0 => op_0_halt(self),
                4 => idx = op_4_eq(self, idx),
                1 => idx = op_1_set(self, idx),
                2 => idx = op_2_push(self, idx),
                3 => idx = op_3_pop(self, idx),
                5 => idx = op_5_gt(self, idx),
                6 => idx = op_6_jmp(self, idx),
                7 => idx = op_7_jt(self, idx),
                8 => idx = op_8_jf(self, idx),
                9 => idx = op_9_add(self, idx),
                10 => idx = op_10_mul(self, idx),
                11 => idx = op_11_mod(self, idx),
                12 => idx = op_12_and(self, idx),
                13 => idx = op_13_or(self, idx),
                14 => idx = op_14_not(self, idx),
                15 => idx = op_15_rmem(self, idx),
                16 => idx = op_16_wmem(self, idx),
                17 => idx = op_17_call(self, idx),
                19 => idx = op_19_out(self, idx),
                21 => idx = op_21_noop(self, idx),
                x => {
                    println!("OP {} unimplemented", x);
                    unimplemented!();
                    }
            }
        }
    }

    pub fn load(&mut self, input: &[u16]) -> () {
        for (i, value) in input.iter().enumerate() {
            self.memory[i] = value.clone();
        }
    }

    pub fn get(&mut self, idx: usize) -> u16 {
        let value = self.memory[idx];
        match value {
            0..=32767 => {
                // println!("Got literal value {}", value);
                value
            }
            32768..=32775 => {
                // println!("Got value {} from register {}", self.registers[(value as usize)-32768], (value as usize)-32768);
                self.registers[(value-32768) as usize]
            }
            _ => unimplemented!(),
        }
    }

    pub fn get_register(&mut self, idx: usize) -> usize {
        let value = self.memory[idx];
        match value {
            0..=32767 => {
                // println!("Got literal value {}", value);
                unimplemented!()
            }
            32768..=32775 => {
                // println!("Got value {} from register {}", self.registers[(value as usize)-32768], (value as usize)-32768);
                (value-32768) as usize
            }
            _ => unimplemented!(),
        }
    }
}

fn op_0_halt(system: &mut System) -> (){
    std::process::exit(0);
}

///set: 1 a b
///set register <a> to the value of <b>
fn op_1_set(system: &mut System, idx: usize) -> usize {
    let mut idx = idx + 1;
    // We want the /register/ not the value at the register.
    let register = system.get_register(idx);
    idx = idx + 1;
    let value = system.get(idx);
    println!("Setting r{} to {}", register, value);
    system.registers[register] = value;
    println!("Registers {:?}", system.registers);
    idx + 1
}

fn op_2_push(system: &mut System, idx: usize) -> usize {
    let idx = idx + 1;
    let value = system.get(idx);
    system.stack.push(value);
    idx + 1
}

fn op_3_pop(system: &mut System, idx: usize) -> usize {
    let idx = idx + 1;
    let register = system.get_register(idx);
    system.registers[register] = system.stack.pop().unwrap();
    idx + 1
}

fn op_4_eq(system: &mut System, idx: usize) -> usize {
    // eq: 4 a b c
    // set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
    // consumes 3
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    idx = idx + 1;
    let rhs: u16 = system.get(idx).try_into().unwrap();
    system.registers[register] = if lhs == rhs {
        1
    } else {
        0
    };
    idx + 1
}

fn op_5_gt(system: &mut System, idx: usize) -> usize {
    // eq: 5 a b c
    // set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
    // consumes 3
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    idx = idx + 1;
    let rhs: u16 = system.get(idx).try_into().unwrap();
    system.registers[register] = if lhs > rhs {
        1
    } else {
        0
    };
    idx + 1
}

fn op_6_jmp(system: &mut System, idx: usize) -> usize {
    let mut idx = idx + 1;
    let addr = system.get(idx);
    addr as usize
}

/// jt: 7 a b
/// if <a> is nonzero, jump to <b>
fn op_7_jt(system: &mut System, idx: usize) -> usize {
    let mut idx = idx + 1;
    let check = system.get(idx);
    idx = idx + 1;
    let addr = system.get(idx);
    if check > 0 {
        // println!("DEBUG: {} nonzero, jt addr {}", check, addr);
        addr as usize
    } else {
        // println!("DEBUG: {} jt zero, continue to {}", check, idx+1);
        idx + 1
    }
}

/// jf: 8 a b
/// if <a> is zero, jump to <b>
fn op_8_jf(system: &mut System, idx: usize) -> usize {
    let mut idx = idx + 1;
    let check = system.get(idx);
    idx = idx + 1;
    let addr = system.get(idx);
    if check == 0 {
        // println!("DEBUG: {} zero, jf addr {}", check, addr);
        addr as usize
    } else {
        // println!("DEBUG: {} jf nonzero, continue to {}", check, idx+1);
        idx + 1
    }
}

fn op_9_add(system: &mut System, idx: usize) -> usize {
    // add: 9 a b c
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    // consumes 3
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    idx = idx + 1;
    let rhs: u16 = system.get(idx).try_into().unwrap();
    // println!("{} = {} + {}", register, lhs, rhs );
    system.registers[register] = (lhs + rhs) % 32768;
    idx + 1
}

fn op_10_mul(system: &mut System, idx: usize) -> usize {
    // add: 9 a b c
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    // consumes 3
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    idx = idx + 1;
    let rhs: u16 = system.get(idx).try_into().unwrap();
    // println!("{} = {} + {}", register, lhs, rhs );
    system.registers[register] = ((lhs as u32 * rhs as u32) % 32768) as u16;
    idx + 1
}

fn op_11_mod(system: &mut System, idx: usize) -> usize {
    // add: 9 a b c
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    // consumes 3
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    idx = idx + 1;
    let rhs: u16 = system.get(idx).try_into().unwrap();
    // println!("{} = {} + {}", register, lhs, rhs );
    let div = lhs/rhs;

    system.registers[register] = lhs - (div * rhs) ;
    idx + 1
}

fn op_12_and(system: &mut System, idx: usize) -> usize {
    // and: 12 a b c
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    // consumes 3
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    idx = idx + 1;
    let rhs: u16 = system.get(idx).try_into().unwrap();
    // println!("{} = {} + {}", register, lhs, rhs );
    system.registers[register] = lhs & rhs;
    idx + 1
}

fn op_13_or(system: &mut System, idx: usize) -> usize {
    // and: 13 a b c
    // assign into <a> the sum of <b> and <c> (modulo 32768)
    // consumes 3
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    idx = idx + 1;
    let rhs: u16 = system.get(idx).try_into().unwrap();
    // println!("{} = {} + {}", register, lhs, rhs );
    system.registers[register] = lhs | rhs;
    idx + 1
}

fn op_14_not(system: &mut System, idx: usize) -> usize {
    // consumes 2
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    system.registers[register] = !lhs & 0b111111111111111;
    idx + 1
}

fn op_15_rmem(system: &mut System, idx: usize) -> usize {
    // consumes 2
    println!("Registers {:?}", system.registers);
    let mut idx = idx + 1;
    let register = system.get_register(idx);
    idx = idx + 1;
    let addr = system.get(idx) as usize;
    let lhs = system.get(addr);
    println!("DEBUG: rmem: set r{} to {}", register, lhs);
    system.registers[register] = lhs;
    idx + 1
}

fn op_16_wmem(system: &mut System, idx: usize) -> usize {
    // consumes 2
    let mut idx = idx + 1;
    let addr = system.get(idx);
    idx = idx + 1;
    let lhs: u16 = system.get(idx).try_into().unwrap();
    println!("DEBUG: wmem: set a{} to {}", addr, lhs);
    system.memory[addr as usize] = lhs;
    idx + 1
}

fn op_17_call(system: &mut System, idx: usize) -> usize {
    system.stack.push((idx as u16) + 2);
    let mut idx = idx + 1;
    system.get(idx) as usize
}

fn op_19_out(system: &mut System, idx: usize) -> usize {
    // consumes one
    let mut idx = idx + 1;
    let raw = system.get(idx);
    // println!("DEBUG: {}", raw);
    let value: u8 = raw.try_into().unwrap();
    // println!("DEBUG: {}", value);
    let valuec: char = value.into();
    print!("{}", valuec);
    idx = idx + 1;
    idx
}

fn op_20_in(system: &mut System, idx: usize) -> usize {
    // consumes one
    let mut idx = idx + 1;
    let register = system.get_register(idx);

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let c = buffer.chars().next().unwrap();
    let ascii = c as u32;
    system.registers[register] = ascii.try_into().unwrap();
    idx = idx + 1;
    idx
}

fn op_21_noop(system: &mut System, idx: usize) -> usize {
    idx + 1
}

enum Value {
    Literal(u16),
    Register(u16),
    Invalid,
}

use std::io::prelude::*;
use std::fs::File;
fn read_bin() -> io::Result<Vec<u16>> {
    let mut f = File::open("materials/challenge.bin")?;
    let mut buffer: Vec<u8> = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;
    let mut bin: Vec<u16> = Vec::new();
    let mut iter = buffer.chunks(2);
    for pair in iter {
        match pair {
            [v1, v2] => {
                // println!("{:b} {:b}", v1, v2);
                bin.push((*v2 as u16).checked_shl(8).unwrap() + (*v1 as u16));
            }
            _ => break
        }

    }
    Ok(bin)
}

fn main() {
    let mut system = System::init();
    let v1 = 255u8;
    let v2 = 255u8;
    let v3 = (v1 as u16) << 8;
    let v4 = v2 as u16;
    let v5 = v3 + v4;
    // println!("{:b} {:b} {:b} {:b} {}", v1, v2, v3, v4, v5);
    let input = read_bin().unwrap();
    system.load(&input);
    system.execute()
}

#[test]
fn test_one(){
    let mut system = System::init();
    let input: [u16; 6] = [9,32768,32769,4,19,32768];
    system.load(&input);
    system.execute()
}
