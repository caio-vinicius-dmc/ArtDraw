# Documento vivo — o que falta para a fidelidade ao CorelDRAW

Referência de pesquisa: CorelDRAW2026 desktop Windows, documentação consultada em17/10/2026. Build executável e comparação integral ainda pendentes. O inventário conhecido não é uma auditoria exaustiva de tudo o que existe no produto; novas descobertas devem ser acrescentadas.

Inventário: 363 itens, 22 famílias; 362 no alvo offline. Indicador interno: **11,6%**. Equivalentes verificados: **0**. Todos os itens sem equivalência comparativa continuam com trabalho em aberto, inclusive os básicos.

A meta inclui funções, ferramentas, gestos, atalhos, foco, cancelamento, descoberta, temas, disposição/medidas de painéis e preservação de documentos. Presets próprios não demonstram temas idênticos. O motor interno proprietário não é conhecido; a comparação deve avaliar comportamento e saída observáveis.

## Regra extraordinária de atualização

Leia este documento antes de implementar. A cada descoberta/implementação/teste, atualize estado, aceite, evidência e lacuna em FEATURE_MATRIX.csv; regenere com scripts/update-parity.ps1. Antes de encerrar a interação ou empacotar, rode o mesmo comando com -Check. Regras persistentes em ../AGENTS.md; memória obrigatória em ../ARCHITECTURE_MEMORY.md.

Este arquivo é gerado. Não editar manualmente. Para detalhar um item, altere sua lacuna/aceite na matriz. Para CDR, veja [plano e critérios](CDR_INTEROPERABILITY.md). Para testes/pesos, veja [protocolo](VALIDATION_PROTOCOL.md).

## Resumo por estado

| Estado | Itens |
| --- | ---: |
| equivalente_verificado | 0 |
| baseline | 25 |
| implementado_basico | 46 |
| parcial | 26 |
| implementado_a_validar | 3 |
| pendente | 259 |
| referencia_a_validar | 3 |
| fora_do_alvo_offline | 1 |

## Prioridades e dependências

1. Completar modelo durável: páginas mestras, reparentamento de grupos, recursos/estilos e identidades em comandos/histórico. Páginas independentes e hierarquia/folhas têm implementação básica nas1.3/1.4.
2. Compatibilidade CDR: corpus por versão, leitor, gravador e testes nos dois sentidos; nenhuma versão homologada hoje. A pesquisa começa em paralelo ao modelo, sem chamar pesquisa de suporte.
3. Geometria e aparência: booleanos/offsets, nós tipados, fills/contornos/recortes e efeitos editáveis.
4. Texto/imagens/cor/produção: fontes/shaping/layout, raster/vetorizar, ICC/CMYK/spot, PDF e impressão.
5. Fidelidade de UI/UX: inventariar cada diálogo, flyout, estado, atalho, tema e fluxo na build de referência, comparando cenas e medidas. Auditar todas as famílias, não apenas criar ferramentas novas.

## ALINHAMENTO — 9 itens

### ALINHAMENTO-001 — Seis alinhamentos pela caixa coletiva

- Estado: **implementado_basico**; marco: M1-M5.
- Aceite: Alinhar unidades pela caixa coletiva sem mover filhos internamente.
- Evidência atual: Teste group_alignment_preserves_relative_position; seis modos usam mesma função
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-002 — Seis alinhamentos pela página

- Estado: **implementado_basico**; marco: M1-M5.
- Aceite: Cada alinhamento corresponde a borda/centro da página.
- Evidência atual: Teste all_alignment_modes_respect_page; UI: centro horizontal
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-003 — Distribuição por centros H/V

- Estado: **implementado_basico**; marco: M1-M5.
- Aceite: Três ou mais unidades têm centros igualmente espaçados H/V.
- Evidência atual: Teste distribution_centers_are_equal horizontal; vertical sem roteiro dedicado
- **Falta / diferença:** Distribuição por centros; vertical carece de roteiro dedicado e faltam todos os modos por intervalos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-004 — Distribuir intervalos iguais

- Estado: **pendente**; marco: M1-M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Distribuir intervalos iguais. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-005 — Distribuir por distância numérica

- Estado: **pendente**; marco: M1-M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Distribuir por distância numérica. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-006 — Referência ao último objeto

- Estado: **pendente**; marco: M1-M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Referência ao último objeto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-007 — Referência ponto/coordenada

- Estado: **pendente**; marco: M1-M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Referência ponto/coordenada. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-008 — Alinhar baselines de texto

- Estado: **pendente**; marco: M1-M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Alinhar baselines de texto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

### ALINHAMENTO-009 — Alinhar à grade de pixels

- Estado: **pendente**; marco: M1-M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Alinhar à grade de pixels. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html

## AUTOMACAO — 6 itens

### AUTOMACAO-001 — API de comandos e plugins

- Estado: **pendente**; marco: M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: API de comandos e plugins. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Use-macros-and-scripts-to-automate-tasks.html

### AUTOMACAO-002 — Macros VBA/VSTA equivalentes

- Estado: **pendente**; marco: M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Macros VBA/VSTA equivalentes. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Use-macros-and-scripts-to-automate-tasks.html

### AUTOMACAO-003 — Scripting JavaScript equivalente

- Estado: **pendente**; marco: M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Scripting JavaScript equivalente. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Use-macros-and-scripts-to-automate-tasks.html

### AUTOMACAO-004 — Execução em lote

- Estado: **pendente**; marco: M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Execução em lote. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Use-macros-and-scripts-to-automate-tasks.html

### AUTOMACAO-005 — Dados variáveis/print merge

- Estado: **referencia_a_validar**; marco: M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Confirmar disponibilidade na edição/build/plataforma Windows de referência; não tratar a pesquisa atual como auditoria completa.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Use-macros-and-scripts-to-automate-tasks.html

### AUTOMACAO-006 — Barcode/QR

- Estado: **referencia_a_validar**; marco: M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Confirmar disponibilidade na edição/build/plataforma Windows de referência; não tratar a pesquisa atual como auditoria completa.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Use-macros-and-scripts-to-automate-tasks.html

## CLIPPING — 6 itens

### CLIPPING-001 — Recorte vetorial editável tipo PowerClip

- Estado: **pendente**; marco: M3-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Recorte vetorial editável tipo PowerClip. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PowerClip-objects.html

### CLIPPING-002 — Editar/extrair conteúdo

- Estado: **pendente**; marco: M3-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Editar/extrair conteúdo. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PowerClip-objects.html

### CLIPPING-003 — Fit/fill/stretch de conteúdo

- Estado: **pendente**; marco: M3-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Fit/fill/stretch de conteúdo. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PowerClip-objects.html

### CLIPPING-004 — Recortes aninhados

- Estado: **pendente**; marco: M3-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Recortes aninhados. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PowerClip-objects.html

### CLIPPING-005 — Máscaras e composição

- Estado: **pendente**; marco: M3-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Máscaras e composição. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PowerClip-objects.html

### CLIPPING-006 — Clip paths SVG

- Estado: **pendente**; marco: M3-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Clip paths SVG. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PowerClip-objects.html

## CONTORNO — 10 itens

### CONTORNO-001 — Cor e largura

- Estado: **baseline**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-002 — Cap e join configuráveis

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Cap e join configuráveis. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-003 — Miter limit

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Miter limit. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-004 — Tracejado e fase

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Tracejado e fase. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-005 — Setas/pontas

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Setas/pontas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-006 — Largura variável

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Largura variável. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-007 — Posição interior/centro/exterior

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Posição interior/centro/exterior. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-008 — Escalar ou preservar espessura

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Escalar ou preservar espessura. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-009 — Converter contorno em objeto

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Converter contorno em objeto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CONTORNO-010 — Contorno caligráfico

- Estado: **pendente**; marco: M3-M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Contorno caligráfico. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

## COR — 10 itens

### COR-001 — RGB sRGB no canvas

- Estado: **baseline**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Textura sRGB não implementa ICC, CMYK, spot, soft proof ou separações.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-002 — CMYK real com perfis

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: CMYK real com perfis. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-003 — Cor spot/separação

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Cor spot/separação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-004 — Perfis ICC de documento

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Perfis ICC de documento. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-005 — Perfis de monitor/saída

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Perfis de monitor/saída. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-006 — Intenções de renderização

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Intenções de renderização. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-007 — Prova de cor/soft proof

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Prova de cor/soft proof. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-008 — Políticas ao abrir/importar

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Políticas ao abrir/importar. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-009 — Overprint e limites de tinta

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Overprint e limites de tinta. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

