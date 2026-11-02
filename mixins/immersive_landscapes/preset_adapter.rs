// Native world-creation integration, MIT, dormyu.
// Complete Epic Terrain recipes retain wonderfulaichen's MIT notice.
use super::immersive_landscapes_preset::FILES;
use super::{Settings, world_loading::WorldRequest};
use horizon_client::{
    Result,
    save::{WorldGen, WorldPreset},
};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(super) const VERSION: &str = "0.2.0-dev.1";
pub(super) const ID: &str = "immersive_landscapes";
const MARKER: &str = "immersive-landscapes.json";
const PACK: &str = "immersive-landscapes";

#[derive(Serialize, Deserialize)]
struct Descriptor {
    version: u32,
    #[serde(default)]
    preset: String,
    #[serde(default)]
    author: String,
    #[serde(default)]
    based_on: String,
}

fn write_embedded(destination: &Path) -> Result<()> {
    for &(name, text) in FILES {
        let path = destination.join(name);
        std::fs::create_dir_all(path.parent().ok_or("invalid preset path")?)?;
        std::fs::write(path, text)?;
    }
    Ok(())
}

// Optional external tuning is copied only when creating a world. Existing
// worlds keep their installed recipe, including after changing pack defaults.
fn copy_preset(source: &Path, destination: &Path) -> Result<()> {
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            return Err("Preset must not contain symbolic links".into());
        }
        let target = destination.join(entry.file_name());
        if kind.is_dir() {
            copy_preset(&entry.path(), &target)?;
        } else if kind.is_file() {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

pub(super) fn prepare(request: &WorldRequest, settings: &Settings) -> Result<()> {
    let root = &request.save_root;
    if root.join(MARKER).is_file() {
        let saved: Descriptor = serde_json::from_slice(&std::fs::read(root.join(MARKER))?)?;
        if saved.version != 4 {
            return Err("This is an archived Immersive Landscapes prototype world. Open it with its pinned experimental build; create a new world for the native Epic-based preset.".into());
        }
        if !root
            .join("datapacks")
            .join(PACK)
            .join("pack.mcmeta")
            .is_file()
        {
            return Err(
                "This world's Immersive Landscapes data pack is missing; restore it from a backup."
                    .into(),
            );
        }
        return Ok(());
    }
    if settings.mods.disabled.contains(ID)
        || root.join("level.dat").exists()
        || root.join("level.dat_old").exists()
        || root.join("region").exists()
        || root.join("dimensions").exists()
        || request
            .create
            .as_ref()
            .is_some_and(|o| o.world_gen().preset != WorldPreset::Normal)
        || WorldGen::read(root).is_some_and(|o| o.preset != WorldPreset::Normal)
    {
        return Ok(());
    }

    let destination = root.join("datapacks").join(PACK);
    if destination.exists() {
        return Err(
            "Refusing to overwrite an existing Immersive Landscapes world data pack".into(),
        );
    }
    let external = std::env::current_dir()?.join("horizon-mods/immersive-landscapes-preset");
    if external.join("pack.mcmeta").is_file() {
        copy_preset(&external, &destination)?;
    } else {
        write_embedded(&destination)?;
    }
    let descriptor = Descriptor {
        version: 4,
        preset: "epic-compact-v1".into(),
        author: "dormyu".into(),
        based_on: "Epic Terrain, wonderfulaichen, MIT, 08507bd3d606ad440b40a5fa726e0098d7309347"
            .into(),
    };
    std::fs::write(root.join(MARKER), serde_json::to_vec_pretty(&descriptor)?)?;
    log::info!(
        "Immersive Landscapes {VERSION}: installed Minecraft-scale native preset in {}",
        root.display()
    );
    Ok(())
}
