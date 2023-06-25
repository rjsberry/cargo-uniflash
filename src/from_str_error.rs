use crate::common::*;

#[derive(Debug)]
pub struct FromStrError {
    pub kind: &'static str,
    pub s: String,
}

impl FromStrError {
    pub fn board(s: &str) -> Self {
        Self { kind: "board", s: s.to_owned() }
    }

    pub fn chip(s: &str) -> Self {
        Self { kind: "chip", s: s.to_owned() }
    }
}

impl Display for FromStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unknown {}: {}", self.kind, self.s)
    }
}

impl Error for FromStrError {}
