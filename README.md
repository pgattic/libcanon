
# Libcanon

A Rewrite of Canon as a rust library (and as much more!)

As of right now, this repository is mainly used for planning and prototyping. No decisions are final.

## Application Stack

| Level             | Description | Technologies |
|-------------------|-------------|--------------|
| 1. Canon Packages | The content of the installed books. Managed by libcanon | JSON, Protobuf, YAML, TOML |
| 2. Marks          | The annotations, notes, bookmarks, and other user data related to canon packages. Managed by libcanon | JSON, Protobuf |
| 3. libcanon       | Expose functions and constants for interacting with canon packages, marks, and preserve user state | Rust |
| 4. frontend/UI    | The platform-specific code for a user-facing application. Can be a command line app, a native app or web app. Essentially UI code coupled with Libcanon | Multiple |
| canonutil         | A separate tool for creating Canon packages from existing formats (pdf, epub, etc.) | Python, Rust, JSON, Protobuf, YAML, TOML |

Note: UI implementations may instantiate multiple instances of the primary struct exposed by libcanon e.g. for tabs, windows, etc.

### Canon Packages

- The main canon package repository may work like NixPkgs, using a monolithic repo to store references to known packages
- For a book, the data is structured according to the `book.proto` file found in this repo.
- Split up pages of canon packages into separate files, and extract their hierarchy into a single "index.pb" or "index.json" so that the structure of the user's library can be read with less overhead, and individual pages can be read from easily.

### libcanon

- Will require a function to specify the path to the canon packages location, and marks location (maybe in its initializer)
- `query(query: String) -> Result<RefData>`: Tells canon to locate a reference based on the query. Retrieves the result as a RefData struct.

### frontend/UI

- Web: Https API to query content from the packages listed in canon's package repo
- TUI: Golang + BubbleTea
- Android: Kotlin + Jetpack Compose
- iOS: Swift + SwiftUI
- Linux: Rust + GTK4
- Windows: I don't know
- Nintendo DS???: If it ever gets solid Rust support

