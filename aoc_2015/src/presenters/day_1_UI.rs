use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use aoc_2015::Day1PublicAPI;

#[get("/")]
async fn hello() -> impl Responder {
    let file_path = "../aoc_2015_inputs/day_1/real_input_from_site.txt"; // -> extern in repositories get day 1 data
    let mut data = Day1PublicAPI::new("entrie".to_string()); // strange why I need to make a data in the new
    let test = data.start_day_1_real_input();
    HttpResponse::Ok().body(
        Day1PublicAPI::new("entrie".to_string())
            .start_day_1_real_input()
            .to_string(),
    )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// async fn get_day_1() -> impl Responder {
//     let file_path = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
//     let a = Day1PublicAPI::new(file_path.to_string());
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
        //.service(echo)
        //.route("/day_1", web::get().to(get_day_1))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
