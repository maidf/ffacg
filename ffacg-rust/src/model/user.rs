use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationError};

use crate::model::anime::Anime;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, Validate)]
pub struct User {
    pub id: Option<String>,
    #[validate(length(min = 3, max = 36))]
    pub account: Option<String>,
    #[validate(length(min = 1, max = 36))]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    #[validate(length(min = 6), custom(function = vali_pass))]
    pub pass: Option<String>,
    pub sex: Option<String>,
    pub birth: Option<i64>,
    pub signature: Option<String>,
    #[validate(range(min = 0, max = 6))]
    pub level: Option<i32>,
    #[validate(range(min = 0))]
    pub exp: Option<i32>,
    pub status: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Profile {
    pub id: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Security {
    pub id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub idc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Medal {
    pub id: Option<String>,
    pub name: Option<String>,
    pub level: Option<i32>,
    pub exp: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct UserDyn {
    pub id: Option<String>,
    pub title: Option<String>,
    pub time: Option<i64>,
    pub msg: Option<String>,
    pub anime: Option<Anime>,
}

fn vali_pass(pass: &str) -> Result<(), ValidationError> {
    let has_alpha = pass.chars().any(|c| c.is_alphabetic());
    let has_digit = pass.chars().any(|c| c.is_ascii_digit());
    let has_symbol = pass.chars().any(|c| !c.is_alphanumeric() && !c.is_whitespace());

    if [has_alpha, has_digit, has_symbol].iter().filter(|&&b| b).count() < 2 {
        return Err(ValidationError::new("must_have_two_categories"));
    }
    Ok(())
}
