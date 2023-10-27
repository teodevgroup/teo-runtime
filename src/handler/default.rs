use crate::request;
use teo_teon::teon;
//
// async fn find_many(ctx: request::Ctx) -> Result<Res> {
//     let action = Action::from_u32(FIND | MANY | ENTRY);
//     let result = graph.find_many_internal(model, input, false, action, source, connection.clone()).await;
//     match result {
//         Ok(results) => {
//             let mut count_input = input.clone();
//             let count_input_obj = count_input.as_hashmap_mut().unwrap();
//             count_input_obj.remove("skip");
//             count_input_obj.remove("take");
//             count_input_obj.remove("pageSize");
//             count_input_obj.remove("pageNumber");
//             let count = graph.count(model, &count_input, connection.clone()).await.unwrap();
//             let mut meta = teon!({"count": count});
//             let page_size = input.get("pageSize");
//             if page_size.is_some() {
//                 let page_size = page_size.unwrap().as_i32().unwrap();
//                 let count = count as i32;
//                 let mut number_of_pages = count / page_size;
//                 if count % page_size != 0 {
//                     number_of_pages += 1;
//                 }
//                 meta.as_hashmap_mut().unwrap().insert("numberOfPages".to_string(), number_of_pages.into());
//             }
//
//             let mut result_json: Vec<Value> = vec![];
//             for (index, result) in results.iter().enumerate() {
//                 match result.to_json_internal(&path!["data", index]).await {
//                     Ok(result) => result_json.push(result),
//                     Err(_) => return Err(Error::permission_error(path!["data"], "not allowed to read")),
//                 }
//             }
//             Ok(Res::TeonDataMetaRes(Value::Vec(result_json), meta))
//         },
//         Err(err) => Err(err)
//     }
// }
