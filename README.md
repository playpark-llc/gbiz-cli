# gbiz-cli

gBizINFO (経済産業省) 法人公開情報 REST API のコマンドラインインターフェース。

## セットアップ

### インストール

```sh
cargo install --path .
```

`~/.cargo/bin/gbiz` にインストールされる。更新時も同じコマンドで上書きできる。

> `~/.cargo/bin` に PATH が通っていることを確認すること。

### 認証

環境変数または `--token` フラグで API トークンを指定する。

```sh
export GBIZ_API_TOKEN=your-api-token
```

トークンは https://info.gbiz.go.jp/api/registration から申請・取得する。

## 使い方

```sh
# 法人名で検索
gbiz search "プレイパーク"
gbiz search "環境公害センター" --limit 5

# 法人詳細 (全カテゴリ一括)
gbiz get 1234567890123

# 財務情報
gbiz finance 1234567890123

# 特許・商標
gbiz patent 1234567890123

# 官公需調達実績
gbiz procurement 1234567890123

# 補助金情報
gbiz subsidy 1234567890123

# 届出・認定
gbiz certification 1234567890123

# 表彰情報
gbiz commendation 1234567890123

# 職場情報 (平均年齢, 勤続年数, 残業, 女性比率)
gbiz workplace 1234567890123

# 事業所情報
gbiz corporation 1234567890123
```

### 検索オプション

```sh
gbiz search "企業名" \
  --prefecture 13 \
  --capital-from 1000000 \
  --capital-to 100000000 \
  --employee-from 10 \
  --employee-to 1000 \
  --limit 20 \
  --page 1
```

## グローバルオプション

| フラグ | 環境変数 | 説明 |
|--------|----------|------|
| `--token` | `GBIZ_API_TOKEN` | API トークン (必須) |
| `-o, --output` | - | 出力形式: `json` (default), `table`, `compact` |
| `-v, --verbose` | - | 生レスポンスを stderr に出力 |

## 出力形式

- **json** — 整形済み JSON (デフォルト)
- **table** — 罫線付きテーブル表示
- **compact** — 1行1レコードの簡易表示

テーブル表示に対応しているコマンド: `search`, `get`, `finance`, `subsidy`, `workplace`。
その他のコマンドは table/compact 指定時も JSON にフォールバックする。
