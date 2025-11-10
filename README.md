# hamlib-sys

Low-level Rust bindings to **Hamlib** (radio control library) version **4.0**.

This crate provides unsafe FFI bindings generated from the Hamlib C headers. It does **not** offer a high-level Rust API. Users should depend on a safe wrapper crate instead (like a future `hamlib` crate I intend to make) unless direct access to the C interface is needed.

## Requirements

Hamlib must be installed on the system before building:

**Debian/Ubuntu:**
```sh
sudo apt install libhamlib-dev
```

**Arch Linux:**
```sh
sudo pacman -S hamlib
```
