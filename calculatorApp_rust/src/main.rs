use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
struct CalcRequest{
    num1:f64,
    num2:f64,
    operator: String,
}
#[derive(Serialize)]
struct CalcResponse{
    result:f64,
}

async fn calculate(req: web::Json<CalcRequest>)-> impl Responder{
    let result = match req.operator.as_str(){
        "+"=>req.num1 + req.num2,
        "-"=>req.num1- req.num2,
        "*"=>req.num1 * req.num2,
        "/"=>{
            if req.num2== 0.0 {
                return HttpResponse::BadRequest().body("Cannot divide by zero");
            }
            req.num1/req.num2
        }
        _=>return HttpResponse::BadRequest().body("Invalid operator"),
    };
    HttpResponse::Ok().json(CalcResponse{result})
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    println!("Server running at http://127.0.0.1:8080");
    HttpServer::new(||{
        App::new().route("/calculate",web::post().to(calculate))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}