# [Writing an OS in Rust](https://github.com/phil-opp/blog_os)を読みながら実装

Mac mini + macOS Mojaveの環境で表記の記事を読みながら実装していく。

## post-06: Double Faults

- gdtを設定して実行したところ、トリプルフォルトによるエンドレスリセットが発生。
    * コメント欄にあるようにTSSのスタックサイズを`4096 * 5`に変更すると解決した。
    * さらに、releaseモードで実行した場合は`4096`でも問題は発生しなかった。