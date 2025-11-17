// use surrealdb::engine::remote::ws::{Ws,Client};
// // use surrealdb::engine::remote::http::{Client, Http};
// use surrealdb::opt::auth::Root;
// use surrealdb::{Error, Surreal};

use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::pizza::Pizza;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        // Connect to SurrealDB
        let client = Surreal::new::<Http>("127.0.0.1:8000").await?;

        // Authenticate with root credentials
        client
            .signin(Root {
                username: "root".to_string(),
                password: "root".to_string(),
            })
            .await
            .map_err(|e| {
                eprintln!("Signin error: {:?}", e);
                e
            })?;

        // Set namespace and database
        client
            .use_ns("surreal")
            .use_db("pizzas")
            .await
            .map_err(|e| {
                eprintln!("use_ns/use_db error: {:?}", e);
                e
            })?;

        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("pizzas"),
        })
    }

    pub async fn get_all_pizzas(&self) -> Option<Vec<Pizza>> {
        let mut response = self
            .client
            .query("SELECT uuid, pizza_name FROM pizza")
            .await
            .ok()?;

        let values_result: Result<Vec<serde_json::Value>, _> = response.take(0);

        match values_result {
            Ok(values) => {
                let pizzas: Result<Vec<Pizza>, _> = values
                    .into_iter()
                    .map(|v| serde_json::from_value(v))
                    .collect();
                match pizzas {
                    Ok(p) => Some(p),
                    Err(e) => {
                        eprintln!("Error deserializing pizzas: {:?}", e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Error taking response: {:?}", e);
                None
            }
        }
    }

    pub async fn add_pizza(&self, new_pizza: Pizza) -> Option<Pizza> {
        let query = format!(
            "CREATE pizza:{} SET uuid = '{}', pizza_name = '{}' RETURN uuid, pizza_name",
            new_pizza.uuid, new_pizza.uuid, new_pizza.pizza_name
        );
        let mut response = self.client.query(query).await.ok()?;
        let created: Result<Vec<serde_json::Value>, _> = response.take(0);
        match created {
            Ok(mut values) => {
                if let Some(first_value) = values.pop() {
                    serde_json::from_value(first_value).ok()
                } else {
                    eprintln!("No values returned from CREATE query");
                    None
                }
            }
            Err(e) => {
                eprintln!("Error taking response: {:?}", e);
                None
            }
        }
    }
}

// surreal start file:pizzadb --user root --password root
