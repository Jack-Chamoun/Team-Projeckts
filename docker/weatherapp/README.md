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

# 4. WetterApp im Container ausführen

## Docker installieren 

### Ubuntu

```bash
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null 

sudo apt-get update 

sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

### Windows

Folge den Befehlen auf dieser Seite: https://docs.docker.com/desktop/setup/install/windows-install/

## Image bauen und ausführen

```bash
cd weatherapp

docker build --no-cache -t wetterapp .

docker run -p 8080:8080 wetterapp:latest
```