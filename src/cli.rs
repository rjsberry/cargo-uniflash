use crate::common::*;

pub fn cli() -> anyhow::Result<()> {
    let args = Args::parse()?;
    let cfg = args.flash_target.into_configuration()?;
    let elf = args.elf.map(Ok).unwrap_or_else(|| cargo_build(&args.cargo))?;

    let meta = MetadataCommand::new().exec()?;

    for i in 0..2 {
        if let Some(dslite) = Dslite::new()? {
            dslite.flash(&meta, &cfg, &elf, args.verify)?;
            return Ok(());
        }

        if i == 0 {
            if confirm_install()? {
                install_uniflash()?;
            } else {
                return Ok(());
            }
        }
    }

    panic!("bug: Something went wrong during TI UniFlash installation!");
}
