![Kuramoto Logo](./assets/kuramoto-logo.png)

<h1 align="center">Kuramoto</h1>

<p align="center">
  <a href="https://github.com/leeni514/kuramoto/releases/latest">
    <img src="https://img.shields.io/github/v/release/leeni514/kuramoto?label=latest%20release" alt="Latest release">
  </a>
  <a href="https://github.com/leeni514/kuramoto/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/leeni514/kuramoto" alt="License">
  </a>
  <a href="https://github.com/leeni514/kuramoto/releases">
    <img src="https://img.shields.io/github/downloads/leeni514/kuramoto/total" alt="Release downloads">
  </a>
</p>

Kuramoto is a visual novel library, launcher, and playtime tracker built with Rust and Dioxus-CLI. It's a fork of [Kakera](https://github.com/enotan/kakera) by enotan, themed after China Kuramoto from Gakuen iDOLM@STER.

## Overview

Kuramoto works as a hub for launching and keeping track of all your visual novels.
The majority of people use Steam as a central launcher for all their games, but many visual novels aren't available on Steam, and many are forced to buy from other sources which can get hard to keep track of. Kuramoto lets you add visual novels from VNDB, launch them, show a Discord Rich Presence, and track playtime.

## Features

- Add visual novels manually or search from VNDB
- Browse visual novels in a cover-art library grid
- View and edit descriptions, notes, routes, and cover images
- Add and track active routes and completed routes
- Allows running visual novels with Wine for Linux operating systems, with settings to change locale and add custom arguments
- Track play sessions and total playtime
- See playtime per day, a summary of games played, and a year-long activity calendar on the Playtime page
- Caches VN cover images locally so internet connection isn't always required to keep a nice looking library
- Shows Discord Rich Presence while a VN is running, customizable in settings
- Stores library and settings as local JSON files

## Coming from Kakera

The first time Kuramoto starts, it looks for a Kakera library on the same computer and copies it over: your visual novels with their playtime and sessions, your settings, and cached covers. Kakera's own data isn't changed, so both apps can stay installed. After the first launch the two keep separate libraries.

If you launch through Steam on Linux, update the Steam launch option to the one shown in Kuramoto's settings, since the wrapper now lives in Kuramoto's data folder.

## Supported Platforms

Kuramoto currently supports:

- Linux
- Windows

macOS isn't supported yet.

## Installation

### Windows

```
Download the latest installer from Releases and that's it.
```

### All Linux Distributions

#### Flatpak (Recommended)

Flatpak is the recommended package for most Linux distributions. It can avoid certain issues that may be faced when trying to use the AppImage.

```bash
# Download the latest .flatpak from Releases
# Run:
flatpak install --user ./Kuramoto_*.flatpak

flatpak run io.github.leeni514.Kuramoto
```

#### AppImage

```bash
# Download the latest .AppImage from Releases
# Run:
chmod +x kuramoto_*.AppImage

./kuramoto_*.AppImage

```

### NixOS

Run Kuramoto directly without installing it:

```bash
nix run github:leeni514/kuramoto
```

Install Kuramoto into your user profile:

```bash
nix profile add github:leeni514/kuramoto
```

#### NixOS Flake

Installing Kuramoto through your NixOS config is recommended, as it installs it as
a system application with its icon and desktop entry.

Add Kuramoto to your `flake.nix` inputs:

```nix
{
    inputs.kuramoto.url = "github:leeni514/kuramoto";
}
```

Pass the input to your NixOS modules:
```nix
outputs =
    { nixpkgs, kuramoto, ... }:
    {
        nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
            system = "x86_64-linux";
            specialArgs = { inherit kuramoto; };
            modules = [ ./configuration.nix ];
        };
    };
```

Then add Kuramoto to `environment.systemPackages` in `configuration.nix`:

```nix
{ pkgs, kuramoto, ... }:

{
    environment.systemPackages = [
        kuramoto.packages.${pkgs.stdenv.hostPlatform.system}.default
    ];
}
```

## Building from Source

### Dependencies

#### Arch Linux Based Systems

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module libappindicator-gtk3 librsvg xdotool xdg-utils rust cargo dioxus-cli
```

#### Debian Based Systems

```bash
sudo apt install rustc cargo libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev xdg-utils lld
cargo install dioxus-cli
```

#### Fedora Based Systems

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libxdo-devel libappindicator-gtk3-devel librsvg2-devel xdg-utils rust cargo
cargo install --locked dioxus-cli
```

#### Optional Runtime Tools

```
# For running Windows visual novels on Linux
wine

# For using Proton
umu-launcher

# For Discord Rich Presence
discord # obviously. vesktop, betterdiscord, etc should all work too
```

### Building

```bash
git clone https://github.com/leeni514/kuramoto

cd kuramoto

dx build --release
```

## Data Storage

Kuramoto stores data in the platform app-data directory.

On Linux systems, this is usually:

```text
~/.local/share/kuramoto
```

On Windows, this is usually:

```text
C:\Users\<USER>\AppData\Roaming\kuramoto
```

## Planned Features

- Controller navigation mainly for handheld PCs
- Windows locale emulator support
- Fuller notes view with characters (can be imported from VNDB), rich-text formatting and editing (For all the mystery lovers like me)
- More themes + custom theming

## Tech Stack

- Rust 
- Dioxus for UI
- VNDB HTTP API
- Discord Rich Presence IPC
