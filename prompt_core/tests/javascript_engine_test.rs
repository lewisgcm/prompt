use prompt_core::javascript_engine::{modules, JavascriptEngineModule};
use prompt_core::{eval_module, javascript_engine};
use rquickjs::promise::MaybePromise;
use rquickjs::{CaughtResult, Function, Value};
use std::io;
use std::io::Write;

// #[tokio::test]
// async fn test_mad_bug_request() {
//     let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
//         name: String::from("test"),
//         code: String::from(
//             r#"
//             function createRequest(url, requestOptions) {
//               return new Request(url, requestOptions);
//             }
//             const request = createRequest('https://google.com');
//             //console.log("before keepalive");
//             //console.log(request);
//             // const supported = Boolean(
//             //     typeof Request !== "undefined" && "keepalive" in createRequest("https://[::1]")
//             // );
//             // console.log(supported);
//             // console.log("after keepalive");
//             export const testy = () => {
//                 throw Error("OOps, all borked!");
//             }
//         "#,
//         ),
//     }])
//     .await;
//
//     match engine_result {
//         Err(err) => {
//             eprintln!("Engine error: {}", err);
//         }
//         Ok(engine) => {
//             let result = eval_module!(&engine, "test", |ctx, value| {
//                 let object = value.as_object().unwrap();
//                 let handler: rquickjs::Function = object.get("testy")?;
//                 let handler_promise: CaughtResult<MaybePromise> = handler.call(()).catch(&ctx);
//                 match handler_promise {
//                     Err(err) => {
//                         println!("{:#?}", err);
//                     }
//                     Ok(handler_promise) => {
//                         let handler_result = handler_promise.into_future::<Value>().await;
//                         if let Err(err) = handler_result {
//                             let e = ctx.catch();
//                             println!("Ere: {:#?}", e);
//                         }
//                     }
//                 }
//
//                 // let handler_result = handler_promise.into_future::<Value>().await?;
//                 // let json = ctx.json_stringify(handler_result).unwrap();
//                 // println!("{:#?}", json);
//                 return Ok(String::from("test"));
//             });
//             if let Err(e) = result {
//                 println!("Test ERR: {:?}", e);
//             }
//         }
//     }
// }
//
// #[tokio::test]
// async fn test_mad_tings() {
//     let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
//         name: String::from("test"),
//         code: String::from(
//             r#"
//             const p = require('process');
//             console.log(p);
//             console.log(JSON.stringify(JSON.parse('{}')));
//             export const testy = () => {
//                 throw Error("OOps, all borked!");
//             }
//         "#,
//         ),
//     }])
//     .await;
//
//     match engine_result {
//         Err(err) => {
//             eprintln!("Engine error: {}", err);
//         }
//         Ok(engine) => {
//             let result = eval_module!(&engine, "test", |ctx, value| {
//                 let object = value.as_object().unwrap();
//                 let handler: rquickjs::Function = object.get("testy")?;
//                 let handler_promise: CaughtResult<MaybePromise> = handler.call(()).catch(&ctx);
//                 match handler_promise {
//                     Err(err) => {
//                         println!("{:#?}", err);
//                     }
//                     Ok(handler_promise) => {
//                         let handler_result = handler_promise.into_future::<Value>().await;
//                         if let Err(err) = handler_result {
//                             let e = ctx.catch();
//                             println!("Ere: {:#?}", e);
//                         }
//                     }
//                 }
//
//                 // let handler_result = handler_promise.into_future::<Value>().await?;
//                 // let json = ctx.json_stringify(handler_result).unwrap();
//                 // println!("{:#?}", json);
//                 return Ok(String::from("test"));
//             });
//             if let Err(e) = result {
//                 println!("Test ERR: {:?}", e);
//             }
//         }
//     }
// }

