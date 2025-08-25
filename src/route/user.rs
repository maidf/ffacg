
use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use validator::Validate;

use crate::{model::user::User, service};

fn vali_user(payload: User) -> Result<(String, String), String> {
    
    if let Err(e) = payload.validate() {
        return Err(e.to_string());
    }

    let account = match payload.account {
        Some(acc) => acc,
        None => return Err("参数错误".to_string()),
    };
    let pass = match payload.pass {
        Some(pass) => pass,
        None => return Err("参数错误".to_string()),
    };
    
    Ok((account, pass))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    
    let (account, pass) = match vali_user(payload) {
        Ok((acc, pass)) => (acc, pass),
        Err(e) => return Err((StatusCode::from_u16(255).unwrap(), e)),
    };
    
    
    let res = service::user::create_user(&pool, account, pass).await;
    match res {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            let mut msg = String::from("err: ");
            msg.push_str(e.to_string().as_str());
            Err((StatusCode::from_u16(355).unwrap(), msg))
        }
    }
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<User>,
) -> Result<Json<()>, (StatusCode, String)> {
    let (account, pass) = match vali_user(payload) {
        Ok((acc, pass)) => (acc, pass),
        Err(e) => return Err((StatusCode::from_u16(255).unwrap(), e)),
    };

    let res = service::user::login(&pool, account, pass).await;
    match res {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            let mut msg = String::from("err: ");
            msg.push_str(e.to_string().as_str());
            Err((StatusCode::from_u16(355).unwrap(), msg))
        }
    }
}
