use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::recipe::{Recipe, RecipeRequest},
};

use super::response::APIResponse;

pub fn recipe_routes() -> Vec<rocket::Route> {
    routes![
        get_all_recipes,
        get_recipe,
        create_recipe,
        update_recipe,
        delete_recipe
    ]
}

#[get("/")]
async fn get_all_recipes(db: &State<MongoDB>) -> Json<Option<Vec<Recipe>>> {
    Json(db.get_all_recipes())
}

#[get("/<iid>")]
async fn get_recipe(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<Option<Recipe>>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: db.get_recipe(&id),
            message: "".to_string(),
        }),
        Err(ex) => {
            println!("{:?}", ex);
            Json(APIResponse {
                code: 500,
                data: None,
                message: "ID bad format.".to_string(),
            })
        }
    }
}

#[post("/", format = "application/json", data = "<info>")]
async fn create_recipe(
    info: Json<RecipeRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let recipe = db.create_recipe(&info.0);
    let message = if recipe.is_some() {
        "Recipe created."
    } else {
        "Recipe was not created."
    }
    .to_string();
    let response = APIResponse {
        code: 200,
        data: recipe,
        message,
    };
    Json(response)
}

#[put("/<iid>", format = "application/json", data = "<info>")]
async fn update_recipe(
    iid: &str,
    info: Json<RecipeRequest>,
    db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = db.update_recipe(&id, &info.0);
            let message = if update {
                "Recipe updated."
            } else {
                "Error while updating."
            }
            .to_string();
            let response = APIResponse {
                code: 200,
                data: update,
                message,
            };
            Json(response)
        }
        Err(ex) => {
            println!("{:?}", ex);
            Json(APIResponse {
                code: 500,
                data: false,
                message: "ID bad format".to_string(),
            })
        }
    }
}

#[delete("/<iid>")]
async fn delete_recipe(iid: &str, db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = db.delete_recipe(&id);
            let message = if update {
                "Recipe deleted."
            } else {
                "Error while deleting."
            }
            .to_string();
            let response = APIResponse {
                code: 200,
                data: update,
                message,
            };
            Json(response)
        }
        Err(ex) => {
            println!("{:?}", ex);
            Json(APIResponse {
                code: 500,
                data: false,
                message: "ID bad format".to_string(),
            })
        }
    }
}
