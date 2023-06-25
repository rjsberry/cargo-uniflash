use crate::common::*;

#[derive(Debug)]
pub struct Args {
    pub cargo: Vec<String>,
    pub flash_target: FlashTarget,
    pub elf: Option<PathBuf>,
    pub verify: bool,
}

impl Args {
    pub fn parse() -> anyhow::Result<Self> {
        RawArgs::parse(env::args()).map(|(cargo, raw)| {
            let RawArgs { board, custom_config, elf, verify, .. } = raw;

            // parsing raw args successfully guarantees we have exactly one flash target
            let flash_target = match (board, custom_config) {
                (Some(board), None) => FlashTarget::Board(board),
                (None, Some(custom_config)) => FlashTarget::CustomConfig(custom_config),
                _ => panic!("[bug]: more than one flash target detected"),
            };

            Self { cargo, flash_target, elf, verify }
        })
    }
}
