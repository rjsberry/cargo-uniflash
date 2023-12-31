use crate::common::*;

const VERSION_MAJ_MIN_PATCH: &str = "8.3.0";
const VERSION_BUILD_METADATA: &str = "4307";

pub fn install_uniflash() -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    let ext = "dmg";
    #[cfg(target_os = "linux")]
    let ext = "run";
    #[cfg(windows)]
    let ext = "exe";

    #[cfg(target_os = "macos")]
    let md5 = hex::decode("c98cd29a004dfe1ad54ab4547a42179f").unwrap();
    #[cfg(target_os = "linux")]
    let md5 = hex::decode("1c81dba06aea8f6e26dba87f591214c4").unwrap();
    #[cfg(windows)]
    let md5 = hex::decode("7a14b94d31965a577aea013c3cd156e4").unwrap();

    let url = format!(
        "https://dr-download.ti.com/software-development/software-programming-tool/MD-QeJBJLj8gq/\
        {VERSION_MAJ_MIN_PATCH}/uniflash_sl.{VERSION_MAJ_MIN_PATCH}.{VERSION_BUILD_METADATA}.{ext}"
    );

    let tempdir = TempDir::new()?;
    let installer = tempdir.path().join(format!("uniflash.{ext}"));

    let t0 = Instant::now();
    download_file(Url::parse(&url).unwrap(), &installer)?;

    let mut pb = Progress::spinner("Verifying");
    if !md5_verify(&installer, &md5)? {
        anyhow::bail!("md5 mismatch in '{}'", installer.display());
    }

    pb.set_message("Installing");
    do_install(&installer)?;

    let elapsed = t0.elapsed();
    let elapsed = if elapsed < Duration::from_secs(60) {
        format!("{:0.2}s", elapsed.as_secs_f32())
    } else {
        format!("{}m {}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
    };

    pb.finish("Finished", &format!("installed ti uniflash in {elapsed}"));

    Ok(())
}

#[cfg(target_os = "macos")]
fn do_install(dmg: &Path) -> anyhow::Result<()> {
    let mount_dir = Path::new("/Volumes/UniFlash");

    let script = mount_dir.join(format!(
        "uniflash_sl.{VERSION_MAJ_MIN_PATCH}.{VERSION_BUILD_METADATA}.app/\
            Contents/MacOS/installbuilder.sh"
    ));
    let args = ["--mode", "unattended", "--prefix", "/Applications/TI/UniFlash"];

    cmd!("hdiutil", "attach", dmg).stdout_null().run()?;
    let result = cmd(script, args).run();
    cmd!("hdiutil", "detach", "/Volumes/UniFlash").stdout_null().run()?;

    result.map(|_| ()).map_err(Into::into)
}

#[cfg(target_os = "linux")]
fn do_install(run: &Path) -> anyhow::Result<()> {
    cmd!("chmod", "+x", run).run()?;
    cmd!(run, "--mode", "unattended", "--prefix", "/opt/ti/uniflash").run()?;
    Ok(())
}

#[cfg(windows)]
fn do_install(exe: &Path) -> anyhow::Result<()> {
    let install_dir = data_local_dir().unwrap().join("TI/UniFlash");
    cmd!(exe, "--mode", "unattended", "--prefix", &install_dir).run()?;
    Ok(())
}
