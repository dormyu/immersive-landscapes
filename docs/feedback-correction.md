# Feedback attribution correction — 3 November 2026

The previous statement that the owner accepted Immersive Landscapes v4 and its
near16 / DH128 medium profile was an assistant error. It is withdrawn.

Launcher history places the positive-feedback session in **DormLab-EpicReference**,
with IL disabled and DH's own enabled setting false. The later unsuccessful
session was in **DormLab**, with IL enabled, 16 near chunks and DH128 medium.
The reference instance's saved near distance was 32. These are different worlds,
mod states and rendering conditions; the feedback was not a comparison of two
versions of this mod. The owner subsequently identified the liked result as
ordinary generation.

The reference instance contains both a prepared Epic Terrain world and ordinary
worlds. Its name does not make newly created worlds use Epic Terrain. The ordinary
world checked here has no IL marker or terrain data pack and stores the standard
overworld generator. The prepared reference uses its own world-local Epic pack.

The UI-only `0.2.0-dev.2` update retained every terrain recipe file and the DH
adaptation. Reverting that label change would not reproduce the other instance.
Functional world creation, persistence, UI and two-client checks remain valid;
they do not establish visual acceptance, fully loaded distant terrain or smooth
fresh-world gameplay. Current terrain and DH loading require further work.

For subsequent comparisons, record the exact instance, runtime, world, seed,
world-local preset, near/DH settings and cold/warm cache state. Compare ordinary,
unmodified Epic and IL generation under the same conditions, first without DH
and then with it. Preserve all existing saves and user settings. Future custom
structures remain planned for the eventually approved geography; v4 is not an
approved foundation.

The incorrectly named acceptance tag is withdrawn. Source commits and functional
test evidence are preserved. No gameplay code, runtime, world or setting was
changed for this documentation correction.
