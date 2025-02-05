## Depedências instaladas

```toml
[package]
name = "snake_game"
version = "0.1.0"
edition = "2021"

[dependencies]
# O wasm-bindgen é uma biblioteca essencial para integração entre Rust e WebAssembly
wasm-bindgen = "0.2.63"

[lib]
# Define o tipo de crate (unidade de código em Rust) que será produzido como saída. O cdylib significa que o projeto será compilado como uma biblioteca dinâmica (shared library) compatível com C e outras linguagens (neste caso, para WebAssembly)
crate-type = ["cdylib"]


```

## Comandos executados

### Instalar pacote
```bash
cargo install wasm-pack
```

### Criar o pkg

- O comando abaixo cria o pkg, porém dentro da pasta www -> package.json está em um script para ser rodado

```bash
wasm-pack build --target web
```
> ℹ️ **Info**
> Esse pkg é referenciado dentro do package.json da seguinte forma: `"snake_game": "file:../pkg"`
