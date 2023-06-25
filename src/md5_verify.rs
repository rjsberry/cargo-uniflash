use crate::common::*;

pub fn md5_verify(path: &Path, expected: &[u8]) -> anyhow::Result<bool> {
    let data = fs::read(path)?;
    let mut hasher = Md5::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(&result[..] == expected)
}
