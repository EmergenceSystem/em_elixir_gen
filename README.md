# em_elixir_gen

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Emergence agent — generates Elixir source code from an [AlephTree](https://github.com/aleph-lang/aleph-syntax-tree) intermediate representation.

This agent is the Emergence network integration of the [`elixirgen`](https://crates.io/crates/elixirgen) aleph-lang crate.

---

## Role in the Aleph pipeline

```
Source code (any language)
      ↓  em_*_parser
  AlephTree (JSON)
      ↓  em_betareduction / em_constant_folding  (optional)
  AlephTree (JSON)
      ↓  em_elixir_gen  (this agent)
   Elixir source code
```

---

## Query protocol

**Input** (`body`): JSON-serialized `aleph_tree` embryo

**Output**: JSON array containing one `code` embryo:
```json
[{
  "type": "code",
  "properties": {
    "language": "elixir",
    "source_language": "python",
    "content": "# generated elixir..."
  }
}]
```

---

## Capabilities

```rust
vec!["generator", "elixir"]
```

---

## Deployment

```bash
git clone https://github.com/EmergenceSystem/em_elixir_gen
cd em_elixir_gen
cargo build --release
./target/release/em_elixir_gen
```

Configure via environment variables:
- `EM_DISCO_NODES` — comma-separated `host:port` list (default: `localhost:8080`)
- `EM_FILTER_JWT` — optional JWT token
- `EM_FILTER_RECONNECT_MS` — reconnect delay in ms (default: 5000)
