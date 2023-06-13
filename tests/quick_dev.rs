use anyhow::Result;
use serde_json::json;
#[tokio::test]
async fn quick_dev()->Result<()>{
    let hc =httpc_test::new_client("http://localhost:8080")?;
    // hc.do_get("/?name=엄").await?.print().await?;
    hc.do_get("/static/test.txt").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login", 
        json!({
            "email":"yumzi114@gmail.com",
            "psw":"dkTkdjawl1!"
        }));
    req_login.await?.print().await?;
    hc.do_get("/").await?.print().await?;    
    Ok(())
}