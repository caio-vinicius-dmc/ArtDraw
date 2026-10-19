# Indicador de aproximação — ArtDraw

**Indicador interno conservador: 11,6%**. Base: 363 itens conhecidos/22 famílias, 362 no alvo offline. Isso não certifica esse percentual de equivalência ao CorelDRAW. Ainda estamos longe de100%; não houve comparação interativa completa com a referência.

| Estado | Itens | Peso |
| --- | ---: | ---: |
| baseline | 25 | 0,5 |
| implementado_basico | 46 | 0,5 |
| parcial | 26 | 0,25 |
| implementado_a_validar | 3 | 0 |
| pendente | 259 | 0 |
| referencia_a_validar | 3 | 0 |
| equivalente_verificado | 0 | 1 |
| fora_do_alvo_offline | 1 | excluído |

Cálculo: 42,00 créditos / 362 itens ×100 = 11,6%. Granularidade/esforço variam e capacidades podem se sobrepor. Pesos fixos no [protocolo](VALIDATION_PROTOCOL.md). A versão1.1 tinha9,3% pelas mesmas regras.

A versão1.9 acrescenta comandos à frente/atrás de alvo por clique com bloco de unidades, transferência de camada e cancelamento seguro.93 unitários/139 checks GPU e desktop próprio passaram; não há comparação CorelDRAW. Folha interna, mestras, legados intercalados/multicamadas, inversão entre camadas e semântica de dirty após transferências opostas continuam pendentes. Ver STACKING_ORDER.md e VALIDATION.md.

A versão1.6 exporta documento/página ativa em PDF1.4 vetorial RGB com texto em curvas. Parser strict, raster Poppler, amostras GPU e desktop passaram no corpus próprio; fontes editáveis, ICC/CMYK, PDF/X/A, presets, seleção/intervalos/lote e segurança permanecem pendentes. Ver PDF_EXPORT.md.

A versão1.5 distingue frente/trás de página/camada e passo com transferência entre camadas editáveis, preservando grupos e histórico. Inversão entre camadas e ordenar folhas internas ainda faltam; alvo por clique recebeu subconjunto1.9 e estados no limite implementação1.8, sem equivalência comparativa. Detalhes em STACKING_ORDER.md.

A versão1.4 acrescenta grupos aninhados, edição de folha, separação por nível/tudo, cópia/importação de árvore e schema3 com leitura1/2, sobre as páginas independentes da1.3. Continuam pendentes reparentamento/foco/seleção progressiva, visão simultânea/mestras, booleanos/offsets, fills avançados, tipografia, raster/efeitos, ICC/CMYK/PDF profissional e CDR bidirecional. Testes do subconjunto atual não eliminam essas lacunas.

Próximo marco: organização interna de grupos/recursos, completar páginas, kernel geométrico e ampliar pesquisa/corpus CDR. Leitor/gravador CDR e qualquer alegação sem perdas exigem o [protocolo CDR](CDR_INTEROPERABILITY.md). Lista detalhada em [documento vivo](CORELDRAW_GAP_LEDGER.md).

Gerado por scripts/update-parity.ps1. Matriz SHA256: 787a9b5cb0e69a2a1d53cce791070c474a12262e706300879d57c487dfd2dbad
