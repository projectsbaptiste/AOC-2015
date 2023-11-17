//! # presenters
//!
//! `presenters` is an actix port to have an api
//!
//!

use crate::presenters::day_1_presenter::DayRestPublicAPI;
use crate::repositories::day_1_entries::Day1PublicEntities;
use actix_web::http::header::ContentType;
use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use actix_cors::Cors;

use log::info;

/// # V1.0/day_1_aoc
///
/// exposed api to use day 1 part 1 exercice
///
/// # Statement (TODO voir le nom pour l'url de rest )
///
/// * `url` -V1.0/day_1_aoc
///
/// # Examples
///
///
/// exemple 1
/// '(())' -> entry or '()()' -> entry
/// should return 0
///
/// exemple 2
/// '(((' -> entry or '(()(()(' -> entry
/// should return 3
///
/// exemple 3
/// '))(((((' -> entry
/// should return 3
///
/// exemple 4
/// '())' -> entry or '))(' -> entry
/// should return -1
///
/// exemple 5
/// ')))' -> entry or ')())())' -> entry
/// should return -3
///
#[get("V1.0/day_1_aoc")]
async fn day_1_aoc() -> impl Responder {
    let mut v = Vec::new();
    v[0] = Day1PublicEntities::get_data_day_1_real_data();
    let t = DayRestPublicAPI::new(1, 1, v.clone()).launch_day_x_part_y(v.clone());
    let responce = appli_json_api_format(1, 1, t);
    info!("day_1_aoc response {}", responce.clone());
    HttpResponse::Ok()
        // .append_header(header)
        .content_type(ContentType::json())
        .body(responce)
}

// fn find_day(day_id: String, part: String) -> i32 {
//     match (day_id.as_str(), part.as_str()) {
//         ("1", "1") => DayRestPublicAPI::new(1, 1).launch_day_x_part_y(1, 1, day_data, day_result),
//         ("1", "2") => DayRestPublicAPI::new().start_day_1_part_2_real_input(),
//         _ => -1,
//     }
// }

// fn find_custum_day(day_id: String, part: String, entrie: Vec<String>) -> i32 {
//     match (day_id.as_str(), part.as_str()) {
//         ("1", "1") => DayRestPublicAPI::new(day_id, part, entrie).launch_day_x_part_y(entrie),
//         ("1", "2") => DayRestPublicAPI::new().launch_day_x_part_y(
//             day_id.trim().parse().expect("msg"),
//             part.trim().parse().expect("msg"),
//             vec![],
//             entrie,
//         ),
//         _ => -1,
//     }
// }

use const_str;

fn get_rest_info_i32(req: HttpRequest, url_var: &str) -> i32 {
    let url_value = req
        .match_info()
        .get(url_var)
        .unwrap_or("unknow")
        .to_string();
    url_value
        .clone()
        .trim()
        .parse()
        .expect("url is not a number")
}

/// # V1.0/day/{day_id}/part/{part_id}
///
/// exposed api to use day 1 part 1 exercice
///
/// # Statement (TODO voir le nom pour l'url de rest )
///
/// * `url` -V1.0/day_1_aoc
///
/// # Examples
//  #[get(DAY_API)]
async fn day_api_by_part(req: HttpRequest) -> impl Responder {
    let day_id_n: i32 = get_rest_info_i32(req.clone(), "day_id");
    let part_id_n: i32 = get_rest_info_i32(req.clone(), "part_id");
    let mut v = Vec::new();
    v.push(Day1PublicEntities::get_data_day_1_real_data());
    let result =
        DayRestPublicAPI::new(day_id_n, part_id_n, v.clone()).launch_day_x_part_y(v.clone());

    let responce = appli_json_api_format(day_id_n, part_id_n, result).clone();
    info!(
        "day_api_by_part day {} part {} responce {}",
        day_id_n.clone(),
        part_id_n.clone(),
        responce.clone()
    );
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(responce)
}

async fn file_day_api_by_part(req: HttpRequest) -> impl Responder {
    let day_id_n: i32 = get_rest_info_i32(req.clone(), "day_id");
    let part_id_n: i32 = get_rest_info_i32(req.clone(), "part_id");
    let mut v = Vec::new();
    let mut test = Day1PublicEntities::get_data_day_1_real_data();
    v.push(Day1PublicEntities::get_data_day_1_real_data());
    let result =
        DayRestPublicAPI::new(day_id_n, part_id_n, v.clone()).launch_day_x_part_y(v.clone());

    let responce = appli_json_api_format(day_id_n, part_id_n, result).clone();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(responce)
}

#[derive(Deserialize)]
struct FormData {
    data: String,
}

async fn custum_day_by_part(form: web::Form<FormData>, req: HttpRequest) -> HttpResponse {
    let day_id_n: i32 = get_rest_info_i32(req.clone(), "day_id");
    let part_id_n: i32 = get_rest_info_i32(req.clone(), "part_id");

    //let result = find_custum_day(day_id, part_id, form.data.clone());
    let mut v = Vec::new();
    v.push(form.data.clone());
    let result =
        DayRestPublicAPI::new(day_id_n, part_id_n, v.clone()).launch_day_x_part_y(v.clone());
    let responce = appli_json_api_format(day_id_n, part_id_n, result).clone();
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
    Success,
    Error,
}
/// # day 1 structure
#[derive(Serialize, Deserialize, Debug)]
struct DayData {
    day_number: i32,
    day_part: i32,
    day_result: i32,
}

/// # transform restult to json day 1
fn appli_json_api_format(day_number: i32, day_part: i32, result: i32) -> String {
    let result_day_1_data = DayData {
        day_number,
        day_part,
        day_result: result,
    };
    serde_json::to_string(&vec![result_day_1_data]).unwrap()
}

// CONSTRUCTION OF API STATEMENT
const API_VERSION: &str = "/V1.0";
const DAY_URL: &str = "/day/{day_id}/part/{part_id}";
const FILE_DAY_URL: &str = "file/day/{day_id}/part/{part_id}";
// Normal way to day api
const FILE_DAYS_API: &str = const_str::concat!(API_VERSION, FILE_DAY_URL);
const DAYS_API: &str = const_str::concat!(API_VERSION, DAY_URL);

const CUSTUM_URL: &str = "/custum";
const CUSTUM_DAYS: &str = const_str::concat!(CUSTUM_URL, DAY_URL);
// Custum (with entry post fonction) way to day api
const CUSTUM_DAYS_API: &str = const_str::concat!(API_VERSION, CUSTUM_DAYS);

const WEB_SERVER_IP: &str = "127.0.0.1";
const WEB_SERVER_PORT: u16 = 8080;
// do env file with urls and server ip and port
use actix_web::middleware::Logger;
/// # start server
#[actix_web::main]
pub async fn start_actix_server() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(day_1_aoc)
            .route(DAYS_API, web::get().to(day_api_by_part))
            //.route(FILE_DAYS_API, web::get().to(file_day_api_by_part))
            .route(CUSTUM_DAYS_API, web::post().to(custum_day_by_part))
            .wrap(Logger::default())
    })
    .bind((WEB_SERVER_IP, WEB_SERVER_PORT))?
    .run()
    .await
}

mod tests {

    #[test]
    fn should_retourn_json_with_syntaxe_bellow() {
        let instantiated = json::parse(
            r#"
            [{
                "day_number":1,
                "day_part":1,
                "day_result":280
            }]
            "#,
        )
        .unwrap();
        let day_1_part_1 = crate::presenters::day_1_actix_ui::appli_json_api_format(1, 1, 280);
        let attendees = instantiated.dump();
        assert_eq!(attendees, day_1_part_1);
    }
}
