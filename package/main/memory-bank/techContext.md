# UMT Technical Context

## 使用技術

### コア技術

- TypeScript: メイン開発言語
- Bun: パッケージマネージャー & ランタイム（Nix flake の `devShell` 経由）
- Node.js 20 / 22 / 24: 実行環境（ESM）
- ESLint: コード品質管理
- Biome: コードフォーマットとリント
- Jest + SWC: テスト

### ビルドツール

- TypeScript Compiler (`tsc`) + `tsc-alias`（パスエイリアス解決）
- SWC (`.swcrc`): Jest 用の高速 TypeScript 変換
- 出力は ESM のみ（`package.json` の `"type": "module"`）。CommonJS / Babel ビルドは v5 で削除

### 開発環境

- Git: バージョン管理
- Nix flakes: `package/main` の npm scripts は `nix develop --command make …` を呼ぶ
- npm: パッケージ公開

## 開発環境セットアップ

### 必要要件

- Nix（flakes）: `package/main` の `bun run *` スクリプト用
- Bun: flake の `devShell` が提供
- Node.js: LTS（公開パッケージの実行環境）
- Git

### インストール手順

```bash
git clone https://github.com/riya-amemiya/UMT.git
cd UMT/package/main

# flake の bun を使う
nix develop -c bun install
nix develop -c bun run test
```

リポジトリ全体の Cloud Agent bootstrap は `.cursor/install.sh`。

### 開発コマンド

- `bun run build`: TypeScript を `./module/` にコンパイル
- `bun run lint`: ESLint + Biome + `tsc`
- `bun run test`: Jest
- `bun run format`: Biome format
- `bun run test src/tests/unit/path/to/test.test.ts`: 個別テスト
- `bun run readme`: typedoc 出力から README の Function List を再生成（`## Function List` 以降を置き換える）

## 技術的制約

### コーディング規約

- ファイル命名: camelCaseまたはPascalCase
- インデント: 2スペース
- 行幅: 80文字
- デフォルトエクスポート禁止（named exportsを使用）
- インポート順序: builtin → external → internal → parent → sibling → index
- 1 ファイル 1 実行時 export（型だけのファイルは別）

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
- Tree-shakingの考慮（ESM subpath exports）
- 不要な依存関係の回避（ランタイム依存ゼロ）
- 入力検証は呼び出し側の責任（無効引数で throw しない）

## 依存関係

ランタイム依存なし。開発依存のみ（ESLint、Biome、Jest、TypeScript、typedoc など）。

### バージョニング

- セマンティックバージョニングの採用
- v5: ESM-only。詳細は `COMPATIBILITY.md`
- 定期的な依存関係の更新
- 破壊的変更の明確な記録
- 下位互換性の維持（同一メジャー内）
