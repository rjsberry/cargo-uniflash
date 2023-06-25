use crate::common::*;

pub fn read_until_matches(
    i: impl Iterator<Item = io::Result<String>>,
    predicate: impl Fn(&str) -> bool,
) -> anyhow::Result<()> {
    for line in i {
        if predicate(line?.trim_start()) {
            return Ok(());
        }
    }
    unreachable!("bug: exhausted iterator before predicate match")
}
