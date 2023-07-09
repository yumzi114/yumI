use clap::builder::Str;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use sqlx::{postgres::PgPoolOptions,Row,};

#[derive(Debug, Deserialize,Serialize)]
pub struct PackageManager{
    pub id:i32,
    pub manager:String,
    pub description:String,
}
impl PackageManager{
    fn new (row:sqlx::postgres::PgRow)->Self{
        let id =  row.get::<i32,_>("id");
        let manager =  row.get::<String,_>("manager");
        let description =  row.get::<String,_>("description");
        Self { 
            id, 
            manager, 
            description, 
        }
    }
}
#[derive(Debug, Deserialize,Serialize,sqlx::FromRow)]
pub struct Package{
    pub id:i32,
    pub package_name:String,
    pub description:String,
    pub manager_id:i32,
    pub manager:String,
}
impl Package{
    fn new (row:sqlx::postgres::PgRow)->Self{
        let id =  row.get::<i32,_>("id");
        let package_name =  row.get::<String,_>("package_name");
        let description =  row.get::<String,_>("description");
        let manager_id =  row.get::<i32,_>("manager_id");
        let manager =  row.get::<String,_>("manager");
        Self { 
            id, 
            package_name, 
            description, 
            manager_id,
            manager
        }
    }
}
#[derive(Debug, Deserialize,Serialize)]
pub struct CmdList{
    pub id:i32,
    pub cmd:String,
    pub description:String,
}
impl CmdList{
    fn new (row:sqlx::postgres::PgRow)->Self{
        let id =  row.get::<i32,_>("id");
        let cmd =  row.get::<String,_>("cmd");
        let description =  row.get::<String,_>("description");
        Self { 
            id, 
            cmd, 
            description, 
        }
    }
}
pub async fn manager_getdb(manager_name:&str, description:&str)->Result<Vec<PackageManager>,sqlx::Error>{
    dotenv().ok();
    let mut mutresult: Vec<PackageManager>= vec![];
    let pg_url =env::var("PG_CONNETION").unwrap();
    let pool = PgPoolOptions::new()
        .connect(&pg_url)
        .await?;
    let query_str = format!("SELECT * FROM packagemanager WHERE manager LIKE '%{}%' AND description LIKE '%{}%'", manager_name,description);
    let pgrow = sqlx::query(&query_str)
    .fetch_all(&pool).await?;
    for row in pgrow{
        let data = PackageManager::new(row);
        mutresult.push(data);
    }
    Ok(mutresult) 
    // let test = row.try_get::<i32,_>("role_id").unwrap();
    // println!("row ????{:?}",test);
    // let hash = ?;
    // if let Ok(hash)=row.try_get::<String,_>("psw"){
    //     let userinfo = PackageManager::new(row);
    //     // let result = password_check(psw.to_string(),hash);
    //     return Ok(userinfo);
    // } else {
    //     return Err(sqlx::Error::RowNotFound);
    // }
    // Ok(hash)
}
pub async fn package_getdb(package_name:&str,manager_name:&str, description:&str)->Result<Vec<Package>,sqlx::Error>{
    dotenv().ok();
    let mut mutresult: Vec<Package>= vec![];
    let pg_url =env::var("PG_CONNETION").unwrap();
    let pool = PgPoolOptions::new()
        .connect(&pg_url)
        .await?;
    let query_str = 
    format!("SELECT package.*,packagemanager.manager FROM package JOIN packagemanager ON package.manager_id = packagemanager.id WHERE package.package_name LIKE '%{}%' AND package.description LIKE '%{}%' AND packagemanager.manager LIKE '%{}%'", package_name,description,manager_name);
    let pgrow = sqlx::query(&query_str)
    .fetch_all(&pool).await?;
    for row in pgrow{
        let data = Package::new(row);
        mutresult.push(data);
    }
    Ok(mutresult)
}
pub async fn cmdlist_getdb(cmd_name:&str, description:&str)->Result<Vec<CmdList>,sqlx::Error>{
    dotenv().ok();
    let mut mutresult: Vec<CmdList>= vec![];
    let pg_url =env::var("PG_CONNETION").unwrap();
    let pool = PgPoolOptions::new()
        .connect(&pg_url)
        .await?;
    let query_str = format!("SELECT * FROM cmdlist WHERE cmd LIKE '%{}%' AND description LIKE '%{}%'", cmd_name,description);
    let pgrow = sqlx::query(&query_str)
    .fetch_all(&pool).await?;
    for row in pgrow{
        let data = CmdList::new(row);
        mutresult.push(data);
    }
    Ok(mutresult) 
}