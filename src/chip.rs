use crate::common::*;

#[derive(Debug, Copy, Clone)]
pub enum Chip {
    RM42L432,
}

impl Chip {
    pub fn xml(&self) -> &str {
        match self {
            Self::RM42L432 => "rm42l432.xml",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::RM42L432 => "RM42L432",
        }
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl FromStr for Chip {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rm42l432" | "RM42L432" => Ok(Self::RM42L432),
            _ => Err(FromStrError::chip(s)),
        }
    }
}
