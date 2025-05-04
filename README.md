# Markdown to HTML Converter

MarkdownファイルをHTMLに変換するシンプルなコマンドラインツールです。

## 機能

- MarkdownファイルをHTMLに変換
- 以下のMarkdown要素をサポート：
  - 見出し
  - 水平線
  - 太文字
  - リスト
  - 番号付きリスト
  - テーブル

## インストール

```bash
git clone https://github.com/yourusername/markdown2html.git
cd markdown2html
cargo build --release
```

## 使用方法

```bash
cargo run markdown <入力ファイル> <出力ファイル>
```

例：
```bash
cargo run markdown inputfile.md output.html
```
