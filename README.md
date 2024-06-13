
# Libcanon

A Rewrite of Canon as a shared library (and as much more!)

## Specification

For a book, the data is structured according to the `book.proto` file found in this repo.

This library exposes the folowing functions:

- `set_texts_dir(dir: String)`: Sets the directory where libcanon will store and retrieve content.
- `query(query: String) -> Bool`: Tells canon to locate a reference based on the query.
- `get() -> <RefData>`: Retrieves the result of the previous `query()` as as RefData struct.

## RefData



