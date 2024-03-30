use std::str::FromStr;

use mongodb::bson::{doc, oid::ObjectId, Decimal128};

use crate::model::{
    inventory::InventoryRequest,
    product_order::{ProductOrder, ProductOrderRequest, ProductOrderResponse},
};

use super::{mongo_tables::Tables, mongodb::MongoDB};

impl MongoDB {
    pub fn get_restaurant_orders(&self, _rid: ObjectId) -> Option<Vec<ProductOrderResponse>> {
        self.find::<ProductOrder>(
            Tables::ProductOrder.value(),
            doc! {
                "restaurant_id": _rid
            },
            None,
        )
        .map(|o| o.into_iter().map(|p| p.into()).collect())
    }

    pub fn get_restaurant_order_by_ingredient(
        &self,
        _iid: ObjectId,
    ) -> Option<ProductOrderResponse> {
        self.find_one::<ProductOrder>(
            Tables::ProductOrder.value(),
            doc! {
                "ingredient_id": _iid
            },
            None,
        )
        .map(|x| x.into())
    }

    pub fn create_product_order(&self, _product_order: &ProductOrderRequest) -> Option<ObjectId> {
        let info = match MongoDB::doc_from(_product_order) {
            Some(ndoc) => self.create_one::<ProductOrder>(Tables::ProductOrder.value(), ndoc, None),
            None => None,
        };

        let inventory_update_status = self.update_inventory(&InventoryRequest {
            ingredient_id: _product_order.ingredient_id.clone(),
            stock: Decimal128::from_str(_product_order.quantity.to_string().as_str()).unwrap(),
        });

        if !inventory_update_status {
            println!("Failed to update inventory");
        }

        return info;
    }

    pub fn delete_product_order(&self, ingredient_id: &ObjectId) -> bool {
        let product_order = self.get_restaurant_order_by_ingredient(ingredient_id.clone());
        let quantity = product_order.map(|order| order.quantity).unwrap_or(0f64);

        let info = self.delete_one(
            Tables::ProductOrder.value(),
            doc! { "ingredient_id": ingredient_id.clone() },
            None,
        );

        let inventory_update_status = self.update_inventory(&InventoryRequest {
            ingredient_id: ingredient_id.clone(),
            stock: Decimal128::from_str((-1f64 * quantity).to_string().as_str()).unwrap(),
        });

        if !inventory_update_status {
            println!("Failed to update inventory");
        }

        return info;
    }
}
