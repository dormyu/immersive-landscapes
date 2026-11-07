# Immersive Landscapes 0.5.1 handoff

Native streaming fix over the existing descriptor10 / vanilla-natural-v6 recipe.
Built IL commit `a626cad3800fa1358b92c2ac880969f4987f2260`, Horizon f42ef42, DH dormyu.16 and unchanged Zoom2.
Runtime `808b212ac8ed618c3c49ed658e3bdf979e152a444a34e0cd8c2d7b624d9bea5a`.
Read [0.5.1 validation](validation-0.5.1.md) and the installed native-build.json /
release notes for the later install and Prism transactions.

Owner reported stopped chunks and a freeze after a full Prism restart. The old
build reproduces with near2/DHoff: a nearby stronghold triggers all128 biome
searches and hydrology across remote regions. Exact local ring resolution fixes
that reproduced case. Five native tests, the matched near2 regression, owner-copy
near32/DH128 travel/busy exit/full restart, fresh same-camera vanilla/IL captures
and two-client loopback checks pass. Do not repeat completed porting or change the
seed/recipe as a workaround. The existing world can use this fix.

Current bounds: explicit full structure queries can still be expensive. This is
not universal cancellation. No new FPS claim; retain the 0.5.0 measurements with
their exact conditions and ~98ms maximum cached movement hitch. Initial DH fill,
material/tint boundaries, some angular channels, full biome coverage, long
exploration, heavy-pack and owner aesthetic acceptance remain separate.
Future structures read saved geography; structure redesign is not implemented.

QA: RustCore/qa/hang-recovery-2026-11-07. Preserve historical receipts.
All files/temp/caches/worlds on E:, Enter-RustDev.ps1, stable Rust, one Cargo job.
Only root owns build/game slot. Preserve user saves/accounts/settings; no Teams
or other owner apps closed. Keep near32/DH128 Medium/sim8/maxFps260/VSync off and
control off. Git identity dormyu; never push DH or tooling upstream.
