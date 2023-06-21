use crate::{Error,};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};
use sqlx::{postgres::PgPoolOptions,Row};

#[derive(Debug, Deserialize,Serialize)]
pub struct UserInfo{
    pub id:i32,
    pub email:String,
    pub role_id:i32,
    pub result:bool
}
impl UserInfo{
    fn new (row:sqlx::postgres::PgRow, hash:String, psw:&str)->Self{
        let id =  row.get::<i32,_>("id");
        let email =  row.get::<String,_>("email");
        let role_id =  row.get::<i32,_>("role_id");
        let result = password_check(psw.to_string(),hash);
        Self { 
            id, 
            email, 
            role_id, 
            result 
        }
    }
}
pub async fn user_get(email:&str, psw:&str)->Result<UserInfo,sqlx::Error>{
    dotenv().ok();
    let pg_url =env::var("PG_CONNETION").unwrap();
    let pool = PgPoolOptions::new()
        .connect(&pg_url)
        .await?;
    let query_str = format!("SELECT * FROM users WHERE email = '{}' ", email);
    let row = sqlx::query(&query_str)
    .fetch_one(&pool).await?;
    // let test = row.try_get::<i32,_>("role_id").unwrap();
    // println!("row ????{:?}",test);
    // let hash = ?;
    if let Ok(hash)=row.try_get::<String,_>("psw"){
        let userinfo = UserInfo::new(row, hash.clone(), psw);
        // let result = password_check(psw.to_string(),hash);
        return Ok(userinfo);
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