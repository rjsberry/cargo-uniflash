use crate::common::*;

#[allow(clippy::vec_init_then_push)]
pub fn ccxml(chip: Chip, connection: Connection, drivers: &[Driver]) -> String {
    let connection_instance = format!(
        concat!(
            r#"    <instance XML_version="1.2" "#,
            r#"desc="{desc}" href="connections/{xml}" id="{desc}" xml="{xml}" "#,
            r#"xmlpath="connections"/>"#
        ),
        desc = connection.description(),
        xml = connection.xml(),
    );

    let connection = format!(
        r#"    <connection XML_version="1.2" id="{desc}">"#,
        desc = connection.description(),
    );

    let platform_instance = format!(
        concat!(
            r#"        <instance XML_version="1.2" "#,
            r#"desc="{desc}" href="devices/{xml}" id="{desc}" xml="{xml}" "#,
            r#"xmlpath="devices"/>"#,
        ),
        desc = chip.description(),
        xml = chip.xml(),
    );

    let driver_instances: Vec<_> = drivers
        .iter()
        .map(|driver| {
            format!(
                concat!(
                    r#"      <instance XML_version="1.2" "#,
                    r#"href="drivers/{xml}" id="drivers" xml="{xml}" "#,
                    r#"xmlpath="drivers"/>"#,
                ),
                xml = driver.xml(),
            )
        })
        .collect();

    let mut lines = Vec::new();
    lines.push(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#);
    lines.push(r#"<configurations XML_version="1.2" id="configurations_0">"#);
    lines.push(r#"  <configuration XML_version="1.2" id="configuration_0">"#);
    lines.push(&connection_instance);
    lines.push(&connection);
    lines.extend(driver_instances.iter().map(|s| &**s));
    lines.push(r#"      <platform XML_version="1.2" id="platform_0">"#);
    lines.push(&platform_instance);
    lines.push(r#"      </platform>"#);
    lines.push(r#"    </connection>"#);
    lines.push(r#"  </configuration>"#);
    lines.push(r#"</configurations>"#);
    lines.join("\n")
}
