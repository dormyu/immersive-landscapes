# Continue here: build the playable mod

Updated 2026-11-02. This is a handoff, not evidence of a completed game mod.

## Objective and next action

Read [the implementation plan](implementation-plan.ru.md). **G0 is next.**
The user authorized implementation, local builds, DormLab installation, and Git
commits as dormyu. Complete one integration slice before more landscape research:
create a fresh experimental world whose actual blocks and DH terrain both come
from `Landscape::sample`. A standalone generator is insufficient.

The user wants balance between appearance and performance, ideally **32 near
chunks / 120 FPS** on the current PC, options for old hardware, and substantial
headroom for a heavy pack. Up to roughly 10x longer preparation is acceptable.
Do not silently reduce that goal to 32 DH chunks or call 8 near chunks a pass.

## Read in this order

1. This repository's `AGENTS.md`, then `docs/implementation-plan.ru.md`.
2. Parent `E:/Games/Minecraft/AGENTS.md` and `RustCore/AGENTS.md`.
3. `RustCore/reports/development-2026-11-02/WORK-IN-PROGRESS.md` for existing QA,
   native control gotchas, rollback, identity and storage state.
4. Only the source files needed for the next gate, listed in the plan. Read the
   pinned engine's own AGENTS.md before touching a generated build stage.

All relative `RustCore/...` paths below are based at `E:/Games/Minecraft`.

## Verified starting point

- Own source: `E:/Games/Minecraft/immersive-landscapes`; published implementation
  before this plan is commit `8727b9b`. Eleven terrain tests passed in recorded
  QA/CI. Current turn changed planning documents, not runtime code.
- DH source: `RustCore/development/distant-horizons`, local commit `2ab4f24`.
  Sixteen tests and native solo/two-client tests passed; it is installed in
  DormLab. Do not port DH again. Persistent remote cache and performance remain
  incomplete; the last remote snapshot drew 52/53 sections with queued work.
- Pinned engine: `c9d474dd9116e84c243f8992d75c11314df56f06`.
  Source: `RustCore/reports/terrain-research-2026-11-02/nightly-source-c9d474d/horizon-c9d474d`.
- Mixins: `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`.
- Delivered lab: `RustCore/packs/DormLab`; manifest `rustcore/native-build.json`.
- DH rollback: `RustCore/qa/dormlab-before-dh-2026-11-02`.
  Retain it and take another verified rollback before combined installation.
- Evidence: `RustCore/qa/distant-horizons-2026-11-02/reports/final-verification.json`.
  At that check, original 198 save files were unchanged and QA processes stopped.
- GitHub login/author is now **dormyu**. Auth and push already worked after rename.
- PC: i5-9400F (6 cores/6 threads), RTX 2060, approximately 16 GiB RAM;
  current desktop 1920x1080, 144 Hz. E is HDD. Do not infer VRAM from the truncated
  Windows `AdapterRAM` field. Re-measure runtime budgets on this machine.
- Pagefile changes awaited manual reboot in previous QA. Do not reboot or assume
  the pending state still holds. User explicitly permitted continued full builds.

## Safe existing commands

PowerShell, repository cwd:

```powershell
. E:\Games\Minecraft\RustCore\development\scripts\Enter-RustDev.ps1
$env:CARGO_TARGET_DIR = 'E:\Games\Minecraft\immersive-landscapes\target'
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

Existing combined **staging** interface, after creating our bundle:

```powershell
Set-Location E:\Games\Minecraft\RustCore\development\distant-horizons
python -X utf8 tools/horizon.py stage --with E:\Games\Minecraft\immersive-landscapes\mixins\immersive_landscapes
```

That bundle path is planned and does not exist yet. Do not run it until G0 creates
it. The current `Run-GuardedBuild.py` runs **DH alone**, with no argument forwarding.
G0 must add a guarded combined-build entry point before compiling; otherwise a
later build silently restages without our mod. Guard C free >=6 GiB; only stop
the process tree owned by that build. Keep TEMP/TMP/Cargo/Rustup/targets on E.

Native control supports `bench`, `stats`, `gpu-passes`, and `fly ... raw`; inspect
the pinned help and implementation before scripting. Existing wrappers and private
QA socket paths are documented in WORK-IN-PROGRESS. Never print socket JSON tokens.
Use CLI/native control; offscreen is useful for correctness, not final displayed FPS.

## How to execute cheaply without losing correctness

- One gate and one focused patch at a time. Read bounded file ranges; avoid dumping
  whole engine trees or repeating research. No new agents are requested by this plan.
- Start with G0's hook/build feasibility; then finish G1/G2 into a playable slice.
  Do not spend the first turn polishing a preview, adding another framework, or
  rewriting hydrology while there is no in-game adapter.
- Unit-check pure algorithms before engine rebuilds. Reuse the pinned stage/cache;
  one combined build per coherent integration change, not per document edit.
- Every gate gets evidence: build/source hashes, command, observed result, unresolved
  failure, and next action. Update the ledger in the plan and the local WIP report.
- A compiler error is not a reason to replace the architecture wholesale. Inspect
  the first root error. If hook support is genuinely absent, write the smallest
  proposed extension with a reproducer and impact; preserve the native-mod goal.
- Stop calling a visual/performance gate passed if the data fails. Record the
  measured limit and optimize the dominant cost. Never manufacture FPS or user approval.
- The final gate needs the owner's actual visual judgement; prepare a usable build
  and short route first, then invite that review. No repeated permission to develop.
