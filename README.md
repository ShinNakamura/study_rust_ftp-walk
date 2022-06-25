# FTP(S)に接続して、rootに指定したディレクトリ以下を再帰的に辿ってファイルをリストアップする

Rust 勉強中の自習課題。

いろいろ雑です。 クローン、利用は自己責任でお願い致します。

## commadline

```sh
cargo run -- ftp.host.url 21 username passwd path/to/root
```

## Thank you

[SuppaFTP](https://github.com/veeso/suppaftp)
