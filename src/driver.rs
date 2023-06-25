pub enum Driver {
    Xds100v2CortexR,
    Xds100V2CsDap,
    Xds100v2IcepickC,
}

impl Driver {
    pub fn xml(&self) -> &str {
        match self {
            Self::Xds100v2CortexR => "tixds100v2cortexR.xml",
            Self::Xds100V2CsDap => "tixds100v2cs_dap.xml",
            Self::Xds100v2IcepickC => "tixds100v2icepick_c.xml",
        }
    }
}
