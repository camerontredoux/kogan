use std::{fmt, str::FromStr};

use serenity::{
    builder::{CreateActionRow, CreateButton},
    model::interactions::message_component::ButtonStyle,
};

use crate::components::ParseComponentError;

use super::animal::Animal;

#[derive(Debug)]
pub enum Sound {
    Meow,
    Bark,
    Neigh,
    Moo,
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sound::Meow => write!(f, "meow!"),
            Sound::Bark => write!(f, "ruff ruff!"),
            Sound::Neigh => write!(f, "neighhh!"),
            Sound::Moo => write!(f, "moooo!"),
        }
    }
}

impl FromStr for Sound {
    type Err = ParseComponentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "meow!" => Ok(Sound::Meow),
            "ruff ruff!" => Ok(Sound::Bark),
            "neighhh!" => Ok(Sound::Neigh),
            "moooo!" => Ok(Sound::Moo),
            _ => Err(ParseComponentError(s.into())),
        }
    }
}

impl Sound {
    pub fn emoji(&self) -> char {
        match self {
            Sound::Meow => Animal::Cat.emoji(),
            Sound::Bark => Animal::Dog.emoji(),
            Sound::Neigh => Animal::Horse.emoji(),
            Sound::Moo => Animal::Alpaca.emoji(),
        }
    }

    fn button(&self) -> CreateButton {
        let mut b = CreateButton::default();
        b.custom_id(self.to_string().to_ascii_lowercase());
        b.emoji(self.emoji());
        b.label(self);
        b.style(ButtonStyle::Primary);
        b
    }

    pub fn action_row() -> CreateActionRow {
        let mut ar = CreateActionRow::default();
        ar.add_button(Sound::Meow.button());
        ar.add_button(Sound::Bark.button());
        ar.add_button(Sound::Neigh.button());
        ar.add_button(Sound::Moo.button());
        ar
    }
}
