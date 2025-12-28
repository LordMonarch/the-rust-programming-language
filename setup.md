# Installation 
## Windows
Installer von Webseite herunterladen.
Darauf achten VisualStudio für C++ Desktopanwendungen zu installieren!

## Linux
Herunterladen und Installieren:
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Version prüfen
Prüfen welche Version installiert wurde:
```
rustc --version
```

## PATH prüfen
Windows CMD:
```
echo %PATH%
```

Windows PowerShell:
```
echo $env:Path
```

Linux:
```
echo $PATH
```

# Update
```
rustup update
```

# Uninstall
```
rustup self uninstall
```
# Cargo
Neues Projekt anlegen:
```
cargo new <projekt_name>
```

Cargo.toml Datei anlegen:
```
cargo init
```

Code bauen (Zielordner ist *taget/debug*):
```
cargo build
```

Code bauen **und ausführen**:
```cargo run```

Code auf Fehler prüfen:
```cargo check```

Code Release bauen (Zielordner ist *target/release*) Bauen dauert länger, der Build wird aber Performance Optimiert:
```cargo build --release```

Versionen updaten. Erst *Cargo.toml* anpassen dann:
```cargo update```

Dokumentation der verwendeten Bibliotheken im Browser öffen:
```cargo doc --open```

