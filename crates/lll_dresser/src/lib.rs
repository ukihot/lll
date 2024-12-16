use crossterm::{
    cursor,
    style::{Color, PrintStyledContent, Stylize},
    terminal,
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use lll_recette::Dresser;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Cercle;

impl Dresser for Cercle {
    fn render(&self) {
        let mut stdout = stdout();
        // 初回の画面クリア
        stdout.execute(Clear(ClearType::All)).unwrap();

        // ターミナルのサイズを取得
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));

        // ターミナルの制御（カーソルを非表示にする）
        stdout.execute(cursor::Hide).unwrap();

        loop {
            // 枠の描画
            for row in 0..height {
                for col in 0..width {
                    // 枠の縁を描画
                    if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
                        stdout
                            .queue(cursor::MoveTo(col, row)) // カーソル移動
                            .unwrap()
                            .queue(PrintStyledContent("■".green())) // 緑色の■を表示
                            .unwrap();
                    }
                }
            }

            // 描画を反映
            stdout.flush().unwrap();
        }
    }
}
