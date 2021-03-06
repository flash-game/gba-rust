use crate::cpu::reg::{Register, OpType, Mode};

struct SoftwareInterrupt {}

impl SoftwareInterrupt {
    ///
    ///
    fn execute(reg: &mut Register, old_pc: u32) {
        let old_cpsr = reg.cspr.value();
        let current_pc = old_pc;
        reg.cspr.set_op_type(OpType::ARM);
        reg.cspr.set_mode(Mode::Supervisor);
        // set irq_disable true
        reg.cspr.set_irq_disable(true);
        reg.set_spsr(old_cpsr);
        // set LR to the next instruction
        reg.set_lr(current_pc);
        reg.set_pc(0x08);
    }
}