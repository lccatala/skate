mod dom;
mod html;
mod css;
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

    let stylesheet = css::parse("h1 {color: #ff0000; margin: 10px; }".to_string());
    println!("\nStylesheet rules: {}", stylesheet.rules.len());
    for rule in &stylesheet.rules {
        for selector in &rule.selectors {
            let css::Selector::Simple(s) = selector;
            println!("Selector: {:?}", s.tag_name);
        }
        for decl in &rule.declarations {
            println!("  Declaration: {}", decl.name);
        }
    }
}
