// Native world-creation integration, MIT, dormyu.
// New worlds use our small vanilla-based overlay. Existing v4 Epic-based worlds
// retain their own recipes and wonderfulaichen's MIT notice.
use super::immersive_landscapes_preset::FILES;
use super::{Settings, world_loading::WorldRequest};
use horizon_client::{
    Result,
    save::{WorldGen, WorldPreset},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

pub(super) const VERSION: &str = "0.5.0";
pub(super) const ID: &str = "immersive_landscapes";
const MARKER: &str = "immersive-landscapes.json";
const PACK: &str = "immersive-landscapes";
const PRESET_META: &str = "IL-PRESET.json";

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

#[derive(Deserialize)]
struct PresetIdentity {
    schema: u32,
    descriptor_version: u32,
    preset: String,
    author: String,
    based_on: String,
    #[serde(default)]
    vanilla_aliases: BTreeMap<String, String>,
}

fn preset_identity(external: Option<&Path>) -> Result<PresetIdentity> {
    let text = match external {
        Some(source) => std::fs::read_to_string(source.join(PRESET_META))?,
        None => FILES
            .iter()
            .find(|(name, _)| *name == PRESET_META)
            .ok_or("Embedded Immersive Landscapes preset metadata is missing")?
            .1
            .to_owned(),
    };
    let identity: PresetIdentity = serde_json::from_str(&text)?;
    if identity.schema != 1
        || !matches!(identity.descriptor_version, 5 | 6 | 7 | 8 | 9 | 10)
        || identity.preset.is_empty()
        || identity.author.is_empty()
    {
        return Err("Unsupported Immersive Landscapes preset metadata".into());
    }
    Ok(identity)
}

fn write_vanilla_aliases(
    identity: &PresetIdentity,
    resources: &Path,
    destination: &Path,
) -> Result<()> {
    if identity.vanilla_aliases.is_empty() {
        return Ok(());
    }
    let densities: BTreeMap<String, String> =
        serde_json::from_slice(&std::fs::read(resources.join("worldgen/densities.json"))?)?;
    for (alias, original) in &identity.vanilla_aliases {
        let name = alias
            .strip_prefix("immersive_landscapes:")
            .ok_or("Vanilla density alias must be in the immersive_landscapes namespace")?;
        if name.is_empty()
            || !name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        {
            return Err("Invalid vanilla density alias path".into());
        }
        if !original.starts_with("minecraft:overworld/") {
            return Err("Vanilla aliases may only reference overworld density functions".into());
        }
        let text = densities
            .get(original)
            .ok_or("Vanilla density source is missing")?;
        let target = destination
            .join("data/immersive_landscapes/worldgen/density_function")
            .join(format!("{name}.json"));
        if target.exists() {
            return Err("Refusing to overwrite a preset file with a vanilla alias".into());
        }
        std::fs::create_dir_all(target.parent().ok_or("Invalid density alias directory")?)?;
        std::fs::write(target, text)?;
    }
    Ok(())
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

pub(super) fn prepare(request: &WorldRequest, settings: &Settings, resources: &Path) -> Result<()> {
    let root = &request.save_root;
    if root.join(MARKER).is_file() {
        let saved: Descriptor = serde_json::from_slice(&std::fs::read(root.join(MARKER))?)?;
        if !matches!(saved.version, 4 | 5 | 6 | 7 | 8 | 9 | 10) {
            return Err("This is an archived Immersive Landscapes prototype world. Open it with its pinned experimental build; create a new world for the current preset.".into());
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
        log::info!(
            "Immersive Landscapes {VERSION}: retaining saved preset {} (descriptor {}) in {}",
            saved.preset,
            saved.version,
            root.display()
        );
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
    let source = external
        .join("pack.mcmeta")
        .is_file()
        .then_some(external.as_path());
    // Determine the actual recipe before writing any world data. An external
    // override must never be stamped with an unrelated embedded preset's name.
    let identity = preset_identity(source)?;
    if let Some(source) = source {
        copy_preset(source, &destination)?;
    } else {
        write_embedded(&destination)?;
    }
    write_vanilla_aliases(&identity, resources, &destination)?;
    let descriptor = Descriptor {
        version: identity.descriptor_version,
        preset: identity.preset,
        author: identity.author,
        based_on: identity.based_on,
    };
    std::fs::write(root.join(MARKER), serde_json::to_vec_pretty(&descriptor)?)?;
    log::info!(
        "Immersive Landscapes {VERSION}: installed preset {} (descriptor {}) in {}",
        descriptor.preset,
        descriptor.version,
        root.display()
    );
    Ok(())
}
