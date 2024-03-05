/*!
# Interrupts

The Interrupt Vector Table (IVT) has 256 entries.
Each entry is a function address,
making the full table 2048 bytes wide.
The location of an interrupt's IVT entry is defined as
`IVT_BASE_ADDRESS + (int * 8)`, where `int` is the interrupt number.
When an interrupt is triggered, it will exit to kernel mode
(if not in kernel mode already) and run code
at the address defined at the relevant IVT entry.
Interrupt handlers should be returned from
with the [`iret`](crate::instruction::instruction_set::InstructionSet::Iret) instruction
or marked as resolved with the [`ires`](crate::instruction::instruction_set::InstructionSet::Ires) instruction.

The IVT's location is initialized to `0`, so should be
initialized somewhere else as soon as possible after startup.
For information about setting the location of the IVT,
see [reserved ports](crate::TODO).

Aphelion's reserved interrupts are as follows:

| Code | Name | Description |
| ---- | ---- | ----------- |
| `0x00` | Divide By Zero     | Triggers when the second argument of a div, mod, or rem instruction is zero. |
| `0x01` | Breakpoint         | Reserved for debugger breakpoints. |
| `0x02` | Invalid Operation  | Triggers when some kind of restricted or invalid operation occurs. This includes unrecognized opcode, unrecognized secondaryfunction values, or when a restricted instruction is encountered / modification of a restricted register is attempted in user mode. |
| `0x03` | Stack Underflow    | Triggers when sp > fp, which means a stack underflow has occurred. |
| `0x04` | Unaligned Access   | Memory has been accessed across type width boundaries. |
| `0x05` | Access Violation   | Memory has been accessed in an invalid way: In kernel mode, this triggers due to accesses outside physical memory bounds. In user mode, this triggers when unmapped / invalid memory is accessed or when virtual memory permissions do not allow the access. |
| `0x06` | Interrupt Overflow | Interrupt controller has experienced an interrupt queue overflow, meaning too many interrupts have triggered in a certain time. |

The Interrupt Controller has an internal FIFO 32-item queue
for pending interrupt signals. If an interrupt triggers
when a handler has not yet returned or resolved,
it is pushed to the queue and will trigger immediately after
the current interrupt handler returns or resolves.
If this queue overflows, the queue will reset and an
*Interrupt Overflow* interrupt will be pushed onto it,
so that it will trigger immediately
after the current interrupt handler is complete.

If there are interrupts waiting to be handled
in the interrupt queue and a handler returns using [`iret`](crate::instruction::instruction_set::InstructionSet::Iret),
the next handler will _immediately_ be executed
instead of immediately returning to the code that triggered it.
The return address will be stored so that execution can
smoothly return once the interrrupt queue is clear.

If an [`ires`](crate::instruction::instruction_set::InstructionSet::Ires) instruction is used
instead of an [`iret`](crate::instruction::instruction_set::InstructionSet::Iret) instruction,
execution resumes after the [`ires`](crate::instruction::instruction_set::InstructionSet::Ires) instruction
itself.
Queued handlers will return to the location after [`ires`](crate::instruction::instruction_set::InstructionSet::Ires).
This is useful for interrupts that must be
considered "resolved" at some point but that may not
return to where they were triggered (such as an exit syscall).

[`iret`](crate::instruction::instruction_set::InstructionSet::Iret) and [`ires`](crate::instruction::instruction_set::InstructionSet::Ires) are interpreted
as [`nop`](crate::TODO) when the interrupt queue is empty.
*/

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interrupt(pub u8);

impl Interrupt {
    /// Triggers when the second argument of a div, mod, or rem instruction is zero
    pub const DIVIDE_BY_ZERO: Self = Interrupt(0x00);
    /// Reserved for debugger breakpoints
    pub const BREAK_POINT: Self = Interrupt(0x01);
    /// Triggers when some kind of restricted or invalid operation occurs.
    /// This includes unrecognized opcode, unrecognized secondary function values,
    /// or when a restricted instruction is encountered /
    /// modification of a restricted register is attempted in user mode
    pub const INVALID_OPERATION: Self = Interrupt(0x02);
    /// Triggers when sp > fp, which means a stack underflow has occurred
    pub const STACK_UNDERFLOW: Self = Interrupt(0x03);
    /// Memory has been accessed across type width boundaries
    pub const UNALIGNED_ACCESS: Self = Interrupt(0x04);
    /// Memory has been accessed in an invalid way: In kernel mode,
    /// this triggers due to accesses outside physical memory bounds.
    /// In user mode, this triggers when unmapped / invalid memory is
    /// accessed or when virtual memory permissions do not allow the access
    pub const ACCESS_VIOLATION: Self = Interrupt(0x05);
    /// Interrupt controller has experienced an interrupt queue overflow,
    /// meaning too many interrupts have triggered in a certain time
    pub const INTERRUPT_OVERFLOW: Self = Interrupt(0x06);
}
impl Interrupt {
    #[must_use]
    pub const fn is_reserved(self) -> bool {
        matches!(
            self,
            Self::DIVIDE_BY_ZERO
                | Self::BREAK_POINT
                | Self::INVALID_OPERATION
                | Self::STACK_UNDERFLOW
                | Self::UNALIGNED_ACCESS
                | Self::ACCESS_VIOLATION
                | Self::INTERRUPT_OVERFLOW
        )
    }
    #[must_use]
    pub const fn try_from_u16(value: u16) -> Option<Self> {
        match value.to_le_bytes() {
            [value, 0] => Some(Self(value)),
            _ => None,
        }
    }
}
impl Display for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::DIVIDE_BY_ZERO => write!(f, "Divide By Zero"),
            Self::BREAK_POINT => write!(f, "Breakpoint"),
            Self::INVALID_OPERATION => write!(f, "Invalid Operation"),
            Self::STACK_UNDERFLOW => write!(f, "Stack Underflow"),
            Self::UNALIGNED_ACCESS => write!(f, "Unaligned Access"),
            Self::ACCESS_VIOLATION => write!(f, "Access Violation"),
            Self::INTERRUPT_OVERFLOW => write!(f, "Interrupt Overflow"),
            _ => write!(f, "Interrupt 0x{:02X}", self.0),
        }
    }
}
