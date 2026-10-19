# Roteiro de aproximação profissional

O objetivo ampliado continua aberto. Não há promessa de paridade100% nem estimativa artificial de prazo. O inventário é atualizado a cada incremento com evidência; detalhes não auditados permanecem por verificar.

| Marco | Entrega concreta | Dependência / aceite |
| --- | --- | --- |
| M1 — edição coletiva | seleção múltipla/marquee, grupos simples, clipboard interno, ordem, alinhamento, distribuição, giro/espelho/inclinação, camadas simples, primitivas extras, temas | Matemática, SVG com metadados, undo/redo e gestos no release |
| M2 — documento durável | IDs estáveis, grupos aninhados, formato nativo versionado, recuperação, páginas independentes e mestras | Migrações e corpus de arquivos; salvar/reabrir sem perdas |
| M3 — geometria de produção | nós tipados, juntar/quebrar curvas, booleanos, offsets, recorte editável, arredondamentos e snapping completo | Testes de topologia/degeneração e comparação visual |
| M4 — aparência | degradês, stops, padrões, texturas, transparência, contornos completos, estilos e conta-gotas | Render/export/import coerentes e gestão de recursos |
| M5 — tipografia | fontes do sistema, shaping, bidi, edição direta, parágrafos, fluxo, texto em caminho e tabelas | Corpus multilingue, fontes livres e testes de quebra/seleção |
| M6 — imagem e efeitos | importação raster, filtros, shadows, blend, envelope, perspectiva, simetria e vetorizar | Grafo de efeitos, invalidação e tolerância de qualidade |
| M7 — saída profissional | PNG/JPEG/TIFF/PDF, impressão, ICC/CMYK/spot, separações, preflight e lote | Renderização de referência, perfis e validação por consumidor externo |
| M8 — produtividade | símbolos, localizar/substituir, estilos globais, templates próprios, atalhos, automação, plugins e acessibilidade | API estável de comandos/modelo e cenários reais |
| M9 — compatibilidade avançada | AI/EPS/DXF/DWG/CDR e outros, por direção/versão | Corpus autorizado e relatório de perdas; sem selo genérico de compatibilidade |
| M10 — recursos da suíte/online | editor raster dedicado, gerenciador de fontes, colaboração e IA local | Produtos separados e modelos/infraestrutura compatíveis com offline |

Cadência: especificar → implementar verticalmente → testar → atualizar matriz/memória → compilar/empacotar. Evitar criar uma barra cheia de ferramentas inativas. Priorizar casos de trabalho completos. O baseline1.0 está preservado como histórico; os marcos seguintes não estão concluídos apenas por aparecerem neste documento.

## Posição após1.2

M1 tem implementação básica e cobertura parcial por opção. M2 agora possui IDs persistentes, schema XML nativo1, gravação atômica/backup e recuperação por sessão. Seleção/histórico ainda usam índices planos; páginas independentes/mestras, grupos aninhados e migrações entre schemas futuros seguem pendentes. Próximo incremento: múltiplas páginas com persistência/histórico e organização por árvore; depois kernel booleano.

## Trilha obrigatória CDR e fidelidade

CDR não fica adiado integralmente ao M9: pesquisa/corpus e arquitetura começam junto do M2; leitura/escrita evoluem com representação de páginas, hierarquia, texto, estilos, recursos e cor. O M9 consolida homologação ampla por versão. Critérios em CDR_INTEROPERABILITY.md; diferenças por recurso em CORELDRAW_GAP_LEDGER.md. Não existe suporte CDR no release1.2.

A regra extraordinária em ../AGENTS.md exige atualizar lacunas/aceite/evidência a cada alteração e executar scripts/update-parity.ps1 -Check antes da entrega. Fidelidade de UI/UX/temas é uma trilha de comparação transversal, ainda sem build executável de referência auditada.

## Posição após1.3

M2 agora inclui páginas independentes com camadas/objetos isolados, navegação/ordem/orientação, operações de página com histórico, schema2 completo e migração1. Não inclui visão simultânea, mestras nem grupos aninhados. Próxima dependência: hierarquia de grupos e recursos sem perder a compatibilidade de documentos existentes. O estudo CDR prossegue por direção/versão; nenhuma versão CDR é suportada.

## Posição após implementação1.4

M2 inclui árvore de grupos até64 níveis, seleção externa e edição de folha, cópia/importação com remapeamento, separação por nível/toda árvore, schema3 e migração1/2. A homologação da entrega está em VALIDATION.md. Permanecem mestras/visão simultânea, reparentamento no docker, Focus mode, seleção progressiva de subgrupos e recursos compartilhados; M2 não está completo. Próximas dependências: organização interna/recursos e kernel geométrico de produção. CDR permanece requisito obrigatório sem implementação.

## Posição após implementação1.5

Ordenação distingue página/camada/passo e transfere conjuntos entre camadas editáveis, com plano/undo e persistência. Kernel67 unitários, roteiro93 GPU e desktop de sobreposição aprovados; detalhes em VALIDATION.md e STACKING_ORDER.md. Ainda faltam inverter ordem, posição por alvo, ordenação interna de folhas e equivalência de estados UI no limite, além das pendências do M2 e marcos seguintes.

## Posição após implementação1.6

M7 iniciou PDF vetorial RGB com texto em curvas, documento multipágina/página ativa e verificação externa. Não conclui produção: ICC/CMYK/spot, fontes, PDF/X/A, presets, seleção/intervalos/lote, segurança, raster e impressão permanecem ausentes. M2 e CDR conservam suas pendências. Matriz363 itens; docs/PDF_EXPORT.md descreve matemática/limites.

## Posição após implementação1.7

Organização agora inclui inverter ordem de unidades selecionadas da mesma camada com disponibilidade coerente, histórico/persistência e prova de atributos por ID. Unitários79/GPU101/desktop canônico e intercalado passaram; não conclui ordenação completa. Entre camadas, posição por alvo, ordenação interna, mestras e reparentamento continuam abertos. M7 mantém PDF básico1.6 e CDR permanece sem implementação.

## Revisão1.7.1

Deslocamentos sem transferência de camada preservam slots globais, atributos e estado salvo.81 unitários,107 GPU e desktop de passo/extrema de camada PASS. Transferências entre camadas, estados desabilitados completos e equivalência original continuam abertos. Esse ajuste de regressão não aumenta o indicador interno.

## Posição após1.8

Transferências preservam slots das camadas não envolvidas; disponibilidade dos comandos cobre limites e seleção suportada.86 unitários/116 GPU/desktop PASS. Permanecem semântica de dirty após transferências opostas, mestras, alvo por clique, folha interna, comparação original e benchmarks. Não há aumento do indicador por esse refinamento; próximas capacidades seguem matriz e trilha CDR independente.

## Posição após1.9

Ordem relativa por clique implementada para unidades completas: frente/atrás de alvo, bloco ordenado, transferência e cancelamento.93 unitários/139 GPU/3 recuperação e desktop com menu/painel/undo/redo/SaveAs/XML/reabrir/fechar PASS. Não conclui M2: mestras, folhas internas, reparentamento/seleção progressiva, legados intercalados/multicamadas, comparação UI/UX original e benchmarks permanecem. Próxima investigação: regras de seleção hierárquica e ordenação interna; trilha CDR continua obrigatória e sem leitor/gravador.
