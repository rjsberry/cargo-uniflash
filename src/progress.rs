use crate::common::*;

pub struct Progress {
    t0: Instant,
    pb: ProgressBar,
}

impl Progress {
    pub fn new(msg: &str, len: u64) -> Self {
        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::with_template(
                "{msg:.cyan.bold} [{bar}] {bytes} / {total_bytes} ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("=> "),
        );
        pb.set_message(format!("{msg:>12}"));

        Self { t0: Instant::now(), pb }
    }

    pub fn spinner(msg: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(200));
        pb.set_style(
            ProgressStyle::with_template("{msg:.cyan.bold} {spinner}")
                .unwrap()
                .tick_strings(&["   ", ".  ", ".. ", "...", " ..", "  .", "   "]),
        );
        pb.set_message(format!("{msg:>12}"));

        Self { t0: Instant::now(), pb }
    }

    pub fn set_message(&mut self, msg: &str) {
        self.pb.set_message(format!("{msg:>12}"));
    }

    pub fn set_position(&mut self, pos: u64) {
        self.pb.set_position(pos);
    }

    pub fn finish(&mut self, msg: &str, extra: &str) {
        let msg = console::Style::new().green().bold().apply_to(msg);
        self.pb.println(format!("{msg:>12} {extra}"));
        self.pb.finish_and_clear();
    }

    pub fn forget(&mut self) {
        self.pb.finish_and_clear();
    }

    pub fn elapsed(&self) -> Duration {
        self.t0.elapsed()
    }
}
