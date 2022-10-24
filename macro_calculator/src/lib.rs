pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    if foods.len() == 0 {
        return json::parse(r#"
            {
                "cals": 0.0,
                "carbs": 0.0,
                "proteins": 0.0,
                "fats": 0.0
            }
            "#).expect("err in len");
    }
    let mut final_cals: f64 = 0.0;
    let mut final_carbs: f64 = 0.0;
    let mut final_proteins: f64 = 0.0;
    let mut final_fats: f64 = 0.0;

    for food in foods.iter()
    {
        final_cals += food.calories[1].replace("kcal", "").parse::<f64>().unwrap() * food.nbr_of_portions;
        final_carbs += format!("{:.2}", food.carbs * food.nbr_of_portions).parse::<f64>().unwrap();
        final_proteins += format!("{:.2}", food.proteins * food.nbr_of_portions).parse::<f64>().unwrap();
        final_fats += format!("{:.2}", food.fats * food.nbr_of_portions).parse::<f64>().unwrap();
    }
    final_cals = format!("{:.2}", final_cals).parse::<f64>().unwrap();

    let json =
        String::from("{") +
            &format!(r#""cals": {},"carbs": {},"proteins": {},"fats": {}"#, final_cals, final_carbs, final_proteins, final_fats) +
            "}";
    json::parse(&json).expect("err in return")
}