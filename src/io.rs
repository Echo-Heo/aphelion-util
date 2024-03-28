//! Input/output utilities.

/// Aphelion uses a port-based I/O system with a theoretical maximum of 65,536 ports available.
/// There are four internal ports defined in the Aphelion ISA:
///
/// 0. [Interrupt controller](Port::INT): Manages interrupts and the Interrupt Vector Table.
/// 1. [Input/output controller](Port::IO): Manages I/O operations and provides I/O information.
/// 2. [Memory management unit](Port::MMU): Oversees memory access and address translation.
/// 3. [System timer](Port::SYSTIMER): Provides time information and can be used as a PIT/HPET.
///
/// These ports are given equivalent constants within the struct for ease of use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Port(pub u16);

impl Port {
    /// Interrupt controller.
    pub const INT: Self = Self(0);
    /// Input/output controller.
    pub const IO: Self = Self(1);
    /// Memory management unit.
    pub const MMU: Self = Self(2);
    /// System timer.
    pub const SYSTIMER: Self = Self(3);
}
