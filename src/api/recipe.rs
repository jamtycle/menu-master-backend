use mongodb::bson::oid::ObjectId;
use rocket::{serde::json::Json, State};

use crate::{
    db::mongodb::MongoDB,
    model::recipe::{Recipe, RecipeRequest},
};

use super::response::APIResponse;

#[get("/")]
pub async fn get_all_recipes(_db: &State<MongoDB>) -> Json<Option<Vec<Recipe>>> {
    Json(_db.get_all_recipes())
}

#[get("/<iid>")]
pub async fn get_recipe(iid: &str, _db: &State<MongoDB>) -> Json<APIResponse<Option<Recipe>>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => Json(APIResponse {
            code: 200,
            data: _db.get_recipe(&id),
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
pub async fn create_recipe(
    info: Json<RecipeRequest>,
    _db: &State<MongoDB>,
) -> Json<APIResponse<Option<ObjectId>>> {
    let recipe = _db.create_recipe(&info.0);
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
pub async fn update_recipe(
    iid: &str,
    info: Json<RecipeRequest>,
    _db: &State<MongoDB>,
) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = _db.update_recipe(&id, &info.0);
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
pub async fn delete_recipe(iid: &str, _db: &State<MongoDB>) -> Json<APIResponse<bool>> {
    match ObjectId::parse_str(iid) {
        Ok(id) => {
            let update = _db.delete_recipe(&id);
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
