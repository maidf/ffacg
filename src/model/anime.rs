use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Anime {
    pub id: Option<String>,
    pub title: Option<String>,
    pub size: Option<i32>,
    pub magnet: Option<String>,
    pub tracker: Option<String>,
    pub fansub: Option<Fansub>,
    pub publisher: Option<Publisher>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Fansub {
    pub id: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
}

pub type Publisher = Fansub;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct AnimeHistory {
    pub id: Option<String>,
    pub anime: Option<Anime>,
    pub time: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct AnimeMask {
    pub id: Option<String>,
    pub anime: Option<Anime>,
    pub group_name: Option<String>,
}