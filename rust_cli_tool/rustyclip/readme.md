
<h2>Rustyclip CLI Tool </h2>
<h3>Save pastes, add nicknames, organize your clipboard</h3>

<p>by: jasonmzx</p>

<h3>Tech Stack</h3>
- Rust & SQLite
<h4>Crates used: </h4>
- [Clap](https://crates.io/crates/clap/2.34.0) for flexible parsing of CLI Args (implemented via the YML Builder Pattern) 
- [Rusqlite](https://crates.io/crates/rusqlite/) for connection to the SQLite DB
- [copypasta](https://crates.io/crates/copypasta) for being able to get & set clipboard contents cross-platform (Windows, X11 (Linux), OSX ? )
- [colored](https://crates.io/crates/colored) for making the CLI feedback eye-catching and pretty

