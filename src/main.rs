use immersive_landscapes::{Basin, Config, HeightField};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c = Config::default();
    let mut output = PathBuf::from("output");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--seed" => c.seed = args.next().ok_or("missing seed")?.parse()?,
            "--side" => c.side = args.next().ok_or("missing side")?.parse()?,
            "--cell-size" => c.cell_size = args.next().ok_or("missing cell size")?.parse()?,
            "--output" => output = PathBuf::from(args.next().ok_or("missing output folder")?),
            "--help" | "-h" => {
                println!(
                    "Immersive Landscapes by dormyu\nRegional generator preview (not a playable mod yet)\n--seed U64 --side 3..1025 --cell-size METRES --output FOLDER"
                );
                return Ok(());
            }
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }
    let start = Instant::now();
    let basin = Basin::compile(HeightField::generate(c)?);
    let elapsed = start.elapsed();
    fs::create_dir_all(&output)?;
    let mut out = BufWriter::new(File::create(output.join("terrain.csv"))?);
    writeln!(
        out,
        "x,z,ground,spill_height,lake_depth,ocean,discharge,temperature,rainfall,moisture"
    )?;
    for (i, cell) in basin.cells.iter().enumerate() {
        let p = basin.field.point(i);
        let climate = basin.field.climate[i];
        writeln!(
            out,
            "{},{},{:.5},{:.5},{:.5},{},{:.8},{:.3},{:.3},{:.5}",
            p[0],
            p[1],
            cell.ground,
            cell.spill_height,
            cell.lake_depth,
            cell.ocean as u8,
            cell.discharge,
            climate.temperature_c,
            climate.rainfall_mm_year,
            climate.moisture
        )?;
    }
    out.flush()?;
    let mut out = BufWriter::new(File::create(output.join("rivers.csv"))?);
    writeln!(
        out,
        "x1,z1,x2,z2,width,depth,floodplain_width,discharge,water_from,water_to"
    )?;
    for r in &basin.rivers {
        let a = basin.field.point(r.from);
        let b = basin.field.point(r.to);
        writeln!(
            out,
            "{},{},{},{},{:.3},{:.3},{:.3},{:.8},{:.4},{:.4}",
            a[0],
            a[1],
            b[0],
            b[1],
            r.width,
            r.depth,
            r.floodplain_width,
            r.discharge,
            r.water_from,
            r.water_to
        )?;
    }
    out.flush()?;
    let mut out = BufWriter::new(File::create(output.join("deltas.csv"))?);
    writeln!(out, "delta,branch,point,x,z,discharge")?;
    for (d, delta) in basin.deltas.iter().enumerate() {
        for (b, branch) in delta.branches.iter().enumerate() {
            for (p, xy) in branch.iter().enumerate() {
                writeln!(
                    out,
                    "{d},{b},{p},{},{},{:.8}",
                    xy[0], xy[1], delta.branch_discharge[b]
                )?;
            }
        }
    }
    out.flush()?;
    let maximum = basin
        .cells
        .iter()
        .map(|c| c.ground)
        .fold(f64::NEG_INFINITY, f64::max);
    let report = format!(
        "{{\n  \"schema\": 1,\n  \"seed\": {},\n  \"side\": {},\n  \"cell_size\": {},\n  \"sea_level\": {},\n  \"region_span\": {},\n  \"maximum_height\": {:.3},\n  \"compile_ms\": {:.3},\n  \"rivers\": {},\n  \"delta_mouths\": {},\n  \"lake_cells\": {},\n  \"runoff_input\": {:.10},\n  \"outlet_discharge\": {:.10}\n}}\n",
        c.seed,
        c.side,
        c.cell_size,
        c.sea_level,
        (c.side - 1) as f64 * c.cell_size,
        maximum,
        elapsed.as_secs_f64() * 1000.0,
        basin.rivers.len(),
        basin.deltas.len(),
        basin
            .cells
            .iter()
            .filter(|c| !c.ocean && c.lake_depth > 1.5)
            .count(),
        basin.runoff_input,
        basin.outlet_discharge
    );
    fs::write(output.join("summary.json"), &report)?;
    print!("{report}");
    Ok(())
}
