use crate::cpu::clock::Clock;
use crate::cpu::instructions;
use crate::cpu::instructions::ExecutionType;
use crate::cpu::interrupts::handle_interrupts;
use crate::cpu::registers::Registers;
use crate::memory::mmu::{Mmu, Opcode};

pub struct Cpu {
    pub registers: Registers,
    clock: Clock,
    pub interrupt_master_enabled: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        let registers = Registers::new();
        let clock = Clock::new();

        Cpu {
            registers,
            clock,
            interrupt_master_enabled: false,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> u8 {
        let op_code = mmu.read_opcode(self.registers.pc);

        let instruction = match instructions::get_instruction_by_op_code(&op_code) {
            Some(instruction) => instruction,
            None => {
                match op_code {
                    Opcode::CB(value) => eprintln!(
                        "Unimplemented CB Opcode! 0x{:X} PC: 0x{:X}",
                        value, self.registers.pc
                    ),
                    Opcode::Regular(value) => eprintln!(
                        "Unimplemented Opcode! 0x{:X} PC: 0x{:X}",
                        value, self.registers.pc
                    ),
                };

                std::process::exit(1);
            }
        };

        /*  match op_code {
            Opcode::Regular(value) => {
                println!("PC: 0x{:X} 0x{:X}: {}",self.registers.pc, value, instruction.description);
            },
            Opcode::CB(value) => {
                println!("PC: 0x{:X} CB 0x{:X}: {}",self.registers.pc, value, instruction.description);
            }
        }*/

        /*if self.registers.pc == 0xC007 {
            println!("0x{:X}", self.registers.a);
        }*/

        let result = (instruction.handler)(self, mmu, &op_code);

        //Use the correct value if action of conditional instruction is taken or not
        let mut clock_cycles = match result {
            ExecutionType::ActionTaken => {
                self.registers.pc += instruction.length;
                instruction.clock_cycles_condition.unwrap()
            }
            ExecutionType::Jumped => instruction.clock_cycles,
            ExecutionType::JumpedActionTaken => instruction.clock_cycles_condition.unwrap(),
            _ => {
                self.registers.pc += instruction.length;
                instruction.clock_cycles
            }
        };

        match handle_interrupts(self, mmu) {
            Some(cycles) => clock_cycles += cycles,
            None => {}
        }

        /* if mmu.dma {
                    clock_cycles += 160;
                    mmu.dma = false;
                }
        */
        self.clock.cycle(clock_cycles as usize);
        clock_cycles
    }
}
