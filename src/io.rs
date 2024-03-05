#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Port(pub u16);
impl Port {
    pub const INT: Self = Self(0);
    pub const IO: Self = Self(1);
    pub const MMU: Self = Self(2);
    pub const SYSTIMER: Self = Self(3);
    
}