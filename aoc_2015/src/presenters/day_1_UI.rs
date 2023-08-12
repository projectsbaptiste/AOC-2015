use actix_web::{get, post, HttpResponse, Responder};
use aoc_2015::Day1PublicAPI;

#[get("/")]
async fn hello() -> impl Responder {
    let file_path = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
    HttpResponse::Ok().body(
        Day1PublicAPI::new(file_path.to_string())
            .launch_day_1_from_file()
            .to_string(),
    )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn get_day_1() -> impl Responder {
    let file_path = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
    let a = Day1PublicAPI::new(file_path.to_string());
    HttpResponse::Ok().body("Hey there!")
}
