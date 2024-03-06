/*!
# Instruction Set

#
*/

use std::fmt::Display;

use crate::nibble::Nibble;

use self::{
    encoding::{B, E, F, M, R},
    instruction_set::InstructionSet,
};

/// instruction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction(pub u32);
impl Instruction {
    /// Get nth position of nibble.
    ///
    /// # Panics
    ///
    /// panics if `idx` is greater than or equal to `8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use asteroid_rs::{instruction::Instruction, nibble::Nibble};
    ///
    /// assert_eq!(Instruction(0x01234567).nth_nibble(1), Nibble::X6);
    /// ```
    #[must_use]
    pub const fn nth_nibble(self, idx: usize) -> Nibble {
        if idx % 2 == 0 {
            Nibble::from_u8(self.0.to_le_bytes()[idx / 2])
        } else {
            Nibble::from_u8_upper(self.0.to_le_bytes()[idx / 2])
        }
    }
    /// Destructure using the [`E`] format.
    #[must_use]
    pub const fn e(self) -> E { E::from_u32(self.0) }
    /// Destructure using the [`R`] format.
    #[must_use]
    pub const fn r(self) -> R { R::from_u32(self.0) }
    /// Destructure using the [`M`] format.
    #[must_use]
    pub const fn m(self) -> M { M::from_u32(self.0) }
    /// Destructure using the [`F`] format.
    #[must_use]
    pub const fn f(self) -> F { F::from_u32(self.0) }
    /// Destructure using the [`B`] format.
    #[must_use]
    pub const fn b(self) -> B { B::from_u32(self.0) }
    #[must_use]
    pub const fn opcode(self) -> u8 { self.0.to_le_bytes()[0] }
    #[must_use]
    pub fn try_into_instruction_set(self) -> Option<InstructionSet> { InstructionSet::try_from_instruction(self) }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(i) = self.try_into_instruction_set() {
            write!(f, "{i}")
        } else {
            write!(f, "Instruction 0x{:08x}", self.0)
        }
    }
}

pub mod encoding {

    /*!
    # Instruction Encoding

    Each instruction follows an encoding format,
    which separates the instruction's 32 bits into disctinct fields.

    ```plaintext
        31..28│ 27..24│ 23..20│ 19..16│          15..8│           7..0│
      ┌───────┼───────┼───────┼───────┼───────────────┼───────────────┤
    E │   rde │   rs1 │   rs2 │  func │        imm(8) │        opcode │
      ├───────┼───────┼───────┼───────┴───────────────┼───────────────┤
    R │   rde │   rs1 │   rs2 │               imm(12) │        opcode │
      ├───────┼───────┼───────┴───────────────────────┼───────────────┤
    M │   rde │   rs1 │                       imm(16) │        opcode │
      ├───────┼───────┼───────────────────────────────┼───────────────┤
    F │   rde │  func │                       imm(16) │        opcode │
      ├───────┼───────┴───────────────────────────────┼───────────────┤
    B │  func │                               imm(20) │        opcode │
      └───────┴───────────────────────────────────────┴───────────────┘
    ```
    */

