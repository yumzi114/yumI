use time::{OffsetDateTime,Duration};
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey,errors};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    aud: String,
    permission:String,
    // sub: String,
    // company: String,
    #[serde(with = "jwt_numeric_date")]
    iat: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    exp: OffsetDateTime,
}
impl Claims {
    fn new(aud:&str, permission:&str,iat: OffsetDateTime, exp: OffsetDateTime)->Self{
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();
        let iss = String::from("yumi.town");
        let aud = aud.to_string();
        let permission = permission.to_string();

        Self {
            iss,aud,permission,iat,exp
        }
    }
}
mod jwt_numeric_date {
    //! Custom serialization of OffsetDateTime to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    /// Serializes an OffsetDateTime to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

pub fn get_token(user_id:&str, user_permission:&str)->Result<String, errors::Error>{
    dotenv().ok();
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::hours(3);
    // let valid = password_check(payload.psw.clone(),db_hash);
    let my_claims = Claims::new(user_id,user_permission,iat,exp);
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(env::var("TOKEN_SECRET").unwrap().as_ref()))?;
    // println!("serialized token: {}", &token);
    // let token_data = jsonwebtoken::decode::<Claims>(
    //     &token,
    //     &DecodingKey::from_secret("secret".as_ref()),
    //     &Validation::new(Algorithm::HS256),
    // )?;
    // println!("token data:\n{:#?}", &token_data);
    Ok(token)
}