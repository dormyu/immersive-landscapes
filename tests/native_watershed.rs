// Exercise the actual native support module, rather than a copied test model.
#[path = "../mixins/immersive_landscapes/watershed.rs"]
pub mod watershed;

// Published v10 geography remains a compatibility fixture when a later recipe
// opts into a new channel shape. Hash final voxel decisions, not float bits.
#[test]
fn saved_v10_channel_and_bank_voxels_remain_unchanged() {
    use watershed::{Basin, Coast, Config, Key, Land, Terrain};
    #[derive(Debug)]
    struct Valley;
    impl Terrain for Valley {
        fn land(&self, x: i32, z: i32) -> Land {
            let height = 63. + f64::from(x) * 0.12 + (f64::from(z) / 70.).sin() * 3.;
            Land {
                height,
                ocean: height <= 63.,
            }
        }
    }
    let coast = Coast {
        key: Key(0, 0),
        x: -4,
        z: 0,
        inland_level: None,
    };
    let basin = Basin::compile(
        coast,
        Config {
            radius: 512,
            root_cell: 192,
            margin: 8.,
            min_cells: 3.,
            advanced_rivers: true,
            meander_amplitude: 28.,
            meander_scale: 80.,
            ..Config::default()
        },
        &Valley,
        &[coast],
    );
    assert!(
        basin.segments.len() > 40,
        "compatibility fixture has no channels"
    );
    let mut hash = 0xcbf29ce484222325_u64;
    let mut cores = 0;
    for z in (-448..=448).step_by(4) {
        for x in (-448..=448).step_by(4) {
            let raw = Valley.land(x, z).height;
            let value = basin.sample(x, z, raw).map_or([i32::MIN; 3], |p| {
                cores += usize::from(p.channel_core);
                [
                    p.raster_floor_versioned(raw.ceil() as i32, true, true),
                    p.voxel_water(true),
                    i32::from(p.lake) + 2 * i32::from(p.channel_core),
                ]
            });
            for byte in value.into_iter().flat_map(i32::to_le_bytes) {
                hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
            }
        }
    }
    assert!(
        cores > 100,
        "compatibility fixture did not exercise wet channels"
    );
    assert_eq!(hash, 0xc1a838a52e2cb5cb);
}
