use crate::common::*;

pub fn confirm_install() -> anyhow::Result<bool> {
    const URL: &str = "https://www.ti.com/tool/UNIFLASH";

    #[cfg(unix)]
    const BIN: &str = "dslite.sh";
    #[cfg(windows)]
    const BIN: &str = "dslite.bat";

    #[cfg(target_os = "macos")]
    fn print_suggestion() {
        println!(
            "Ok, to install it yourself run `brew install uniflash` or visit \"{URL}\" and ensure \
            `{BIN}` is on your PATH."
        );
    }

    #[cfg(not(target_os = "macos"))]
    fn print_suggestion() {
        println!("Ok, to install it yourself visit \"{URL}\" and ensure `{BIN}` is on your PATH.");
    }

    if Confirm::new()
        .with_prompt("TI UniFlash does not appear to be installed, want me to install it for you?")
        .interact()?
    {
        Term::stdout().clear_last_lines(1)?;
        Ok(true)
    } else {
        print_suggestion();
        Ok(false)
    }
}
