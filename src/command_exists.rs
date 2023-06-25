use crate::common::*;

#[cfg(unix)]
pub fn command_exists<S: AsRef<OsStr>>(command: S) -> anyhow::Result<bool> {
    Ok(cmd!("command", "-v", command.as_ref())
        .stdout_null()
        .stderr_null()
        .unchecked()
        .run()?
        .status
        .success())
}

#[cfg(windows)]
pub fn command_exists<S: AsRef<OsStr>>(command: S) -> anyhow::Result<bool> {
    Ok(cmd!("WHERE", command.as_ref())
        .stdout_null()
        .stderr_null()
        .unchecked()
        .run()?
        .status
        .success())
}
