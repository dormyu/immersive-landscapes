use immersive_landscapes::{Basin, Config, HeightField};

#[test]
fn bowl_has_a_real_spill_level_and_outlet() {
    let c = Config {
        side: 5,
        sea_level: 0.0,
        cell_size: 10.0,
        ..Config::default()
    };
    let h = vec![
        9., 9., 3., 9., 9., 9., 5., 3., 5., 9., 9., 5., 1., 5., 9., 9., 5., 5., 5., 9., 9., 9., 9.,
        9., 9.,
    ];
    let b = Basin::compile(HeightField::from_elevations(c, h).unwrap());
    assert_eq!(b.cells[12].spill_height, 3.0);
    assert_eq!(b.cells[12].lake_depth, 2.0);
    assert_eq!(b.cells[12].downstream, Some(7));
    assert_eq!(b.cells[7].downstream, Some(2));
}

#[test]
fn rainfall_is_conserved_and_drainage_never_cycles_or_climbs() {
    for seed in [0, 1, 8675309, u64::MAX] {
        let b = Basin::compile(
            HeightField::generate(Config {
                seed,
                side: 129,
                ..Config::default()
            })
            .unwrap(),
        );
        assert!((b.runoff_input - b.outlet_discharge).abs() < b.runoff_input * 1e-10);
        for (i, c) in b.cells.iter().enumerate() {
            if let Some(j) = c.downstream {
                assert!(b.rank[j] < b.rank[i]);
                assert!(b.cells[j].spill_height <= c.spill_height);
                assert!(b.cells[j].discharge >= c.discharge);
            }
        }
        for r in &b.rivers {
            assert!(r.water_from >= r.water_to);
            assert!(r.width > 0.0 && r.depth > 0.0 && r.floodplain_width >= r.width);
        }
        for d in &b.deltas {
            assert!((d.branch_discharge.iter().sum::<f64>() - d.incoming_discharge).abs() < 1e-9);
            assert!(d.branches.len() >= 2);
            for branch in &d.branches {
                assert_eq!(branch[0], b.field.point(d.mouth));
            }
        }
    }
}

#[test]
fn confluence_accumulates_both_tributaries() {
    let c = Config {
        side: 5,
        sea_level: -1.0,
        cell_size: 10.0,
        ..Config::default()
    };
    let b = Basin::compile(
        HeightField::from_elevations(
            c,
            vec![
                9., 9., 9., 9., 9., 9., 8., 9., 8., 9., 9., 9., 5., 9., 9., 9., 9., 2., 9., 9., 9.,
                9., 0., 9., 9.,
            ],
        )
        .unwrap(),
    );
    assert_eq!(b.cells[6].downstream, Some(12));
    assert_eq!(b.cells[8].downstream, Some(12));
    assert!(b.cells[12].discharge > b.cells[6].discharge + b.cells[8].discharge);
    assert_eq!(b.cells[12].downstream, Some(17));
}

#[test]
fn enclosed_depression_below_sea_is_a_lake_not_an_ocean() {
    let c = Config {
        side: 3,
        sea_level: 0.0,
        ..Config::default()
    };
    let b = Basin::compile(
        HeightField::from_elevations(c, vec![4., 4., 4., 4., -8., 4., 4., 4., 4.]).unwrap(),
    );
    assert!(!b.cells[4].ocean);
    assert_eq!(b.cells[4].spill_height, 4.0);
    assert_eq!(b.cells[4].lake_depth, 12.0);
}

#[test]
fn generation_is_reproducible_and_negative_coordinates_are_continuous() {
    let c = Config {
        side: 65,
        ..Config::default()
    };
    let a = HeightField::generate(c).unwrap();
    let b = HeightField::generate(c).unwrap();
    assert_eq!(a.ground, b.ground);
    let basin = Basin::compile(a);
    for x in [-256.0, -0.001, 0.0, 256.0] {
        assert!((basin.height(x - 1e-6, -128.0) - basin.height(x + 1e-6, -128.0)).abs() < 0.001);
    }
}

#[test]
fn invalid_rasters_are_rejected_before_allocation() {
    assert!(
        HeightField::generate(Config {
            side: usize::MAX,
            ..Config::default()
        })
        .is_err()
    );
    assert!(
        HeightField::generate(Config {
            cell_size: f64::NAN,
            ..Config::default()
        })
        .is_err()
    );
    assert!(
        HeightField::from_elevations(
            Config {
                side: 3,
                ..Config::default()
            },
            vec![0.0; 8]
        )
        .is_err()
    );
}

fn coastal_basin(coast_row: usize, slope_per_cell: f64) -> Basin {
    let c = Config {
        side: 33,
        cell_size: 512.0,
        sea_level: 0.0,
        ..Config::default()
    };
    let h = (0..c.side * c.side)
        .map(|i| {
            let x = i % c.side;
            let z = i / c.side;
            if z >= coast_row {
                -5.0
            } else {
                (coast_row - z) as f64 * slope_per_cell
                    + x.abs_diff(16) as f64 * (slope_per_cell * 5.0).max(20.0)
            }
        })
        .collect();
    Basin::compile(HeightField::from_elevations(c, h).unwrap())
}

#[test]
fn low_gradient_mouth_splits_flow_into_real_distributaries() {
    // A wide V-shaped catchment empties into twelve rows of open receiving sea.
    // Unlike checking a generated seed, this fixture must actually create a delta.
    let b = coastal_basin(20, 2.0);
    assert!(!b.deltas.is_empty());
    for d in &b.deltas {
        assert!(b.cells[d.mouth].ocean);
        assert!(d.incoming_discharge >= 0.8);
        assert_eq!(d.branches.len(), 3);
        assert_eq!(d.branches.len(), d.branch_discharge.len());
        assert!((d.branch_discharge.iter().sum::<f64>() - d.incoming_discharge).abs() < 1e-12);
        let ends: Vec<_> = d.branches.iter().map(|p| *p.last().unwrap()).collect();
        assert_ne!(ends[0], ends[1]);
        assert_ne!(ends[1], ends[2]);
        for path in &d.branches {
            assert_eq!(path[0], b.field.point(d.mouth));
            assert!(path.windows(2).all(|p| p[1][1] > p[0][1]));
        }
    }
}

#[test]
fn steep_mouths_and_unknown_water_beyond_region_are_not_deltas() {
    let steep = coastal_basin(20, 80.0);
    assert!(
        steep
            .rivers
            .iter()
            .any(|r| steep.cells[r.to].ocean && r.discharge >= 0.8)
    );
    assert!(steep.deltas.is_empty());
    // At the last row the receiving sea has no known extent for the fan.
    let edge = coastal_basin(32, 2.0);
    assert!(
        edge.rivers
            .iter()
            .any(|r| edge.cells[r.to].ocean && r.discharge >= 0.8)
    );
    assert!(edge.deltas.is_empty());
}