### COR-010 — Bibliotecas de paleta licenciadas

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Bibliotecas de paleta licenciadas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html

## CURVAS — 23 itens

### CURVAS-001 — Editar nó/duas alças

- Estado: **baseline**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-002 — Cúbica matemática

- Estado: **baseline**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-003 — Inserir nó sem mudar curva

- Estado: **baseline**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-004 — Nós cusp/smooth/symmetric

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Nós cusp/smooth/symmetric. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-005 — Seleção múltipla de nós

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Seleção múltipla de nós. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-006 — Adicionar/remover nó

- Estado: **parcial**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Adicionar via divisão e remover nó simples; faltam seleção de múltiplos nós e variantes de preservação/topologia.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-007 — Converter segmento linha/curva

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Converter segmento linha/curva. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-008 — Juntar nós e extremidades

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Juntar nós e extremidades. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-009 — Quebrar caminho

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Quebrar caminho. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-010 — Fechar/abrir caminho

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Fechar/abrir caminho. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-011 — Reduzir nós por tolerância

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Reduzir nós por tolerância. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-012 — Alinhar/distribuir nós

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Alinhar/distribuir nós. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-013 — Espelhar edição de nós

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Espelhar edição de nós. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-014 — Escalar/girar seleção de nós

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Escalar/girar seleção de nós. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-015 — Editar todos subcaminhos de composto

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Editar todos subcaminhos de composto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-016 — Corte e apagamento geométrico

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Corte e apagamento geométrico. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-017 — União booleana

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: União booleana. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-018 — Interseção booleana

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Interseção booleana. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-019 — Diferença/aparar

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Diferença/aparar. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-020 — Combinar/separar subcaminhos

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Combinar/separar subcaminhos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-021 — Boundary

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Boundary. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-022 — Arredondar/chanfrar/filetar

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Arredondar/chanfrar/filetar. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### CURVAS-023 — Contorno e offset robusto

- Estado: **pendente**; marco: M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Contorno e offset robusto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

## EDICAO — 9 itens

### EDICAO-001 — Undo/redo básico

- Estado: **baseline**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Snapshots limitados a100; faltam orçamento para documentos grandes e equivalência de todos os comandos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-002 — Histórico por gesto coletivo

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Uma ação coletiva produz um estado de histórico; undo/redo restaura grupos.
- Evidência atual: UI: undo/redo de grupo
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-003 — Duplicação de conjunto

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Cópia mantém arranjo, remapeia grupos e seleciona o resultado.
- Evidência atual: Teste copying_remaps_group_ids; UI: copiar grupo
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-004 — Clipboard interno copiar/recortar/colar

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Ctrl+C/V preserva conjunto dentro do aplicativo; recorte usa exclusão comum.
- Evidência atual: UI: copiar/colar grupo; recortar ainda sem gesto dedicado; sem clipboard externo
- **Falta / diferença:** Clipboard é interno ao ArtDraw; falta interoperabilidade de tipos/dados com outros aplicativos e teste dedicado de recortar.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-005 — Clipboard Windows SVG/bitmap/texto

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Clipboard Windows SVG/bitmap/texto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-006 — Colar especial

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Colar especial. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-007 — Clones vinculados

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Clones vinculados. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-008 — Símbolos e instâncias

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Símbolos e instâncias. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### EDICAO-009 — Localizar/substituir propriedades

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Localizar/substituir propriedades. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

## EFEITOS — 16 itens

### EFEITOS-001 — Sombra externa editável

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Sombra externa editável. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-002 — Sombra interna

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Sombra interna. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-003 — Sombra vetorial sólida

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Sombra vetorial sólida. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-004 — Blend entre objetos

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Blend entre objetos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-005 — Contorno interativo

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Contorno interativo. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-006 — Envelope editável

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Envelope editável. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-007 — Distorções parametrizadas

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Distorções parametrizadas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-008 — Extrusão e iluminação

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Extrusão e iluminação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-009 — Bevel

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Bevel. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-010 — Perspectiva de objeto

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Perspectiva de objeto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-011 — Grupos de perspectiva

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Grupos de perspectiva. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-012 — Simetria viva

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Simetria viva. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-013 — Desenho paralelo

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Desenho paralelo. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-014 — Lentes

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Lentes. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-015 — Transparência com modos de mistura

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Transparência com modos de mistura. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

### EFEITOS-016 — Efeitos bitmap não destrutivos

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Efeitos bitmap não destrutivos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html

## FILL — 16 itens

### FILL-001 — RGBA uniforme

- Estado: **baseline**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-002 — Sem preenchimento

- Estado: **baseline**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-003 — Paleta esquerdo fill/direito stroke

- Estado: **baseline**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-004 — Transparência uniforme RGBA

- Estado: **baseline**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-005 — Degradê linear

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Degradê linear. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-006 — Degradê elíptico/radial

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Degradê elíptico/radial. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-007 — Degradê cônico/retangular

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Degradê cônico/retangular. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-008 — Stops e interpolação editáveis

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Stops e interpolação editáveis. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-009 — Preenchimento bitmap

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Preenchimento bitmap. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-010 — Padrão vetorial

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Padrão vetorial. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-011 — Padrão bicolor

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Padrão bicolor. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-012 — Textura procedural

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Textura procedural. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-013 — Preenchimento PostScript

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Preenchimento PostScript. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-014 — Malha de preenchimento

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Malha de preenchimento. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-015 — Smart fill por regiões

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Smart fill por regiões. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

### FILL-016 — Amostragem de cor/atributos

- Estado: **pendente**; marco: M4.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Amostragem de cor/atributos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html

## FORMAS — 12 itens

### FORMAS-001 — Retângulo/quadrado

- Estado: **baseline**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-002 — Elipse/círculo

- Estado: **baseline**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-003 — Polígono regular com quantidade

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Criar polígono regular; quantidade escolhida antes de desenhar.
- Evidência atual: UI: polígono; forma vira caminho
- **Falta / diferença:** Quantidade definida na criação; após criar vira caminho, sem edição paramétrica persistente.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-004 — Estrela com quantidade

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Criar estrela de 2N nós com raios alternados.
- Evidência atual: UI: estrela; teste polygon_and_star_have_expected_nodes
- **Falta / diferença:** Estrela vira caminho; faltam agudeza/handles/parâmetros e equivalência de tipos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-005 — Espiral simétrica aproximada

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Criar espiral poligonal aproximada; não é espiral paramétrica editável.
- Evidência atual: UI: espiral
- **Falta / diferença:** Polilinha aproximada; faltam modos logarítmico/simétrico completos e edição persistente.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-006 — Espiral logarítmica

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Espiral logarítmica. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-007 — Parametrização editável após criação

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Parametrização editável após criação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-008 — Retângulo de cantos individuais

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Retângulo de cantos individuais. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-009 — Arco/setor de elipse

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Arco/setor de elipse. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-010 — Formas básicas/setas/banners

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Formas básicas/setas/banners. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-011 — Papel gráfico paramétrico

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Papel gráfico paramétrico. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

### FORMAS-012 — Formas de impacto

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Formas de impacto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html

## FORMATOS — 22 itens

### FORMATOS-001 — SVG básico bidirecional

- Estado: **parcial**; marco: M2-M9.
- Aceite: Import/export SVG de uma página com aviso de limites; salvar documento multipágina como SVG é recusado; exportação ativa explícita.
- Evidência atual: UI SVG round-trip e proteção multipágina; teste de página com identidade não inicial.
- **Falta / diferença:** SVG não preserva gradientes, padrões, raster, filtros/máscaras/recortes/CSS/use/texto avançado. Exporta somente página ativa; sem garantia de ida/volta CorelDRAW.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-002 — Metadados SVG de camadas/grupos

