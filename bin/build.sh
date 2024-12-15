#!/bin/bash

# ターゲットプラットフォームの変数
TARGET_PLATFORM="x86_64-apple-darwin"
APP_PATH="target/$TARGET_PLATFORM/debug/bundle/osx/Ahead.app"
PLIST_COMMANDS=(
    ":NSCalendarsFullAccessUsageDescription string 'カレンダーの予定を参照するために権限が必要です。'"
)

# ビルドとバンドル作成
cargo bundle --target $TARGET_PLATFORM

# ターゲットとなるInfo.plistのパス
INFO_PLIST_PATH="$APP_PATH/Contents/Info.plist"

# Info.plistが存在することを確認
if [ ! -f "$INFO_PLIST_PATH" ]; then
    echo "Error: Info.plist not found at $INFO_PLIST_PATH"
    exit 1
fi

# 各コマンドを実行
for cmd in "${PLIST_COMMANDS[@]}"; do
    # PlistBuddyを使用してキーを追加
    /usr/libexec/PlistBuddy -c "Add $cmd" "$INFO_PLIST_PATH"
    
    # キーが既に存在する場合は、値を設定
    if [ $? -ne 0 ]; then
        /usr/libexec/PlistBuddy -c "Set $cmd" "$INFO_PLIST_PATH"
    fi
done

echo "Successfully updated Info.plist"

# アプリケーションに署名
codesign --force --deep --entitlements Ahead.entitlements --sign - "$APP_PATH"