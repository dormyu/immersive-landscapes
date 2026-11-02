"""Render exported terrain data, not an artist's reconstruction or game screenshot."""
import argparse
import csv
import json
import math
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont


def font(size):
    for path in (Path('C:/Windows/Fonts/segoeui.ttf'),
                 Path('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')):
        if path.is_file():
            return ImageFont.truetype(str(path), size)
    return ImageFont.load_default(size=size)


def blend(a, b, t):
    return tuple(round(x + (y - x) * max(0, min(1, t))) for x, y in zip(a, b))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('data', type=Path)
    parser.add_argument('--output', type=Path, default=Path('output/terrain-preview.png'))
    args = parser.parse_args()
    meta = json.loads((args.data / 'summary.json').read_text())
    with (args.data / 'terrain.csv').open() as source:
        rows = [{k: float(v) for k, v in row.items()} for row in csv.DictReader(source)]
    n = meta['side']
    assert len(rows) == n * n
    image = Image.new('RGB', (1500, 940), '#f5f3eb')
    draw = ImageDraw.Draw(image)
    draw.text((38, 23), 'IMMERSIVE LANDSCAPES', font=font(32), fill='#19352e')
    draw.text((40, 70), 'Regional terrain study / dormyu', font=font(18), fill='#60736b')
    span = meta['region_span']
    half = span / 2
    map_size = 696
    origins = [(38, 152), (766, 152)]
    draw.text((38, 116), 'Relief, lakes and drainage', font=font(23), fill='#19352e')
    draw.text((766, 116), 'Continuous rainfall / rain shadow', font=font(23), fill='#19352e')
    for panel, origin in enumerate(origins):
        raster = Image.new('RGB', (n, n))
        colors = []
        for i, row in enumerate(rows):
            x, z = i % n, i // n
            height = row['ground'] - meta['sea_level']
            if row['ocean'] or row['lake_depth'] > 1.5:
                color = blend((54, 103, 125), (153, 201, 211), 0.7 if not row['ocean'] else 0.3)
            elif panel:
                color = blend((217, 190, 134), (44, 112, 99), (row['rainfall'] - 180) / 1800)
            else:
                stops = [(0, (180, 195, 140)), (250, (122, 157, 115)),
                         (650, (136, 143, 112)), (1100, (174, 164, 143)), (1550, (239, 235, 222))]
                color = stops[-1][1]
                for (lo, a), (hi, b) in zip(stops, stops[1:]):
                    if height <= hi:
                        color = blend(a, b, (height - lo) / (hi - lo))
                        break
                west = rows[z*n + max(x-1, 0)]['ground']
                east = rows[z*n + min(x+1, n-1)]['ground']
                north = rows[max(z-1, 0)*n + x]['ground']
                south = rows[min(z+1, n-1)*n + x]['ground']
                sx = (east-west) / (2*meta['cell_size'])
                sz = (south-north) / (2*meta['cell_size'])
                shade = 0.78 + 0.28 * (1 - sx + sz) / math.sqrt(1+sx*sx+sz*sz)
                color = tuple(max(0, min(255, round(c*shade))) for c in color)
            colors.append(color)
        raster.putdata(colors)
        image.paste(raster.resize((map_size, map_size), Image.Resampling.BILINEAR), origin)
        if panel == 0:
            def xy(x, z):
                return (origin[0] + (x+half)/span*(map_size-1), origin[1] + (z+half)/span*(map_size-1))
            with (args.data / 'rivers.csv').open() as source:
                for r in csv.DictReader(source):
                    draw.line([xy(float(r['x1']), float(r['z1'])), xy(float(r['x2']), float(r['z2']))], fill='#86c9df', width=2)
            branches = {}
            with (args.data / 'deltas.csv').open() as source:
                for p in csv.DictReader(source):
                    branches.setdefault((p['delta'], p['branch']), []).append(xy(float(p['x']), float(p['z'])))
            for points in branches.values():
                draw.line(points, fill='#cbf2ed', width=3)
        draw.rectangle([origin, (origin[0]+map_size-1, origin[1]+map_size-1)], outline='#61796d')
    summary = f"Seed {meta['seed']}  /  {span/1000:.3f} km region  /  peak Y {meta['maximum_height']:.0f}  /  {meta['delta_mouths']} delta candidates"
    draw.text((38, 865), summary, font=font(18), fill='#19352e')
    draw.text((38, 899), 'Generated data preview. River strokes enlarged. Finite prototype; not an in-game screenshot.', font=font(17), fill='#60736b')
    args.output.parent.mkdir(parents=True, exist_ok=True)
    image.save(args.output)
    print(args.output)


if __name__ == '__main__':
    main()
