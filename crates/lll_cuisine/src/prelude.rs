pub use lll_dresser::*;
pub use lll_frigo::*;
pub use lll_ingredient::*;
pub use lll_recette::*;

pub struct Pot<I, R, D>
where
    I: Ingredient,
    R: Recette<I, D>,
    D: Dresser + Clone,
{
    ingredients: Vec<I>, // 食材リスト
    recette: R,          // 現在のルセット
    dresser: D,          // 現在のドレッサー
}

impl<I, R, D> Pot<I, R, D>
where
    I: Ingredient,
    R: Recette<I, D>,
    D: Dresser + Clone,
{
    pub fn init(recette: R, dresser: D) -> Self {
        Self {
            ingredients: Vec::new(),
            recette,
            dresser,
        }
    }

    pub fn add_ingredient(&mut self, ingredient: I) {
        self.ingredients.push(ingredient);
    }

    pub fn cook(&mut self) {
        loop {
            // レシピ実行処理
            if !self.recette.execute(&mut self.ingredients, &self.dresser) {
                break;
            }
        }
    }

    pub fn change_pot<N>(&mut self, new_recette: N) -> Pot<I, N, D>
    where
        N: Recette<I, D>,
    {
        Pot {
            ingredients: self.ingredients.drain(..).collect(),
            recette: new_recette,
            dresser: self.dresser.clone(),
        }
    }
}
