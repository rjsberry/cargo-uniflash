use crate::common::*;

pub struct Configuration {
    inner: Inner,
}

enum Inner {
    Custom(PathBuf),
    Temp(NamedTempFile),
}

impl Configuration {
    pub fn from_path(path: PathBuf) -> Self {
        Self { inner: Inner::Custom(path) }
    }

    pub fn from_board(board: Board) -> anyhow::Result<Self> {
        Self::from_parts(board.chip(), board.connection(), board.drivers())
    }

    pub fn from_parts(
        chip: Chip,
        connection: Connection,
        drivers: &[Driver],
    ) -> anyhow::Result<Self> {
        let mut f = NamedTempFile::new()?;
        f.write_all(ccxml(chip, connection, drivers).as_bytes())?;

        Ok(Self { inner: Inner::Temp(f) })
    }

    pub fn path(&self) -> &Path {
        match self.inner {
            Inner::Custom(ref path) => path,
            Inner::Temp(ref file) => file.path(),
        }
    }
}

impl Deref for Configuration {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path()
    }
}
