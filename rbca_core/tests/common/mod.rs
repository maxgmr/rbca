use rbca_core::{Cpu, Registers};

#[derive(Debug, Clone)]
pub struct CpuState {
    pub pc: u16,
    pub sp: u16,
    pub regs: Registers,
    pub is_halted: bool,
    pub is_stopped: bool,
    pub di_countdown: usize,
    pub ei_countdown: usize,
    pub interrupts_enabled: bool,
}
impl CpuState {
    pub fn save_state(cpu: &Cpu) -> Self {
        Self {
            pc: cpu.pc,
            sp: cpu.sp,
            regs: cpu.regs.clone(),
            is_halted: cpu.is_halted,
            is_stopped: cpu.is_stopped,
            di_countdown: cpu.di_countdown,
            ei_countdown: cpu.ei_countdown,
            interrupts_enabled: cpu.interrupts_enabled,
        }
    }

    pub fn update(&mut self, cpu: &Cpu) {
        self.pc = cpu.pc;
        self.sp = cpu.sp;
        if self.regs != cpu.regs {
            self.regs = cpu.regs.clone()
        }
        self.is_halted = cpu.is_halted;
        self.is_stopped = cpu.is_stopped;
        self.di_countdown = cpu.di_countdown;
        self.ei_countdown = cpu.ei_countdown;
        self.interrupts_enabled = cpu.interrupts_enabled;
    }
}
