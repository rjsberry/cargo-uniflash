use crate::common::*;

pub struct Dslite {
    command: Cow<'static, OsStr>,
}

impl Dslite {
    #[cfg(target_os = "macos")]
    pub fn new() -> anyhow::Result<Option<Self>> {
        let dslite = OsStr::new("dslite");
        let dslite_sh = OsStr::new("dslite.sh");
        let canonical_dslite_sh = OsStr::new("/Applications/TI/UniFlash/dslite.sh");

        let command = if command_exists(dslite)? {
            Some(dslite)
        } else if command_exists(dslite_sh)? {
            Some(dslite_sh)
        } else if command_exists(canonical_dslite_sh)? {
            Some(canonical_dslite_sh)
        } else {
            None
        };

        Ok(command.map(Cow::from).map(|command| Self { command }))
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> anyhow::Result<Option<Self>> {
        let dslite_sh = OsStr::new("dslite.sh");
        let canonical_dslite_sh = OsStr::new("/opt/ti/uniflash/dslite.sh");

        let command = if command_exists(dslite_sh)? {
            Some(dslite_sh)
        } else if command_exists(canonical_dslite_sh)? {
            Some(canonical_dslite_sh)
        } else {
            None
        };

        Ok(command.map(Cow::from).map(|command| Self { command }))
    }

    #[cfg(windows)]
    pub fn new() -> anyhow::Result<Option<Self>> {
        let dslite_bat = OsStr::new("dslite.bat");
        let canonical_dslite_bat = data_local_dir().unwrap().join("TI/UniFlash/dslite.bat");

        let command = if command_exists(dslite_bat)? {
            Some(Cow::from(dslite_bat))
        } else if command_exists(&canonical_dslite_bat)? {
            Some(Cow::from(canonical_dslite_bat.into_os_string()))
        } else {
            None
        };

        Ok(command.map(|command| Self { command }))
    }

    /// Actually flashes firmware using the `dslite` utility.
    ///
    /// This will display a progress bar to the user to report status as the
    /// process progresses.
    pub fn flash(
        &self,
        meta: &Metadata,
        config: &Path,
        elf: &Path,
        verify: bool,
    ) -> anyhow::Result<()> {
        let mut config_arg = OsString::from("--config=");
        config_arg.push(config.as_os_str());

        let mut args = vec![&config_arg, OsStr::new("-f"), OsStr::new("-e")];
        if verify {
            args.push(OsStr::new("-v"));
        }
        args.push(elf.as_os_str());

        let mut dslite = cmd(&*self.command, &args).reader()?;
        let mut rdr = BufReader::new(&mut dslite).lines();
        let mut spinner = Progress::spinner("Connecting");

        if let Err(err) = (|| -> anyhow::Result<()> {
            read_until_matches(&mut rdr, |line| line.starts_with("Erasing"))?;
            spinner.set_message("Erasing");

            read_until_matches(&mut rdr, |line| line.starts_with('.'))?;
            spinner.set_message("Flashing");

            if verify {
                read_until_matches(&mut rdr, |line| line.starts_with("Verifying"))?;
                spinner.set_message("Verifying");
            }

            read_until_matches(&mut rdr, |line| line.starts_with("Success"))?;

            Ok(())
        })() {
            spinner.forget();
            return Err(err);
        }

        let elapsed = spinner.elapsed();
        let elapsed = if elapsed < Duration::from_secs(60) {
            format!("{:0.2}s", elapsed.as_secs_f32())
        } else {
            format!("{}m {}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
        };

        let elf = elf.strip_prefix(&meta.workspace_root).unwrap_or(elf);

        spinner.finish(
            "Finished",
            &format!("flashed '{}' in {elapsed}", elf.as_os_str().to_string_lossy()),
        );

        Ok(())
    }
}
