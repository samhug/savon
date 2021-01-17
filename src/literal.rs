use crate::rpser::xml::BuildElement;
use crate::{gen, Error, rpser};

//请参参数
#[derive(Clone, Debug, Default)]
pub struct LiteralRequest {
    pub args: Option<Vec<String>>
}

impl gen::ToElements for LiteralRequest {
    fn to_elements(&self) -> Vec<xmltree::Element> {
        let mut v = vec![];
        if (self.args.as_ref().is_some()) {
            for (i, n) in self.args.as_ref().unwrap().iter().enumerate() {
                v.push(xmltree::Element::node(format!("arg{}", i)).with_text(n));
            }
        }
        return v;
    }
}

//响应结果
#[derive(Clone, Debug, Default)]
pub struct LiteralResponse {
    pub result: String
}

impl gen::FromElement for LiteralResponse {
    fn from_element(element: &xmltree::Element) -> Result<Self, Error> {
        Ok(LiteralResponse {
            result: element.get_at_path(&["return"]).and_then(|e| e.get_text().map(|s| s.to_string()).ok_or(rpser::xml::Error::Empty))?
        })
    }
}
