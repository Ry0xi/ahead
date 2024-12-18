# 要件定義 TODO リスト

## 共通要件
- [x] バックグラウンド実行の基本要件
- [x] システム要件（macOS 12以降）
- [x] 基本的な技術スタックの決定
- [ ] システムリソース使用量の具体的な制限値の設定
- [ ] エラーログ取得・管理方針の策定
- [ ] デバッグモードの要件定義
- [ ] アプリケーションの配布方法の要件
- [ ] アップデート方法の要件

## カレンダー連携機能
- [x] EventKitを使用した基本連携の要件
- [x] 権限管理の要件
- [x] カレンダーデータの同期頻度の定義
- [x] 取得する予定情報の範囲の定義
  - [x] 過去の予定の扱い
  - [x] 未来の予定の取得範囲
- [x] エラー時のリトライ戦略の詳細化
- [ ] オフライン時の動作要件
  - [ ] オフライン時のデータ保持期間
  - [ ] オフライン時の通知動作
  - [ ] オンライン復帰時の同期戦略

## リマインド表示機能
- [x] 基本的な表示要件
- [x] 技術スタック（iced + objc2）の決定
- [ ] UI/UXの詳細仕様
  - [ ] 通知ウィンドウのサイズ
  - [ ] レイアウトの詳細
  - [ ] カラースキーム
  - [ ] フォントタイポグラフィ
- [ ] 通知の表示ルール
  - [ ] 表示開始タイミングの詳細
  - [ ] 表示継続時間
  - [ ] 消失条件
- [ ] マルチディスプレイ対応
  - [ ] 表示位置の決定ロジック
  - [ ] ディスプレイ切り替え時の動作
- [ ] キーボードショートカットの定義
- [ ] アクセシビリティ要件の詳細化

## テスト要件
- [ ] 単体テストの要件
- [ ] 統合テストの要件
- [ ] E2Eテストの要件
- [ ] パフォーマンステストの要件
- [ ] セキュリティテストの要件

## ドキュメント要件
- [ ] ユーザーマニュアルの要件
- [ ] 開発者ドキュメントの要件
- [ ] APIドキュメントの要件
- [ ] リリースノートの要件

## セキュリティ要件
- [ ] データ保護要件
- [ ] 権限管理の詳細要件
- [ ] プライバシー保護要件
- [ ] セキュリティ監査要件

## パフォーマンス要件
- [ ] 起動時間の目標値
- [ ] メモリ使用量の目標値
- [ ] CPU使用率の目標値
- [ ] ディスク使用量の制限 