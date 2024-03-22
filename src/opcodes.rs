pub struct OpcodeLDA {
    pub IMMEDIATE_0xA9: u8,
    pub ZERO_PAGE_0xA5: u8
}
pub const OPCODE_LDA: OpcodeLDA = OpcodeLDA {
    IMMEDIATE_0xA9: 0xA9,
    ZERO_PAGE_0xA5: 0xA5,
};

pub mod LDA {
    pub const IMMEDIATE_0xA9: u8 = 0xA9;
    pub const ZERO_PAGE_0xA5: u8 = 0xA5;
}