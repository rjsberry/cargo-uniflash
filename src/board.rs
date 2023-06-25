use crate::common::*;

#[derive(Debug, Copy, Clone)]
pub enum Board {
    LaunchXlRm42,
}

impl Board {
    pub fn all() -> &'static [Self] {
        &[Self::LaunchXlRm42]
    }

    pub fn print_all() -> ! {
        println!("Boards:");
        for board in Self::all() {
            println!("    {board}");
        }
        process::exit(0);
    }

    pub fn chip(&self) -> Chip {
        match self {
            Self::LaunchXlRm42 => Chip::RM42L432,
        }
    }

    pub fn connection(&self) -> Connection {
        match self {
            Self::LaunchXlRm42 => Connection::Xds100v2,
        }
    }

    pub fn drivers(&self) -> &'static [Driver] {
        match self {
            Self::LaunchXlRm42 => {
                &[Driver::Xds100v2IcepickC, Driver::Xds100V2CsDap, Driver::Xds100v2CortexR]
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("LAUNCHXL-RM42")
    }
}

impl FromStr for Board {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "launchxl-rm42" | "LAUNCHXL-RM42" => Ok(Self::LaunchXlRm42),
            _ => Err(FromStrError::board(s)),
        }
    }
}
