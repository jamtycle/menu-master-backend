use mongodb::bson::{doc, oid::ObjectId};

use crate::model::{
    inventory::InventoryRequest,
    product_order::{ProductOrder, ProductOrderRequest, ProductOrderResponse},
};

use super::{
    mongo_tables::Tables,
    mongodb::{MongoDB, MongoDBResult},
};

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

    pub async fn create_product_order(
        &self,
        _product_order: &ProductOrderRequest,
    ) -> MongoDBResult<ObjectId> {
        let doc = MongoDB::doc_from(_product_order)?;

        let data = self
            .create_one::<ProductOrder>(Tables::ProductOrder.value(), doc, None)
            .await?;
        let inventory_update_status = self
            .update_inventory(&InventoryRequest {
                ingredient_id: _product_order.ingredient_id.clone(),
                stock: _product_order.quantity,
            })
            .await?;

        if !inventory_update_status {
            return Err(mongodb::error::Error::custom("Error updating inventory"));
        }

        return Ok(data);
    }

    pub async fn delete_product_order(&self, ingredient_id: &ObjectId) -> MongoDBResult<bool> {
        let product_order = self.get_restaurant_order_by_ingredient(ingredient_id.clone());
        let quantity = product_order.map(|order| order.quantity).unwrap_or(0f64);

        let info = self
            .delete_one(
                Tables::ProductOrder.value(),
                doc! { "ingredient_id": ingredient_id.clone() },
                None,
            )
            .await?;

        let inventory_update_status = self
            .update_inventory(&InventoryRequest {
                ingredient_id: ingredient_id.clone(),
                stock: (-1f64 * quantity),
            })
            .await?;

        if !inventory_update_status {
            return Err(mongodb::error::Error::custom("Error updating inventory"));
        }

        return Ok(info);
    }
}
