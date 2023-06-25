use crate::common::*;

#[derive(Debug, Copy, Clone)]
pub enum Connection {
    Xds100v2,
}

impl Connection {
    pub fn xml(&self) -> &str {
        match self {
            Self::Xds100v2 => "TIXDS100v2_Connection.xml",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Xds100v2 => "Texas Instruments XDS100v2 USB Debug Probe",
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}
