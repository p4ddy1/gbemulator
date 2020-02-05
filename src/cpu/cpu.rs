use crate::cpu::clock::Clock;
use crate::cpu::instructions;
use crate::cpu::instructions::ExecutionType;
use crate::cpu::interrupts::handle_interrupts;
use crate::cpu::registers::Registers;
use crate::memory::mmu::{Opcode, INTERRUPT_FLAGS_ADDRESS};
use crate::Mmu;

pub struct Cpu<'a> {
    pub registers: Registers,
    clock: Clock,
    pub mmu: &'a mut Mmu<'a>,
    pub interrupt_master_enabled: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(mmu: &'a mut Mmu<'a>) -> Cpu<'a> {
        let registers = Registers::new();
        let clock = Clock::new();

        Cpu {
            registers,
            clock,
            mmu,
            interrupt_master_enabled: false,
        }
    }

    pub fn execute_program_counter(&mut self) -> u8 {
        //TODO: Remove! Only for testing
        if self.mmu.gpu.v_blank {
            self.mmu.gpu.v_blank = false;
            self.mmu.write(INTERRUPT_FLAGS_ADDRESS, 0x01);
        }

        //TODO: Remove! Only for testing
        if self.mmu.gpu.lcd_stat {
            self.mmu.gpu.lcd_stat = false;
            self.mmu.write(INTERRUPT_FLAGS_ADDRESS, 0x02);
        }

        let op_code = self.mmu.read_opcode(self.registers.pc);

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

        /*   match op_code {
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

        let result = (instruction.handler)(self, &op_code);

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

        match handle_interrupts(self) {
            Some(cycles) => clock_cycles += cycles,
            None => {}
        }

        self.clock.cycle(clock_cycles as usize);
        self.mmu.gpu.step(clock_cycles);
        clock_cycles
    }

    pub fn get_attribute_for_op_code(&self, index: u16) -> u8 {
        self.mmu.read(self.registers.pc + (index + 1))
    }
}
