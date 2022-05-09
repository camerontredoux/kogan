use std::{error, fmt, str::FromStr};

use serenity::builder::{CreateActionRow, CreateSelectMenu, CreateSelectMenuOption};

#[derive(Debug)]
pub enum Animal {
    Cat,
    Dog,
    Horse,
    Alpaca,
}

#[derive(Debug)]
pub struct ParseComponentError(String);

impl fmt::Display for ParseComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse {} as a component", self.0)
    }
}

impl error::Error for ParseComponentError {}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Animal::Cat => write!(f, "Cat"),
            Animal::Dog => write!(f, "Dog"),
            Animal::Horse => write!(f, "Horse"),
            Animal::Alpaca => write!(f, "Alpaca"),
        }
    }
}

impl FromStr for Animal {
    type Err = ParseComponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cat" => Ok(Animal::Cat),
            "dog" => Ok(Animal::Dog),
            "horse" => Ok(Animal::Horse),
            "alpaca" => Ok(Animal::Alpaca),
            _ => Err(ParseComponentError(s.into())),
        }
    }
}

impl Animal {
    fn emoji(&self) -> char {
        match self {
            Animal::Cat => '🐈',
            Animal::Dog => '🐕',
            Animal::Horse => '🐎',
            Animal::Alpaca => '🦙',
        }
    }

    fn menu_option(&self) -> CreateSelectMenuOption {
        let mut opt = CreateSelectMenuOption::default();

        opt.label(format!("`{}` {}", self.emoji(), self));
        opt.value(self.to_string().to_ascii_lowercase());
        opt
    }

    fn select_menu() -> CreateSelectMenu {
        let mut menu = CreateSelectMenu::default();
        menu.custom_id("animal_select");
        menu.placeholder("No animal selected");
        menu.options(|o| {
            o.add_option(Animal::Cat.menu_option())
                .add_option(Animal::Dog.menu_option())
                .add_option(Animal::Horse.menu_option())
                .add_option(Animal::Alpaca.menu_option())
        });
        menu
    }

    pub fn action_row() -> CreateActionRow {
        let mut ar = CreateActionRow::default();
        ar.add_select_menu(Animal::select_menu());
        ar
    }
}
