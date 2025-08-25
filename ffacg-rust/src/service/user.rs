use base64::{Engine as _, engine::general_purpose::STANDARD};
use sha3::{Digest, Sha3_256};
use sqlx::PgPool;

use crate::{model::user::User, repo};

pub async fn create_user(
    pool: &PgPool,
    account: String,
    pass: String,
) -> Result<User, sqlx::Error> {
    let mut hasher = Sha3_256::new();
    hasher.update(pass);
    let hash = hasher.finalize();
    let pass = STANDARD.encode(hash);
    repo::user::create_user(pool, account, pass).await
}

pub async fn login(pool: &PgPool, account: String, pass: String) -> Result<(), sqlx::Error> {
    let mut hasher = Sha3_256::new();
    hasher.update(pass);
    let hash = hasher.finalize();
    let pass = STANDARD.encode(hash);

    let user = match repo::user::select_user_by_account(pool, &account).await {
        Ok(u) => u,
        _ => {
            return Err(sqlx::Error::InvalidArgument(
                "account not found".to_string(),
            ));
        }
    };

    user.pass
        .unwrap()
        .eq(&pass)
        .then(|| Ok(()))
        .or_else(|| Some(Err(sqlx::Error::InvalidArgument("pass err".to_string()))))
        .unwrap()
}
