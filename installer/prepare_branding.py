"""Generate size/format variants of the supplied logo, without redrawing it."""
from pathlib import Path
from PIL import Image, ImageOps, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
MEMORY = ROOT / 'ARCHITECTURE_MEMORY.md'
ASSETS = ROOT / 'assets' / 'branding'
SOURCE = Image.open(ASSETS / 'logo-source.jpg').convert('RGB')

def record(message):
    memory = MEMORY.read_text(encoding='utf-8-sig')
    MEMORY.write_text(memory + '\n- ' + message + '\n', encoding='utf-8')

def save(image, name, **options):
    record('GERAR assets/branding/' + name + ': variante de tamanho/formato da imagem fornecida.')
    image.save(ASSETS / name, **options)
    record('CONCLUÍDO assets/branding/' + name + '.')

def fitted(size):
    image = Image.new('RGB', size, 'white')
    logo = ImageOps.contain(SOURCE, size, Image.Resampling.LANCZOS)
    image.paste(logo, ((size[0]-logo.width)//2, (size[1]-logo.height)//2))
    return image

save(fitted((640,358)), 'splash.bmp')
save(fitted((256,256)), 'app.png')
save(fitted((256,256)), 'app.ico', sizes=[(16,16),(24,24),(32,32),(48,48),(64,64),(128,128),(256,256)])
sidebar = Image.new('RGB', (492,942), '#f7faf4')
logo = fitted((468,330))
sidebar.paste(logo, (12,210))
draw=ImageDraw.Draw(sidebar)
font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 56)
small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 25)
draw.rectangle((0,0,492,10), fill='#43a51b')
draw.text((246,590), 'ArtDraw', anchor='mm', font=font, fill='#243321')
draw.text((246,650), 'Crie em vetores.', anchor='mm', font=small, fill='#52614d')
draw.text((246,688), 'Trabalhe offline.', anchor='mm', font=small, fill='#52614d')
save(sidebar, 'wizard.bmp')
save(fitted((144,144)), 'wizard-small.bmp')
