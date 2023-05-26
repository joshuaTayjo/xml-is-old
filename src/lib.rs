use std::{collections::HashMap, fs, iter, rc::Weak, str::Chars};
pub type Attributes = HashMap<String, String>;

pub struct XMLTree {
    version: String,
    encoding: String,
    root: XMLNode,
}

#[derive(Debug)]
pub struct XMLNode {
    node_contents: NodeType,
    children: Vec<XMLNode>,
    parent: Weak<XMLNode>,
}
#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: Attributes,
}

impl Default for XMLNode {
    fn default() -> Self {
        XMLNode {
            node_contents: NodeType::Element(ElementData {
                tag_name: String::from("Default"),
                attributes: Attributes::new(),
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
                node_contents: NodeType::Element(ElementData {
                    tag_name: String::from("root"),
                    attributes: Attributes::new(),
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

    let mut inner_tag = tag[1..tag.len() - 1].trim().chars();
    let tag_name: String = inner_tag
        .by_ref()
        .take_while(|x| !x.is_whitespace())
        .collect();
    let binding = inner_tag.collect::<String>();
    let attributes: Vec<(String, String)> = binding
        .split_whitespace()
        .map(|elt| {
            let mut attr = elt.split("=");
            (
                attr.next().unwrap().to_string(),
                attr.next().unwrap().to_string(),
            )
        })
        .collect();
    let mut tag_attributes = Attributes::new();
    attributes.into_iter().for_each(|attr| {
        tag_attributes.insert(attr.0, attr.1);
    });

    println!("{}", tag_name);
    println!("{:#?}", tag_attributes);

    let new_node = XMLNode {
        node_contents: NodeType::Element(ElementData {
            tag_name,
            attributes: tag_attributes,
        }),
        ..Default::default()
    };
    println!("{:#?}", new_node);

    Ok(new_node)
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
