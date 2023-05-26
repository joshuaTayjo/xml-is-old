use std::{collections::HashMap, fs, iter, rc::Weak, str::Chars};
type Attrributes = HashMap<String, String>;

pub struct XMLTree {
    version: String,
    encoding: String,
    root: XMLNode,
}

pub struct XMLNode {
    node_type: NodeType,
    children: Vec<XMLNode>,
    parent: Weak<XMLNode>,
}
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

pub struct ElementData {
    tag_name: String,
    attributes: Attrributes,
}

pub enum NodeContents {
    NodeType,
    String,
}

impl Default for XMLNode {
    fn default() -> Self {
        XMLNode {
            node_type: NodeType::Element(ElementData {
                tag_name: String::from("Default"),
                attributes: Attrributes::new(),
            }),
            children: Vec::new(),
            parent: Weak::new(),
        }
    }
}

impl Default for XMLTree {
    fn default() -> Self {
        XMLTree {
            version: String::from("1.0"),
            encoding: String::from("UTF-8"),
            root: XMLNode {
                node_type: NodeType::Element(ElementData {
                    tag_name: String::from("root"),
                    attributes: Attrributes::new(),
                }),
                children: Vec::new(),
                parent: Weak::new(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeError(char);

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tag should begin and end with < and > characters. Got {}",
            self.0
        )
    }
}

pub fn parse_tag(itr: &mut iter::Peekable<Chars>) -> Result<XMLNode, NodeError> {
    let mut tag = String::new();
    if *itr.peek().unwrap() != '<' {
        return Err(NodeError(*itr.peek().unwrap()));
    }
    while *itr.peek().unwrap() != '>' {
        tag.push(itr.next().unwrap());
    }
    tag.push(itr.next().unwrap());
    println!("{}", tag);
    let mut new_node = XMLNode {
        ..Default::default()
    };
    new_node.node_type = NodeType::Element(ElementData {
        tag_name: tag[1..3].to_string(),
        attributes: Attrributes::new(),
    });

    Ok(XMLNode {
        ..Default::default()
    })
}

pub fn deserialize(file_path: &str) -> XMLTree {
    let res = fs::read_to_string(file_path).expect("The requested file doesn't exist!");
    println!("{}", res);

    let mut chars = res.chars().peekable();
    while chars.peek().unwrap_or(&'c').is_whitespace() {
        chars.next();
    }
    parse_tag(&mut chars);
    println!("{}", chars.next().unwrap());
    XMLTree {
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_file() {
        deserialize("src/test_files/test_file1.xml");
    }
}
