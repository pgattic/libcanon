
# Libcanon

A Rewrite of Canon as a rust library (and as much more!)

As of right now, this repository is mainly used for planning and prototyping, and no decisions are final.

## Application Stack

| Level             | Description | Technologies |
|-------------------|-------------|--------------|
| 1. Canon Packages | The content of the installed books. Managed by libcanon | JSON, Protobuf, YAML |
| 2. Marks          | The annotations, notes, bookmarks, and other user data related to canon packages. Managed by libcanon | JSON, Protobuf |
| 3. Libcanon       | Expose functions and constants for interacting with canon packages, marks, and preserve user state | Rust |
| 4. UI             | The platform-specific code for a user-facing application. Can be a command line app, a native app or web app. Essentially UI code coupled with Libcanon | Multiple |

Note: UI implementations may instantiate multiple instances of the primary struct exposed by libcanon e.g. for tabs, windows, etc.

## Specification

For a book, the data is structured according to the `book.proto` file found in this repo.

This library exposes the folowing functions:

- `set_texts_dir(dir: String)`: Sets the directory where libcanon will store and retrieve content.
- `query(query: String) -> Bool`: Tells canon to locate a reference based on the query.
- `get() -> <RefData>`: Retrieves the result of the previous `query()` as as RefData struct.

## Ideas

- Split up pages of canon packages into separate files, and extract their hierarchy into a single "index.pb" or "index.json" so that the structure of the user's library can be read with less overhead, and individual pages can be read from easily.

