use crate::{
    chip,
    chip::Interrupt,
    util,
    ElementExt,
};

pub fn parse_list(ints: &xmltree::Element) -> crate::Result<Vec<chip::Interrupt>> {
    let mut interrupts = vec![];

    for int in ints.children.iter() {
        int.check_name("interrupt")?;

        let name = int.attr("name")?;
        let index = util::parse_int(int.attr("index")?)?;
        let desc = int.attr("caption").ok().map(|x| x.clone());

        interrupts.push(Interrupt {
            name: name.clone(),
            description: desc,
            index,
        })
    }

    Ok(interrupts)
}
