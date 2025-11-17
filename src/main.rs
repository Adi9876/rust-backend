use actix_web::{
    get, patch, post, web::Data, web::Json, web::Path, App, HttpResponse, HttpServer, Responder,
};
mod db;
mod models;
use crate::db::Database;
use crate::models::pizza::{BuyPizzaRequest, Pizza, UpdatePizzaURL};
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(found_pizzas) => match serde_json::to_string(&found_pizzas) {
            Ok(json) => HttpResponse::Ok()
                .content_type("application/json")
                .body(json),
            Err(e) => {
                eprintln!("Error serializing pizzas: {:?}", e);
                HttpResponse::InternalServerError().body(format!("Error serializing pizzas: {}", e))
            }
        },
        None => {
            eprintln!("Failed to fetch pizzas from database");
            HttpResponse::InternalServerError().body("Error: Failed to fetch pizzas from database")
        }
    }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            println!("Pizzas name {}", pizza_name);
            let new_uuid = uuid::Uuid::new_v4().simple().to_string();
            println!("new uuid {}", new_uuid);
            let new_pizza = db.add_pizza(Pizza::new(new_uuid, pizza_name)).await;
            println!("New pizza {:?}", new_pizza);
            match new_pizza {
                Some(created) => {
                    println!("created {:?}", created);
                    HttpResponse::Ok().body(format!("Created new pizza {:?}", created))
                }
                None => HttpResponse::Ok().body("Error buying pizza"),
            }
            // HttpResponse::Ok().body(format!("Pizza entered is {}", pizza_name))
        }
        Err(_) => HttpResponse::Ok().body(format!("Pizza name required")),
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
    let db = Database::init().await.expect("Error connecting to db...");
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
