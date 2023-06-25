use crate::common::*;

pub struct Uniflash {
    path: PathBuf,
}

static UNIFLASH_PATH: Lazy<Option<PathBuf>> =
    Lazy::new(|| cache_dir().map(|dir| dir.join("cargo-uniflash").join("uniflash")));

impl Uniflash {
    pub fn new() -> Self {
        Self { path: PathBuf::new() }
    }

    pub fn download() -> anyhow::Result<Self> {
        download_file(Url::parse(URL).unwrap(), UNIFLASH_PATH.as_ref().unwrap())?;
        Ok(Self::new())
    }
}
