use crate::cpu::instructions;
use crate::cpu::instructions::{ExecutionType, Instruction};
use crate::cpu::interrupt_handler::handle_interrupts;
use crate::cpu::registers::Registers;
use crate::memory::interrupts::Interrupt;
use crate::memory::mmu::{Mmu, Opcode};

pub enum InterruptAction {
    None,
    Enable,
    Disable,
}

pub struct Cpu {
    pub registers: Registers,
    pub interrupt_action: InterruptAction,
    pub interrupt_master_enabled: bool,
    pub is_halted: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        let registers = Registers::new();

        Cpu {
            registers,
            interrupt_action: InterruptAction::None,
            interrupt_master_enabled: false,
            is_halted: false,
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

        if self.is_halted && any_interrupt_fired(mmu) {
            self.is_halted = false;
            if !self.interrupt_master_enabled {
                //HALT Bug
                self.registers.pc -= 1;
            }
        }

        if self.is_halted {
            return 4;
        }

        if self.interrupt_master_enabled {
            match handle_interrupts(self, mmu) {
                Some(cycles) => return cycles,
                None => {}
            }
        }

        match self.interrupt_action {
            InterruptAction::Enable => {
                self.interrupt_master_enabled = true;
                self.interrupt_action = InterruptAction::None;
            }
            InterruptAction::Disable => {
                self.interrupt_master_enabled = false;
                self.interrupt_action = InterruptAction::None;
            }
            _ => {}
        }

        self.execute_instruction(instruction, mmu, &op_code)
    }

    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
        mmu: &mut Mmu,
        op_code: &Opcode,
    ) -> u8 {
        let result = (instruction.handler)(self, mmu, &op_code);

        //Use the correct value if action of conditional instruction is taken or not
        match result {
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
        }
    }
}

pub fn any_interrupt_fired(mmu: &Mmu) -> bool {
    mmu.interrupts.interrupt_fired(&Interrupt::Vblank)
        || mmu.interrupts.interrupt_fired(&Interrupt::LcdStat)
        || mmu.interrupts.interrupt_fired(&Interrupt::Timer)
        || mmu.interrupts.interrupt_fired(&Interrupt::Serial)
        || mmu.interrupts.interrupt_fired(&Interrupt::Joypad)
}
