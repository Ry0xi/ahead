# ahead

Get ahead of time with your upcoming events!

This app shows you fullscreen reminders of your upcoming events.

## 要件定義

アプリケーションの要件定義は `docs/requirements/` 配下のYAMLファイルで管理しています。

YAMLファイルに記載した詳細な条件ををもとに生成AIでコードの生成を行えるようにするのが目的です。

YAMLファイルの更新自体を生成AIで行うことも可能です。

## 開発環境のセットアップ

### 必要なツール
- Rust (stable)
- ShellCheck: `brew install shellcheck`

### VSCode拡張機能
- [shellcheck](https://marketplace.visualstudio.com/items?itemName=timonwong.shellcheck): シェルスクリプトの静的解析

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

以下のチェックを自動実行しています：

**Rust CI**
- コードフォーマット（`cargo fmt`）
- 静的解析（`cargo clippy`）
- テスト（`cargo test`）

**Shell Check**
- シェルスクリプトの静的解析（`shellcheck`）

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
