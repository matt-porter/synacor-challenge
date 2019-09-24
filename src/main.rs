
use std::convert::TryInto;

#[derive(Default)]
struct System {
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    r8: u16,
    memory: Vec<u16>,
    stack: Vec<u16>,
}

impl System {
    pub fn init() -> System {
        let mut state = System::default();
        state.memory = vec![0; 32768];
        state
    }

    pub fn execute(&mut self) -> () {
        println!("Running...");
        let mut idx: usize = 0;
        loop {
            println!("DEBUG: idx: {}", idx);
            let instruction = self.memory[idx];
            println!("DEBUG: instruction: {}", instruction);
            match instruction {
                0 => op_0_halt(self),
                19 => idx = op_19_out(self, idx),
                _ => unimplemented!()
            }
        }
        for instruction in &mut self.memory {

        }

    }

    pub fn load(&mut self, input: &[u16]) -> () {
        for (i, value) in input.iter().enumerate() {
            self.memory[i] = value.clone();
        }
    }
}

fn op_0_halt(system: &mut System) -> (){
    std::process::exit(0);
}

fn op_19_out(system: &mut System, idx: usize) -> usize {
    // consumes one
    let mut idx = idx + 1;
    let value: u8 = system.memory[idx].try_into().unwrap();
    let valuec: char = value.into();
    print!("{}", valuec);
    idx = idx + 1;
    idx
}

enum Value {
    Literal(u16),
    Register(u16),
    Invalid,
}

fn main() {
    let mut system = System::init();
    let input: [u16; 6] = [9,32768,32769,4,19,32768];
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