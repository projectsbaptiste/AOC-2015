use crate::presenters::day_1_presenter::Day1RestPublicAPI;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/day_1_aoc")]
async fn day_1_aoc() -> impl Responder {
    HttpResponse::Ok().body(
        Day1RestPublicAPI::new()
            .start_day_1_real_input()
            .to_string(),
    )
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

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(day_1_aoc).service(day_1_part_2_aoc)
        //.route("/day_1", web::get().to(get_day_1))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
