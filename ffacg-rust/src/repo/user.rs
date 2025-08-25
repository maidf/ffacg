use crate::model::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(
    pool: &PgPool,
    account: String,
    pass: String,
) -> Result<User, sqlx::Error> {
    if let Ok(_) = select_user_by_account(pool, &account).await {
        return Err(sqlx::Error::InvalidArgument("account registered".to_string()));
    }

    let uuid = Uuid::now_v7().to_string();
    sqlx::query!(
        "insert into t_user(id, account, name, pass) values($1, $2, $3, $4)",
        uuid,
        account.clone(),
        account,
        pass,
    )
    .execute(pool)
    .await?;

    select_user(pool, &uuid).await
}

pub async fn select_user_by_account(pool: &PgPool, account: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as("select * from t_user where account=$1").bind(account).fetch_one(pool).await
}

pub async fn select_user(pool: &PgPool, id: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as("select * from t_user where id=$1").bind(id).fetch_one(pool).await
}
