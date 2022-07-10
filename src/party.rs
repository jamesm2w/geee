use std::fmt;

#[derive(Debug)]
pub enum Party {
    Con, Lab, LD, Grn, Ind, Oth
}

impl fmt::Display for Party {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Party::Con => write!(f, "Con"),
            Party::Lab => write!(f, "Lab"),
            Party::LD  => write!(f, "LD"),
            Party::Grn => write!(f, "Grn"),
            Party::Ind => write!(f, "Ind"),
            Party::Oth => write!(f, "Oth"),
        }
    }
}

impl Party {
    pub fn parse(str: &str) -> Party {
        match str {
            "Con" => Party::Con,
            "Lab" => Party::Lab,
            "LD" => Party::LD,
            "Grn" => Party::Grn,
            "Ind" => Party::Ind,
            _ => Party::Oth
        }
    }

    pub fn colour(&self) -> &'static str {
        match *self {
            Party::Con => "blue",
            Party::Lab => "red",
            Party::LD  => "orange",
            Party::Grn => "green",
            Party::Ind => "gray",
            Party::Oth => "black"
        }
    }
}