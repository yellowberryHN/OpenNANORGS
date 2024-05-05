#![allow(dead_code)]

use std::ops::Deref;

#[derive(Debug)]
enum ItemType {
    Sludge,
    CollectionPoint
}
#[derive(Debug)]
struct Item {
    id: u16,
    position: Position,
    item_type: ItemType,
}

#[derive(Debug)]
struct Tank {
    bots: Vec<Bot>,
    bounds: Position,
    score: u64,

    seed: u64,

    sludge_amount: u8,
    toxic_sludge: Vec<u8>,

    elements: Vec<Option<Item>>
}

impl Tank {
    fn new(bounds: Position, seed: u64) -> Tank {
        let mut tank = Tank {
            bots: vec![],
            score: 0,
            sludge_amount: 0,
            toxic_sludge: vec![],
            elements: vec![],
            bounds,
            seed,
        };
        tank.bots = Self::create_bots(&tank);
        tank
    }

    fn create_bots(&self) -> Vec<Bot> {
        let mut bots: Vec<Bot> = vec![];
        for n in 1..=100 {
            bots.push(Bot::new(n.to_string(), n as u8 as char, Self::find_empty(self)));
        }
        bots
    }

    fn check_occupied(&self, pos: Position) -> bool {
        return false;
    }

    fn find_empty(&self) -> Position {
        let mut pos_list = vec![];
        for ele in &self.elements {
            match ele {
                None => {}
                Some(item) => {
                    pos_list.push(&item.position);
                }
            }
        }
        pos_list.pop().unwrap()
    }
}

#[derive(Debug)]
struct Bot {
    name: String,
    glyph: char,
    //cpu: CPU;
    position: Position,
    energy: u16,
    sleeping: bool,
}

impl Bot {
    fn new(name: String, glyph: char, position: Position) -> Bot {
        Bot {
            name,
            glyph,
            position,
            energy: 10000,
            sleeping: false,
        }
    }


}

#[derive(Debug)]
struct Position {
    x: u8,
    y: u8,
    z: u8, // depth
}



fn main() {
    let mut test = Bot::new("hhh".to_string(), 'h', Position { x: 0, y: 0, z: 0 });
    println!("{test:?}");
    let mut test2 = Tank::new(Position { x: 0, y: 0, z: 0 }, 69420);
    println!("{test2:?}");
    println!("Hello, world!");
}
