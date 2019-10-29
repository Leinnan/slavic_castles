use crate::consts;
use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub enum ResourceType {
    Tools,
    Magic,
    Soldiers,
}

pub struct Resource {
    pub amount: i32,
    pub production: i32,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            amount: consts::BASE_RESOURCE_AMOUNT,
            production: consts::BASE_RESOURCE_PRODUCTION,
        }
    }

    pub fn produce(&mut self) {
        self.amount += self.production;
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},({})", self.amount, self.production)
    }
}
