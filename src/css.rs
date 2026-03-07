// A CSS stylesheet is a series of rules
pub struct Stylesheet {
    pub rules: Vec<Rule>
}

// A rule includes one or more selectors separated by commas,
// followed by a series of declarations enclosed in braces
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

// A simple selector can include a tag name, an ID prefixed by '#',
// any number of class names prefixed by '.', or some combination of the above.
// If the tag name is empty or '*', then it is a universal selector that can match any tag
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

// A declaration is a name/value pair, separated by a colon and ending with a semicolon.
// For example, "margin: auto;" is a declaration.
pub struct Declaration {
    pub name: String,
    pub value: Value
}

// I will only support a small subset of all the possible CSS value types
#[derive(Debug, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, Clone)]
pub enum Unit {
    Px,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8, pub g: u8, pub b: u8, pub a: u8
}

struct Parser {
    pos: usize, input: String,
}
impl Parser {
    // Parse a rule set: `<selectors> { <declarations> }`
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    // Parse a comma-separated list of selectors
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => { self.consume_char(); self.consume_whitespace(); }
                '{' => break, // Start of declarations
                c => panic!("Unexpected character {} in selector list", c)
            }
        }

        // Return selectors with highest specificity first, for use in matching
        selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
        return selectors;
    }

    // Parse a list of declarations enclosed in '{ ... }'
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        self.expect_char('{');
        let mut declarations = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declarations.push(self.parse_declaration());
        }
        return declarations;
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_identifier();
        self.consume_whitespace();
        self.expect_char(':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        self.expect_char(';');

        return Declaration {name, value};
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier())
        }
    }

    fn parse_length(&mut self) -> Value {
        return Value::Length(self.parse_float(), self.parse_unit());
    }

    fn parse_float(&mut self) -> f32 {
        return self.consume_while(|c| matches!(c, '0'..='9' | '.')).parse().unwrap();
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("Unrecognized unit")
        }
    }

    fn parse_color(&mut self) -> Value {
        self.expect_char('#');
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255
        })
    }

    // Parse two hexadecimal digits
    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos .. self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(s, 16).unwrap()
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector { tag_name: None, id: None, class: Vec::new() };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }
        return selector;
    }

    // Parse a property name or keyword.
    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }
    // Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
    // Consume characters until `test` returns false.
    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }
    // Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    // If the exact string 's' is found at the current position, consume it.
    // Otherwise, panic.
    fn expect_char(&mut self, c: char) {
        if self.consume_char() != c {
            panic!("Expected {:?} at byte {} but it was not found", c, self.pos);
        }
    }
    // Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Return true if all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

fn valid_identifier_char(c: char) -> bool {
    // TODO: include U+00A0 and higher
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}

pub fn parse(source: String) -> Stylesheet {
    let mut rules = Vec::new();
    let mut parser = Parser { pos: 0, input: source };
    while !parser.eof() {
        parser.consume_whitespace();
        if parser.eof() { break; }
        rules.push(parser.parse_rule());
    }
    Stylesheet { rules }
}

pub type Specificity = (usize, usize, usize);
impl Selector {

    // The specificity of a selector is based on its components.
    // An ID selector is more specific than a class selector, which is more specific than a tag selector.
    // Within each of these "levels," more selectors beats fewer.
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        return (a, b, c);
    }
}