- Estado: **implementado_basico**; marco: M2-M9.
- Aceite: Importar SVG g aninhado; exportar wrappers padrão e metadados ArtDraw preservando hierarquia, IDs, camadas, estados e coordenadas.
- Evidência atual: Unitários wrappers g aninhados/roundtrip PASS; GPU80 e arquivo nativo/SVG ativo preservam árvore/estilos/IDs. Fixture externa importada e salva no desktop.
- **Falta / diferença:** Editores externos podem remover metadados; grupos legados intercalados geram wrappers separados para preservar z-order. Texto continua exportado em contornos. Sem QA no CorelDRAW.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-003 — Formato nativo versionado

- Estado: **implementado_basico**; marco: M2-M9.
- Aceite: Schema XML2 guarda páginas/nomes/dimensões/camadas/objetos/IDs e página ativa; lê schema1.
- Evidência atual: storage::page_tests; UI salvar/reler multipágina; cópia desktop preservou3 páginas/16 objetos.
- **Falta / diferença:** Não é CDR. Árvore de grupos/recursos/texto/efeitos avançados ainda ausentes do modelo. ArtDraw1.2 não lê schema2.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-004 — CDR leitura por versão

- Estado: **pendente**; marco: M2-M9.
- Aceite: Abrir CDR por versão/build declarada preservando páginas/hierarquia, geometria, texto/fontes, estilos/efeitos, imagens, cor/recursos e editabilidade; validar no CorelDRAW e relatar toda perda.
- Evidência atual: Sem leitor integrado.1.6: API e fluxo público libcdr estudados, incluindo envelope RIFF/Waldo e streams estruturados root.dat/riffData.cdr, arquivos externos e perfis; somente pesquisa. CDR_INTEROPERABILITY.md.
- **Falta / diferença:** Nenhum leitor CDR integrado, nenhuma versão homologada. Detalhar versões, recursos e testes CDR-R01..R07/CDR-Q em CDR_INTEROPERABILITY.md.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-005 — CDR escrita por versão

- Estado: **pendente**; marco: M2-M9.
- Aceite: Gravar CDR real por versão, abrir/editar no CorelDRAW e retornar ao ArtDraw com equivalência visual e semântica; não achatar nem converter tipos silenciosamente; manter original/backup.
- Evidência atual: Não implementado; nenhuma escrita/ida e volta CDR homologada; plano CDR_INTEROPERABILITY.md.
- **Falta / diferença:** Nenhum gravador CDR. Exigir estrutura real e ida/volta visual/semântica/editável nas duas aplicações; CDR-W01..W03/CDR-Q.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-006 — AI import/export por versão

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: AI import/export por versão. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-007 — EPS/PS import/export

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: EPS/PS import/export. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-008 — PDF importação

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: PDF importação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-009 — PDF exportação

- Estado: **implementado_basico**; marco: M2-M9.
- Aceite: Exportar documento multipágina ou página ativa como PDF vetorial legível em parser/render externo; dimensões96dpi→pt, curvas, transformações, flags e save seguro.
- Evidência atual: ArtDraw1.6:7 unitários PDF; pypdf strict/Poppler3 páginas inspecionadas;6 amostras iguais a Vello;95 checks GPU; desktop doc3 páginas e página2 isolada/cancelar. docs/PDF_EXPORT.md e VALIDATION.md.
- **Falta / diferença:** Saída PDF1.4 RGB com texto em curvas. Sem importação, presets, texto editável/fontes, ICC/CMYK/spot/PDFX/PDFA, seleção/intervalos/lote ou preflight; comparação Corel ausente.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### FORMATOS-010 — PNG exportação

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: PNG exportação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-011 — JPEG exportação

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: JPEG exportação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-012 — TIFF exportação

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: TIFF exportação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-013 — WEBP

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: WEBP. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-014 — DXF/DWG por versão

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: DXF/DWG por versão. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-015 — PLT/HPGL

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: PLT/HPGL. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-016 — CMX

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: CMX. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-017 — PSD/CPT

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: PSD/CPT. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-018 — DOCX/RTF texto

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: DOCX/RTF texto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-019 — WMF/EMF

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: WMF/EMF. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-020 — Formatos legados adicionais

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Formatos legados adicionais. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-021 — Relatório de perdas por filtro

- Estado: **pendente**; marco: M2-M9.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Relatório de perdas por filtro. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html

### FORMATOS-022 — Validação de documentos nativos

- Estado: **implementado_basico**; marco: M2.
- Aceite: Rejeitar IDs duplicados, valores inválidos, versão desconhecida e referências de camada inválidas.
- Evidência atual: Testes storage; limite32MiB/50mil objetos
- **Falta / diferença:** Valida XML1 atual; faltam testes por futuros schemas, corpus hostil/extenso e adaptadores CDR.
- Referência: ArtDraw; requisito próprio de produto; não é evidência de internals Corel

## INFRA — 15 itens

### INFRA-001 — Execução offline

- Estado: **baseline**; marco: M2-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-002 — Render GPU Vello

- Estado: **baseline**; marco: M2-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Vello/wgpu é implementação própria; faltam corpus extenso de composição/desempenho e comparação de saída. Internals Corel desconhecidos.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-003 — Cache de cena

- Estado: **parcial**; marco: M2-M8.
- Aceite: Cache gráfico da página ativa invalida por página/conteúdo, vista, draft, tema e tamanho, preservando a renderização GPU.
- Evidência atual: src/render.rs guarda Page, não Document; roteiro multipágina e captura GPU no release1.3.
- **Falta / diferença:** Faltam grafo incremental por recursos/efeitos, orçamento e benchmarks de documentos grandes; histórico ainda usa snapshots completos.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-004 — Tiling/culling e índice espacial

- Estado: **pendente**; marco: M2-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Tiling/culling e índice espacial. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-005 — Orçamento de memória

- Estado: **pendente**; marco: M2-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Orçamento de memória. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-006 — Recuperação de documento

- Estado: **implementado_basico**; marco: M2-M8.
- Aceite: Oferecer sessão encerrada e restaurar como documento editável sem sobrescrever original.
- Evidência atual: 4 testes recovery e smoke3 PASS. Na revisão1.7 houve1 falha DirectoryNotEmpty na limpeza de pasta de teste; pasta vazia depois. Harness passou a repetir só remove_dir vazio por até1s;79 unitários PASS e3 repetições adicionais dos4 recovery PASS. Produto não alterado; VALIDATION.md. Release1.9:93 unitários e3 checks de recuperação GPU PASS, sessão QA isolada.
- **Falta / diferença:** Recuperação de sessões encerradas testada; falta corpus de falhas/energia e opções completas. Não recupera toda alteração antes do checkpoint.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-007 — Autosave e backups

- Estado: **implementado_basico**; marco: M2-M8.
- Aceite: Checkpoint validado por sessão após gestos/intervalo15s e backup anterior no save; documento completo inclui páginas.
- Evidência atual: Testes checkpoint com duas páginas, undo/backup, nativo2; UI recuperação3.
- **Falta / diferença:** Faltam configuração de intervalo, desligamento físico, concorrência e matriz de volumes; não garante cada gesto antes do checkpoint.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-008 — Migrações de arquivo

- Estado: **parcial**; marco: M2-M8.
- Aceite: Migrar schema1 para2 mantendo geometria/IDs; alocar IDs de SVG legado; rejeitar versão futura.
- Evidência atual: Fixture versionada tests/fixtures/legacy-v1.artdraw e testes storage::page_tests.
- **Falta / diferença:** Migração1→2 implementada; futuras versões e CDR continuam não suportados. Sem corpus amplo entre versões.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-009 — Arrastar/soltar arquivos

- Estado: **implementado_basico**; marco: M2-M8.
- Aceite: Soltar abre documento completo ArtDraw/SVG; importar documento multipágina na página ativa é recusado com diagnóstico.
- Evidência atual: UI drop nativo; caminhos comuns de abertura/importação; testes do modelo nativo.
- **Falta / diferença:** Faltam múltiplas janelas e opções de importação por página. A recusa de importar multipágina ainda precisa de roteiro dedicado de diálogo.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-010 — Múltiplos documentos/abas

- Estado: **pendente**; marco: M2-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Múltiplos documentos/abas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-011 — Benchmarks de cenas grandes

- Estado: **pendente**; marco: M2-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Benchmarks de cenas grandes. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-012 — Tolerância a perda de GPU

- Estado: **pendente**; marco: M2-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Tolerância a perda de GPU. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### INFRA-013 — Instalador por usuário

