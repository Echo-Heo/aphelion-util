/*!
# Instruction Set

#
*/

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

    31..28│ 27..24│ 23..20│ 19..16│          15..8│           7..0│
E │   rde │   rs1 │   rs2 │  func │        imm(8) │        opcode │
R │   rde │   rs1 │   rs2 │               imm(12) │        opcode │
M │   rde │   rs1 │                       imm(16) │        opcode │
F │   rde │  func │                       imm(16) │        opcode │
B │  func │                               imm(20) │        opcode │
```
*/


}