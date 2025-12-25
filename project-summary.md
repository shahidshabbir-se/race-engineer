# Race Engineer - Technical Implementation Summary

## Tech Stack

**Frontend:** Svelte/SvelteKit - picked this because it's reactive and lightweight, which works well for displaying racing data without unnecessary overhead.

**Backend:** Rust + Tauri - went with this instead of Electron because Rust handles file parsing really efficiently, and the final app size is way smaller.

**UI:** TailwindCSS for styling

## How It Works

The app has a pretty straightforward setup - Rust backend handles all the heavy lifting with data parsing, and Svelte frontend displays everything.

### Backend (Rust side - `src-tauri/src/core/`)

Built custom parsers for RBR configuration files. Each data type (cars, stages, settings, etc.) has its own module with parsing logic. The main `data_loader.rs` reads INI and JSON files from the RBR installation directory.

All the data models are defined in Rust structs, which makes type safety much easier to manage.

### Communication Layer

Used Tauri commands to expose Rust functions to the frontend. The Svelte components call these using `invoke()`, and Tauri handles all the serialization between Rust and JavaScript automatically.

### Frontend (Svelte - `app/src/`)

Split everything into components for different views - cars, stages, settings, etc. The dashboard ties everything together. Svelte's reactivity means the UI updates automatically when data changes, no manual DOM manipulation needed.

## Main Features

- Parses various RBR data formats (JSON and INI files)
- Reads car data, stage information, car groups, and model details
- Settings management with persistence (writes back to INI format to stay compatible with RBR)
- All data is cached in memory for quick access

## Why These Choices

Rust was necessary because some of these data files are large, and parsing them in JavaScript would be slow. Tauri gives us native performance while keeping the bundle small.

Svelte made the frontend development faster - less boilerplate than React, and the compiled output is tiny.

## Project Layout

```
src-tauri/          - Rust backend, parsing logic, Tauri config
app/src/            - Svelte components and routes
data/rbr/           - Sample data files for testing
```

The app reads directly from RBR's installation folder, processes the config files, and presents everything in a clean interface. Works on Windows, macOS, and Linux since Tauri handles cross-platform builds.