- Estado: **parcial**; marco: M2-M8.
- Aceite: Instalar com logo/licença/pasta/atalhos, registrar versão e payload correto, executar app e desinstalar preservando documentos; validar políticas Windows.
- Evidência atual: 1.4/1.5 ciclo completo PASS.1.6 QA16-pdf instalação/registro/atalhos/hashes/self-test PASS; uninstaller bloqueado pelo App Control, eventos3033/3077/3089.1.9 TESTMODE19-relative-payload PASS20 hashes e self-test1.9.0, registros inalterados, sem atalhos/uninstaller. Reconferência final registrada em relatório separado; VALIDATION.md e WINDOWS_DISTRIBUTION.md.
- **Falta / diferença:** Regressão ambiental aberta: unins000.exe1.6 sem assinatura foi bloqueado; ciclo completo não homologado. Instalação QA ficou registrada em target. Falta assinatura de publicação e matriz Windows/DPI/permissões; não replica instalador Corel.
- Referência: https://jrsoftware.org/ishelp/topic_setup_signeduninstaller.htm

### INFRA-014 — IDs persistentes de objetos

- Estado: **implementado_basico**; marco: M2.
- Aceite: IDs de objetos únicos em todo o documento, inclusive cópia entre páginas e exclusão; IDs de páginas estáveis.
- Evidência atual: Testes multipágina e UI duplicação sem colisão; contadores persistentes no schema2.
- **Falta / diferença:** Seleção/histórico ainda usam índices locais e snapshots; árvore de grupos/recursos continua pendente.
- Referência: ArtDraw; requisito próprio de produto; não é evidência de internals Corel

### INFRA-015 — Gravação atômica e backup anterior

- Estado: **implementado_basico**; marco: M2.
- Aceite: Escrever/sincronizar temporário e trocar destino somente após sucesso.
- Evidência atual: Teste atomic_save_keeps_previous_backup; MoveFileExW no mesmo diretório
- **Falta / diferença:** Backup/atomicidade testados; falta desligamento físico, concorrência e matriz de arquivos bloqueados/volumes.
- Referência: ArtDraw; requisito próprio de produto; não é evidência de internals Corel

## ORGANIZACAO — 24 itens

### ORGANIZACAO-001 — Grupos simples

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Mover/copiar grupo como unidade preservando atributos e disposição; camada comum para agrupamentos novos.
- Evidência atual: 59 unitários/80 verificações GPU PASS; desktop: selecionar conjunto, separar, undo/salvar/reabrir; VALIDATION.md.
- **Falta / diferença:** Comparação com CorelDRAW ainda ausente; editar filhos e reparentear pelo docker são itens separados.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### ORGANIZACAO-002 — Grupos aninhados

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Agrupar unidades preservando subgrupos; seleção, transformação, cópia, histórico e persistência nativa/SVG conservam a árvore.
- Evidência atual: 59 unitários/80 GPU PASS: árvore, cópia, IDs, histórico, schema3/migração1/2/SVG; desktop fixture3 folhas/2 níveis reaberta; docs/GROUP_HIERARCHY.md.
- **Falta / diferença:** Máximo64 níveis; sem seleção progressiva de subgrupos, Focus mode ou drag/reparentamento. Controles agregados V/B e cadeias extensas precisam corpus UI adicional; equivalência original não comparada.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### ORGANIZACAO-003 — Desagrupar simples

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Desagrupar um nível preserva subgrupos, estilos e transformações; undo/redo exato.
- Evidência atual: Unitário e GPU PASS; desktop Ctrl+U preservou subgrupo de2 folhas, arquivo e backup conferidos; undo restaurou raiz.
- **Falta / diferença:** Sem comparação interativa com CorelDRAW; grupos com efeitos/recursos ainda não representados no modelo.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### ORGANIZACAO-004 — Desagrupar todos os níveis

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Separar nível externo preserva filhos; separar tudo remove todos os níveis selecionados; undo/redo e reabertura exatos.
- Evidência atual: Unitário/GPU/desktop PASS: botão separa todos os níveis; undo e save/reabertura exatos, XML sem arestas/grupos.
- **Falta / diferença:** Sem comparação com original; efeitos de grupo e casos de recursos compartilhados ainda ausentes.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### ORGANIZACAO-005 — Trazer/enviar seleção

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Frente/trás de página e camada distintos; preservar grupos/ordem relativa, pular camada não editável, histórico e reabertura. Menu/painel desabilitados no limite, sem alteração/histórico ao clicar ou usar atalho neutro.
- Evidência atual: 1.8:86 unitários/116 GPU130frames PASS.4 regressões de transferência falharam antes e passaram depois;1120 combinações de disponibilidade. Desktop transfere pulando camada bloqueada, undo/redo, menu/painel desabilitados no limite, clique neutro, SaveAs/XML/reabrir PASS. Objetos íntegros salvo camada esperada; docs/STACKING_ORDER.md e VALIDATION.md. Regressão1.9:93 unitários/139 GPU138frames PASS após compartilhar finalização com alvo; provas desktop anteriores preservadas.
- **Falta / diferença:** Sem comparação original. Mestras, folha interna e grupos legados intercalados não suportados. Visibilidade também controla destino no ArtDraw. Projeção conserva slots de camadas não envolvidas; comandos opostos não garantem interleaving bruto/estado salvo original, use undo para restauração exata. Falta ampliar corpus entre camadas e auditar semântica de dirty/desempenho/estados na build original.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Change-the-order-of-objects.html

### ORGANIZACAO-006 — Subir/descer um nível

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Avançar/recuar uma posição; no limite transferir unidade para camada editável vizinha pulando bloqueadas; grupos intactos, IDs e histórico exatos. Menu/painel desabilitados no limite, sem alteração/histórico ao clicar ou usar atalho neutro.
- Evidência atual: 1.8:86 unitários/116 GPU130frames PASS.4 regressões de transferência falharam antes e passaram depois;1120 combinações de disponibilidade. Desktop transfere pulando camada bloqueada, undo/redo, menu/painel desabilitados no limite, clique neutro, SaveAs/XML/reabrir PASS. Objetos íntegros salvo camada esperada; docs/STACKING_ORDER.md e VALIDATION.md. Regressão1.9:93 unitários/139 GPU138frames PASS após compartilhar finalização com alvo; provas desktop anteriores preservadas.
- **Falta / diferença:** Sem comparação original. Mestras, folha interna e grupos legados intercalados não suportados. Visibilidade também controla destino no ArtDraw. Projeção conserva slots de camadas não envolvidas; comandos opostos não garantem interleaving bruto/estado salvo original, use undo para restauração exata. Falta ampliar corpus entre camadas e auditar semântica de dirty/desempenho/estados na build original.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Change-the-order-of-objects.html

### ORGANIZACAO-007 — Nome de objeto

- Estado: **baseline**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-008 — Ocultar objeto

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Objeto oculto não renderiza nem participa do hit-test; SVG mantém estado.
- Evidência atual: Teste layer_lock_and_visibility_affect_hit; SVG nativo
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-009 — Bloquear objeto

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Objeto bloqueado não edita; grupo com membro bloqueado não transforma parcialmente.
- Evidência atual: Testes de hit e groups_do_not_transform_partially_when_locked
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-010 — Criar/renomear camada

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Criar camada com ID distinto, nome editável e histórico.
- Evidência atual: UI verifica criação; renomeação exige roteiro manual adicional
- **Falta / diferença:** Criar foi testado; renomear exige gesto dedicado; camadas mestras/páginas ainda ausentes.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-011 — Ordenar camadas

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Ordem das camadas governa pintura independentemente dos índices dos objetos.
- Evidência atual: Teste layer_order_is_independent_of_indices; gesto de ordenação ainda sem teste dedicado
- **Falta / diferença:** Ordem matemática testada; falta roteiro do gesto, hierarquia e comparação no original.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-012 — Camada ativa

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Novo desenho recebe ID da camada ativa editável.
- Evidência atual: UI: desenho na camada ativa
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-013 — Ocultar/bloquear camada

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Ocultar remove da pintura/hit; bloquear impede edição mantendo pintura.
- Evidência atual: UI: ocultar/bloquear camada; testes de hit
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-014 — Mover seleção entre camadas

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Mover seleção entre camadas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-015 — Imprimir/exportar por camada

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Imprimir/exportar por camada. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-016 — Camadas mestras

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Camadas mestras. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-017 — Pesquisar objetos

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Pesquisar objetos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-018 — Estilos de objeto

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Estilos de objeto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-019 — Estilos de cor

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Estilos de cor. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html

