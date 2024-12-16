use lll::prelude::*;

#[derive(Debug)]
struct Vegetable {
    name: String,
}

impl Ingredient for Vegetable {}

fn main() {
    // デフォルトのレシピを作成
    let default_recipe = DefaultRecette;
    let dresser = Cercle;

    // 鍋を初期化
    let mut pot = Pot::init(default_recipe, dresser);

    // 食材を鍋に追加
    pot.add_ingredient(Vegetable {
        name: "Carrot".to_string(),
    });
    // 鍋を調理開始
    pot.cook();
}
