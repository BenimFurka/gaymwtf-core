# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-06-23

### Added
- Comprehensive documentation
- Additional test coverage
- New UI elements (`Element`, `Label`, `Button`)
- `on_left_interact` and `on_right_interact` methods to `Tile` and `Object`

### Changed
- Reorganized project structure:
  - Flattened module paths (e.g. `src/core/world/world` → `src/core/world`, `src/core/tile/tile` → `src/core/tile`, etc.)
  - Renamed `menu` module to `ui`
- Renamed interaction methods:
  - `Tile` and `Object` methods updated to `on_left_interact` and `on_right_interact`

## [0.1.0] - 2025-06-20
- Initial project setup
