## volvelle / codex32-website

This repository hosts the website for [secretcodex32.com](https://secretcodex32.com).

The site can be built using the [Nix](https://nixos.org)
package manager (with flakes enabled): 

1. `$ nix build .#codex32-website`
2. `cd results/www`
3. `nix run nixpkgs#python3 -- webserver.py`