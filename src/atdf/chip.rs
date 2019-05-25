use crate::atdf;
use crate::chip;
use crate::ElementExt;

pub fn parse(el: &xmltree::Element) -> crate::Result<chip::Chip> {
    let devices = el.first_child("devices")?;
    if devices.children.len() != 1 {
        return Err(
            atdf::error::UnsupportedError::new("more than one device definition", devices).into(),
        );
    }

    let device = devices.first_child("device")?;

    let peripherals = atdf::peripheral::parse_list(
        device.first_child("peripherals")?,
        el.first_child("modules")?,
    )?
    .into_iter()
    .map(|p| (p.name.clone(), p))
    .collect();

    let interrupts = atdf::interrupt::parse_list(device.first_child("interrupts")?)?
        .into_iter()
        .map(|int| (int.name.clone(), int))
        .collect();

    Ok(chip::Chip {
        name: device.attr("name")?.clone(),
        description: None,
        vendor: None,
        version: None,
        peripherals,
        interrupts,
    })
}