### ORGANIZACAO-020 — Adicionar membro ao grupo pelo docker

- Estado: **pendente**; marco: M2.
- Aceite: Arrastar objeto para grupo existente, preservar posição e estilo, adotar camada do conjunto; impedir ciclos; undo e persistência.
- Evidência atual: Comportamento identificado na ajuda oficial2026 Windows, To add an object to a group.
- **Falta / diferença:** Não há drag/reparentamento na árvore. Faltam indicador de destino, validação transacional e casos com grupos ocultos/bloqueados.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### ORGANIZACAO-021 — Remover membro do grupo pelo docker

- Estado: **pendente**; marco: M2.
- Aceite: Arrastar membro para fora do grupo preservando geometria/estilo/ordem explícita; podar ancestrais vazios e recuperar por undo.
- Evidência atual: Comportamento identificado na ajuda oficial2026 Windows, To remove an object from a group.
- **Falta / diferença:** Não há drag para fora ou mudança de pai. Remover membro sem apagar e desagrupar são operações distintas.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### ORGANIZACAO-022 — Inverter ordem da seleção

- Estado: **parcial**; marco: M2.
- Aceite: Inverter somente unidades selecionadas da mesma camada nos seus slots, preservar estrutura/atributos/demais unidades; involução, undo/redo e persistência.
- Evidência atual: 1.8 final:86 unitários e116 checks GPU130frames PASS;5 testes de inversão incluem interleaving/involução/encode exatos. Desktop canônico e intercalado PASS: menu/botão/dupla inversão limpa dirty/undo/redo/SaveAs/XML/reabertura. Variante intercalada salva raw3,4,2,5,1 com todos objetos integrais por ID, camadas e página2 intactos. Payload1.7 TESTMODE17-reverse-payload PASS20 hashes/self-test, sem registro/atalhos/uninstaller. VALIDATION.md. Regressão1.9:93 unitários/139 GPU138frames PASS após compartilhar finalização com alvo; provas desktop anteriores preservadas.
- **Falta / diferença:** Somente>=2 unidades na mesma camada. Inversão entre camadas, folha interna de grupo, grupos legados intercalados e equivalência comparativa continuam pendentes. Desabilitação de seis comandos1.8 testada no planner/GPU/desktop; estados comparativos na build original continuam pendentes.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Change-the-order-of-objects.html

### ORGANIZACAO-023 — Ordenar imediatamente à frente de um alvo

- Estado: **parcial**; marco: M2.
- Aceite: Selecionar unidades completas e clicar no alvo; inserir bloco imediatamente antes/depois na camada editável do alvo, preservar IDs/atributos/arestas e ordem interna. Cancelar sem conteúdo/histórico; undo/redo e nativo/SVG reabertos.
- Evidência atual: 1.9:93 unitários PASS, sete novos de alvo;139 checks GPU/138frames e3 recovery PASS. GPU cobre frente/atrás, alvos inválidos, Esc/direito, não arrastar, histórico, nativo/SVG. Desktop frente pelo painel/atrás pelo menu com transferência1→3, cancelar, undo/redo, SaveAs/XML integral/reabrir/fechar PASS. target/desktop-relative19-report.json; STACKING_ORDER.md e VALIDATION.md.
- **Falta / diferença:** Sem comparação CorelDRAW. Folha isolada, mestras, inversão entre camadas e reparentamento ausentes. Documento com qualquer raiz legada intercalada ou grupo plano multicamada recusa o modo, mesmo fora da seleção. Alvo próprio/oculto/bloqueado ou raiz com membro não editável recusado; sem ciclo para alvo ocluído nem destaque de hover. Regras exatas de grupos/camadas/gestos/estados exigem build original; aparência/estrutura/edição CDR não suportadas.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Change-the-order-of-objects.html

### ORGANIZACAO-024 — Ordenar imediatamente atrás de um alvo

- Estado: **parcial**; marco: M2.
- Aceite: Selecionar unidades completas e clicar no alvo; inserir bloco imediatamente antes/depois na camada editável do alvo, preservar IDs/atributos/arestas e ordem interna. Cancelar sem conteúdo/histórico; undo/redo e nativo/SVG reabertos.
- Evidência atual: 1.9:93 unitários PASS, sete novos de alvo;139 checks GPU/138frames e3 recovery PASS. GPU cobre frente/atrás, alvos inválidos, Esc/direito, não arrastar, histórico, nativo/SVG. Desktop frente pelo painel/atrás pelo menu com transferência1→3, cancelar, undo/redo, SaveAs/XML integral/reabrir/fechar PASS. target/desktop-relative19-report.json; STACKING_ORDER.md e VALIDATION.md.
- **Falta / diferença:** Sem comparação CorelDRAW. Folha isolada, mestras, inversão entre camadas e reparentamento ausentes. Documento com qualquer raiz legada intercalada ou grupo plano multicamada recusa o modo, mesmo fora da seleção. Alvo próprio/oculto/bloqueado ou raiz com membro não editável recusado; sem ciclo para alvo ocluído nem destaque de hover. Regras exatas de grupos/camadas/gestos/estados exigem build original; aparência/estrutura/edição CDR não suportadas.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Change-the-order-of-objects.html

## PAGINAS — 12 itens

### PAGINAS-001 — Dimensão de uma página

- Estado: **baseline**; marco: M2-M7.
- Aceite: Dimensões em px da página ativa persistidas individualmente; histórico restaura alterações.
- Evidência atual: Modelo Page; testes de isolamento/round-trip e UI orientação independente.
- **Falta / diferença:** Faltam unidades físicas/presets completos e comparação com o original; visão simultânea, mestras e imposição pendentes.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-002 — Orientação

- Estado: **implementado_basico**; marco: M2-M7.
- Aceite: Retrato/paisagem alteram somente dimensões da página ativa, mantendo coordenadas dos objetos; undo/redo e nativo.
- Evidência atual: UI release: orientação independente; desktop: páginas em retrato e paisagem após reabrir.
- **Falta / diferença:** Sem aplicar em lote, impressora/presets completos ou comparação do gesto com a build CorelDRAW.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-003 — Presets de página

- Estado: **pendente**; marco: M2-M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Presets de página. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-004 — Adicionar/excluir/duplicar páginas

- Estado: **implementado_basico**; marco: M2-M7.
- Aceite: Adicionar vazia/duplicar/excluir com confirmação; nunca excluir última; cópias com IDs únicos; undo/redo restaura página.
- Evidência atual: 3 testes do modelo; UI release criação, desenho isolado, duplicação, exclusão confirmada e undo/redo.
- **Falta / diferença:** Faltam inserção/duplicação por intervalos, opções avançadas e equivalência visual/gestual com a referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-005 — Dimensões independentes

- Estado: **implementado_basico**; marco: M2-M7.
- Aceite: Dimensões/objetos/camadas independentes por página, preservados no schema2 e recuperação.
- Evidência atual: Testes isolamento/round-trip; UI orientação independente; desktop cópia e reabertura de3 páginas/16 objetos.
- **Falta / diferença:** Unidades atuais são px; presets/unidades físicas, páginas mestras e mudanças em lote continuam pendentes.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-006 — Visão multipágina

- Estado: **pendente**; marco: M2-M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Visão multipágina. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-007 — Ordenação/navegação de páginas

- Estado: **implementado_basico**; marco: M2-M7.
- Aceite: Navegar por seletor/prev/next/PageUp/PageDown sem dirty/histórico; mover página antes/depois com undo e IDs estáveis.
- Evidência atual: Modelo e UI release: ordenar/navegar sem mudar conteúdo; PageDown e gerenciador exercitados no desktop.
- **Falta / diferença:** Faltam drag-and-drop de páginas, navegação de intervalos e comparação completa de UI/UX com CorelDRAW.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-008 — Páginas opostas

- Estado: **pendente**; marco: M2-M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Páginas opostas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-009 — Margens e sangria

- Estado: **pendente**; marco: M2-M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Margens e sangria. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-010 — Imposição e cadernos

- Estado: **pendente**; marco: M2-M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Imposição e cadernos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-011 — Etiquetas e marcas

