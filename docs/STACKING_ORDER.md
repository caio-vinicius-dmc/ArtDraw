# Ordenação de objetos — ArtDraw1.9

Referência: [CorelDRAW2026 Windows, Change the order of objects](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Change-the-order-of-objects.html). Kernel próprio em src/ordering.rs.

| Comando | Atalho | Comportamento implementado |
| --- | --- | --- |
| Frente da página | Ctrl+Home | Reúne seleção na frente da camada editável mais alta |
| Trás da página | Ctrl+End | Reúne seleção atrás da camada editável mais baixa |
| Frente da camada | Shift+PageUp | Move unidades ao final da ordem da própria camada |
| Trás da camada | Shift+PageDown | Move unidades ao início da própria camada |
| Avançar um nível | Ctrl+PageUp | Troca uma posição com unidade não selecionada; no limite entra atrás na camada editável acima |
| Recuar um nível | Ctrl+PageDown | Troca uma posição com unidade não selecionada; no limite entra na frente da camada editável abaixo |
| À frente de um alvo… | Menu Objeto → Ordem ou painel, depois clique | Insere seleção em bloco imediatamente após a unidade do alvo na ordem de pintura |
| Atrás de um alvo… | Menu Objeto → Ordem ou painel, depois clique | Insere seleção em bloco imediatamente antes da unidade do alvo na ordem de pintura |
| Inverter ordem | Menu Objeto → Ordem ou botão | Inverte unidades selecionadas nos seus slots, dentro da mesma camada |

Unidade é o grupo raiz ou um objeto avulso. Grupos aninhados movem todas as folhas juntas, sem alterar arestas, IDs, geometria ou Affine. Comandos de deslocamento preservam a ordem relativa da multisseleção; Inverter ordem troca intencionalmente a ordem das unidades selecionadas. Camadas bloqueadas/ocultas não são destinos; o ArtDraw usa visibilidade e bloqueio como editabilidade. A seleção continua nos mesmos objetos e a camada ativa acompanha o objeto primário após transferência.

O plano é calculado sobre índices antes de alterar objetos. Blocos já no limite são capturados antes das trocas locais; unidades recebidas não avançam outra camada no mesmo comando. Falha de validação restaura objetos anteriores. Noop não altera documento nem histórico. Ao cruzar para uma camada vazia, aparência pode permanecer igual enquanto a associação de camada muda.

## Inverter ordem1.7

A seleção deve conter ao menos duas unidades da mesma camada; um grupo inteiro conta como uma unidade. O menu e botão ficam desabilitados fora desse caso e explicam o requisito por tooltip. A sequência das unidades selecionadas ocupa os mesmos slots em ordem reversa; unidades não selecionadas não são permutadas e não são trazidas junto à seleção. Um grupo troca de slot inteiro e conserva a ordem interna das folhas, todos os atributos e as arestas.

Uma segunda inversão com a mesma seleção restaura a ordem original. Os slots globais de cada camada também são preservados quando o vetor original intercala camadas, para restaurar rawDoc/serialização exatos. Uma unidade, nenhuma seleção ou apenas folhas da mesma raiz não cria alteração/histórico. Undo/redo opera a transação completa; nativo/SVG persistem a nova pintura. O formato continua schema3.

O subconjunto entre camadas ainda não é implementado; não se presume a regra exata da referência. UI desabilita; chamada ao kernel recusa múltiplas camadas antes da mutação. Também conserva a recusa de grupos legados intercalados e de folha isolada dentro de grupo. Fixture própria tests/fixtures/reverse-order.artdraw tem duas páginas, três camadas e cinco objetos com nomes distintos.

## Limitações

Não há ordenação de folha isolada dentro de um grupo; selecione o conjunto. Grupos planos legados intercalados com outros objetos são recusados com diagnóstico para evitar consolidar sua estrutura e modificar a ordem alheia silenciosamente. Desagrupar permite ordená-los como objetos.

Ainda faltam inversão entre camadas, ordenação de folhas internas, regras de mestras e comparação de flags/casos com o original. Comandos por alvo receberam subconjunto1.9, descrito abaixo. Na1.8, os seis comandos de deslocamento ficam desabilitados nos limites previstos pelo planner. A equivalência comparativa de todos os estados continua pendente. Inverter ordem já possui estado desabilitado para sua seleção suportada. O formato permanece schema3. Texto/efeitos/cor/CDR continuam limitados conforme matriz.

## Validação1.7

