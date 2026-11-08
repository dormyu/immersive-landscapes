// Exercise the actual native support module, rather than a copied test model.
#[path = "../mixins/immersive_landscapes/watershed.rs"]
pub mod watershed;

#[test]
fn refined_dry_banks_do_not_create_hanging_water_shelves() {
    use watershed::Profile;
    // Measured native (-305,-2411), seed4312740467388825473: dry stone65,
    // weak bank profile water66.75 made an isolated [65,67) water shelf.
    let bank = Profile {
        bed: 62.96484375,
        water: 66.75,
        strength: 0.10876344986145092,
        raw: 62.96484375,
        lake: false,
        channel_core: false,
    };
    assert_eq!(bank.raster_water_versioned(65, None, true, false), 67);
    assert_eq!(bank.raster_water_versioned(65, None, true, true), 65);
    assert_eq!(bank.raster_water_versioned(65, None, false, true), 67);
    // Existing native water is not proven dry just because lake=false; the
    // native reconciler deliberately excludes frozen and below-sea water.
    assert_eq!(bank.raster_water_versioned(35, Some(40), true, true), 40);
    assert_eq!(bank.raster_water_versioned(65, Some(67), true, true), 67);
    for wet in [
        Profile {
            channel_core: true,
            ..bank
        },
        Profile { lake: true, ..bank },
    ] {
        assert_eq!(wet.raster_water_versioned(65, None, true, true), 67);
    }
}

#[test]
fn refined_river_mouth_and_shallow_steps_have_connected_water_voxels() {
    use watershed::{WaterJoinColumn as C, waterfall_join_floor};
    // Measured native (-269,-2327) flowing into (-269,-2328): the upper
    // [70,71) river and receiving [55,63) native water did not touch.
    let high = C {
        floor: 70,
        water: 71,
        river_core: true,
    };
    let low = C {
        floor: 55,
        water: 63,
        river_core: false,
    };
    assert_eq!(
        waterfall_join_floor(high, [Some(low), None, None, None], false),
        70
    );
    let floor = waterfall_join_floor(high, [Some(low), None, None, None], true);
    assert_eq!(floor, 62);
    assert!(floor.max(low.floor) < high.water.min(low.water));
    // A one-voxel-deep stair is equally disconnected without an opening.
    let stair = C {
        floor: 74,
        water: 75,
        river_core: true,
    };
    let below = C {
        floor: 73,
        water: 74,
        river_core: true,
    };
    assert_eq!(
        waterfall_join_floor(stair, [None, Some(below), None, None], true),
        73
    );
    // Reversing the query never raises or lowers the receiving bed.
    assert_eq!(
        waterfall_join_floor(low, [Some(high), None, None, None], true),
        low.floor
    );
}

#[test]
fn refined_waterfall_openings_are_bounded_and_require_a_real_river() {
    use watershed::{WaterJoinColumn as C, waterfall_join_floor as join};
    let native = C {
        floor: 70,
        water: 75,
        river_core: false,
    };
    let low = C {
        floor: 60,
        water: 65,
        river_core: false,
    };
    assert_eq!(join(native, [Some(low), None, None, None], true), 70);
    let river = C {
        river_core: true,
        ..low
    };
    assert_eq!(join(native, [None, Some(river), None, None], true), 64);
    let dry = C {
        floor: 75,
        ..native
    };
    assert_eq!(join(dry, [Some(river), None, None, None], true), 75);
    let dry_neighbor = C { floor: 65, ..river };
    assert_eq!(
        join(native, [Some(dry_neighbor), None, None, None], true),
        70
    );
    // Twelve extra blocks is the hard excavation limit, regardless of order.
    let limit = C {
        floor: 76,
        water: 80,
        river_core: true,
    };
    assert_eq!(join(limit, [Some(low), None, None, None], true), 64);
    let excessive = C { floor: 77, ..limit };
    assert_eq!(join(excessive, [Some(low), None, None, None], true), 77);
    let close = C {
        floor: 68,
        water: 69,
        river_core: true,
    };
    let a = join(native, [Some(river), None, Some(close), None], true);
    let b = join(native, [Some(close), None, Some(river), None], true);
    assert_eq!((a, b), (64, 64));
}

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
