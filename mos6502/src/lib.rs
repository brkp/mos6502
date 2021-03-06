pub mod cpu;
pub mod bus;
pub mod opcode;

pub mod prelude {
    pub use crate::bus::{Bus, Peripheral};

    pub use crate::cpu::{
        CPU, Flags, Interrupt,
        NMI_VECTOR, RST_VECTOR, IRQ_VECTOR,
    };

    pub use crate::opcode::{
        Opcode, OPCODE_MAP, Operand,
        AddressingMode, TickModifier,
    };
}
