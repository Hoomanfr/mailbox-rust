# To Update rust
```
rustup update
```

# Uninstall rust
```
$ rustup self uninstall

```

# Rust Version
```
rustc --version
```
# Run rust
```
rustc main.rs
./main
```

# New cargo project
checkout:
https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
```
cargo new <project_name>
```

# Building and Running cargo
```
cargo build
./target/debug/<project_name>
```
**OR**
```
cargo run
```
**For Release**
it'll generate executable at: `target/release`
```
cargo build --release
```

**Instal SQLX Migration CLI PostgreSql**
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```
for more info checkout:
```
https://crates.io/crates/sqlx-cli
```