    use crate::nibble::Nibble;
    /// Instruction format type E, for destructuring.
    /// Opcode is omitted.
    #[derive(Debug, Clone, Copy)]
    pub struct E {
        /// `8..15` (8 bits)
        pub imm:  u8,
        /// `16..19`
        pub func: Nibble,
        /// `20..23`
        pub rs2:  Nibble,
        /// `24..27`
        pub rs1:  Nibble,
        /// `28..31`
        pub rde:  Nibble,
    }
    impl E {
        #[must_use]
        pub const fn from_u32(value: u32) -> Self {
            let [_, b1, b2, b3] = value.to_le_bytes();
            E {
                imm:  b1,
                func: Nibble::from_u8(b2),
                rs2:  Nibble::from_u8_upper(b2),
                rs1:  Nibble::from_u8(b3),
                rde:  Nibble::from_u8_upper(b3),
            }
        }
        #[must_use]
        pub const fn to_u32(self, opcode: u8) -> u32 {
            let E { imm, func, rs2, rs1, rde } = self;
            u32::from_le_bytes([opcode, imm, func.compose(rs2), rs1.compose(rde)])
        }
    }
    /// Instruction format type R, for destructuring.
    /// Opcode is omitted.
    #[derive(Debug, Clone, Copy)]
    pub struct R {
        /// `8..19` (12 bits)
        pub imm: u16,
        /// `20..23`
        pub rs2: Nibble,
        /// `24..27`
        pub rs1: Nibble,
        /// `28..31`
        pub rde: Nibble,
    }
    impl R {
        #[must_use]
        pub const fn from_u32(value: u32) -> Self {
            let [.., b2, b3] = value.to_le_bytes();
            R {
                imm: ((value >> 8) & 0x0FFF) as u16,
                rs2: Nibble::from_u8_upper(b2),
                rs1: Nibble::from_u8(b3),
                rde: Nibble::from_u8_upper(b3),
            }
        }
        #[must_use]
        pub const fn to_u32(self, opcode: u8) -> u32 {
            let R { imm, rs2, rs1, rde } = self;
            let [imm0, imm1] = imm.to_le_bytes();
            u32::from_le_bytes([opcode, imm0, Nibble::from_u8(imm1).compose(rs2), rs1.compose(rde)])
        }
    }
    /// Instruction format type M, for destructuring.
    /// Opcode is omitted.
    #[derive(Debug, Clone, Copy)]
    pub struct M {
        /// `8..23` (16 bits)
        pub imm: u16,
        /// `24..27`
        pub rs1: Nibble,
        /// `28..31`
        pub rde: Nibble,
    }
    impl M {
        #[must_use]
        pub const fn from_u32(value: u32) -> Self {
            let [_, b1, b2, b3] = value.to_le_bytes();
            M {
                imm: u16::from_le_bytes([b1, b2]),
                rs1: Nibble::from_u8(b3),
                rde: Nibble::from_u8_upper(b3),
            }
        }
        #[must_use]
        pub const fn to_u32(self, opcode: u8) -> u32 {
            let M { imm, rs1, rde } = self;
            let [imm0, imm1] = imm.to_le_bytes();
            u32::from_le_bytes([opcode, imm0, imm1, rs1.compose(rde)])
        }
    }
    /// Instruction format type F, for destructuring.
    /// Opcode is omitted.
    #[derive(Debug, Clone, Copy)]
    pub struct F {
        /// `8..23` (16 bits)
        pub imm:  u16,
        /// `24..27`
        pub func: Nibble,
        /// `28..31`
        pub rde:  Nibble,
    }
    impl F {
        #[must_use]
        pub const fn from_u32(value: u32) -> Self {
            let [_, b1, b2, b3] = value.to_le_bytes();
            F {
                imm:  u16::from_le_bytes([b1, b2]),
                func: Nibble::from_u8(b3),
                rde:  Nibble::from_u8_upper(b3),
            }
        }
        #[must_use]
        pub const fn to_u32(self, opcode: u8) -> u32 {
            let F { imm, func, rde } = self;
            let [imm0, imm1] = imm.to_le_bytes();
            u32::from_le_bytes([opcode, imm0, imm1, func.compose(rde)])
        }
    }
    /// Instruction format type B, for destructuring.
    /// Opcode is omitted.
    #[derive(Debug, Clone, Copy)]
    pub struct B {
        /// `8..27` (20 bits)
        pub imm:  u32,
        /// `28..31`
        pub func: Nibble,
    }
    impl B {
        #[must_use]
        pub const fn from_u32(value: u32) -> Self {
            let [.., b3] = value.to_le_bytes();
            B {
                imm:  (value >> 8) & 0x000F_FFFF,
                func: Nibble::from_u8_upper(b3),
            }
        }
        #[must_use]
        pub const fn to_u32(self, opcode: u8) -> u32 {
            let B { imm, func } = self;
            (opcode as u32) | (imm << 8) | ((func.as_u8() as u32) << 28)
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub mod instruction_set {
    /*!
    # Instruction Set
    */
    use std::fmt::Display;

    use crate::{interrupt::Interrupt, io::Port, nibble::Nibble, registers::Register};

    use super::{
        encoding::{B, E, F, M, R},
        Instruction,
    };
    /// # Branch Conditions
    ///
    /// | Mnemonic | Code | With `cmpr, A, B` |
    /// | :------- | :--- | :---------------- |
    /// | [`bra` ](BranchCond::Bra ) | `0x0` | `true`                    |
    /// | [`beq` ](BranchCond::Beq ) | `0x1` | `A = B`                   |
    /// | [`bez` ](BranchCond::Bez ) | `0x2` | `A = 0`                   |
    /// | [`blt` ](BranchCond::Blt ) | `0x3` | `(A as i64) < (B as i64)` |
    /// | [`ble` ](BranchCond::Ble ) | `0x4` | `(A as i64) ≤ (B as i64)` |
    /// | [`bltu`](BranchCond::Bltu) | `0x5` | `(A as u64) < (B as u64)` |
    /// | [`bleu`](BranchCond::Bleu) | `0x6` | `(A as u64) ≤ (B as u64)` |
    /// | [`bne` ](BranchCond::Bne ) | `0x9` | `A ≠ B`                   |
    /// | [`bnz` ](BranchCond::Bnz ) | `0xA` | `A ≠ 0`                   |
    /// | [`bge` ](BranchCond::Bge ) | `0xB` | `(A as i64) ≥ (B as i64)` |
    /// | [`bgt` ](BranchCond::Bgt ) | `0xC` | `(A as i64) > (B as i64)` |
    /// | [`bgeu`](BranchCond::Bgeu) | `0xD` | `(A as u64) ≥ (B as u64)` |
    /// | [`bgtu`](BranchCond::Bgtu) | `0xE` | `(A as u64) > (B as u64)` |
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum BranchCond {
        Bra  = 0x0,
        Beq  = 0x1,
        Bez  = 0x2,
        Blt  = 0x3,
        Ble  = 0x4,
        Bltu = 0x5,
        Bleu = 0x6,
        Bne  = 0x9,
        Bnz  = 0xA,
        Bge  = 0xB,
        Bgt  = 0xC,
        Bgeu = 0xD,
        Bgtu = 0xE,
    }
    impl BranchCond {
        #[must_use]
        pub const fn try_from_nibble(value: Nibble) -> Option<Self> {
            match value {
                Nibble::X0 => Some(Self::Bra),
                Nibble::X1 => Some(Self::Beq),
                Nibble::X2 => Some(Self::Bez),
                Nibble::X3 => Some(Self::Blt),
                Nibble::X4 => Some(Self::Ble),
                Nibble::X5 => Some(Self::Bltu),
                Nibble::X6 => Some(Self::Bleu),
                Nibble::X9 => Some(Self::Bne),
                Nibble::XA => Some(Self::Bnz),
                Nibble::XB => Some(Self::Bge),
                Nibble::XC => Some(Self::Bgt),
                Nibble::XD => Some(Self::Bgeu),
                Nibble::XE => Some(Self::Bgtu),
                _ => None,
            }
        }
        const fn string(self) -> &'static str {
            match self {
                Self::Bra => "bra",
                Self::Beq => "beq",
                Self::Bez => "bez",
                Self::Blt => "blt",
                Self::Ble => "ble",
                Self::Bltu => "bltu",
                Self::Bleu => "bleu",
                Self::Bne => "bne",
                Self::Bnz => "bnz",
                Self::Bge => "bge",
                Self::Bgt => "bgt",
                Self::Bgeu => "bgeu",
                Self::Bgtu => "bgtu",
            }
        }
    }
    impl Display for BranchCond {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.string()) }
    }
    /// load immediate type.
    ///
    /// | Mnemonic | Code | With `rd`, `imm` |
    /// | :------- | :--- | :--------------- |
    /// | [`lli`  ](LiType::Lli  ) | `0` | `rd[15..0]  ← imm`                |
    /// | [`llis` ](LiType::Llis ) | `1` | `rd         ← (imm as i64)`       |
    /// | [`lui`  ](LiType::Lui  ) | `2` | `rd[31..16] ← imm`                |
    /// | [`luis` ](LiType::Luis ) | `3` | `rd         ← (imm as i64) << 16` |
    /// | [`lti`  ](LiType::Lti  ) | `4` | `rd[47..32] ← imm`                |
    /// | [`ltis` ](LiType::Ltis ) | `5` | `rd         ← (imm as i64) << 32` |
    /// | [`ltui` ](LiType::Ltui ) | `6` | `rd[63..48] ← imm`                |
    /// | [`ltuis`](LiType::Ltuis) | `7` | `rd         ← (imm as i64) << 48` |
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum LiType {
        Lli   = 0,
        Llis  = 1,
        Lui   = 2,
        Luis  = 3,
        Lti   = 4,
        Ltis  = 5,
        Ltui  = 6,
        Ltuis = 7,
    }
    impl LiType {
        #[must_use]
        pub const fn try_from_nibble(value: Nibble) -> Option<Self> {
            match value {
                Nibble::X0 => Some(Self::Lli),
                Nibble::X1 => Some(Self::Llis),
                Nibble::X2 => Some(Self::Lui),
                Nibble::X3 => Some(Self::Luis),
                Nibble::X4 => Some(Self::Lti),
                Nibble::X5 => Some(Self::Ltis),
                Nibble::X6 => Some(Self::Ltui),
                Nibble::X7 => Some(Self::Ltuis),
                _ => None,
            }
        }
        const fn string(self) -> &'static str {
            match self {
                Self::Lli => "lli",
                Self::Llis => "llis",
                Self::Lui => "lui",
                Self::Luis => "luis",
                Self::Lti => "lti",
                Self::Ltis => "ltis",
                Self::Ltui => "ltui",
                Self::Ltuis => "ltuis",
            }
        }
    }
    impl Display for LiType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.string()) }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FloatPrecision {
        F16 = 0,
        F32 = 1,
        F64 = 2,
    }
    impl FloatPrecision {
        #[must_use]
        pub const fn try_from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(Self::F16),
                1 => Some(Self::F32),
                2 => Some(Self::F64),
                _ => None,
            }
        }
        #[must_use]
        pub const fn try_from_nibble(value: Nibble) -> Option<Self> {
            match value {
                Nibble::X0 => Some(Self::F16),
                Nibble::X1 => Some(Self::F32),
                Nibble::X2 => Some(Self::F64),
                _ => None,
            }
        }
    }
    impl Display for FloatPrecision {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::F16 => write!(f, ".16"),
                Self::F32 => write!(f, ".32"),
                Self::F64 => write!(f, ".64"),
            }
        }
    }
    #[derive(Debug, Clone, Copy)]
    pub struct FloatCastType {
        pub to:   FloatPrecision,
        pub from: FloatPrecision,
    }
    impl FloatCastType {
        #[must_use]
        pub const fn try_from_nibble(value: Nibble) -> Option<Self> {
            if let (Some(to), Some(from)) = (
                FloatPrecision::try_from_u8((value as u8) & 0x11),
                FloatPrecision::try_from_u8((value as u8) >> 2),
            ) {
                Some(Self { to, from })
            } else {
                None
            }
        }
    }
    impl Display for FloatCastType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}{}", self.to, self.from) }
    }
    #[derive(Debug, Clone, Copy)]
    /// instruction set, for destructuring [`Instruction`].
    pub enum InstructionSet {
        // System Control
        /// trigger interrupt `imm8` (see [Interrupts](crate::interrupt))
        Int { imm8: Interrupt },
        /// return from interrupt
        Iret,
        /// resolve interrupt
        Ires,
        /// enter user mode and jump to address in `rd`
        Usr { rd: Register },

        // Input & Output
        /// output data in `rs` to port `rd`
        Outr { rd: Register, rs: Register },
        /// output data in `rs` to port `imm16`
        Outi { imm16: Port, rs: Register },
        /// read data from port `rs` to `rd`
        Inr { rd: Register, rs: Register },
        /// read data from port `imm16` to `rd`
        Ini { rd: Register, imm16: Port },

        // Control Flow
        /// push `ip`, `ip ← rs + 4 × (imm16 as i64)`
        Jal { rs: Register, imm16: u16 },
        /// `rd ← ip`, `ip ← rs + 4 × (imm16 as i64)`
        Jalr { rd: Register, rs: Register, imm16: u16 },
        /// pop `ip`
        Ret,
        /// `ip ← rs`
        Retr { rs: Register },
        /// `ip ← pc + 4 × (imm20 as i64)`, branch on condition (see [`BranchCond`])
        Branch { cc: BranchCond, imm20: u32 },

        // Stack Operations
        /// `sp ← sp - 8`, `mem[sp] ← rs`
        Push { rs: Register },
        /// `rd ← mem[sp]`, `sp ← sp + 8`
        Pop { rd: Register },
        /// push `fp`, `fp = sp`; enter stack frame
        Enter,
        /// `sp = fp`, pop `fp`; leave stack frame
        Leave,

        // Data Flow
        /// load immediate; see [`LiType`]
        Li { rd: Register, func: LiType, imm: u16 },
        /// `rd ← mem[rs + (off as i64) + (rn << sh)]`
        Lw {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `rd[31..0] ← mem[rs + (off as i64) + (rn << sh)]`
        Lh {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `rd ← mem[rs + (off as i64) + (rn << sh)]`
        Lhs {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `rd[15..0] ← mem[rs + (off as i64) + (rn << sh)]`
        Lq {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `rd ← mem[rs + (off as i64) + (rn << sh)]`
        Lqs {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `rd[7..0] ← mem[rs + (off as i64) + (rn << sh)]`
        Lb {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `rd ← mem[rs + (off as i64) + (rn << sh)]`
        Lbs {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `mem[rs + off + (rs << sh)] ← (rd as i64)`
        Sw {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `mem[rs + off + (rs << sh)] ← (rd as i32)`
        Sh {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `mem[rs + off + (rs << sh)] ← (rd as i16)`
        Sq {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },
        /// `mem[rs + off + (rs << sh)] ← (rd as i8)`
        Sb {
            rd:  Register,
            rs:  Register,
            rn:  Register,
            sh:  Nibble,
            off: u8,
        },

        // Comparisons
        /// compare and set flags (see [status register](crate::registers#st--status-register))
        Cmpr { r1: Register, r2: Register },
        /// compare and set flags (see [status register](crate::registers#st--status-register)).
        /// `imm` is sign-extended.
        /// if the immediate value is first, `s` is set to 1, else 0.
        Cmpi { r1: Register, s: bool, imm: u16 },

        // Arithmetic Operations
        /// `rd ← r1 + r2`
        Addr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 + (imm16 as i64)`
        Addi { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 - r2`
        Subr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 - (imm16 as i64)`
        Subi { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 × r2 (signed)`
        Imulr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 × (imm16 as i64) (signed)`
        Imuli { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 ÷ r2 (signed)`
        Idivr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 ÷ (imm16 as i64) (signed)`
        Idivi { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 × r2 (unsigned)`
        Umulr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 × (imm16 as u64) (unsigned)`
        Umuli { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 ÷ r2 (unsigned)`
        Udivr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 ÷ (imm16 as u64) (unsigned)`
        Udivi { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← rem(r1, r2)`
        Remr { rd: Register, r1: Register, r2: Register },
        /// `rd ← rem(r1, (imm16 as i64))`
        Remi { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← mod(r1, r2)`
        Modr { rd: Register, r1: Register, r2: Register },
        /// `rd ← mod(r1, (imm16 as i64))`
        Modi { rd: Register, r1: Register, imm16: u16 },

        // Bitwise Operations
        /// `rd ← r1 & r2`
        Andr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 & (imm16 as u64)`
        Andi { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 | r2`
        Orr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 | (imm16 as u64)`
        Ori { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← !(r1 | r2)`
        Norr { rd: Register, r1: Register, r2: Register },
        /// `rd ← !(r1 | (imm16 as u64))`
        Nori { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 ^ r2`
        Xorr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 ^ (imm16 as u64)`
        Xori { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← r1 << r2`
        Shlr { rd: Register, r1: Register, r2: Register },
        /// `rd ← r1 << (imm16 as u64)`
        Shli { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← (r1 as i64) >> r2`
        Asrr { rd: Register, r1: Register, r2: Register },
        /// `rd ← (r1 as i64)1 >> (imm16 as u64)`
        Asri { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← (r1 as i64) >> r2`
        Lsrr { rd: Register, r1: Register, r2: Register },
        /// `rd ← (r1 as i64) >> (imm16 as u64)`
        Lsri { rd: Register, r1: Register, imm16: u16 },
        /// `rd ← if r2 in 0..64 { r1[r2] } else { 0 }`
        Bitr { rd: Register, r1: Register, r2: Register },
        /// `rd ← if imm16 in 0..64 { r1[imm16] } else { 0 }`
        Biti { rd: Register, r1: Register, imm16: u16 },

        // Floating-Point Operations
        /// `rd ← comp(r1, r2)`
        Fcmp { r1: Register, r2: Register, p: FloatPrecision },
        /// `rd ← rs as f`
        Fto { rd: Register, rs: Register, p: FloatPrecision },
        /// `rd ← rs as i64`
        Ffrom { rd: Register, rs: Register, p: FloatPrecision },
        /// `rd ← -rs`
        Fneg { rd: Register, rs: Register, p: FloatPrecision },
        /// `rd ← |rs|`
        Fabs { rd: Register, rs: Register, p: FloatPrecision },
        /// `rd ← r1 + r2`
        Fadd {
            rd: Register,
            r1: Register,
            r2: Register,
            p:  FloatPrecision,
        },
        /// `rd ← r1 - r2`
        Fsub {
            rd: Register,
            r1: Register,
            r2: Register,
            p:  FloatPrecision,
        },
        /// `rd ← r1 × r2`
        Fmul {
            rd: Register,
            r1: Register,
            r2: Register,
            p:  FloatPrecision,
        },
        /// `rd ← r1 ÷ r2`
        Fdiv {
            rd: Register,
            r1: Register,
            r2: Register,
            p:  FloatPrecision,
        },
        /// `rd +← r1 × r2`
        Fma {
            rd: Register,
            r1: Register,
            r2: Register,
            p:  FloatPrecision,
        },
        /// `rd ← √r1`
        Fsqrt { rd: Register, r1: Register, p: FloatPrecision },
        /// `rd ← min(r1, r2)`
        Fmin {
            rd: Register,
            r1: Register,
            r2: Register,
            p:  FloatPrecision,
        },
        /// `rd ← max(r1, r2)`
        Fmax {
            rd: Register,
            r1: Register,
            r2: Register,
            p:  FloatPrecision,
        },
        /// `rd ← ceil(r1)`
        Fsat { rd: Register, r1: Register, p: FloatPrecision },
        /// `rd ← cast(r1)`
        Fcnv { rd: Register, r1: Register, p: FloatCastType },
        /// `rd ← isnan(r1)`
        Fnan { rd: Register, r1: Register, p: FloatPrecision },
    }
    impl InstructionSet {
        #[must_use]
        #[allow(clippy::too_many_lines)]
        pub fn try_from_instruction(i: Instruction) -> Option<Self> {
            let res = match i.opcode() {
                // System Control
                0x01 => {
                    let F { imm, func, rde } = i.f();
                    let imm8 = Interrupt::try_from_u16(imm);
                    let rd = Register::from_nibble(rde);
                    match func {
                        Nibble::X0 => Self::Int { imm8: imm8? },
                        Nibble::X1 => Self::Iret,
                        Nibble::X2 => Self::Ires,
                        Nibble::X3 => Self::Usr { rd },
                        _ => None?,
                    }
                }
                // Input & Output
                opcode @ 0x02..=0x05 => {
                    let M { imm, rs1, rde } = i.m();
                    let rs = Register::from_nibble(rs1);
                    let rd = Register::from_nibble(rde);
                    let imm16 = Port(imm);
                    match opcode {
                        0x02 => Self::Outr { rd, rs },
                        0x03 => Self::Outi { imm16, rs },
                        0x04 => Self::Inr { rd, rs },
                        0x05 => Self::Ini { rd, imm16 },
                        _ => unreachable!(),
                    }
                }
                // Control Flow
                opcode @ 0x06..=0x09 => {
                    let M { imm: imm16, rs1, rde } = i.m();
                    let rs = Register::from_nibble(rs1);
                    let rd = Register::from_nibble(rde);
                    match opcode {
                        0x06 => Self::Jal { rs, imm16 },
                        0x07 => Self::Jalr { rd, rs, imm16 },
                        0x08 => Self::Ret,
                        0x09 => Self::Retr { rs },
                        _ => unreachable!(),
                    }
                }
                0x0A => {
                    let B { imm, func } = i.b();
                    Self::Branch {
                        cc:    BranchCond::try_from_nibble(func)?,
                        imm20: imm,
                    }
                }
                // Stack Operations
                0x0B => Self::Push {
                    rs: Register::from_nibble(i.m().rs1),
                },
                0x0C => Self::Pop {
                    rd: Register::from_nibble(i.m().rde),
                },
                0x0D => Self::Enter,
                0x0E => Self::Leave,
                // Data Flow
                0x10 => {
                    let F { imm, func, rde } = i.f();
                    let func = LiType::try_from_nibble(func)?;
                    let rd = Register::from_nibble(rde);
                    Self::Li { rd, func, imm }
                }
                opcode @ 0x11..=0x1B => {
                    let E {
                        imm: off,
                        func: sh,
                        rs2,
                        rs1,
                        rde,
                    } = i.e();
                    let rn = Register::from_nibble(rs2);
                    let rs = Register::from_nibble(rs1);
                    let rd = Register::from_nibble(rde);
                    match opcode {
                        0x11 => Self::Lw { rd, rs, rn, sh, off },
                        0x12 => Self::Lh { rd, rs, rn, sh, off },
                        0x13 => Self::Lhs { rd, rs, rn, sh, off },
                        0x14 => Self::Lq { rd, rs, rn, sh, off },
                        0x15 => Self::Lqs { rd, rs, rn, sh, off },
                        0x16 => Self::Lb { rd, rs, rn, sh, off },
                        0x17 => Self::Lbs { rd, rs, rn, sh, off },
                        0x18 => Self::Sw { rd, rs, rn, sh, off },
                        0x19 => Self::Sh { rd, rs, rn, sh, off },
                        0x1A => Self::Sq { rd, rs, rn, sh, off },
                        0x1B => Self::Sb { rd, rs, rn, sh, off },
                        _ => unreachable!(),
                    }
                }
                // Comparisons
                0x1E => {
                    let r1 = Register::from_nibble(i.m().rde);
                    let r2 = Register::from_nibble(i.m().rs1);
                    Self::Cmpr { r1, r2 }
                }
                0x1F => {
                    let F { imm, func, rde } = i.f();
                    let r1 = Register::from_nibble(rde);
                    let s = match func {
                        Nibble::X0 => false,
                        Nibble::X1 => true,
                        _ => None?,
                    };
                    Self::Cmpi { r1, s, imm }
                }
                // Arithmetic & Bitwise Operations
                opcode @ 0x20..=0x3F if opcode % 2 == 0 => {
                    let R { rs2, rs1, rde, .. } = i.r();
                    let rd = Register::from_nibble(rde);
                    let r1 = Register::from_nibble(rs1);
                    let r2 = Register::from_nibble(rs2);
                    match opcode {
                        0x20 => Self::Addr { rd, r1, r2 },
                        0x22 => Self::Subr { rd, r1, r2 },
                        0x24 => Self::Imulr { rd, r1, r2 },
                        0x26 => Self::Idivr { rd, r1, r2 },
                        0x28 => Self::Umulr { rd, r1, r2 },
                        0x2A => Self::Udivr { rd, r1, r2 },
                        0x2C => Self::Remr { rd, r1, r2 },
                        0x2E => Self::Modr { rd, r1, r2 },
                        0x30 => Self::Andr { rd, r1, r2 },
                        0x32 => Self::Orr { rd, r1, r2 },
                        0x34 => Self::Norr { rd, r1, r2 },
                        0x36 => Self::Xorr { rd, r1, r2 },
                        0x38 => Self::Shlr { rd, r1, r2 },
                        0x3A => Self::Asrr { rd, r1, r2 },
                        0x3C => Self::Lsrr { rd, r1, r2 },
                        0x3E => Self::Bitr { rd, r1, r2 },
                        _ => unreachable!(),
                    }
                }
                opcode @ 0x20..=0x3F => {
                    let M { imm: imm16, rs1, rde } = i.m();
                    let rd = Register::from_nibble(rde);
                    let r1 = Register::from_nibble(rs1);
                    match opcode {
                        0x21 => Self::Addi { rd, r1, imm16 },
                        0x23 => Self::Subi { rd, r1, imm16 },
                        0x25 => Self::Imuli { rd, r1, imm16 },
                        0x27 => Self::Idivi { rd, r1, imm16 },
                        0x29 => Self::Umuli { rd, r1, imm16 },
                        0x2B => Self::Udivi { rd, r1, imm16 },
                        0x2D => Self::Remi { rd, r1, imm16 },
                        0x2F => Self::Modi { rd, r1, imm16 },
                        0x31 => Self::Andi { rd, r1, imm16 },
                        0x33 => Self::Ori { rd, r1, imm16 },
                        0x35 => Self::Nori { rd, r1, imm16 },
                        0x37 => Self::Xori { rd, r1, imm16 },
                        0x39 => Self::Shli { rd, r1, imm16 },
                        0x3B => Self::Asri { rd, r1, imm16 },
                        0x3D => Self::Lsri { rd, r1, imm16 },
                        0x3F => Self::Biti { rd, r1, imm16 },
                        _ => unreachable!(),
                    }
                }
                // Floating Point Operations
                opcode @ 0x40..=0x4F => {
                    let E { func, rs2, rs1, rde, .. } = i.e();
                    let rd = Register::from_nibble(rde);
                    let r1 = Register::from_nibble(rs1);
                    let r2 = Register::from_nibble(rs2);
                    let p = FloatPrecision::try_from_nibble(func);
                    let pp = FloatCastType::try_from_nibble(func);
                    match opcode {
                        0x40 => Self::Fcmp { r1, r2, p: p? },
                        0x41 => Self::Fto { rd, rs: r1, p: p? },
                        0x42 => Self::Ffrom { rd, rs: r1, p: p? },
                        0x43 => Self::Fneg { rd, rs: r1, p: p? },
                        0x44 => Self::Fabs { rd, rs: r1, p: p? },
                        0x45 => Self::Fadd { rd, r1, r2, p: p? },
                        0x46 => Self::Fsub { rd, r1, r2, p: p? },
                        0x47 => Self::Fmul { rd, r1, r2, p: p? },
                        0x48 => Self::Fdiv { rd, r1, r2, p: p? },
                        0x49 => Self::Fma { rd, r1, r2, p: p? },
                        0x4A => Self::Fsqrt { rd, r1, p: p? },
                        0x4B => Self::Fmin { rd, r1, r2, p: p? },
                        0x4C => Self::Fmax { rd, r1, r2, p: p? },
                        0x4D => Self::Fsat { rd, r1, p: p? },
                        0x4E => Self::Fcnv { rd, r1, p: pp? },
                        0x4F => Self::Fnan { rd, r1, p: p? },
                        _ => unreachable!(),
                    }
                }
                _ => None?,
            };
            Some(res)
        }
    }
    impl Display for InstructionSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Int { imm8 } => write!(f, "int {}", imm8.0),
                Self::Iret => write!(f, "iret"),
                Self::Ires => write!(f, "ires"),
                Self::Usr { rd } => write!(f, "usr {rd}"),
                Self::Outr { rd, rs } => write!(f, "outr {rd}, {rs}"),
                Self::Outi { imm16, rs } => write!(f, "outi {}, {rs}", imm16.0),
                Self::Inr { rd, rs } => write!(f, "inr {rd}, {rs}"),
                Self::Ini { rd, imm16 } => write!(f, "ini {rd}, {}", imm16.0),
                Self::Jal { rs, imm16 } => write!(f, "jal {rs}, {imm16}"),
                Self::Jalr { rd, rs, imm16 } => write!(f, "jalr {rs}, {imm16}, {rd}"),
                Self::Ret => write!(f, "ret"),
                Self::Retr { rs } => write!(f, "retr {rs}"),
                Self::Branch { cc, imm20 } => write!(f, "{cc} {imm20}"),
                Self::Push { rs } => write!(f, "push {rs}"),
                Self::Pop { rd } => write!(f, "pop {rd}"),
                Self::Enter => write!(f, "enter"),
                Self::Leave => write!(f, "leave"),
                Self::Li { rd, func, imm } => write!(f, "{func} {rd}, {imm}"),
                Self::Lw { rd, rs, rn, sh, off } => write!(f, "lw {rd}, {rs}, {off}, {rn}, {sh}"),
                Self::Lh { rd, rs, rn, sh, off } => write!(f, "lh {rd}, {rs}, {off}, {rn}, {sh}"),
                Self::Lhs { rd, rs, rn, sh, off } => write!(f, "lhs {rd}, {rs}, {off}, {rn}, {sh}"),
                Self::Lq { rd, rs, rn, sh, off } => write!(f, "lq {rd}, {rs}, {off}, {rn}, {sh}"),
                Self::Lqs { rd, rs, rn, sh, off } => write!(f, "lqs {rd}, {rs}, {off}, {rn}, {sh}"),
                Self::Lb { rd, rs, rn, sh, off } => write!(f, "lb {rd}, {rs}, {off}, {rn}, {sh}"),
                Self::Lbs { rd, rs, rn, sh, off } => write!(f, "lbs {rd}, {rs}, {off}, {rn}, {sh}"),
                Self::Sw { rd, rs, rn, sh, off } => write!(f, "sw {rs}, {off}, {rn}, {sh}, {rd}"),
                Self::Sh { rd, rs, rn, sh, off } => write!(f, "sh {rs}, {off}, {rn}, {sh}, {rd}"),
                Self::Sq { rd, rs, rn, sh, off } => write!(f, "sq {rs}, {off}, {rn}, {sh}, {rd}"),
                Self::Sb { rd, rs, rn, sh, off } => write!(f, "sb {rs}, {off}, {rn}, {sh}, {rd}"),
                Self::Cmpr { r1, r2 } => write!(f, "cmpr {r1}, {r2}"),
                Self::Cmpi { r1, s, imm } => {
                    if *s {
                        write!(f, "cmpi {imm} {r1}")
                    } else {
                        write!(f, "cmpi {r1} {imm}")
                    }
                }
                Self::Addr { rd, r1, r2 } => write!(f, "addr {rd}, {r1}, {r2}"),
                Self::Subr { rd, r1, r2 } => write!(f, "subr {rd}, {r1}, {r2}"),
                Self::Imulr { rd, r1, r2 } => write!(f, "imulr {rd}, {r1}, {r2}"),
                Self::Idivr { rd, r1, r2 } => write!(f, "idivr {rd}, {r1}, {r2}"),
                Self::Umulr { rd, r1, r2 } => write!(f, "umulr {rd}, {r1}, {r2}"),
                Self::Udivr { rd, r1, r2 } => write!(f, "udivr {rd}, {r1}, {r2}"),
                Self::Remr { rd, r1, r2 } => write!(f, "remr {rd}, {r1}, {r2}"),
                Self::Modr { rd, r1, r2 } => write!(f, "modr {rd}, {r1}, {r2}"),
                Self::Andr { rd, r1, r2 } => write!(f, "andr {rd}, {r1}, {r2}"),
                Self::Orr { rd, r1, r2 } => write!(f, "orr {rd}, {r1}, {r2}"),
                Self::Norr { rd, r1, r2 } => write!(f, "norr {rd}, {r1}, {r2}"),
                Self::Xorr { rd, r1, r2 } => write!(f, "xorr {rd}, {r1}, {r2}"),
                Self::Shlr { rd, r1, r2 } => write!(f, "shlr {rd}, {r1}, {r2}"),
                Self::Asrr { rd, r1, r2 } => write!(f, "asrr {rd}, {r1}, {r2}"),
                Self::Lsrr { rd, r1, r2 } => write!(f, "lsrr {rd}, {r1}, {r2}"),
                Self::Bitr { rd, r1, r2 } => write!(f, "bitr {rd}, {r1}, {r2}"),
                Self::Addi { rd, r1, imm16 } => write!(f, "addi {rd}, {r1}, {imm16}"),
                Self::Subi { rd, r1, imm16 } => write!(f, "subi {rd}, {r1}, {imm16}"),
                Self::Imuli { rd, r1, imm16 } => write!(f, "imuli {rd}, {r1}, {imm16}"),
                Self::Idivi { rd, r1, imm16 } => write!(f, "idivi {rd}, {r1}, {imm16}"),
                Self::Umuli { rd, r1, imm16 } => write!(f, "umuli {rd}, {r1}, {imm16}"),
                Self::Udivi { rd, r1, imm16 } => write!(f, "udivi {rd}, {r1}, {imm16}"),
                Self::Remi { rd, r1, imm16 } => write!(f, "remi {rd}, {r1}, {imm16}"),
                Self::Modi { rd, r1, imm16 } => write!(f, "modi {rd}, {r1}, {imm16}"),
                Self::Andi { rd, r1, imm16 } => write!(f, "andi {rd}, {r1}, {imm16}"),
                Self::Ori { rd, r1, imm16 } => write!(f, "ori {rd}, {r1}, {imm16}"),
                Self::Nori { rd, r1, imm16 } => write!(f, "nori {rd}, {r1}, {imm16}"),
                Self::Xori { rd, r1, imm16 } => write!(f, "xori {rd}, {r1}, {imm16}"),
                Self::Shli { rd, r1, imm16 } => write!(f, "shli {rd}, {r1}, {imm16}"),
                Self::Asri { rd, r1, imm16 } => write!(f, "asri {rd}, {r1}, {imm16}"),
                Self::Lsri { rd, r1, imm16 } => write!(f, "lsri {rd}, {r1}, {imm16}"),
                Self::Biti { rd, r1, imm16 } => write!(f, "biti {rd}, {r1}, {imm16}"),
                Self::Fcmp { r1, r2, p } => write!(f, "fcmp{p} {r1}, {r2}"),
                Self::Fto { rd, rs, p } => write!(f, "fto{p} {rd}, {rs}"),
                Self::Ffrom { rd, rs, p } => write!(f, "ffrom{p} {rd}, {rs}"),
                Self::Fneg { rd, rs, p } => write!(f, "fneg{p} {rd}, {rs}"),
                Self::Fabs { rd, rs, p } => write!(f, "fabs{p} {rd}, {rs}"),
                Self::Fadd { rd, r1, r2, p } => write!(f, "fadd{p} {rd}, {r1}, {r2}"),
                Self::Fsub { rd, r1, r2, p } => write!(f, "fsub{p} {rd}, {r1}, {r2}"),
                Self::Fmul { rd, r1, r2, p } => write!(f, "fmul{p} {rd}, {r1}, {r2}"),
                Self::Fdiv { rd, r1, r2, p } => write!(f, "fdiv{p} {rd}, {r1}, {r2}"),
                Self::Fma { rd, r1, r2, p } => write!(f, "fma{p} {rd}, {r1}, {r2}"),
                Self::Fsqrt { rd, r1, p } => write!(f, "fsqrt{p} {rd}, {r1}"),
                Self::Fmin { rd, r1, r2, p } => write!(f, "fmin{p} {rd}, {r1}, {r2}"),
                Self::Fmax { rd, r1, r2, p } => write!(f, "fmax{p} {rd}, {r1}, {r2}"),
                Self::Fsat { rd, r1, p } => write!(f, "fat{p} {rd}, {r1}"),
                Self::Fcnv { rd, r1, p } => write!(f, "fcnv{p} {rd}, {r1}"),
                Self::Fnan { rd, r1, p } => write!(f, "fnan{p} {rd}, {r1}"),
            }
        }
    }
}
