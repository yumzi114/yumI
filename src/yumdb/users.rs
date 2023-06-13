use crate::{Error,};
use dotenv::dotenv;
use std::env;
use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};
use sqlx::{postgres::PgPoolOptions,Row};
struct UserInfo{
    user_id:i32,
    permission:i32,
}
pub async fn user_get(email:&str, psw:&str)->Result<bool,sqlx::Error>{
    dotenv().ok();
    let pg_url =env::var("PG_CONNETION").unwrap();
    let pool = PgPoolOptions::new()
        .connect(&pg_url)
        .await
        .unwrap();
    let query_str = format!("SELECT * FROM users WHERE email = '{}' ", email);
    let row = sqlx::query(&query_str)
    .fetch_one(&pool).await?;
    // println!("row ????{:?}",row.columns());
    // let hash = ?;
    if let Ok(hash)=row.try_get::<String,_>("psw"){
        let result = password_check(psw.to_string(),hash);
        return Ok(result);
    } else {
        return Err(sqlx::Error::RowNotFound);
    }
    // Ok(hash)
}

fn password_check(psw:String, db_hash:String)->bool{
    let hash_str =env::var("HASH_STRING").unwrap();
    //hash
    let mut hasher = Sha256::new();
    hasher.update(psw.as_bytes());
    hasher.update(hash_str);
    let hash = hasher.finalize();
    let base64_hash = Base64::encode_string(&hash);
    if db_hash == base64_hash{
        true
    }else{
        false
    }

    //HMAC
    // let mut buf = [0u8; 32];
    // hash_password::<Sha256>(&payload.psw, &hash_str, &mut buf);
    // let base64_hash = Base64::encode_string(&buf);
}