# ArtDraw 1.9.0 —18/10/2026

- Comandos À frente/Atrás de um alvo por clique, no menu Objeto/Ordem e painel. Grupos inteiros, seleção em bloco ordenado e transferência para a camada do alvo.
- Estado transitório por IDs estáveis; Esc/direito cancela preservando seleção/ferramenta. Mudanças de página, ferramenta, documento ou seleção encerram a escolha. Clique de alvo não inicia arrasto nem edição por duplo clique.
- 93 unitários,139 verificações GPU/138 frames,3 recuperação e desktop de painel/menu/cancelamento/undo/redo/SaveAs/XML integral/reabrir/fechar PASS. Sete testes novos de kernel. Payload TESTMODE19-relative-payload20 hashes/self-test/registros intactos PASS. Sem alteração de schema/dependências; distribuição completa continua parcial por assinatura/App Control.
- Legados intercalados/multicamadas, folha interna, mestras e equivalência comparativa continuam pendentes. CDR ainda ausente.

# ArtDraw 1.8.0 —18/10/2026

- Transferências entre camadas projetam ordem sobre slots associados às camadas finais, preservando camadas não envolvidas e atributos/grupos.
- Disponibilidade dos sete comandos calculada nas mesmas unidades do planner; menus/botões respeitam limites e seleção suportada. Sem clonar Document por botão.
- Quatro regressões de transferência reproduzidas antes da correção; novo teste de1120 combinações de disponibilidade e nove checks GPU acrescentados. 86 unitários,116 checks GPU/130 frames,3 recuperação e desktop de transferência/menu/SaveAs/XML/reabrir PASS; payload TESTMODE18-transfers-payload20 hashes/self-test PASS. Ciclo completo de instalação/desinstalação segue parcial.
- CDR, mestras, seleção interna e equivalência comparativa continuam pendentes.

# ArtDraw 1.7.1 —17/10/2026

- Corrigir normalização indevida dos slots de camadas alheias em deslocamentos sem transferência.
- Dois testes de regressão para seis comandos, incluindo grupos aninhados/atributos e retorno ao documento/serialização originais; ambos falhavam antes da correção.
- Seis cenários de UI GPU acrescentados para estado salvo, histórico, atributos e persistência. 81 unitários,107 checks GPU/126 frames,3 recuperação e desktop de atalhos/SaveAs/XML/reabrir PASS. Payload TESTMODE171-slots-payload20 hashes/self-test PASS; ciclo completo de distribuição continua parcial.
- Transferências entre camadas e demais lacunas continuam abertas; não há nova equivalência ou CDR.

# ArtDraw 1.7.0 —17/10/2026

- Inverter ordem no menu Objeto → Ordem e painel Objetos, por slots de unidades selecionadas da mesma camada. Conjuntos preservam a estrutura interna e atributos; unidades não selecionadas não são permutadas.
- Inversão preserva também slots globais entre camadas intercaladas, permitindo retorno exato do documento após duas inversões.
- Estado desabilitado quando há menos de duas unidades ou várias camadas. Kernel recusa operação entre camadas antes da mutação. Não cria alteração para seleção neutra.
- 79 unitários,101 verificações de edição/arquivo GPU e3 de recuperação PASS. Menu/botão, involução, undo/redo, salvar/reabrir e integridade por ID conferidos no desktop.
- Inversão entre camadas, ordenação interna, posição por alvo e equivalência com CorelDRAW continuam pendentes. CDR não implementado. Distribuição segue parcial: bloqueio do desinstalador1.6 sem assinatura ainda aberto.

# ArtDraw 1.6.0 —17/10/2026

- PDF1.4 vetorial multipágina/página ativa, RGB com texto em curvas, transformações, alpha independente fill/stroke, evenodd e recorte por página.
- Escritor clean-room com xref correto, números finitos, limites de saída e gravação atômica/backup. Exportar/cancelar conserva nativo e histórico.
- Corpus próprio de3 páginas renderizado por Poppler e parser strict; seis amostras coincidiram com Vello. Exportação e cancelamento pela interface Windows também testados.
- 74 unitários,95 verificações de edição/GPU e3 de recuperação aprovados. CDR e saída profissional continuam ausentes; docs/PDF_EXPORT.md e VALIDATION.md detalham critérios.
- 363 itens conhecidos após seis lacunas PDF independentes; nenhum equivalente verificado.

# ArtDraw 1.5.0 —17/10/2026

- Seis comandos de ordem por página/camada/passo com atalhos próprios, preservando grupos, IDs, atributos e ordem relativa.
- Passo cruza camada no limite, pula destinos não editáveis e não transfere unidade recém-chegada outra vez no mesmo comando.
- Camada ativa acompanha transferência; Shift+PageUp/Down não navega páginas. Limite é neutro para histórico/dirty.
- Kernel planejado antes da mutação e recusa explícita de grupos legados intercalados para evitar alteração silenciosa de outros objetos.
- Inventário357 itens, incluindo inverter ordem e posicionar em relação a alvo ainda pendentes. Resultados em VALIDATION.md. CDR permanece sem implementação.

