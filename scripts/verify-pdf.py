"""Independent PDF parser, raster checks and optional comparison to ArtDraw's GPU output."""
import argparse, json
from pathlib import Path
import numpy as np
from PIL import Image
from pypdf import PdfReader
from pypdf.generic import ContentStream

parser = argparse.ArgumentParser()
parser.add_argument('pdf', type=Path)
parser.add_argument('raster_prefix', type=Path)
parser.add_argument('--gpu', type=Path)
parser.add_argument('--report', type=Path, default=Path('target/pdf-qa-report.json'))
args = parser.parse_args()
reader = PdfReader(args.pdf, strict=True)
assert len(reader.pages) == 3, 'Expected three independent pages'
expected = [(600, 450), (300, 225), (225, 337.5)]
for page, (w, h) in zip(reader.pages, expected):
    assert abs(float(page.mediabox.width) - w) < 1e-6
    assert abs(float(page.mediabox.height) - h) < 1e-6
    assert '/Font' not in page['/Resources'], 'Text must be explicitly outlined in this version'
    assert page['/Group']['/S'] == '/Transparency'
assert '/OutputIntents' not in reader.trailer['/Root'], 'Do not pretend ICC or PDF/X compliance'
ops = ContentStream(reader.pages[0]['/Contents'], reader).operations
assert any(op == b'c' for _, op in ops), 'Missing vector cubic outlines'
assert any(op == b'B*' for _, op in ops), 'Missing even-odd fill/stroke'
alpha = [obj.get_object() for obj in reader.pages[0]['/Resources']['/ExtGState'].values()]
assert any(abs(float(a['/ca']) - 128 / 255) < 1e-8 for a in alpha)
assert any(abs(float(a['/CA']) - 192 / 255) < 1e-8 for a in alpha)
assert reader.metadata.title == 'Prova PDF - ação'
samples = [(80, 80, (255, 0, 0)), (180, 150, (127, 0, 128)), (280, 240, (127, 127, 255)),
           (520, 420, (255, 255, 255)), (430, 330, (255, 180, 0)), (730, 530, (255, 255, 255))]
pages = [Image.open(str(args.raster_prefix) + f'-{i}.png').convert('RGB') for i in (1, 2, 3)]
assert [im.size for im in pages] == [(800, 600), (400, 300), (300, 450)]
observed = []
for x, y, expected_color in samples:
    color = pages[0].getpixel((x, y))
    assert max(abs(a-b) for a,b in zip(color, expected_color)) <= 3, (x,y,color,expected_color)
    observed.append({'point':[x,y], 'pdf':list(color)})
assert np.asarray(pages[2]).min() == 255, 'Empty third page must remain white'
if args.gpu:
    gpu = np.asarray(Image.open(args.gpu).convert('RGB'))
    white = np.all(gpu >= 250, axis=2)
    candidates = []
    for y in range(gpu.shape[0]//5, gpu.shape[0]):
        row = np.r_[False, white[y], False].astype(np.int8)
        edges = np.flatnonzero(np.diff(row))
        for left, right in zip(edges[::2], edges[1::2]):
            width = right-left
            if 300 < width < gpu.shape[1]*0.9:
                candidates.append((width, int(left), int(right), y))
    assert candidates, 'Could not locate the white drawing page in GPU output'
    width, left, right, y = max(candidates)
    top, bottom = y, y+1
    while top > 0 and white[top-1,left+4] and white[top-1,right-5]: top -= 1
    while bottom < len(white) and white[bottom,left+4] and white[bottom,right-5]: bottom += 1
    height = bottom-top
    assert abs(width/height - 800/600) < 0.01, (left, top, right, bottom)
    for sample, (x,y,_) in zip(observed,samples):
        px = round(left + x*width/800)
        py = round(top + y*height/600)
        color = gpu[py,px].astype(int).tolist()
        assert max(abs(a-b) for a,b in zip(color,sample['pdf'])) <= 3, (sample,color)
        sample['gpu'] = color
result = {'result':'PASS', 'pages':3, 'sizes_pt':expected, 'rgb_alpha_and_hidden_checks':observed,
          'vector_cubics':True, 'evenodd_hole':True, 'text_outlined':True, 'icc_pdfx_claimed':False,
          'gpu_compared':bool(args.gpu)}
args.report.parent.mkdir(parents=True, exist_ok=True)
args.report.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf8')
print(json.dumps(result, ensure_ascii=False))
