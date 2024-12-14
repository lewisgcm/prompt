use prompt_core::javascript_engine;
use prompt_core::javascript_engine::{eval_module, JavascriptEngineModule};

#[tokio::test]
async fn test_basic_module() {
    let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
        name: String::from("test"),
        code: String::from("export const deep = async () => { return 'derp'; }"),
    }])
    .await;

    assert!(engine_result.is_ok());

    if let Ok(engine) = engine_result {
        let eval_result = eval_module(&engine, "test").await;
        assert!(eval_result.is_ok());
    }
}

#[tokio::test]
async fn test_provided_module() {
    let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
        name: String::from("test"),
        code: String::from(
            r#"
             import path from "path";

             export const deep = async () => { return 'derp' + path.sep; }
        "#,
        ),
    }])
    .await;

    assert!(engine_result.is_ok());

    if let Ok(engine) = engine_result {
        let eval_result = eval_module(&engine, "test").await;
        assert!(eval_result.is_ok());
    }
}

#[tokio::test]
async fn test_invalid_module() {
    let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
        name: String::from("test"),
        code: String::from("invalid-js-syntax"),
    }])
    .await;

    assert!(engine_result.is_ok());

    if let Ok(engine) = engine_result {
        let eval_result = eval_module(&engine, "test").await;
        assert!(eval_result.is_err());
    }
}
