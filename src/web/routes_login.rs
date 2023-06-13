use crate::{Error, Result,yumdb::users::user_get, web,token,token::web_token::get_token};
use axum::{Json, routing::{Route, post}, Router};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use jsonwebtoken::errors;
use tower_cookies::{Cookies, Cookie};
use std::env;
use dotenv::dotenv;
// use sha2::{Sha256, Digest};
// use base64ct::{Base64, Encoding};



#[derive(Debug, Deserialize)]
struct LoginPayload{
    email: String,
    psw: String,
}

async fn api_login(cookies:Cookies,payload: Json<LoginPayload>)->Result<Json<Value>>{
    dotenv().ok();
    // let default_cost =env::var("HASH_CONST").unwrap().parse().unwrap();
    let temp= payload.email.as_str();
    // let mut db_hash = String::new();
    let mut valid = false;
    println!("->> {:<12} - api_login","HANDLER");
    if let Ok(data)=user_get(temp,&payload.psw).await{
        valid=data;
        if !valid {
            return  Err(Error::LoginFail);
        }
    }else{
        return Err(Error::NotFoundUser);
    }
    let token = get_token("1","permi");
    if let Err(_)=token{
        return Err(Error::TokenError);
    }
    cookies.add(Cookie::new(token::AUTH_TOKEN, token.unwrap()));
    let body = Json(json!({
        "result":{
            "email":&payload.email,
            "success":&valid
        }
    }));
    Ok(body) 
}
// fn hash_password<D: Digest>(password: &str, salt: &str, output: &mut [u8]) {
//     let mut hasher = D::new();
//     hasher.update(password.as_bytes());
//     hasher.update(b"$");
//     hasher.update(salt.as_bytes());
//     output.copy_from_slice(&hasher.finalize())
// }
pub fn routes()->Router{
    Router::new().route("/api/login", post(api_login))
}