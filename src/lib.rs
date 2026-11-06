//! SigmaOS-as — assembler model.
//!
//! Models an assembler: parsing assembly mnemonics into machine-code
//! opcodes, labels/symbols, relocations, and emitting object sections.
//! Deterministic and std-only (no actual codegen).

pub mod asm;
pub mod label;
pub mod obj;

pub use asm::{Instruction, Operand, Opcode};
pub use label::{Label, SymbolTable};
pub use obj::{Object, Section, Relocation};
