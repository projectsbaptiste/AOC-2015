use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum State {
    success,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonApiFormat {
    code: i32,
    sucess: State,
    data: Vec<String>,
}

/// #
///

// fn appli_json_api_format(code: i32, sucess: State, data: Vec<String>) -> String {
//     let result_day_1_data = DayOneData {
//         day_1_result: result,
//     };
//     let data_struct = JsonApiFormat {
//         code: 200,
//         sucess: sucess,
//         data: vec![result_day_1_data],
//     };
//     //let serialize =
//     serde_json::to_string(&data_struct).unwrap()
//     //let test = json::parse(&serialize).unwrap();
//     //return serialize;
// }

mod tests {
    use super::*;
    #[test]
    fn should_retourn_json_with_syntaxe_bellow() {
        let instantiated = json::parse(
            r#"
        {
            "code": 200,
            "success": "success",
            "data": [{
                "day_1": "280"
            }
            ]
        }
        "#,
        )
        .unwrap();
        // println!("TEst {}", appli_json_api_format(code: i32, sucess: State, data: Vec<String>));
        //let u = instantiated.as_str();
        //println!("TEst2 {:?}", u);
        println!("TEst1 {}", instantiated);
        //assert_eq!(u.to_string(), appli_json_api_format(280));
    }
}