# ArtDraw 1.4.0 —17/10/2026

- Grupos aninhados com árvore expansível; agrupar preserva filhos, separar um nível promove subgrupos e separar tudo remove os níveis selecionados.
- Ctrl+clique e folha no painel isolam edição: movimento/escala, cor, texto, F10, cópia e exclusão preservam irmãos; retorno explícito à seleção do conjunto.
- Cópia/duplicação/importação remapeiam grupos e objetos; duplicar página conserva hierarquia com objetos de identidades novas.
- Nativo schema3 persiste arestas e migra1/2. Validação de ciclos/arestas órfãs/profundidade/camadas e save/checkpoint seguros preservados. Grupos planos legados entre camadas continuam legíveis.
- SVG importa/exporta g padrão aninhado e mantém metadados ArtDraw; limitações externas descritas em docs/GROUP_HIERARCHY.md.
- Painel indexa filhos/descendentes uma vez por quadro; teste estrutural com10mil folhas. Não é benchmark de latência/GPU.
- Inventário ampliado para354 itens, incluindo reparentamento no docker e opções de seleção que continuam ausentes. CDR segue sem implementação. Resultados de homologação em VALIDATION.md.

# ArtDraw 1.3.0 —17/10/2026

- Páginas com identidades estáveis e dimensões/objetos/camadas independentes; navegação sem dirty e sem entrada de histórico.
- Adicionar, duplicar com novos IDs de objetos, renomear, ordenar, orientação e excluir com confirmação/undo/redo; última página protegida.
- Schema XML2 para o documento completo e migração do schema1. Save/checkpoint validam antes de gravar e preservam atomicidade/backup.
- SVG de página ativa explícito; salvar multipágina como SVG e importar documento multipágina na página corrente são recusados para evitar descarte silencioso.
-48 testes unitários,57 verificações de edição/arquivo e3 de recuperação na homologação; fluxo de abrir/navegar/gerenciar/salvar/reabrir exercitado no desktop.
- Indicador interno11,0%/348 itens conhecidos; não certifica equivalência. Visão simultânea, mestras, grupos aninhados e CDR continuam pendentes.

# ArtDraw 1.2.0 —17/10/2026

- Logomarca fornecida incorporada no splash nativo anterior à GPU e nos ícones de janela/executável/instalador.
- Assistente Inno Setup em português com licença, pasta, atalhos, progresso e desinstalação padrão.
- Formato nativo XML.artdraw v1, identidades persistentes, gravação atômica e .bak anterior.
- Checkpoints por sessão em worker, recuperação de sessões encerradas e preservação dos originais.
- Abrir/importar ArtDraw e SVG, exportar SVG e abrir arquivo solto na janela.
- Documento vivo de348 lacunas, regras obrigatórias em AGENTS.md, gerador e bloqueio de documentação desatualizada nos builds/pacotes; plano CDR de leitura/escrita e preservação. CDR continua não implementado.
- Protocolo e indicador conservador de aproximação. M2 foi iniciado; páginas múltiplas, grupos aninhados e migrações entre futuros schemas continuam pendentes.
- Correções verificadas no desktop para arrastos e atalhos rápidos entregues no mesmo quadro; modificadores de teclado são lidos por evento.
- 40 testes unitários aprovados; 44 verificações da janela de edição e 3 da recuperação no release. Splash, desenho, Ctrl+S, diálogo Salvar e backup foram exercitados no desktop; limites em VALIDATION.md.

# ArtDraw 1.1.0 — 17/10/2026

Esta versão amplia a edição coletiva e estabelece um inventário de aproximação do CorelDRAW 2026. O objetivo de paridade integral continua aberto.

- Seleção múltipla, grupos simples, histórico coletivo e clipboard interno.
- Camadas, visibilidade/bloqueio por camada e objeto, ordem de pintura independente dos índices.
- Giro/inclinação numéricos, espelhos, alinhamento e distribuição por centros.
- Polígonos, estrelas, espirais aproximadas, linhas e desenho livre em polilinha.
- Quatro temas próprios, preferência local e snapping à grade.
- SVG com metadados para camadas/grupos/estados e coordenadas exatas.
- Inventário de 344 itens em22 famílias, fontes oficiais e roteiro de dez marcos; documentação incluída no instalador e ZIP.

Próxima dependência estrutural: IDs estáveis, grupos aninhados, documento nativo versionado, recuperação e múltiplas páginas (M2). Booleanos, degradês, tipografia profissional, raster/efeitos, ICC/CMYK/PDF e CDR continuam pendentes. Recursos da suíte e serviços online têm escopo separado.

Veja VALIDATION.md para os resultados observados e suas limitações; cada linha da matriz detalha o escopo verificado. A contagem de itens não é um percentual de paridade.
