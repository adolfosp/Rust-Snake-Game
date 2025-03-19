## Projeto Original

- [GitHub](https://github.com/Jerga99/snake-rust-game)


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


## Explicação

### Derive

Essa linha de código em Rust está utilizando a macro `derive` para automaticamente implementar a trait `PartialEq` para a estrutura ou enum que está sendo definida. A trait `PartialEq` permite comparar se duas instâncias de um tipo são iguais ou não, utilizando o operador `==`.

Por exemplo, se você tiver uma estrutura como esta:

```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

Com a derivação de `PartialEq`, você pode comparar duas instâncias de `Point` diretamente:

```rust
let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 1, y: 2 };
let p3 = Point { x: 3, y: 4 };

assert!(p1 == p2); // true
assert!(p1 != p3); // true
```

Sem a derivação de `PartialEq`, você teria que implementar manualmente a lógica de comparação para a estrutura `Point`.