use super::{format_xml_string, Action};

pub struct Dial {
    pub number: String,
}

impl Action for Dial {
    fn as_twiml(&self) -> String {
        format_xml_string("Dial", &vec![], &self.number)
    }
}
