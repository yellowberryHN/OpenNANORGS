use std::collections::HashMap;
use crate::parser::ParserToken;

pub struct SymbolTable {
    pub label_to_address: HashMap<String, u16>,
    position: u16,
}

impl SymbolTable {
    pub fn new(ast: &Vec<ParserToken>) -> SymbolTable {
        let mut table = SymbolTable {
            label_to_address: HashMap::new(),
            position: 0,
        };

        table.generate(ast);

        table
    }

    fn add_label(&mut self, label: &String, position: u16) {
        if self.label_to_address.contains_key(&label.to_lowercase()) {
            todo!()
        }
        else {
            self.label_to_address.insert(label.to_lowercase(), position);
        }
    }

    fn generate(&mut self, ast: &Vec<ParserToken>) {
        for i in 0..ast.len() {
            let node = &ast[i];

            match node {
                ParserToken::Label(label) => {
                    if self.position % 3 != 0 {
                        match &ast[i + 1] {
                            ParserToken::Instruction(_) => {
                                self.position += (3 - self.position % 3)
                            }
                            _ => {}
                        }
                    }

                    self.add_label(label, self.position);
                }
                ParserToken::Instruction(instruction) => {
                    // realign addresses to 3 word border
                    if self.position % 3 != 0 {
                        self.position += (3 - self.position % 3);
                    }

                    // every operation takes up 3 words of space
                    self.position += 3;
                }
                ParserToken::Data(data) => {
                    for _ in data {
                        self.position += 1;
                    }

                }
                _ => {},
            }
        }
    }
}