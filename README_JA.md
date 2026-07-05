# Rustの学習

『The Rust Programming Language』（https://doc.rust-lang.org/stable/book/）（通称「The Book」）に沿った個人的な学習プロジェクト。

## プロジェクト

| ディレクトリ | 書籍の章 | トピック |
|---|---|---|
| `hello_cargo/` | 第1章 — はじめに | Cargoを使ったHello World; 基本的な入出力（初期段階の推測ゲームのスケッチ） |
| `src/` (ルート) | 第2章 — 推測ゲームのプログラミング | 乱数、ループ、およびmatchを使用した完全な推測ゲーム |
| `variables/` | 第3章 — 一般的なプログラミング概念 | 変数、可変性、およびシャドウイング |

## プロジェクトの実行

[Rustがインストールされていること](https://www.rust-lang.org/tools/install)を確認してから、Cargoを使用します:

```bash
# 推測ゲームを実行（ルートプロジェクト）
cargo run

# サブプロジェクトを実行
cargo run --manifest-path hello_cargo/Cargo.toml
cargo run --manifest-path variables/Cargo.toml
```

## 進捗状況

- [x] 第1章 — はじめに
- [x] 第2章 — 推測ゲームのプログラミング
- [x] 第3章 — 一般的なプログラミング概念（変数とシャドウイング）
- [x] 第4章 — 所有権の理解
- [ ] 第5章 — 構造体の使用
  -  5－1まで終わり  
- [ ] 第6章 — 列挙型とパターンマッチング
- [ ] ...

## リファレンス

- 書籍: https://doc.rust-lang.org/stable/book/
- 標準ライブラリのドキュメント: https://doc.rust-lang.org/std/
- Cargoのドキュメント: https://doc.rust-lang.org/cargo/
