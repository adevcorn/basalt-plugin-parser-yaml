# basalt-plugin-parser-yaml

Tree-sitter YAML parser WASM plugin for Basalt.

## File extensions
- `.yaml`, `.yml`

## Exports
- `basalt_lang` — returns pointer to `"yaml\0"`
- `basalt_src_ptr` / `basalt_out_ptr`
- `basalt_parse(src_ptr, src_len, out_ptr, max_spans) -> i32`
- `basalt_retrieval_chunks(src_ptr, src_len, out_ptr, max_chunks) -> i32`
- `basalt_call_sites` — always returns 0 (not applicable)