Testes cobrem passo, camada bloqueada/oculta, destino vazio, múltipla seleção, grupo aninhado, extremos de página versus camada, limite neutro, recusa sem mutação e transferência única. Roteiro da janela inclui duas páginas e três camadas para detectar troca indevida de página com Shift+PageDown; resultados em VALIDATION.md.

Cinco unitários de inversão acrescentados aos74 anteriores, todos79 PASS. Roteiro da janela:101 checks/121 frames Vello, incluindo botão desabilitado para uma unidade, duas unidades não adjacentes invertidas, persistência nativa/SVG e undo/redo. Desktop: menu e botão com3 retângulos, segunda inversão restaura arte/dirty, CtrlZ/Y, seleção entre camadas desabilitada, SaveAs/reabertura com pilha3,2,1,4,5. Objetos completos comparados por ID, camadas e página2 intactos. Fixture salva SHA2565f0c93e2b796b6cc8cdb32e848cb062d85fed61018f8fbb930a1ef699ad71bd0. Não houve comparação interativa no CorelDRAW.

Revisão adicional: tests/fixtures/reverse-interleaved.artdraw tem vetor global1,4,2,5,3 e pintura1,2,3,4,5. O kernel de inversão usa os slots globais originais de cada camada, sem canonizar a representação. Novo unitário confirmou retorno exato do Document/encode após duas inversões. A revisão dessa variante para os seis comandos de deslocamento ainda permanece uma lacuna separada.

Desktop adicional da fixture intercalada PASS: marquee3 objetos, botão inverteu sobreposição; segunda inversão restaurou arte e removeu dirty. Terceira inversão salva via diálogo nativo em target/desktop-reverse-interleaved17.artdraw e reaberta com sobreposição correta. XML raw3,4,2,5,1, pintura3,2,1,4,5; todos os objetos completos por ID, definições de camadas, atributos raiz e página2 idênticos à origem. SHA256872f74806a0732107c2c8d27468e44e4d898827cf37ed8d64bad284b5ed13bd0. Fechamento normal já salvo; inventário confirmou zero janelas ArtDraw QA.

## Correção1.7.1 — deslocamentos sem transferência

A projeção sobre slots globais foi estendida a todos os comandos cujo plano conserva as associações de camada de cada objeto. Dois testes novos, antes reprovados e agora aprovados, cobrem seis comandos em objetos/grupos: aparência e representação retornam ao original nos pares inversos preparados.81 unitários e107 checks GPU/126 frames PASS. Desktop CtrlPgUp/Down e ShiftPgUp/Down restaurou dirty; SaveAs/reabrir conservou raw2,4,1,5,3, com objetos/camadas/página2 idênticos por ID. SHAee3900b834d0d9b0e64f0368f9da2d568e73f045a04dd98d86d6a67299be727e.

Transferências entre camadas não usam essa projeção e continuam exigindo auditoria da representação. Não presumir que frente/trás são inversos em qualquer posição inicial: mover a um extremo perde a posição intermediária; undo é o mecanismo para restaurá-la. Os testes de pares usam seleção originalmente no extremo oposto.

## Transferências e disponibilidade1.8

Referência2026 relida em18/10/2026. Para cada slot original i, D[i] recebe a camada final da sua fonte no plano. Q(l) é a sequência crescente dos slots onde D[i]=l; P(l) é a sequência planejada de fontes dessa camada. O resultado atribui fonte P(l)[k] ao slot Q(l)[k], alterando só a associação de camada planejada. Todas as fontes aparecem uma vez. Camadas sem mudança de associação/ordem conservam seus slots; camadas participantes podem precisar mover objetos não selecionados dentro de sua sequência para inserir a seleção na posição correta. Atributos desses objetos não mudam.

O builder de unidades é compartilhado com disponibilidade. Uma operação fica disponível quando pode mover alguma unidade suportada; considera extremos por camada/página, vizinhas editáveis, seleção múltipla, grupos e recusa de legado intercalado. A UI desabilita folha isolada de grupo. Menu e painel usam esses estados; atalho neutro não cria histórico. Não há clone de Document por botão, mas faltam benchmarks de sessão grande.

Quatro novas regressões de subida/descida/destino vazio/grupo aninhado foram reproduzidas antes e passaram após a mudança. Quinto teste cobre7 comandos×32 máscaras×5 cenas=1120 combinações de disponibilidade contra resultado do planner. Suite86 PASS, GPU116 checks/130 frames e3 recuperação PASS. Nove checks GPU novos validam slots, atributos, histórico, save/reopen, undo/redo, limites inferiores/superiores e cliques/atalho neutros.

