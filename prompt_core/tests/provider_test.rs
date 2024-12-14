#[tokio::test]
async fn test_javascript() {
    let result = prompt_core::provider::javascript().await;
    println!("{:?}", result);
}
