use actix_web::{get, post, patch, web::Json, web::Path, Data, App, HttpResponse, HttpServer, Responder};
mod models;
mod db;
use validator::Validate;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL};
use crate::db::Database;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    // HttpResponse::Ok().body("Pizzas available !")
    
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {}", pizza_name))
        },
        Err(_) => {
            HttpResponse::Ok().body(format!("Pizza name required"))
        }
    }
    // HttpResponse::Ok().body("Buying pizza")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {

    let uuid = update_pizza_url.into_inner().uuid;

    HttpResponse::Ok().body(format!("Updating the pizza with this id {}", uuid))

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
                .await
                .expect("Error connecting to db...");
    let db_data = Data::new(db);


    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



