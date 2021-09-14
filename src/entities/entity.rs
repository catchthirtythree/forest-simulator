use std::fmt;

#[derive(Debug)]
pub enum EntityType {
    Bear,
    Lumberjack,
    Tree
}

pub trait Entity {
    fn get_entity_type(&self) -> EntityType;
    fn get_position(&self) -> usize;
    fn get_symbol(&self) -> &str;
}

impl fmt::Debug for dyn Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.get_entity_type())
    }
}
