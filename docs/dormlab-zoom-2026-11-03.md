# DormLab: official Zoom and restored graphics profile

The local three-mod set adds [Horizon Zoom 2.0.0](https://github.com/horizon-rust/horizon-mods/tree/725ff7a41017c40104273e407264e0ebee87e567/native/zoom),
MIT, Ludovic Marie and Horizon contributors. Its Rust source is unchanged.
It uses the stable native hooks; only the staged compatibility pin is set to
the lab's Horizon f42ef42. It is a separate client mod, not world-generation code.
Hold Z (initial4x); turn the wheel while holding it; release to return smoothly.
The Mods → Zoom settings page changes its key, level and smoothing.

Immersive Landscapes0.3.0-dev.2 and DH0.1.0+dormyu.4 retain their code and recipes.
The previous native artifact5dd3d0cd is superseded locally by `44f1baf1bb332a291d25aa278962c5de2df8061a7275fcf7501cc00919ff00ad`.
The owner requested near32, DH128 Medium, simulation8, cap120, VSync on,
native LOD0, automatic workers. Enabling Horizon's independent LOD pauses DH;
the earlier explanation of two simultaneous renderers was incorrect.

Three Zoom logic tests passed. The combined executable passed mod/settings UI,
actual Z and wheel screenshot checks, a copied current v5 world, two Rust clients
with a shared block and independent Zoom, and installed Prism launch. Sources,
MIT notice, rollback and exact hashes are retained in the local lab manifest.
Original owner saves were never opened by the QA client and matched after Prism.

Near32 diagnostics (offscreen1080p/FOV90/VSync off/audio off, reopened QA world/cache):

```json
{
  "static": {
    "avg_fps": 120.0023986866718,
    "p99_ms": 9.091899871826172,
    "max_ms": 9.445300102233888,
    "low_1pct_fps": 108.39168263032424
  },
  "flight": {
    "avg_fps": 119.58663003905386,
    "p99_ms": 9.396699905395508,
    "max_ms": 16.839099884033203,
    "low_1pct_fps": 83.24005768584941
  }
}
```

The initial functional run used default background AFK pacing and capped at30;
that was not a hardware ceiling. The corrected measurement opts out of AFK pacing
for the test window only. These do not prove visible32/120, heavy-pack headroom, or a complete near/far water
seam fix. The owner likes the current landscape except for that transient seam.
Full drainage networks and vegetation ecotones remain separate generation work.

Reproduce composition after sourcing Enter-RustDev.ps1 and selecting the tested
patched mixer documented in native-build.md:

```powershell
python E:/Games/Minecraft/RustCore/development/scripts/Run-GuardedBuild.py --with E:/Games/Minecraft/immersive-landscapes/mixins/immersive_landscapes --with E:/Games/Minecraft/RustCore/development/horizon-mods/native/zoom/mixins/zoom
```

Use the pinned upstream checkout725ff7a and one Cargo job, all cache/temp on E:.
No game resources, worlds, accounts or compiled game binaries are published here.
