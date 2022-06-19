### About

Rust CLI/GUI software updater for Linux/Windows/macOS0

### TODO

#### Core

##### General

* [ ] TOML Config 
* [ ] Error handling with proper error messages every step of the way

##### Linux support:

* [x] `pacman`-based distros
  * [x] pacman
  * [x] pamac
  * [x] paru (default for now, falls back to first available)
  * [x] yay
* [x] `deb`-based distros
  * [x] apt 
  * [x] aptitude (default for now, falls back to apt)
* [ ] `rpm`-based distros
  * [ ] yum 
  * [ ] dnf 
  * [ ] zypper 
* [ ] `portage`-based distros
* [ ] `eopkg`-based distros
* [ ] `nix-channel`-based distros
* [ ] `apk`-based distros
* [ ] `snap`
* [ ] `flatpak`
* [ ] `brew`

##### Windows support:
* [ ] `choco`
* [ ] `winget`

##### MacOS support:
* [ ] `brew`

##### Programming languages and installed packages
* [x] Rust
  * [x] rustup
  * [x] cargo (via cargo-update subcommand)
* [ ] Dart
* [ ] Python
* [ ] Go
* [ ] JS
  * [ ] npm
  * [ ] yarn

#### CLI
* [ ] Proper functionality

#### GUI
* [ ] GTK
* [ ] QT
