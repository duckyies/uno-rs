use std::cmp::Ordering;
use std::fmt::Display;
pub struct Card {
    pub(crate) id: String,
    wild: bool,
    pub(crate) color: String,
}

impl Card {
    pub(crate) fn new(id: String, color : &str) -> Self {
        let wild: bool = {
            color == "wild" || color == ""
        };
        let colour = String::from(color);
        Self {
            id,
            color: colour,
            wild
        }
    }
    fn get_color_name(&self) -> &str {
        match self.color.as_str() {
            "R" => "Red",
            "G" => "Green",
            "B" => "Blue",
            "Y" => "Yellow",
            _ => ""
        }
    }
    fn get_color_code(&self) -> i32 {
        match self.color.as_str() {
            "R" => 0xff5555,
            "G" => 0x55aa55,
            "B" => 0x5555ff,
            "Y" => 0xffaa00,
            _ => 0x080808
        }
    }

    fn get_value(&self) -> i32 {
        let mut val: i32 = 0;
        match self.color.as_str() {
            "R" => val += 100000,
            "G" => val += 1000,
            "B" => val += 100,
            "Y" => val += 10000,
            _ => val += 1000000
        };
        match self.id.as_str() {
            "SKIP" => val += 10,
            "REVERSE" => val += 11,
            "+2" => val += 12,
            "WILD" => val += 13,
            "WILD+4" => val += 14,
            _ => {
                let card_num: i32 = self.id.as_str().parse().unwrap();
                val += card_num
            }
        };
        val
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.wild {
            write!(f, "{}", self.id)
        }
        else {
            let color_name = self.get_color_name();
            write!(f, "{} {}", color_name, self.id)
        }
    }
}

impl Eq for Card {

}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}