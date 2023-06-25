use crate::common::*;

pub fn warn_experimental_platforms() {
    let (platform, issue) = if cfg!(target_os = "linux") {
        ("Linux", "https://github.com/rjsberry/cargo-uniflash/issues/1")
    } else if cfg!(windows) {
        ("Windows", "https://github.com/rjsberry/cargo-uniflash/issues/2")
    } else {
        return;
    };

    let warning = format!(
        "{}{}",
        Style::new().bold().yellow().apply_to("warning"),
        Style::new().bold().apply_to(":")
    );

    eprintln!(
        "{warning} {platform} is currently an untested platform and `cargo-uniflash` might not \
        work correctly.\n\
        {warning} Please report any success/issues you have to: {issue}. Thanks!"
    );

    // small delay to notice the message
    thread::sleep(Duration::from_secs(2));
}
