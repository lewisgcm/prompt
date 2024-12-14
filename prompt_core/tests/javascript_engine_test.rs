use prompt_core::javascript_engine::JavascriptEngineModule;
use prompt_core::{eval_module, javascript_engine};
use rquickjs::promise::MaybePromise;
use rquickjs::{FromJs, Value};

#[tokio::test]
async fn test_basic_module() {
    let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
        name: String::from("test"),
        code: String::from("export const deep = async () => { return 'derp'; }"),
    }])
    .await;

    assert!(engine_result.is_ok());

    if let Ok(engine) = engine_result {
        let result = eval_module!(&engine, "test", |ctx, value| {
            let object = value.as_object().unwrap();
            let handler: rquickjs::Function = object.get("deep")?;
            let handler_promise: MaybePromise = handler.call(())?;
            let handler_result = handler_promise.into_future::<Value>().await?;
            return Ok(String::from_js(&ctx, handler_result).unwrap());
        });
        assert!(result.is_ok());
        assert_eq!("derp", result.unwrap());
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
        let eval_result = eval_module!(&engine, "test", |ctx,value| { Ok(format!("{:#?}", value)) });
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
        let eval_result = eval_module!(&engine, "test", |ctx,value| { Ok(format!("{:#?}", value)) });
        assert!(eval_result.is_err());
    }
}

#[tokio::test]
async fn test_run_module_twice() {
    let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
        name: String::from("test"),
        code: String::from("export const deep = async () => { return 'derp'; }"),
    }])
    .await;

    assert!(engine_result.is_ok());

    if let Ok(engine) = engine_result {
        let eval_result = eval_module!(&engine, "test", |ctx,value| { Ok(format!("{:#?}", value)) });
        assert!(eval_result.is_ok());

        let eval_result = eval_module!(&engine, "test", |ctx,value| { Ok(format!("{:#?}", value)) });
        assert!(eval_result.is_ok());
    }
}
