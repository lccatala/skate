mod dom;
use std::{collections::HashMap};

use crate::dom::{elem, text, comment};

fn main() {
    let mut body_attrs = HashMap::new();
    body_attrs.insert("class".to_string(), "container".to_string());

    let dom_tree = elem(
        "html".to_string(),
        HashMap::new(),
        vec![
            elem(
                "head".to_string(),
                HashMap::new(),
                vec![
                    elem(
                        "title".to_string(),
                        HashMap::new(),
                        vec![text("My Page".to_string())]
                    )
                ]
            ),
            elem(
                "body".to_string(),
                body_attrs,
                vec![
                    comment("This is a comment".to_string()),
                    elem(
                        "h1".to_string(),
                        HashMap::new(),
                        vec![text("Hello, world!".to_string())]
                    ),
                    elem(
                        "p".to_string(),
                        HashMap::new(),
                        vec![text("This is a paragraph".to_string())]
                    ),
                ]
            ),
        ]
    );
    println!("Dom tree:");
    println!();
    dom_tree.prettty_print();
}
