# UMT Technical Context

## 使用技術

### コア技術

- TypeScript: メイン開発言語
- Bun: パッケージマネージャー & ランタイム
- Node.js: 実行環境の一つ
- ESLint: コード品質管理
- Biome: コードフォーマットとリント
- Jest: テストフレームワーク

### ビルドツール

- TypeScript Compiler (tsc)
- SWC (.swcrc): 高速なTypeScriptコンパイラ
- Babel: コード変換

### 開発環境

- Visual Studio Code: 推奨IDE
- Git: バージョン管理
- npm: パッケージ公開

## 開発環境セットアップ

### 必要要件

- Bun: 最新バージョン
- Node.js: LTS バージョン
- Git: 最新バージョン

### インストール手順

```bash
# リポジトリのクローン
git clone [repository-url]

# 依存関係のインストール
bun install

# ビルド
bun run build

# テスト実行
bun run test
```

### 開発コマンド

- `bun run build`: プロジェクトのビルド
- `bun run lint`: リント実行
- `bun run test`: テスト実行
- `bun run format`: コードフォーマット
- `bun run test src/tests/unit/path/to/test.test.ts`: 個別テスト実行

## 技術的制約

### コーディング規約

- ファイル命名: camelCaseまたはPascalCase
- インデント: 2スペース
- 行幅: 80文字
- デフォルトエクスポート禁止（named exportsを使用）
- インポート順序: builtin → external → internal → parent → sibling → index

### 型システム

- strict modeの使用必須
- 明示的なany型の使用禁止
- 一貫した配列型表記の使用
- 型定義ファイルの適切な管理

### テスト要件

- 新機能の単体テスト必須
- テストカバレッジの維持
- テストケースの明確な命名
- モック使用の最小化

### パフォーマンス

- バンドルサイズの最適化
- Tree-shakingの考慮
- 不要な依存関係の回避
- 効率的なアルゴリズムの使用

## 依存関係

### 主要な依存関係

```json
{
  "devDependencies": {
    "@swc/core": "最新バージョン",
    "@types/node": "最新バージョン",
    "@typescript-eslint/eslint-plugin": "最新バージョン",
    "@typescript-eslint/parser": "最新バージョン",
    "biome": "最新バージョン",
    "eslint": "最新バージョン",
    "jest": "最新バージョン",
    "typescript": "最新バージョン"
  }
}
```

### バージョニング

- セマンティックバージョニングの採用
- 定期的な依存関係の更新
- 破壊的変更の明確な記録
- 下位互換性の維持
