use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Effect {
    pub name: String,
    pub effect: String,
    pub dur: Option<u8>,
    pub icon: Option<String>,
    pub animation: Option<String>,
    pub display: String,
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match self.dur {
                Some(d) => write!(f, "Name: {}, Duration: {}", self.name, d),
                _ => write!(f, "Name: {}, Duration: 0", self.name)
            }
        } else {
            match self.dur {
                Some(d) => write!(f, "{}, {}", self.name, d),
                _ => write!(f, "{}, 0", self.name)
            }
        }
    }
}

impl Effect {
    pub fn new_effect(name: &str, effect: &str, dur: Option<u8>, icon: Option<String>, animation: Option<String>, display: &str) -> Self {
        Self {
            name: name.to_string(),
            effect: effect.to_string(),
            dur,
            icon,
            animation,
            display: display.to_string(),
        }
    }
}