#[tokio::test]
async fn test_mad_ting() {
    let printer = modules::console::ConsoleLogger::new();
    let js = std::fs::read_to_string("/Users/lewis/Development/prompt-rs/node/bedrock-test/out.js")
        .unwrap();
    let engine_result = javascript_engine::new(
        vec![JavascriptEngineModule {
            name: String::from("test"),
            code: String::from(js.as_str()),
        }],
        &printer,
    )
    .await;

    match engine_result {
        Err(err) => {
            eprintln!("Engine error: {}", err);
        }
        Ok(engine) => {
            let result = eval_module!(&engine, "test", |ctx, value| {
                let object = value.as_object().unwrap();
                let handler: Function = object.get("testy").unwrap();
                let handler_promise: CaughtResult<MaybePromise> = handler.call(()).catch(&ctx);
                match handler_promise {
                    Err(err) => {
                        let e = ctx.catch();
                        println!("{:#?}", e);
                    }
                    Ok(handler_promise) => {
                        let handler_result = handler_promise.into_future::<Value>().await;
                        if let Err(err) = handler_result {
                            let e = ctx.catch();
                            println!("Ere: {:#?}", e);
                        }
                    }
                }

                // let handler_result = handler_promise.into_future::<Value>().await?;
                // let json = ctx.json_stringify(handler_result).unwrap();
                // println!("{:#?}", json);
                return Ok(String::from("test"));
            });

            if let Err(err) = result {
                eprintln!("Engine error: {:#?}", err);
            }

            engine.idle().await;
        }
    }

    io::stderr().flush().unwrap();
    io::stdout().flush().unwrap();
}
//
// #[tokio::test]
// async fn test_basic_module() {
//     let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
//         name: String::from("test"),
//         code: String::from("export const deep = async () => { return 'derp'; }"),
//     }])
//     .await;
//
//     assert!(engine_result.is_ok());
//
//     if let Ok(engine) = engine_result {
//         let result = eval_module!(&engine, "test", |ctx, value| {
//             let object = value.as_object().unwrap();
//             let handler: rquickjs::Function = object.get("deep")?;
//             let handler_promise: MaybePromise = handler.call(())?;
//             let handler_result = handler_promise.into_future::<Value>().await?;
//             return Ok(String::from_js(&ctx, handler_result).unwrap());
//         });
//         //assert!(result.is_ok());
//         //assert_eq!("derp", result.unwrap());
//     }
// }
//
// #[tokio::test]
// async fn test_provided_module() {
//     let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
//         name: String::from("test"),
//         code: String::from(
//             r#"
//              import path from "path";
//
//              export const deep = async () => { return 'derp' + path.sep; }
//         "#,
//         ),
//     }])
//     .await;
//
//     assert!(engine_result.is_ok());
//
//     if let Ok(engine) = engine_result {
//         let eval_result = eval_module!(&engine, "test", |ctx, value| {
//             Ok(format!("{:#?}", value))
//         });
//         assert!(eval_result.is_ok());
//     }
// }
//
// #[tokio::test]
// async fn test_invalid_module() {
//     let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
//         name: String::from("test"),
//         code: String::from("invalid-js-syntax"),
//     }])
//     .await;
//
//     assert!(engine_result.is_ok());
//
//     if let Ok(engine) = engine_result {
//         let eval_result = eval_module!(&engine, "test", |ctx, value| {
//             Ok(format!("{:#?}", value))
//         });
//         assert!(eval_result.is_err());
//     }
// }
//
// #[tokio::test]
// async fn test_run_module_twice() {
//     let engine_result = javascript_engine::new(vec![JavascriptEngineModule {
//         name: String::from("test"),
//         code: String::from("export const deep = async () => { return 'derp'; }"),
//     }])
//     .await;
//
//     assert!(engine_result.is_ok());
//
//     if let Ok(engine) = engine_result {
//         let eval_result = eval_module!(&engine, "test", |ctx, value| {
//             Ok(format!("{:#?}", value))
//         });
//         assert!(eval_result.is_ok());
//
//         let eval_result = eval_module!(&engine, "test", |ctx, value| {
//             Ok(format!("{:#?}", value))
//         });
//         assert!(eval_result.is_ok());
//     }
// }
