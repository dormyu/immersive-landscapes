"""Make a new versioned preset for river tuning without rebuilding Horizon."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

ROOT=Path(__file__).resolve().parents[1]
FIELDS={'amplitude':('meander_amplitude',0.,32.),'wavelength':('meander_scale',48.,256.),
        'width':('width_scale',.75,1.75),'depth':('depth_scale',.5,1.5),'banks':('bank_scale',2.,5.)}

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',type=Path,required=True)
    for name,(_,lo,hi) in FIELDS.items():parser.add_argument('--'+name,type=float,help=f'{lo}..{hi}')
    args=parser.parse_args();out=args.output.resolve()
    if not args.output.is_absolute() or out.exists():parser.error('Use a new absolute output directory; existing files are never overwritten')
    chosen={name:getattr(args,name) for name in FIELDS if getattr(args,name) is not None}
    for name,value in chosen.items():
        _,lo,hi=FIELDS[name]
        if not lo<=value<=hi:parser.error(f'{name} must be within{lo}..{hi}')
    if not chosen:parser.error('Supply at least one river setting')
    source=ROOT/'presets/vanilla-natural'
    shutil.copytree(source,out)
    for name,value in chosen.items():
        key=FIELDS[name][0]
        (out/f'data/immersive_landscapes/worldgen/density_function/river_{key}.json').write_text(json.dumps(value)+'\n',encoding='utf-8',newline='\n')
    meta=json.loads((out/'IL-PRESET.json').read_text(encoding='utf-8'))
    meta['registry_sha256']={p.relative_to(out).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((out/'data').rglob('*.json'))}
    identity=hashlib.sha256(json.dumps(meta['registry_sha256'],sort_keys=True).encode()).hexdigest()[:10]
    meta['preset']='vanilla-natural-v4-tuned-'+identity
    meta['river_tuning']=chosen
    (out/'IL-PRESET.json').write_text(json.dumps(meta,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(json.dumps({'directory':str(out),'preset':meta['preset'],'descriptor':meta['descriptor_version'],'settings':chosen}))

if __name__=='__main__':main()
