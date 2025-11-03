# MiniGrep

### cmd build
cargo build --release

./target/release/minigrep body assets/poem.txt >> runtime/output.txt

### cmd run
cargo run -- 北冥 assets/poemchinese.txt
cargo run -- 庄子 assets/poemchinese.txt

* one
cargo run -- frog assets/poem.txt
* multi
cargo run -- body assets/poem.txt
* no
cargo run -- monomorphization assets/poem.txt
* 带环境变量
IGNORE_CASE=1  cargo run -- to assets/poem.txt

* 将错误信息保存到文件
cargo run > output.txt

* 将获取结果保存到文件中,覆盖
cargo run -- to assets/poem.txt > runtime/output.txt
* 追加内容
cargo run -- to assets/poem.txt >> runtime/output.txt

#### vscode
* debug 调试
CodeLLDB

### 生成文档
cargo doc --open

### 创建 crates
cargo new logger --lib