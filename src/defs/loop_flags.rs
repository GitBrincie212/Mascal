#[derive(Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum LoopFlags {
    Continue,
    Break,
    None,
}
