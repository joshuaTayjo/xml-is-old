// use regex;
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

#[derive(Debug)]
pub struct ClosingTagError(String);

impl std::fmt::Display for ClosingTagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There should be an closing tag for tag {}", self.0)
    }
}

#[derive(Debug)]
pub enum XMLError {
    NodeError(NodeError),
    ClosingTagError(ClosingTagError),
}

pub fn parse_tag(itr: &mut iter::Peekable<Chars>) -> Result<XMLNode, XMLError> {
    let mut tag = String::new();
    if *itr.peek().unwrap() != '<' {
        return Err(XMLError::NodeError(NodeError(*itr.peek().unwrap())));
    }
    while *itr.peek().unwrap() != '>' {
        tag.push(itr.next().unwrap());
    }
    tag.push(itr.next().unwrap());

    //Isolate just the data from the starting tag
    let mut inner_tag = tag[1..tag.len() - 1].trim().chars();
    let tag_name: String = inner_tag
        .by_ref()
        .take_while(|x| !x.is_whitespace())
        .collect();
    //After name is taken, rest of inner_tag should be attributes
    let binding = inner_tag.collect::<String>();
    let mut tag_attributes = Attributes::new();
    binding.split_whitespace().for_each(|elt| {
        let mut attr = elt.split("=");
        tag_attributes.insert(
            attr.next().unwrap().to_string(),
            attr.next().unwrap().to_string(),
        );
    });

    let rest_of_tag: String = itr.collect();

    let end_tag = format!("</{}>", tag_name);
    let end_tag_index = rest_of_tag.find(end_tag.as_str());

    if end_tag_index.is_none() {
        return Err(XMLError::ClosingTagError(ClosingTagError(tag_name)));
    }
    let inside_of_tag = &rest_of_tag[..end_tag_index.unwrap()];

    let new_node = XMLNode {
        node_contents: NodeType::Element(ElementData {
            tag_name,
            attributes: tag_attributes,
        }),
        children: vec![XMLNode {
            node_contents: NodeType::Text(inside_of_tag.trim().to_string()),
            ..Default::default()
        }],
        ..Default::default()
    };

    Ok(new_node)
}

pub fn deserialize(file_path: &str) -> XMLTree {
    let res = fs::read_to_string(file_path).expect("The requested file doesn't exist!");

    let mut chars = res.chars().peekable();
    while chars.peek().unwrap_or(&'c').is_whitespace() {
        chars.next();
    }
    let mut node = match parse_tag(&mut chars) {
        Ok(tag) => tag,
        Err(err) => panic!("{:#?}", err),
    };

    match &node.children[0].node_contents {
        NodeType::Element(_) => {}
        NodeType::Text(text) => {
            if text.starts_with('<') {
                let inner_tag = parse_tag(&mut text.chars().peekable());
                node.children.push(inner_tag.unwrap());
            }
        }
    }
    println!("{:#?}", node.children);

    XMLTree {
        root: node,
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
