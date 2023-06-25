use crate::common::*;

pub fn cargo_build(cargo_args: &[String]) -> anyhow::Result<PathBuf> {
    let cargo = cmd(
        &env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
        ["build", "--message-format=json-diagnostic-rendered-ansi"]
            .into_iter()
            .chain(cargo_args.iter().map(|arg| &**arg)),
    )
    .reader()?;

    let mut artifact = None;
    thread::sleep(Duration::from_millis(200));

    for message in Message::parse_stream(BufReader::new(cargo)) {
        match message? {
            Message::CompilerArtifact(Artifact { executable: Some(e), .. }) => {
                if artifact.is_some() {
                    return Err(anyhow!("Found more than one firmware image to flash"));
                }
                artifact = Some(e);
            }
            Message::CompilerMessage(CompilerMessage {
                message: Diagnostic { rendered: Some(m), .. },
                ..
            }) => {
                println!("{m}");
            }
            _ => (),
        }
    }

    artifact.map(Into::into).ok_or_else(|| anyhow!("Failed to determine firmware image to flash"))
}
