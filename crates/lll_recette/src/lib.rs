use lll_ingredient::*;
use std::fmt::Debug;

/// レシピのインターフェース
/// - `I`: 食材 (Ingredient トレイトを実装)
/// - `D`: 描画機能 (Dresser トレイトを実装)
pub trait Recette<I, D>
where
    I: Ingredient,
    D: Dresser,
{
    /// レシピを実行し、必要に応じて食材を操作します。
    /// 戻り値:
    /// - `true` -> 継続実行
    /// - `false` -> 終了
    fn execute(&mut self, ingredients: &mut Vec<I>, dresser: &D) -> bool;
}

/// 食材を表示・描画するためのインターフェース
pub trait Dresser {
    /// 表示処理を実装する関数
    fn render(&self);
}

/// 単純な例として、デフォルトのレシピを定義
/// - 食材リストを単に出力するだけの動作
pub struct DefaultRecette;

impl<I, D> Recette<I, D> for DefaultRecette
where
    I: Ingredient + Debug,
    D: Dresser,
{
    fn execute(&mut self, ingredients: &mut Vec<I>, dresser: &D) -> bool {
        // 描画処理
        dresser.render();

        true // 常に継続実行
    }
}
