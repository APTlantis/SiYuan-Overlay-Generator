# CLI Contract — Current Experimental Surface

This is the implemented command surface pending CTS release review. It is not a
promise of final flag names or machine-readable output.

```text
siyuan-overlay-generator inspect <SOURCE>
siyuan-overlay-generator generate <SOURCE> <OUTPUT.zip>
```

`inspect` reads one source directory into the normalized model and prints a
structural summary. `generate` performs that discovery and writes one new
Markdown ZIP package. Both commands leave the source tree untouched.

Exit behavior:

- `0` — command completed.
- `1` — source discovery or output generation failed.
- `2` — invalid command or missing required argument.

`generate` requires a `.zip` output path that does not already exist. This is a
deliberate no-overwrite safety guard.