- Estado: **pendente**; marco: M2-M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Etiquetas e marcas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

### PAGINAS-012 — Template de documento

- Estado: **pendente**; marco: M2-M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Template de documento. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html

## PRECISAO — 13 itens

### PRECISAO-001 — Unidades px/mm/cm/in/pt na UI

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Unidades px/mm/cm/in/pt na UI. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-002 — Régua

- Estado: **baseline**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Régua básica em coordenadas do documento; faltam unidades físicas completas, origem arrastável e equivalência de UI.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-003 — Grade de referência

- Estado: **baseline**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-004 — Snapping à grade

- Estado: **implementado_a_validar**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: src/; validação do incremento pendente
- **Falta / diferença:** Código existe, mas falta roteiro específico suficiente; validar gesto, cancelamento, histórico, persistência e comparação com a referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-005 — Snapping a nós/centros/arestas

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Snapping a nós/centros/arestas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-006 — Guias arrastáveis

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Guias arrastáveis. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-007 — Guias dinâmicas

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Guias dinâmicas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-008 — Snapping à página

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Snapping à página. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-009 — Grade de baseline

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Grade de baseline. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-010 — Snapping temporariamente desativado

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Snapping temporariamente desativado. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-011 — Painel de coordenadas

- Estado: **parcial**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Campos básicos X/Y/L/A; faltam âncora, unidades físicas/expressões e equivalência de contextos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-012 — Cotas associativas

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Cotas associativas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

### PRECISAO-013 — Conectores associativos

- Estado: **pendente**; marco: M2-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Conectores associativos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html

## PRODUCAO — 17 itens

### PRODUCAO-001 — Exportar seleção/página/lote

- Estado: **parcial**; marco: M7.
- Aceite: Exportar página atual/documento, seleção, intervalos e lotes sem alterar o nativo; saída comparada por consumidor externo.
- Evidência atual: 1.6: documento inteiro3 páginas e página ativa2 exportados pelo desktop; parser/raster independente confirmou escopo/dimensões e nativo permaneceu salvo. docs/PDF_EXPORT.md.
- **Falta / diferença:** Documento/página disponíveis para PDF; SVG página ativa. Seleção, intervalos e lote continuam ausentes; detalhados em PRODUCAO-012 a015. Sem equivalência comparativa.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-002 — Presets de exportação

- Estado: **pendente**; marco: M7.
- Aceite: Selecionar presets e criar/editar/remover configurações reutilizáveis, separando segurança da configuração persistida.
- Evidência atual: Ajuda oficial2026: Publish to PDF permite presets próprios; não implementado.
- **Falta / diferença:** Saída ArtDraw1.6 fixa PDF1.4 RGB/texto em curvas. Não há gerenciador de presets nem equivalência com presets da referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-003 — PDF com texto/fontes

- Estado: **parcial**; marco: M7.
- Aceite: Manter tipografia visual vetorial e oferecer texto/fontes editáveis/incorporados com permissões, shaping e corpus multilingue.
- Evidência atual: 1.6: contornos vetoriais Ubuntu/Unicode/quebras visualizados em Poppler; parser confirmou ausência de recursos Font. Nenhuma implementação de fontes incorporadas/editáveis.
- **Falta / diferença:** Somente aparência em curvas no subconjunto tipográfico atual. Sem texto pesquisável/editável, embedding, fontes do sistema, shaping/bidi ou corpus amplo. PRODUCAO-011 permanece pendente.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-004 — PDF com transparência

- Estado: **parcial**; marco: M7.
- Aceite: Preservar alpha independente de fill/contorno como ExtGState e composição normal; conferir raster versus GPU em regiões interiores.
- Evidência atual: 1.6: ExtGState ca/CA independentes, unitário e parser strict; composição RGB de azul50% sobre vermelho/branco idêntica em6 amostras ao Vello. target/pdf-qa-report.json.
- **Falta / diferença:** Somente alpha RGBA/Normal por folha. Sem modos de mistura, máscaras, isolamento por grupo ou gestão de cor; amostras interiores não certificam todas as bordas ou blend spaces.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-005 — PDF/X e PDF/A

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: PDF/X e PDF/A. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-006 — Impressão Windows

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Impressão Windows. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-007 — Preview/imposição

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Preview/imposição. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-008 — Separações e retículas

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Separações e retículas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-009 — Marcas e sangria na saída

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Marcas e sangria na saída. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-010 — Preflight de produção

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Preflight de produção. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-011 — Gestão de fontes incorporadas

- Estado: **pendente**; marco: M7.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Gestão de fontes incorporadas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-012 — PDF da seleção

- Estado: **pendente**; marco: M7.
- Aceite: Exportar apenas unidades selecionadas sem mutar documento, com flags e recursos necessários.
- Evidência atual: Capacidade independente mapeada na ajuda oficial2026 Windows em17/10/2026; sem implementação.
- **Falta / diferença:** A versão1.6 só exporta documento inteiro/página ativa; seleção não possui escopo PDF.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-013 — PDF por intervalo de páginas

- Estado: **pendente**; marco: M7.
- Aceite: Validar lista/intervalos de páginas e preservar ordem/dimensões ao exportar, com mensagens claras de erro.
- Evidência atual: Capacidade independente mapeada na ajuda oficial2026 Windows em17/10/2026; sem implementação.
- **Falta / diferença:** Não há entrada de intervalos ou lista de páginas para PDF.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-014 — PDF com limites definidos pelos objetos

- Estado: **pendente**; marco: M7.
- Aceite: Oferecer tamanho da página original ou limites dos objetos escolhidos, considerando stroke/transformações e offset.
- Evidência atual: Capacidade independente mapeada na ajuda oficial2026 Windows em17/10/2026; sem implementação.
- **Falta / diferença:** A versão1.6 recorta nos limites da página original; não oferece crop pela seleção.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-015 — Vários documentos em um PDF

- Estado: **pendente**; marco: M7.
- Aceite: Escolher documentos e produzir arquivo único sem alterar originais, preservando ordem e dimensões.
- Evidência atual: Capacidade independente mapeada na ajuda oficial2026 Windows em17/10/2026; sem implementação.
- **Falta / diferença:** Não há seleção de múltiplos documentos nem concatenação PDF.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html

### PRODUCAO-016 — PDF com senha de abertura

- Estado: **pendente**; marco: M7.
- Aceite: Configurar abertura protegida, compatibilidade e cancelamento sem expor senha em logs/arquivos temporários.
- Evidência atual: Capacidade independente mapeada na ajuda oficial2026 Windows em17/10/2026; sem implementação.
- **Falta / diferença:** Não há criptografia PDF nem senha de abertura; formato atual é PDF1.4 sem proteção.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PDF-security-options.html

### PRODUCAO-017 — PDF com permissões de impressão/edição/cópia

- Estado: **pendente**; marco: M7.
- Aceite: Configurar permissões e senha de proprietário compatíveis, sem prometer proteção fora do comportamento de leitores.
- Evidência atual: Capacidade independente mapeada na ajuda oficial2026 Windows em17/10/2026; sem implementação.
- **Falta / diferença:** Não há permissões PDF; configurações incompatíveis com perfis profissionais ainda precisam especificação.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PDF-security-options.html

## RASTER — 14 itens

### RASTER-001 — Importar imagens

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Importar imagens. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-002 — Imagens vinculadas e incorporadas

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Imagens vinculadas e incorporadas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-003 — Reamostragem por DPI

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Reamostragem por DPI. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-004 — Recorte e transformação

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Recorte e transformação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-005 — Correção de cor/tom

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Correção de cor/tom. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-006 — Remoção de fundo local

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Remoção de fundo local. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-007 — Conversão vetor para bitmap

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Conversão vetor para bitmap. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-008 — Filtros não destrutivos

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Filtros não destrutivos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-009 — Gerenciar recursos de imagem

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Gerenciar recursos de imagem. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-010 — Vetorização monocromática

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Vetorização monocromática. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-011 — Vetorização multicolor

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Vetorização multicolor. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-012 — Comparação antes/depois

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Comparação antes/depois. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-013 — Controle de nós e cores

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Controle de nós e cores. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

### RASTER-014 — Repetir vetorização

- Estado: **pendente**; marco: M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Repetir vetorização. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html

