use crate::presenters::day_1_presenter::Day1RestPublicAPI;
use actix_web::http::header::ContentType;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
//use json as jsons;
use serde::{Deserialize, Serialize};
//use serde_json::json; //Value
#[get("/day_1_aoc")]
async fn day_1_aoc() -> impl Responder {
    let mut t = Day1RestPublicAPI::new();
    let responce = appli_json_api_format(t.start_day_1_real_input()).clone();
    HttpResponse::Ok()
        // .append_header(header)
        .content_type(ContentType::json())
        .body(responce)
}

#[get("/day_1_part_2_aoc")]
async fn day_1_part_2_aoc() -> impl Responder {
    HttpResponse::Ok().body(
        Day1RestPublicAPI::new()
            .start_day_1_part_2_real_input()
            .to_string(),
    )
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn get_day_1() -> impl Responder {
//     let file_path = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
//     let a = Day1PublicAPI::new(file_path.to_string());
//     HttpResponse::Ok().body("Hey there!")
// }

/// https://github.com/json-api/json-api/blob/gh-pages/examples/index.md
/// https://jsonapi.org/format/#fetching-sparse-fieldsets
#[derive(Serialize, Deserialize, Debug)]
enum State {
    success,
    Error,
}
#[derive(Serialize, Deserialize, Debug)]
struct DayOneData {
    day_1_result: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct JsonApiFormat {
    code: i32,
    sucess: State,
    data: Vec<DayOneData>,
}

fn appli_json_api_format(result: i32) -> String {
    let result_day_1_data = DayOneData {
        day_1_result: result,
    };
    serde_json::to_string(&vec![result_day_1_data]).unwrap()
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(day_1_aoc).service(day_1_part_2_aoc))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

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
        println!("TEst {}", appli_json_api_format(280));
        let u = instantiated.as_str();
        println!("TEst2 {:?}", u);
        println!("TEst1 {}", instantiated);
        //assert_eq!(u.to_string(), appli_json_api_format(280));
    }
}
