# ahead

Get ahead of time with your upcoming events!

This app shows you fullscreen reminders of your upcoming events.

## 要件定義

アプリケーションの要件定義は `docs/requirements/` 配下のYAMLファイルで管理しています。

YAMLファイルに記載した詳細な条件ををもとに生成AIでコードの生成を行えるようにするのが目的です。

YAMLファイルの更新自体を生成AIで行うことも可能です。

## 開発環境のセットアップ

### Git hooks

コミット時に自動でコードチェックを実行するために、以下のコマンドでGit hooksを設定してください：

```sh
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
```

これにより、コミット時に以下のチェックが自動実行されます：
- `cargo fmt`: コードフォーマットチェック
- `cargo clippy`: 静的解析
- `cargo test`: テストの実行

### CI/CD

GitHub Actionsで以下のチェックを自動実行しています：
- コードフォーマット（`cargo fmt`）
- 静的解析（`cargo clippy`）
- テスト（`cargo test`）

## ビルドと実行

アプリケーションのビルド：
```sh
./bin/build.sh
```

アプリケーションの実行：
```sh
open target/x86_64-apple-darwin/debug/bundle/osx/Ahead.app
```

> [!NOTE]
> アプリケーションの実行はmacOSでのみ可能です。
