pub enum Tables {
    Ingredients,
    Menus,
    PrepLists,
    Recipes,
    Inventory,
    Restaurants,
    ProductOrder,
    Suppliers,
    Users,
}

impl Tables {
    pub fn value(&self) -> &str {
        match *self {
            Tables::Ingredients => "ingredients",
            Tables::Menus => "menu",
            Tables::PrepLists => "prep_list",
            Tables::Recipes => "recipe",
            Tables::Inventory => "inventory",
            Tables::Restaurants => "restaurants",
            Tables::ProductOrder => "product_order",
            Tables::Suppliers => "suppliers",
            Tables::Users => "users",
        }
    }
}
