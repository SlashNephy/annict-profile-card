# annict-profile-card
🔭 Annict の視聴状況などを SVG 画像として出力する API サーバ (WIP)

Annict [GraphQL API](https://developers.annict.jp/graphql-api) を使用しています。

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/SlashNephy/annict-profile-card)](https://github.com/SlashNephy/annict-profile-card/releases)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/SlashNephy/annict-profile-card/Docker)](https://hub.docker.com/r/slashnephy/annict-profile-card)
[![Docker Image Size (tag)](https://img.shields.io/docker/image-size/slashnephy/annict-profile-card/latest)](https://hub.docker.com/r/slashnephy/annict-profile-card)
[![Docker Pulls](https://img.shields.io/docker/pulls/slashnephy/annict-profile-card)](https://hub.docker.com/r/slashnephy/annict-profile-card)
[![license](https://img.shields.io/github/license/SlashNephy/annict-profile-card)](https://github.com/SlashNephy/annict-profile-card/blob/master/LICENSE)
[![issues](https://img.shields.io/github/issues/SlashNephy/annict-profile-card)](https://github.com/SlashNephy/annict-profile-card/issues)
[![pull requests](https://img.shields.io/github/issues-pr/SlashNephy/annict-profile-card)](https://github.com/SlashNephy/annict-profile-card/pulls)

## Endpoints

以下のエンドポイントは私用に設置しているものです。動作を保証しません。

安定版 (master ブランチ): `https://apps.starry.blue/annict-profile-card`  
開発版 (dev ブランチ): `https://apps.starry.blue/annict-profile-card-dev`

### /watching/{username}

今期視聴しているアニメ一覧を返します。表示される作品は Annict 上で「見てる」を設定したものが対象です。

|クエリパラメータ|デフォルト値|説明|
|---|:---:|---|
|`season`|**現在のシーズン**|表示するシーズンを `2021-summer` という形式で指定します。`all` を指定した場合, すべてのシーズンが対象です。|
|`bg_color`|`1a1b27`|背景の色を hex で指定します。|
|`header_color`|`70a5fd`|ヘッダーの色を hex で指定します。|
|`text_color`|`d6e3e1`|文字の色を hex で指定します。|
|`icon_color`|`bf91f3`|アイコンの色を hex で指定します。|
|`title_color`|`38bdae`|タイトルの色を hex で指定します。|
|`limit_works`|`10`| 表示する作品数を指定します。 |
|`limit_images`|`3`| 表示する作品のアイキャッチ画像の数を指定します。 |
|`sort`|`satisfaction`| 作品一覧をソートする方法を指定します。<br>`satisfaction` の場合, 満足度 % の値で降順にソートします。<br>`watcher` の場合, 視聴者数の数で降順にソートします。 |
|`order`|`desc`| ソートする方向を指定します。<br>`desc` の場合は降順で, `asc` の場合は昇順になります。|
|`expose_image_url`|`false`| `true` の場合, SVG 画像内に埋め込まれる画像を外部 URL で埋め込みます。<br>`false` の場合, Base64 エンコードされた画像が埋め込まれます。 |

![image](https://user-images.githubusercontent.com/7302150/153339724-98ebbd59-038e-4abe-89d2-d2ebf6eabb18.png)

## Known Issue

- GitHub 上に貼り付ける場合 `expose_image_url=false` が必要
  GitHub などのサイトに貼り付ける場合には CORS の関係で画像は Base64 エンコードして埋め込む必要があります。

- GitHub 上では作品のアイキャッチ画像が表示できない  
  GitHub では SVG 画像の長さ制限?があるようでエンコードしても表示できません。`limit_images=0` でアイキャッチ画像を無効化できます。

## Docker

`docker-compose.yml`

```yaml
version: '3.8'

services:
  server:
    container_name: annict-profile-card
    image: ghcr.io/slashnephy/annict-profile-card:master
    restart: always
    ports:
      - 8080:8080/tcp
    environment:
      ANNICT_TOKEN: xxx  # https://annict.jp/settings/tokens/new で発行できます
      RUST_LOG: info,annict_profile_card=debug
```
