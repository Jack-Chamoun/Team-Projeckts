# Voraussetzungen

## 1. Rust Version 1.86.0

#### Installation

##### Linux: 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### Windows: 

[Klicke hier, um auf die Installationsseite von Rust zu gelangen.](https://www.rust-lang.org/tools/install)

#### Überprüfung 

```bash
rustc --version
```

#### Update

```bash
rustup update
```

## 2. Trunk

#### Installation

```bash
cargo install trunk
```

#### Überprüfung

```bash
trunk --version
```

## 3. Target for Compiling to WebAssembly (nur bei Fehler von `trunk serve`)

#### Installation

```bash
rustup target add wasm32-unknown-unknown
```

# Ausführung

```bash
trunk serve [--open]
```

Der Parameter `--open` ist hier optional. Er öffnet den Browser, sodass er nicht manuell geöffnet werden muss. 
