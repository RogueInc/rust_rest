use actix_web::{get,post,web::{self, Json},App,HttpResponse,HttpServer,Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User{
    name:String,
    age:i32 
}

#[get("/")]
async fn hello()-> impl Responder{
    HttpResponse::Ok().body("hello world")
}

#[post("/echo")]
async fn postreqhand(req_body: String)-> impl Responder{
    HttpResponse::Ok().body(req_body)
}

#[post("/jsonhand")]
async fn jsonhand(req_body: Json<User>)-> impl Responder{
    HttpResponse::Ok().json(req_body)
}

#[post("/echo/{a_num}/{a_string}")]
async fn pathextrac(path:web::Path<(i32,String)>,jsonresss:web::Json<User>)-> impl Responder{
    let path=path.into_inner();
    // Ok(format!("{} {}",path.0,path.1))
    HttpResponse::Ok().body(format!("{} {}",path.0,path.1))

}


//have to visit this later to learn error handling using result and httpresponse
#[post("/pathext/{a_num}/{a_string}")]
async fn pathextractres(path:web::Path<(i32,String)>,jsonresss:web::Json<User>)-> Result<HttpResponse,actix_web::Error>{
    let path=path.into_inner();
    // Ok(format!("{} {}",path.0,path.1))
    Ok(HttpResponse::Ok().body(format!("{} {}",path.0,path.1)))

}

async fn manual_hello()-> impl Responder{
    HttpResponse::Ok().body("hey there")
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(hello)
        .service(postreqhand)
        .service(jsonhand)
        .service(pathextrac)
        .service(pathextractres)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}