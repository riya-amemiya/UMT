# UMT Go

UMTのGo言語実装です。

## インストール

```bash
go get github.com/riya-amemiya/umt-go
```

## 使用方法

### 乱数生成

```go
package main

import (
    "fmt"
    "github.com/riya-amemiya/umt-go/math"
)

func main() {
    // 0から10までの乱数を生成
    n, err := math.Random(10)
    if err != nil {
        panic(err)
    }
    fmt.Printf("0-10の乱数: %d\n", n)

    // 5から10までの乱数を生成
    n, err = math.Random(10, 5)
    if err != nil {
        panic(err)
    }
    fmt.Printf("5-10の乱数: %d\n", n)

    // エラーハンドリングが不要な場合（エラー時はパニック）
    n = math.MustRandom(10, 5)
    fmt.Printf("5-10の乱数: %d\n", n)
}
```

## 開発

このプロジェクトでは以下のMakeコマンドが利用可能です：

- `make all`: フォーマット、チェック、テスト、ビルドを実行
- `make fmt`: コードのフォーマットを実行
- `make test`: テストを実行
- `make check`: 静的解析を実行
- `make build`: プロジェクトをビルド
- `make clean`: 生成ファイルを削除
- `make coverage`: カバレッジレポートを生成

## バージョン管理

このプロジェクトは[セマンティックバージョニング](https://semver.org/lang/ja/)に従います。
バージョンはGitのタグで管理され、以下のような形式で付与されます：

```bash
git tag v1.0.0
git push origin v1.0.0
```

利用する際は、Go Modulesの機能を使って特定のバージョンを指定できます：

```bash
go get github.com/riya-amemiya/umt-go@v1.0.0
```

## ライセンス

[MIT License](LICENSE)
