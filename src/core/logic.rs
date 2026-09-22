#[derive(Default, Clone)]
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

impl Gate {
    pub fn name(&self) -> &str {
        match self {
            Gate::NOT => "NOT",
            Gate::AND => "AND",
            Gate::NAND => "NAND",
            Gate::OR => "OR",
            Gate::NOR => "NOR",
            Gate::XOR => "XOR",
            Gate::XNOR => "XNOR",
            Gate::BUFFER => "BUFFER",
            Gate::SWITCH => "SWITCH",
        }
    }
}
