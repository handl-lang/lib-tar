# lib-tar

`tar` — tar archive reading and writing for the Handl platform
(stdlib port).

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the interface (op declarations + handle bindings) |
| `src/pack.hnl`, `src/unpack.hnl` | pack/unpack surfaces |
| `src/internal/` | `b64b.hnl` (base64), `headerb.hnl`, `oct.hnl` helpers |
| `src/index_flat.hnl` | flattened variant |
| `src/tar.test.hnl` | the .hnl test suite |
| `src/docs/tar.yml` | package docs |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `ci.sh` | manifest / gate / bupha entry |

## Contract

- Transitional: the README mentions Rust atoms behind
  `src/ops.rs` + a crate manifest, but the repo currently ships only
  `.hnl` sources — plan `stdlib-lib-migration` Phase 7 rewrites the
  ops as `ffi.call` and deletes the crate.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
./ci.sh   # bupha entry
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
