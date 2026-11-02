use immersive_landscapes::{Basin, Config, HeightField, Landscape, RiverSegment};

fn channel_fixture() -> Landscape {
    let c = Config {
        side: 9,
        cell_size: 100.0,
        sea_level: 0.0,
        ..Config::default()
    };
    let mut b = Basin::compile(HeightField::from_elevations(c, vec![40.0; 81]).unwrap());
    b.rivers = vec![RiverSegment {
        from: 2 * 9 + 4,
        to: 6 * 9 + 4,
        discharge: 2.0,
        width: 10.0,
        depth: 3.0,
        floodplain_width: 40.0,
        water_from: 20.0,
        water_to: 16.0,
    }];
    Landscape::new(b)
}

#[test]
fn river_has_a_bed_banks_and_a_downhill_water_surface() {
    let land = channel_fixture();
    let middle = land.sample(0.0, 0.0).unwrap();
    assert_eq!(middle.ground, 15.0);
    assert_eq!(middle.water, Some(18.0));
    let bank = land.sample(5.0, 0.0).unwrap();
    assert_eq!(bank.ground, 18.0);
    assert_eq!(bank.water, None);
    let outside = land.sample(21.0, 0.0).unwrap();
    assert_eq!(outside.ground, 40.0);
    assert_eq!(outside.water, None);
    assert_eq!(land.sample(0.0, -100.0).unwrap().water, Some(19.0));
    assert_eq!(land.sample(0.0, 100.0).unwrap().water, Some(17.0));
}

#[test]
fn indexed_queries_are_continuous_across_negative_tile_boundaries() {
    let land = channel_fixture();
    for z in [-100.0, 0.0, 100.0] {
        let a = land.sample(-1e-7, z - 1e-7).unwrap();
        let b = land.sample(1e-7, z + 1e-7).unwrap();
        assert!((a.ground - b.ground).abs() < 1e-6);
        assert!((a.water.unwrap() - b.water.unwrap()).abs() < 1e-6);
    }
    assert!(land.sample(-400.0, 400.0).is_some());
    assert!(land.sample(-400.001, 0.0).is_none());
    assert!(land.sample(f64::NAN, 0.0).is_none());
}

#[test]
fn lake_keeps_its_spill_level_above_interpolated_bathymetry() {
    let c = Config {
        side: 3,
        cell_size: 100.0,
        sea_level: 0.0,
        ..Config::default()
    };
    let land = Landscape::new(Basin::compile(
        HeightField::from_elevations(c, vec![4.0, 4.0, 4.0, 4.0, -8.0, 4.0, 4.0, 4.0, 4.0])
            .unwrap(),
    ));
    for x in [0.0, 25.0, 50.0, 75.0] {
        assert_eq!(land.sample(x, 0.0).unwrap().water, Some(4.0));
    }
    assert_eq!(land.sample(100.0, 0.0).unwrap().water, None);
}
