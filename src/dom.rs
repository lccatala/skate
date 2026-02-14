use std::collections::{HashMap};

pub struct Node {
    // Data common to all nodes
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
    tag_name: String,
    attrs: AttrMap,
}

type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {children: Vec::new(), node_type: NodeType::Text(data)}
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData{tag_name, attrs})
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Comment(data),
    }
}

impl Node {
    pub fn prettty_print(&self) {
        self.prettty_print_with_indent(8);
    }

    fn prettty_print_with_indent(&self, indent: usize) {
        let indent_str = "  ".repeat(indent);

        match &self.node_type {
            NodeType::Element(elem) => {
                print!("{}<{}", indent_str, elem.tag_name);

                // Print attributes
                for (name, value) in &elem.attrs {
                    print!(" {}=\"{}\"", name, value)
                }

                if self.children.is_empty() {
                    println!(" />");
                } else {
                    println!(">");

                    // Print children
                    for child in &self.children {
                        child.prettty_print_with_indent(indent+1);
                    }

                    println!("{}</{}>", indent_str, elem.tag_name);
                }
            }
            NodeType::Text(text) => {
                println!("{}\"{}\"", indent_str, text);
            }
            NodeType::Comment(comment) => {
                println!("{}<!--{} -->", indent_str, comment)
            }
        }
    }
}