## SELECAO — 22 itens

### SELECAO-001 — Clique individual

- Estado: **baseline**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-002 — Shift para adicionar/remover

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Shift adiciona e remove unidades sem perder outras; grupo é indivisível.
- Evidência atual: UI: Shift+clique; editing::expand_groups
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-003 — Marquee por contenção/cruzamento

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Contenção exige caixa integral da unidade; cruzamento aceita interseção inclusiva.
- Evidência atual: UI: contenção; testes marquee_contains_whole_group/crossing_selects_zero_height_line
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-004 — Seleção por laço

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Seleção por laço. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-005 — Selecionar tudo editável

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Ctrl+A seleciona objetos editáveis respeitando bloqueio/visibilidade e grupos.
- Evidência atual: UI: Ctrl+A; testes de bloqueio/visibilidade
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-006 — Selecionar através de sobreposição

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Selecionar através de sobreposição. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-007 — Editar filho de grupo

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Ctrl+clique e clique na árvore selecionam somente uma folha do grupo; mover, estilo, texto, nós e exclusão preservam irmãos e ancestrais; undo/reabrir exatos.
- Evidência atual: GPU PASS: Ctrl+arrasto coalescido, fill, copiar avulso, excluir, F10/nó0+10 e undo com irmãos idênticos. Desktop: folha pelo docker, arrasto sem Ctrl e paleta, save/reabertura.
- **Falta / diferença:** Isola diretamente uma folha, sem percorrer subgrupos intermediários; Focus mode e ordenação interna pendentes. Texto interno implementado mas sem fixture dedicada; sem equivalência comparativa.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-008 — Modo de foco

- Estado: **pendente**; marco: M1-M3.
- Aceite: Focar objeto/grupo temporariamente à frente, facilitar edição e sair restaurando ordem original sem alterar arquivo.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Modo de foco não implementado; distinção entre ordem temporária de exibição e ordem documental ainda sem modelo.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-009 — Mover conjunto

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Arrasto desloca membros pelo mesmo vetor e desfaz em um gesto.
- Evidência atual: UI: mover grupo; undo/redo
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-010 — Escalar conjunto

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Aplicar uma escala comum, mantendo posições relativas e objetos não selecionados.
- Evidência atual: Teste collective_scale_uses_common_anchor; UI valida escala individual
- **Falta / diferença:** Há escala coletiva por cantos; faltam pivô/handles/modos completos e roteiro externo coletivo.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-011 — Pivô interativo

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Pivô interativo. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-012 — Giro numérico relativo

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Giro relativo ao centro da caixa coletiva; 15 graus verificados.
- Evidência atual: UI: giro numérico 15°
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-013 — Giro por alça

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Giro por alça. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-014 — Espelho horizontal/vertical

- Estado: **implementado_basico**; marco: M1-M3.
- Aceite: Espelhos pelo centro da seleção; duas reflexões restauram geometria.
- Evidência atual: UI: horizontal; teste mirror_twice_restores_geometry; vertical ainda sem gesto dedicado
- **Falta / diferença:** Vertical carece de roteiro dedicado; faltam todos os pivôs/fluxos e comparação da referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-015 — Inclinação numérica horizontal

- Estado: **implementado_a_validar**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: src/; validação do incremento pendente
- **Falta / diferença:** Código existe, mas falta roteiro específico suficiente; validar gesto, cancelamento, histórico, persistência e comparação com a referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-016 — Transformação por referência/âncora

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Transformação por referência/âncora. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-017 — Repetir transformação

- Estado: **pendente**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Repetir transformação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-018 — Tolerância/hit-test zoom

- Estado: **baseline**; marco: M1-M3.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html

### SELECAO-019 — Seleção progressiva em grupos aninhados

- Estado: **pendente**; marco: M2-M8.
- Aceite: Ctrl+cliques sucessivos percorrem níveis até o objeto desejado, com caixa/descrição do alvo e sem mudar conteúdo.
- Evidência atual: Identificado na ajuda oficial2026 Windows, Select objects; sem implementação/homologação.
- **Falta / diferença:** ArtDraw1.4 isola a folha diretamente; não percorre grupos intermediários.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Select-objects.html

### SELECAO-020 — Percorrer objetos com Tab e Shift+Tab

- Estado: **pendente**; marco: M2-M8.
- Aceite: Percorrer ordem de criação nos dois sentidos, ignorar alvos não editáveis e mostrar seleção sem roubar foco de campos.
- Evidência atual: Identificado na ajuda oficial2026 Windows, Select objects; sem implementação/homologação.
- **Falta / diferença:** Não há ciclo de seleção por criação; Tab segue foco de controles egui.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Select-objects.html

### SELECAO-021 — Grupos de seleção numerados0 a9

- Estado: **pendente**; marco: M2-M8.
- Aceite: Ctrl+número associa seleção; número recupera; duplo número seleciona/ajusta zoom; Alt+número adiciona membro; persistir IDs no documento.
- Evidência atual: Identificado na ajuda oficial2026 Windows, Select objects; sem implementação/homologação.
- **Falta / diferença:** Não implementado. Grupos de seleção independem de agrupamento geométrico e precisam sobreviver a reordenação/exclusão/duplicação.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Select-objects.html

### SELECAO-022 — Tratar objetos sem fill como preenchidos na seleção

- Estado: **pendente**; marco: M2-M8.
- Aceite: Preferência alterna hit interior versus apenas contorno, com comando contextual sem seleção e comportamento consistente em grupos.
- Evidência atual: Identificado na ajuda oficial2026 Windows, Select objects; sem implementação/homologação.
- **Falta / diferença:** ArtDraw usa geometria de fill/contorno real; falta preferência e padrão documentado pelo CorelDRAW.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Select-objects.html

## SUITE_ONLINE — 10 itens

### SUITE_ONLINE-001 — PHOTO-PAINT equivalente separado

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: PHOTO-PAINT equivalente separado. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-002 — Gerenciador de fontes separado

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Gerenciador de fontes separado. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-003 — CAPTURE equivalente separado

- Estado: **referencia_a_validar**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Confirmar disponibilidade na edição/build/plataforma Windows de referência; não tratar a pesquisa atual como auditoria completa.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-004 — Biblioteca de ativos próprios

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Biblioteca de ativos próprios. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-005 — Comentário/revisão/colaboração

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Comentário/revisão/colaboração. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-006 — Armazenamento Cloud

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Armazenamento Cloud. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-007 — Geração local texto-para-imagem

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Geração local texto-para-imagem. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-008 — Geração local texto-para-vetor

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Geração local texto-para-vetor. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-009 — Edição de imagem por prompt

- Estado: **pendente**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Edição de imagem por prompt. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

### SUITE_ONLINE-010 — SSO/conta da suíte

- Estado: **fora_do_alvo_offline**; marco: M10.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Serviço externo permanece fora da entrega offline; registrar a diferença e investigar alternativa local. A lacuna não foi resolvida.
- Referência: https://www.coreldraw.com/en/product/coreldraw/

## TEXTO — 21 itens

### TEXTO-001 — Texto artístico básico Unicode

- Estado: **baseline**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Unicode básico com uma fonte incorporada; falta shaping/kerning/bidi/fallback/estilos e edição direta.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-002 — Editor direto no canvas

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Editor direto no canvas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-003 — Fontes do sistema e fallback

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Fontes do sistema e fallback. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-004 — Escolha de família/peso/estilo

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Escolha de família/peso/estilo. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-005 — Shaping e kerning

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Shaping e kerning. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-006 — Ligaduras e alternates OpenType

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Ligaduras e alternates OpenType. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-007 — Scripts complexos e bidi

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Scripts complexos e bidi. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-008 — Seleção/caret e atalhos de texto

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Seleção/caret e atalhos de texto. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-009 — Estilos por trecho

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Estilos por trecho. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-010 — Texto de parágrafo

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Texto de parágrafo. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-011 — Colunas/tabs/indentação

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Colunas/tabs/indentação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-012 — Bullets e listas

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Bullets e listas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-013 — Hifenização e justificação

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Hifenização e justificação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-014 — Fluxo entre caixas

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Fluxo entre caixas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-015 — Texto dentro de forma

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Texto dentro de forma. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-016 — Texto em caminho

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Texto em caminho. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-017 — Texto para curvas

