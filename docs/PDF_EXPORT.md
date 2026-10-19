# Exportação PDF1.6

Arquivo → Exportar documento como PDF produz todas as páginas; Exportar página ativa como PDF produz apenas a atual. O diálogo Windows informa RGB/texto em curvas. Cancelar conserva o documento; exportar não troca o caminho nativo, conteúdo salvo, página ou histórico.

O escritor próprio produz PDF1.4 vetorial, sem dependências novas de execução. Modelo96dpi vira pontos com fator0,75 e inversão vertical; dimensões independentes por página. Pintura segue camadas/objetos visíveis, incluindo objetos bloqueados. Objetos fora da página são recortados. Matriz afim aplica-se a geometria e contorno; caps/joins redondos seguem o render atual. Quadrática é elevada exatamente para cúbica; subcontornos usam nonzero ou evenodd conforme o objeto.

Fill/stroke têm alpha independente em ExtGState ca/CA, composição Normal em DeviceRGB. Cada objeto isola seu estado gráfico. A página é branca; não há opção transparente. Texto usa contornos da fonte atual Ubuntu, conservando aparência do subconjunto atual sem fontes instaladas. Título UTF16BE conserva acentos. Não há texto pesquisável/editável nem fontes incorporadas.

Gravação ocorre após validar/serializar tudo: temporário exclusivo, sync e substituição atômica com backup.bak, pelo mesmo storage do projeto. Limites:1000 páginas,64MiB de PDF, dimensão até100mil unidades e coordenadas finitas até1bilhão. O encoder recusa dados inválidos antes de substituir um arquivo existente. Não há importação de PDF.

## Evidência própria

- 7 unitários específicos: dimensões/escopo sem mutação, xref por offsets reais, quadrática exata, affine/alpha, ocultos, evenodd/texto em curvas/Unicode e arquivo anterior intacto diante de NaN.
- --pdf-proof output/pdf/ArtDraw-proof.pdf gera corpus autoral de3 páginas e fonte.artdraw. Inclui sobreposição alpha, curva, furo, Unicode, camada oculta, objeto bloqueado, ellipse inclinada e página vazia.
- Poppler96dpi renderizou800×600,400×300 e300×450; as três imagens foram inspecionadas. pypdf strict confirmou páginas, dimensões, recursos, operações vetoriais e título. scripts/verify-pdf.py compara seis amostras de cor com expectativas e com o PNG do mesmo corpus no Vello; valores coincidiram byte-a-byte. Antialias de todas as bordas não foi certificado.
- Desktop abriu o nativo, exportou documento idêntico byte-a-byte ao CLI, navegou para página2, exportou PDF único300×225pt cujo raster é idêntico à página2 do corpus, cancelou nova exportação e fechou normalmente sem dirty. Arquivos target/desktop-pdf16.pdf e desktop-pdf16-page2.pdf.
- Roteiro GPU inclui exportação documento/página ativa após edição/grupos, sem alteração nativa; total95 checks e117 frames. Relatório consolidado em ../VALIDATION.md.

## Lacunas conservadas

Sem ICC, CMYK, spot, sangria/marcas, PDF/X/A, preflight, raster, modos de mistura/máscaras, presets, fontes editáveis, seleção, intervalos, lote ou segurança. Os escopos e a tipografia profissional seguem parciais. A forma de acesso à UI não replica Publish to PDF integralmente. Não houve ida/volta ou comparação no CorelDRAW, e PDF não resolve CDR.

Referências: [CorelDRAW2026 Publish to PDF](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html), [segurança PDF](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PDF-security-options.html), [sintaxe PDF](https://pdf-issues.pdfa.org/32000-2-2020/clause07.html), [gráficos](https://pdf-issues.pdfa.org/32000-2-2020/clause08.html) e [transparência](https://pdf-issues.pdfa.org/32000-2-2020/clause11.html). Implementação clean-room; itens ausentes em FEATURE_MATRIX.csv.
