#[derive(Default)]
pub enum Gate {
    NOT,
    #[default]
    AND,
    NAND,
    OR,
    NOR,
    XOR,
    XNOR,
    BUFFER,
    SWITCH,
}
