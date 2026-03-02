# Oxide DB

**Oxide** は、Rust と WebAssembly (Wasm) で、ブラウザ上で爆速かつ大容量のデータ操作を実現する組み込み型データベースです。

## 🌟 ビジョン
「サーバーを立てるほどではないが、ブラウザのJSだけでは処理しきれない大量のデータ」を扱うモダンなWebアプリのための、データエンジンを目指します。

1. **High Speed**: Rustによるメモリ操作で、数万件のデータを一瞬で検索。
2. **Persistent**: IndexedDBと連携し、リロードしてもデータが消えない。
3. **Local-first**: 将来的にはP2PやCRDTを用いた「サーバーレスな共有」をサポート。

## 🛠 開発環境
- **Runtime**: WebAssembly (Wasm)
- **Core Logic**: Rust
- **Storage**: IndexedDB (via Browser)
- **Environment**: Rancher Desktop + VS Code Dev Containers

## 始め方

### 1. ビルド
コンテナ内のターミナルで以下のコマンドを実行します。
```bash
wasm-pack build --target web