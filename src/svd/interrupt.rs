use crate::chip;

pub fn generate(peripherals: &mut xmltree::Element, c: &chip::Chip) {
    // Find the (first) peripheral with the name `CPU` to then add the interrupts to it.
    for peripheral in peripherals.children.iter_mut() {
        if Some("CPU")
            == peripheral.get_child("name".to_string()).map_or_else(
                || None,
                |name| match &name.text {
                    Some(t) => Some(t.as_str()),
                    _ => None,
                },
            )
        {
            c.interrupts.values().for_each(|int| {
                let mut el = xmltree::Element::new("interrupt");
                el.attributes.insert("name".to_string(), int.name.clone());
                el.attributes
                    .insert("value".to_string(), int.index.to_string());
                if let Some(ref desc) = int.description {
                    el.attributes
                        .insert("description".to_string(), desc.clone());
                }

                peripheral.children.push(el);
            });

            break;
        }
    }
}
