//! # Components for creating iso8583 message specifications

#[allow(dead_code)]
pub enum Type {
    ALPHA,
    NUM,
    XN,
    S,
    AN,
    AS,
    NS,
    ANS,
    BIN,
    Z,
}

#[allow(dead_code)]
pub enum Size {
    FIXED(u16),
    LL(u16),
    LLL(u16),
    LLLL(u16),
    BMP,
}

#[allow(dead_code)]
pub struct Spec {
    pub data_type: Type,
    pub data_size: Size,
}
