pub enum Tables {
    Ingredients,
    Menus,
    PrepLists,
    Recipes,
    Restaurants,
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
            Tables::Restaurants => "restaurants",
            Tables::Suppliers => "suppliers",
            Tables::Users => "users",
        }
    }
}