- Estado: **parcial**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Exporta contornos e metadados ArtDraw; falta comando completo de conversão em objetos editáveis e comparação.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-018 — Tabelas e células

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Tabelas e células. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-019 — Substituição/PANOSE

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Substituição/PANOSE. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-020 — Fontes variáveis

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Fontes variáveis. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

### TEXTO-021 — Ortografia e ferramentas linguísticas

- Estado: **pendente**; marco: M5.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Ortografia e ferramentas linguísticas. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html

## TOOLBOX — 62 itens

### TOOLBOX-001 — Pick

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Completar seleção por tipos/profundidade, giro/inclinação por handles, pivô e cursores/feedback equivalentes; grupos ainda simples.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-002 — Freehand pick

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Freehand pick. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-003 — Free transform

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Há transformações numéricas/escala básica; faltam modos interativos completos, pivô e equivalência de gestos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-004 — Shape

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** F10 edita paths simples; faltam tipos de nós persistentes, múltipla seleção de nós e operações completas; texto/paths compostos não têm edição individual.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-005 — Smooth

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Smooth. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-006 — Smear

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Smear. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-007 — Twirl

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Twirl. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-008 — Attract and Repel

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Attract and Repel. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-009 — Smudge

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Smudge. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-010 — Roughen

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Roughen. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-011 — Crop

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Crop. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-012 — Knife

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Knife. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-013 — Virtual segment delete

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Virtual segment delete. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-014 — Eraser

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Eraser. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-015 — Zoom

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Há Ctrl+roda e ajuste de página; faltam todos os modos, seleção de área, atalhos/contextos e comparação de navegação.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-016 — Pan

- Estado: **baseline**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-017 — Freehand

- Estado: **implementado_basico**; marco: M1-M6.
- Aceite: Desenhar polilinha por arrasto; não exige suavização.
- Evidência atual: UI: desenho livre; src/app/validation.rs
- **Falta / diferença:** Traço é polilinha por amostragem; faltam estabilização/suavização e parâmetros equivalentes.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-018 — 2-point line

- Estado: **implementado_basico**; marco: M1-M6.
- Aceite: Arrastar linha inclusive horizontal/vertical e reabrir SVG.
- Evidência atual: UI: linha horizontal; teste crossing_selects_zero_height_line
- **Falta / diferença:** Existe somente o subconjunto indicado na evidência. Faltam auditoria de variantes/limites, roteiro completo e comparação visual/semântica/UI/UX com a build de referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-019 — Bézier

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Há cúbicas e alças por arrasto; faltam variantes de fechamento, tipos de nó e equivalência de estados/gestos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-020 — Pen

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Há pen de clique/arrasto; faltam preview/configuração/gestos completos e comparação com Pen original.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-021 — B-spline

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: B-spline. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-022 — Polyline

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Polyline. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-023 — 3-point curve

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: 3-point curve. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-024 — Painterly Brush

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Painterly Brush. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-025 — Artistic media

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Artistic media. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-026 — Smart drawing

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Smart drawing. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-027 — LiveSketch

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: LiveSketch. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-028 — Rectangle

- Estado: **baseline**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Retângulo básico não oferece todos os cantos paramétricos/chanfros e modos da referência.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-029 — 3-point rectangle

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: 3-point rectangle. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-030 — Ellipse

- Estado: **baseline**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Elipse básica não oferece edição paramétrica completa de arcos/setores e seus handles.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-031 — 3-point ellipse

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: 3-point ellipse. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-032 — Polygon

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Polígono vira caminho; não mantém entidade paramétrica reeditável com todos os handles.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-033 — Star

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Estrela vira caminho; faltam parâmetros e edição interativa equivalentes.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-034 — Spiral

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Espiral é aproximação segmentada; faltam modo logarítmico e parametrização persistente.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-035 — Common shapes

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Common shapes. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-036 — Impact

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Impact. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-037 — Graph paper

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Graph paper. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-038 — Text

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Somente editor modal com Ubuntu Light; faltam fontes, edição direta, shaping e layout profissional.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-039 — Table

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Table. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-040 — Parallel dimension

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Parallel dimension. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-041 — Horizontal or vertical dimension

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Horizontal or vertical dimension. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-042 — Angular dimension

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Angular dimension. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-043 — Segment dimension

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Segment dimension. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-044 — 2-leg callout

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: 2-leg callout. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-045 — Connector

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Connector. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-046 — Anchor editing

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Anchor editing. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-047 — Drop shadow

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Drop shadow. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-048 — Inner shadow

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Inner shadow. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-049 — Contour

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Contour. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-050 — Blend

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Blend. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-051 — Distort

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Distort. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-052 — Envelope

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Envelope. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-053 — Extrude

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Extrude. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-054 — Block shadow

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Block shadow. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-055 — Transparency

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Só alpha uniforme; faltam tipos interativos, modos de mesclagem e controles completos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-056 — Color eyedropper

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Color eyedropper. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-057 — Attributes eyedropper

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Attributes eyedropper. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-058 — Interactive fill

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Interactive fill. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-059 — Smart fill

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Smart fill. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-060 — Mesh fill

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Mesh fill. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-061 — Variable Outline

- Estado: **pendente**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Variable Outline. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

### TOOLBOX-062 — Outline

- Estado: **parcial**; marco: M1-M6.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Só cor/largura básicas; faltam junções/caps/traces/setas/escala e estilos completos.
- Referência: https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html

## WORKSPACE — 14 itens

### WORKSPACE-001 — Tema escuro

- Estado: **baseline**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: VALIDATION.md 1.0
- **Falta / diferença:** Tema escuro próprio; cores, medidas, fontes, flyouts, diálogos e estados não foram comparados ao tema original.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-002 — Quatro presets ArtDraw

- Estado: **implementado_basico**; marco: M1-M8.
- Aceite: Presets próprios claro/cinza/escuro/preto alteram painéis/canvas.
- Evidência atual: UI valida claro/escuro; cinza/preto implementados sem captura dedicada
- **Falta / diferença:** Quatro presets próprios; claro/escuro testados, cinza/preto sem roteiro dedicado. Não são temas idênticos ao CorelDRAW.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-003 — Preferência de tema persistida

- Estado: **implementado_a_validar**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: src/; validação do incremento pendente
- **Falta / diferença:** Código existe, mas falta roteiro específico suficiente; validar gesto, cancelamento, histórico, persistência e comparação com a referência.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-004 — Painéis redimensionáveis

- Estado: **parcial**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Painéis limitados do ArtDraw; faltam docking, flutuação, organização/persistência e layouts equivalentes.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-005 — Flyouts de famílias

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Flyouts de famílias. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-006 — Atalhos configuráveis

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Atalhos configuráveis. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-007 — Escala/tamanho de ícones

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Escala/tamanho de ícones. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-008 — Layouts de workspace persistidos

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Layouts de workspace persistidos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-009 — Busca de comandos

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Busca de comandos. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-010 — Personalização de menus

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Personalização de menus. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-011 — Feedback de nós configurável

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Feedback de nós configurável. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-012 — Acessibilidade e leitor de tela

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Acessibilidade e leitor de tela. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-013 — Tablet/pressão/inclinação

- Estado: **pendente**; marco: M1-M8.
- Aceite: Especificar cenários e validar saída, histórico e persistência.
- Evidência atual: Sem implementação/evidência completa
- **Falta / diferença:** Não implementado: Tablet/pressão/inclinação. Especificar opções/exceções da referência, modelo, gestos/atalhos, persistência e corpus antes de concluir.
- Referência: https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/

### WORKSPACE-014 — Abertura com a logomarca fornecida

- Estado: **implementado_basico**; marco: M2.
- Aceite: Logo antes do GPU; encerrar ao mostrar área de trabalho; falha fecha splash.
- Evidência atual: Splash do release inspecionado no desktop: logomarca, texto e barra; manifesto PerMonitorV2 antes das janelas; app normal abre maximizado
- **Falta / diferença:** Requisito próprio da marca ArtDraw; não conta como equivalência visual Corel. Transições de DPI entre monitores não testadas.
- Referência: ArtDraw; requisito próprio de produto; não é evidência de internals Corel

Matriz de origem SHA256: 787a9b5cb0e69a2a1d53cce791070c474a12262e706300879d57c487dfd2dbad