Desktop abriu fixture intercalada, selecionou Verde e CtrlPgUp transferiu ID3 para Superior, pulando Bloqueada. CtrlZ limpou dirty e CtrlY restaurou transferência. Selecionar amarelo mostrou frente de página/camada e avançar desabilitados no menu, frente/subir desabilitados no painel; clicar no item desabilitado não acrescentou histórico (um undo voltou ao salvo). SaveAs/reabrir manteve raw1,4,2,3,5 e pintura1,2,4,3,5, camada ativa3. XML preservou todos os objetos por ID salvo layer de ID3, definições/flags, raiz e página2. SHA36c1b5423ffd51f9bfae34e8049c46e6648d21a03ace56be9bef5b1930aebfbc. Fechamento normal confirmado.

Limite: a projeção não guarda proveniência histórica de slots. Transferir e executar comando oposto pode devolver pintura/camada sem reproduzir a intercalação bruta inicial, afetando comparação estrita de dirty. Undo restaura a cópia integral exata. Não declarar involução geral nem normalizar/descartar diferenças para elevar paridade. Mestras, ordenação interna, inversão entre camadas e comparação CorelDRAW continuam abertos. O subconjunto por alvo foi acrescentado na1.9.

## Ordem relativa por alvo1.9

Selecione um ou mais objetos ou grupos inteiros, escolha Objeto → Ordem → À frente de um alvo… ou Atrás de um alvo… (também no painel Objetos), e clique em outro objeto no canvas. O modo mostra instrução/cursor e conserva os IDs da seleção e página originais. Escape ou botão direito cancela sem alterar conteúdo/histórico e restaura a ferramenta anterior; alvo inválido mantém a escolha para nova tentativa. Mudar seleção, página, ferramenta ou documento encerra a escolha. Pan continua permitido. O clique aceito é consumido antes de mover/redimensionar ou editar por duplo clique; os objetos conservam suas posições.

O kernel expande fontes e alvo para raízes inteiras, constrói unidades por camada e retira fontes selecionadas na ordem atual de pintura. Depois insere esse bloco imediatamente antes (atrás) ou depois (à frente) da unidade alvo. Fontes de outras camadas recebem a camada editável do alvo; não ocorre reparentamento. IDs, geometria, atributos, descendentes/arestas e ordem interna são preservados. A projeção sobre slots finais da1.8 e validação/rollback são compartilhadas. Uma operação aceita cria uma transação; se já estiver adjacente na posição solicitada, não altera rawDoc nem cria histórico. A camada ativa acompanha a seleção após mudança efetiva.

Recusas: seleção vazia/inválida/não editável, alvo próprio, invisível/bloqueado ou raiz com descendente não editável; folha isolada no grupo. Conservadoramente, qualquer raiz legada intercalada ou grupo plano em várias camadas na página impede a operação, mesmo não participante. Objetos visíveis bloqueados entram no hit-test e são recusados; não se escolhe silenciosamente outro objeto atrás deles. Não há ciclo para atingir alvo ocluído, destaque de hover ou ordenação interna de folhas/mestras. Essas restrições e regras exatas da referência permanecem lacunas.

Sete novos testes de kernel cobrem bloco e adjacência, fontes em múltiplas camadas, raízes aninhadas de origem/alvo, atributos integrais, recusas sem mutação, noop exato/estados e nativo/SVG/página2.93 unitários PASS, check sem avisos.139 verificações GPU/138frames,3 recuperação e self-test1.9 PASS. Roteiro GPU valida cliques em branco/próprio/bloqueado, Esc/direito, seleção/ferramenta/histórico, ponteiro após alvo sem arrasto, frente/atrás, transferência e reabertura nativa/SVG.

Desktop: fixture intercalada; vermelho à frente do verde pelo painel, Esc antes da operação preservou seleção/estado limpo. Um undo após a operação voltou ao salvo. Depois menu Objeto/Ordem → Atrás de um alvo e clique no amarelo transferiram vermelho da camada1 à3, sem mudar suas dimensões/posição e preservando seleção. CtrlZ limpou dirty, CtrlY restaurou. SaveAs target/desktop-relative19.artdraw, reabertura limpa e fechamento normal confirmados. XML inteiro comparado à fixture: somente active-layer da página1 e layer do ID1 mudaram para3; raiz, geometria/atributos de todos os objetos, definições de camadas, grupos e página2 exatos. Raw1,4,2,5,3; pintura2,3,4,1,5. SHA256 fec2530c488c015f347f521511dad0a42cd2bf0475799c0f6d25e4b3738d2265.

Esses resultados são de corpus próprio, sem ensaio na build executável do CorelDRAW. Não demonstram equivalência funcional integral, visual/UI/UX ou compatibilidade CDR.
