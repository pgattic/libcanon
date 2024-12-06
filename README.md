
# Libcanon

A Rewrite of Canon as a rust library (and as much more!)

As of right now, this repository is mainly used for planning and prototyping. No decisions are final.

## Application Stack

| Level             | Description | Technologies |
|-------------------|-------------|--------------|
| 1. Canon Packages | The content of the installed books. Managed by libcanon | JSON, Protobuf, YAML, TOML |
| 2. libcanon       | Expose functions for interacting with canon packages | Rust |
| 3. frontend/UI    | The platform-specific code for a user-facing application. Can be a command line app, a native app or web app. | Dioxus |
| canonutil         | A separate tool for creating Canon packages from existing formats (pdf, epub, etc.) | Python, Rust, JSON, Protobuf, YAML, TOML |

Note: UI implementations may instantiate multiple instances of the primary struct exposed by libcanon e.g. for tabs, windows, etc.

### Canon Packages

- The main canon package repository may work like NixPkgs, using a monolithic repo to store references to known packages
- For a book, the data is structured according to the `book.proto` file found in this repo.
- Split up pages of canon packages into separate files, and extract their hierarchy into a single "index.pb" or "index.json" so that the structure of the user's library can be read with less overhead, and individual pages can be read from easily.

### libcanon

- Full feature parity with Golang implementation
- Search functinality

### frontend/UI

- Will probably use Dioxus for all the GUI

