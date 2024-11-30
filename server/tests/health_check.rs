mod common;
use common::spawn;


#[actix_web::test]
async fn test_health_check() {
    let server_details = spawn().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", server_details.addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);
}
