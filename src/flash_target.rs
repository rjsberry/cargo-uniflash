use crate::common::*;

#[derive(Debug)]
pub enum FlashTarget {
    Board(Board),
    CustomConfig(PathBuf),
}

impl FlashTarget {
    pub fn into_configuration(self) -> anyhow::Result<Configuration> {
        match self {
            Self::Board(board) => Configuration::from_board(board),
            Self::CustomConfig(custom_config) => Ok(Configuration::from_path(custom_config)),
        }
    }
}
