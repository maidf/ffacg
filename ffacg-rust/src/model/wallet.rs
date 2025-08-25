use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Wallet {
    pub id: Option<String>,
    pub coin: Coin,
    pub gold: Gold,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Hash, FromRow)]
pub struct Coin(Decimal);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Hash, FromRow)]
pub struct Gold(Decimal);

pub fn gold_to_coin(gold: Gold) -> Coin {
    Coin(gold.0 * dec!(20))
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Currency {
    Coin,
    Gold,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct WalletRecord {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<Currency>,
    pub time: Option<i64>,
    pub num: Option<Decimal>,
}
