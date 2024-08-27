use super::{registers::StandardRegister, Cpu};

#[derive(Debug)]
pub enum Instruction {
    Nop,
    AddA(StandardRegister),
}
impl Instruction {
    /// Match the given byte with its corresponding instruction.
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            // NOP
            0x00 => Some(Instruction::Nop),
            // ADD A,n
            0x87 => Some(Instruction::AddA(StandardRegister::A)),
            0x80 => Some(Instruction::AddA(StandardRegister::B)),
            0x81 => Some(Instruction::AddA(StandardRegister::C)),
            0x82 => Some(Instruction::AddA(StandardRegister::D)),
            0x83 => Some(Instruction::AddA(StandardRegister::E)),
            0x84 => Some(Instruction::AddA(StandardRegister::H)),
            0x85 => Some(Instruction::AddA(StandardRegister::L)),
            // TODO 0x86
            // TODO 0xC6
            _ => None,
        }
    }
}

pub fn execute(cpu: &mut Cpu, instruction: Instruction) {
    match instruction {
        Instruction::AddA(target_register) => add_a(cpu, target_register),
        _ => unimplemented!(
            "Instruction \"{:?}\" is currently unimplemented.",
            instruction
        ),
    }
}

// INSTRUCTION FUNCTIONS

/// LD r1,r2 - Set `r1` = `r2`.
fn ld_r1_r2(cpu: &mut Cpu, r1: StandardRegister, r2: StandardRegister) {
    r1.set_reg(cpu, r2.get_reg(cpu));
}

/// ADD r - Set register `a` = `a` + `r`.
fn add_a(cpu: &mut Cpu, r: StandardRegister) {
    let value = r.get_reg(cpu);
    let (sum, is_overflow) = cpu.reg.a.overflowing_add(value);
    // Iff sum == 0, set zero flag.
    cpu.reg.set_zero_flag(sum == 0);
    cpu.reg.set_add_sub_flag(false);
    // Iff overflow occurred during add, set carry flag.
    cpu.reg.set_carry_flag(is_overflow);
    // Iff lower nibbles of operands sum to > 0xF, set half carry flag.
    cpu.reg
        .set_half_carry_flag(((cpu.reg.a & 0xF) + (value & 0xF)) > 0xF);
    cpu.reg.a = sum;
}

// HELPER FUNCTIONS

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_ld_r1_r2() {
        let mut cpu = Cpu::default();
        cpu.reg.b = 0xA1;
        cpu.reg.c = 0xBF;
        assert_eq!(cpu.reg.b, 0xA1);
        assert_eq!(cpu.reg.c, 0xBF);

        ld_r1_r2(&mut cpu, StandardRegister::B, StandardRegister::C);
        assert_eq!(cpu.reg.b, 0xBF);
        assert_eq!(cpu.reg.c, 0xBF);
    }

    #[test]
    fn test_add_a() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.reg.a, 0x0);

        cpu.reg.b = 0x01;
        add_a(&mut cpu, StandardRegister::B);
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.b, 0x01);
        assert_eq!(cpu.reg.f, 0b0000_0000);

        // Test half carry
        cpu.reg.a = 0x0E;
        cpu.reg.b = 0x02;
        add_a(&mut cpu, StandardRegister::B);
        assert_eq!(cpu.reg.a, 0x10);
        assert_eq!(cpu.reg.b, 0x02);
        assert_eq!(cpu.reg.f, 0b0010_0000);

        // Test zero
        cpu.reg.a = 0x00;
        cpu.reg.c = 0x00;
        add_a(&mut cpu, StandardRegister::C);
        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.f, 0b1000_0000);

        // Test carry
        cpu.reg.a = 0xF0;
        cpu.reg.e = 0x11;
        add_a(&mut cpu, StandardRegister::E);
        assert_eq!(cpu.reg.a, 0x01);
        assert_eq!(cpu.reg.f, 0b0001_0000);
    }
}
