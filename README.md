# mcpg-glob

> One shared wildcard matcher for MCPG tool-name and path filtering.

`mcpg-glob` is a single-function, dependency-free wildcard matcher used wherever
MCPG lets an operator name a set of tools or paths with a pattern — audit
filters, rate-limit scopes, cacheable-tool lists, allowlists, redaction and
field-encryption `tools` / `exclude_tools` selectors. It exists so
that every one of those config surfaces answers "does this tool match?" with
identical semantics, rather than each plugin shipping its own near-miss copy. It
is intentionally not a filesystem globber: there are no character classes, no
brace expansion, no `**`, and no path-separator awareness.

## What's here

- `glob_match(pattern, text) -> bool` — the crate's entire public surface.

The grammar is three rules: `*` matches any sequence of characters (including
none, and including `.`), `?` matches exactly one, and every other character
matches itself. `glob_match("orders.*", "orders.place_order")` is true;
`glob_match("*.place_*", "orders.list")` is false; an empty pattern matches only
an empty string.

Two properties are worth knowing before you rely on it. Matching is **byte-wise**,
so `?` consumes one byte rather than one Unicode scalar — a multi-byte UTF-8
character needs as many `?` as it has bytes. And the implementation is
non-recursive and linear-time: it walks both strings once and backtracks to the
most recent `*`, so a pattern crowded with wildcards cannot trigger the
exponential blow-up a naive recursive matcher suffers. There are no heap
allocations and no dependencies.

## Used by

- `apps/gateway` — tool-name filters on operator-facing config surfaces.
- Plugins that scope behaviour by tool name or path:
  `libs/plugins/observability/audit`, `libs/plugins/reliability/response-cache`,
  `libs/plugins/reliability/rate-limit`, `libs/plugins/security/ip-allowlist`,
  `libs/plugins/security/guardrails`, `libs/plugins/security/dlp`, and
  `libs/plugins/security/field-crypto`.

## Build / test

```bash
cargo build -p mcpg-glob
cargo test  -p mcpg-glob
```

## Licence

Apache-2.0.

## See also

- [Plugin catalogue](https://mcpg.dev/docs/plugins/plugin-catalogue) — the plugins whose config keys accept these patterns.
- [Gateway configuration reference](https://mcpg.dev/docs/reference/configuration) — where tool-name filters are written.
