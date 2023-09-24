//! # presenters
//!
//! `presenters` is an actix port to have an api
//!
//!

use crate::presenters::day_1_presenter::Day1RestPublicAPI;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[get("/day_1_aoc")]
async fn day_1_aoc() -> impl Responder {
    let mut t = Day1RestPublicAPI::new();
    let responce = appli_json_api_format(t.start_day_1_real_input()).clone();
    HttpResponse::Ok()
        // .append_header(header)
        .content_type(ContentType::json())
        .body(responce)
}
#[derive(Deserialize, Serialize)]
pub struct DayIdentifier {
    day_id: String,
}

fn find_day(day_id: String, part: String) -> i32 {
    match (day_id.as_str(), part.as_str()) {
        ("1", "1") => Day1RestPublicAPI::new().start_day_1_real_input(),
        ("1", "2") => Day1RestPublicAPI::new().start_day_1_part_2_real_input(),
        _ => -1,
    }
}

fn find_custum_day(day_id: String, part: String, entrie: String) -> i32 {
    match (day_id.as_str(), part.as_str()) {
        ("1", "1") => Day1RestPublicAPI::new().launch_day_1(entrie),
        ("1", "2") => Day1RestPublicAPI::new().start_day_1_part_2_real_input(),
        _ => -1,
    }
}

/// # days rest api

#[get("/day/{day_id}/part/{part_id}")]
async fn day_1_part_2_aoc(req: HttpRequest) -> impl Responder {
    let day_id = req.match_info().get("day_id").unwrap_or("1").to_string();
    let part_id = req.match_info().get("part_id").unwrap_or("1").to_string();
    let result = find_day(day_id, part_id);
    let responce = appli_json_api_format(result).clone();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(responce)
}

#[derive(Deserialize)]
struct FormData {
    data: String,
}

async fn custum_day_1(form: web::Form<FormData>, req: HttpRequest) -> HttpResponse {
    let day_id = req.match_info().get("day_id").unwrap_or("1").to_string();
    let part_id = req.match_info().get("part_id").unwrap_or("1").to_string();
    let result = find_custum_day(day_id, part_id, form.data.clone());
    let responce = appli_json_api_format(result).clone();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(responce)
}

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
/// # day 1 structure
#[derive(Serialize, Deserialize, Debug)]
struct DayOneData {
    day_1_result: i32,
}

/// # transform restult to json day 1
fn appli_json_api_format(result: i32) -> String {
    let result_day_1_data = DayOneData {
        day_1_result: result,
    };
    serde_json::to_string(&vec![result_day_1_data]).unwrap()
}

/// # start server
#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(day_1_aoc)
            .service(day_1_part_2_aoc)
            .route(
                "/custum/day/{day_id}/part/{part_id}",
                web::post().to(custum_day_1),
            )
        //.service(custum_day_1)
    })
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
                "day_1": "280"
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
