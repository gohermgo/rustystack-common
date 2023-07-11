use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Props, Eq, Deserialize, Serialize)]
pub struct GitLink {
    pub user: String,
    pub repo: String,
}
impl ToString for String {
    fn to_string(&self) -> String {
        format!("https://www.github.com/{}/{}.git", self.user, self.repo)
    }
}
impl Default for GitLink {
    fn default() -> Self {
        Self {
            user: String::from("gohermgo"),
            repo: String::from("rustystack"),
        }
    }
}

#[derive(Clone, PartialEq)]
enum CardCategory {
    Freetime,
    Proffesional,
    Academic,
}
impl ToString for CardCategory {
    fn to_string(&self) -> String {
        match self {
            CardCategory::Freetime => String::from("freetime"),
            CardCategory::Professional => String::from("professional"),
            CardCategory::Academic => String::from("academic"),
        }
    }
}

#[derive(Clone, PartialEq, Props, Eq, Deserialize, Serialize)]
pub struct Project {
    #[props(default = String::from("blank title"))]
    pub title: String,
    #[props(default = String::from("blank description"))]
    pub description: String,
    #[props(default = CardCategory::Freetime)]
    pub category: CardCategory,
    #[props(default = GitLink::default())]
    pub git: GitLink,
    #[props(default = String::from("https://www.placehold.co/400"))]
    pub image_url: String,
}
