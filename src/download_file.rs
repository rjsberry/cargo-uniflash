use crate::common::*;

/// Downloads a file into the specified directory.
pub fn download_file(url: Url, path: &Path) -> anyhow::Result<()> {
    let mut resp = reqwest::blocking::get(url)?;
    let len = resp.content_length().unwrap();

    let b = Barrier::new(2);
    let mut pb = Progress::new("Downloading", len);

    thread::scope(|s| -> anyhow::Result<()> {
        let h = s.spawn(|| -> anyhow::Result<()> {
            let mut f = File::create(path)?;
            b.wait();
            resp.copy_to(&mut f)?;
            Ok(())
        });

        b.wait();
        let f = File::open(path)?;

        while !h.is_finished() {
            pb.set_position(f.metadata()?.len());
            thread::sleep(Duration::from_millis(200));
        }

        Ok(())
    })
}
