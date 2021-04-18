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

安定版 (master ブランチ): `https://annict-card.starry.blue`  
開発版 (dev ブランチ): `https://annict-card-dev.starry.blue`

### /watching/{username}

今期視聴しているアニメ一覧を返します。表示される作品は Annict 上で「見てる」を設定したものが対象です。

|クエリパラメータ|デフォルト値|説明|
|---|:---:|---|
|`expose_image_url`|`false`| `true` の場合, SVG 画像内に埋め込まれる画像を外部 URL で埋め込みます。<br>`false` の場合, Base64 エンコードされた画像が埋め込まれます。<br>GitHub などのサイトに貼り付ける場合には CORS の関係で Base64 エンコード画像しか表示されません。 |

[![watching](https://annict-card.starry.blue/watching/SlashNephy)](https://annict-card.starry.blue/watching/SlashNephy)

## Docker

`docker-compose.yml`

```yaml
version: '3.8'

services:
  server:
    container_name: annict-profile-card
    image: slashnephy/annict-profile-card
    restart: always
    ports:
      - 8080:8080/tcp
    environment:
      ANNICT_TOKEN: xxx  # https://annict.jp/settings/tokens/new で発行できます
      RUST_LOG: info,annict_profile_card=debug
```
