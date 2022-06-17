### About

Rust CLI/GUI software updater for Linux/Windows/macOS

!!! Doesn't work at the moment because interactive input for the child process is broken (for sudo) !!!

### TODO

#### Core

##### General

* [ ] TOML Config
* [ ] Deal with sudo asking the password 

##### Linux support:

* [x] `pacman`-based distros
  * [x] pacman 
  * AUR:
    * [x] pamac
    * [x] paru
    * [x] yay
* [x] `deb`-based distros
  * [x] apt 
  * [x] aptitude 
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
* [ ] Rust
* [ ] Dart
* [ ] Python
* [ ] Go

#### CLI
* [ ] Proper functionality

#### GUI
* [ ] GTK
* [ ] QT
