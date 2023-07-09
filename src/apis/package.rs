use crate::{Error, Result,yumdb::packagedb::{manager_getdb,package_getdb,PackageManager,cmdlist_getdb}};
use axum::{Json, routing::{Route, get}, Router,extract::Query};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};


#[derive(Debug,Deserialize)]
struct ResultManager{
    name:String,
    description:String,
    data:Vec<PackageManager>,
    result:bool
    // role:Option<UserRole::VIEWER(String::de)>
}


#[derive(Debug,Deserialize)]
struct ManagerParams{
    name:Option<String>,
    description:Option<String>,
    // role:Option<UserRole::VIEWER(String::de)>
}

#[derive(Debug,Deserialize)]
struct PackageParams{
    name:Option<String>,
    description:Option<String>,
    manager_name:Option<String>,
    // role:Option<UserRole::VIEWER(String::de)>
}
#[derive(Debug,Deserialize)]
struct CmdParams{
    name:Option<String>,
    description:Option<String>,
    // role:Option<UserRole::VIEWER(String::de)>
}
async fn get_manager(Query(params):Query<ManagerParams>)->Result<Json<Value>>{
    if cfg!(debug_assertions){
        println!("->> {:<12} - api_manager","HANDLER");
    }
    // let name = params.manager);
    let name = params.name.unwrap_or(String::from(""));
    let description = params.description.unwrap_or(String::from(""));;
    // if let None=manager_name{
    //     manager_name=Some(String::from(""));
    // };
    // if let None=manager_description{
    //     manager_description=Some(String::from(""));
    // };
    if let Ok(data) = manager_getdb(name.as_str(),description.as_str()).await{
        let result = json!({
            "search_name":&name,
            "description":&description,
            "data":&data,
            "result":true
        });
        // ResultManager { name, description, data, result:true };
        return Ok(Json(result));
    }else{
        return Err(Error::DBNotFound);
    }
}
async fn get_package(Query(params):Query<PackageParams>)->Result<Json<Value>>{
    if cfg!(debug_assertions){
        println!("->> {:<12} - api_package","HANDLER");
    }
    let package_name = params.name.unwrap_or(String::from(""));
    let description = params.description.unwrap_or(String::from(""));
    let manager_name = params.manager_name.unwrap_or(String::from(""));
    if let Ok(data) = package_getdb(package_name.as_str(),manager_name.as_str(),description.as_str()).await{
        let result = json!({
            "search_name":&package_name,
            "search_manager":&manager_name,
            "description":&description,
            "data":&data,
            "result":true
        });
        // ResultManager { name, description, data, result:true };
        return Ok(Json(result));
    }else{
        return Err(Error::DBNotFound);
    }
}

async fn get_cmd(Query(params):Query<CmdParams>)->Result<Json<Value>>{
    if cfg!(debug_assertions){
        println!("->> {:<12} - api_cmd","HANDLER");
    }
    let name = params.name.unwrap_or(String::from(""));
    let description = params.description.unwrap_or(String::from(""));;

    if let Ok(data) = cmdlist_getdb(name.as_str(),description.as_str()).await{
        let result = json!({
            "search_name":&name,
            "description":&description,
            "data":&data,
            "result":true
        });
        return Ok(Json(result));
    }else{
        return Err(Error::DBNotFound);
    }
}
pub fn routes()->Router{
    Router::new()
    .route("/api/manager", get(get_manager))
    .route("/api/package", get(get_package))
    .route("/api/cmd", get(get_cmd))
}