# Changelog
## 0.3.0
* Update to egui 0.31.0 by @hacknus.
* Raise MSRV to 1.81.0 (this matches egui's MSRV).

## 0.2.3
* Raise MSRV to 1.80.0 (this matches egui's MSRV).

## 0.2.2
* Update to egui 0.30.0.

## 0.2.1
* Remove PNG files from package.

## 0.2.0
* Update to egui 0.29.0
  * egui now provides its own `ThemePreference` type, so ours was removed.
* Added a new `global_theme_switch` function that controls egui's global theme preference.

## 0.1.1
* Added serde `Serialize` and `Deserialize` support to `ThemePreference` (gated by the `serde` feature).

## 0.1.0
Initial release
