# ArtDraw — memória de arquitetura e continuidade

## Contrato
Editor vetorial desktop offline para Windows x64 em Rust, clean-room. Nome de produto ArtDraw; pacote/binário coreldraw_clone. Interface organizada conforme pedido, sem código ou ativos proprietários da Corel. O conjunto de ferramentas básico não equivale a todo o CorelDRAW.
Antes de criar/modificar cada arquivo autoral, ler esta memória, registrar a operação e salvar; depois registrar conclusão. `scripts/write-tracked.ps1` automatiza esse contrato. Builds e provisionamento registram árvores geradas por operação (não se enumeram arquivos internos de dependências de terceiros). Formatação também é registrada por arquivo.

## Regra extraordinária de lacunas e fidelidade
AGENTS.md é obrigatório: ler documento vivo e linha da matriz antes de implementar; a cada descoberta/recurso/teste atualizar estado/aceite/evidência/lacuna em docs/FEATURE_MATRIX.csv; gerar scripts/update-parity.ps1 e conferir -Check antes de encerrar/compilar/empacotar. Saídas geradas: CORELDRAW_GAP_LEDGER.md, FEATURE_MATRIX.md e PARITY_STATUS.md. Nunca promover pesquisa/testes internos a equivalência comparativa. Meta explícita inclui fidelidade UI/UX/temas e CDR bidirecional sem perdas; CDR não implementado/nenhuma versão homologada. Plano em CDR_INTEROPERABILITY.md. Inventário é conhecido, não auditoria exaustiva do original.

## Estado atual — 1.9.0 validada; distribuição completa parcial
- Check sem avisos,93 unitários PASS (7 novos alvo), release opt3/ThinLTO.139 checks GPU/138frames,3 recuperação e self-test1.9.0 PASS. Intel Iris Xe/Vulkan1800×1080. Schema3/dependências preservados.
- Ordem relativa por alvo em kernel/unidades e estado transitório por IDs/página: frente/atrás, bloco em ordem de pintura, transferência para camada alvo, cancelar Esc/direito conserva seleção/ferramenta. Clique consumido não arrasta fonte. Recusas sem mutação; noop/histórico seguros. Detalhes matemáticos em STACKING_ORDER.md.
- Desktop1.9: painel frente do verde; cancelar preserva seleção/limpo; undo pós-operação limpa dirty. Menu atrás do amarelo transfere Vermelho ID1 camada1→3; seleção/bounds preservados. CtrlZ/Y PASS. SaveAs/reabrir/AltF4 normal PASS, inventário zero janelas QA. target/desktop-relative19.artdraw raw1,4,2,5,3/pintura2,3,4,1,5. XML integral só page1.active-layer e object1.layer para3; restante exato. SHA fec2530c488c015f347f521511dad0a42cd2bf0475799c0f6d25e4b3738d2265; relatório target/desktop-relative19-report.json.
- Automação mostrou desvio vertical captura/clique e alguns cliques UIA sem efeito; reobservação e Return com caminho verificado resolveram. Causa não atribuída ao app, sem alegar homologação DPI nem corrigir hipótese.
- Exe1.9 SHA b2fdebaab82d968eb48e041630e90da38b710720f21c0f9358c538fd1c51eb5b. Pacote1.9 final: TESTMODE19-relative-payload e19-relative-payload-final PASS20 hashes/self-test/registros intactos, sem atalhos/uninstaller. ZIP19 PASS20 entradas/fontes sem extras/duplicadas/faltantes;3 SHA256SUMS conferidos. Checkpoint dist/releases/1.9.0-PARTIAL completo, cópias hash exato e RELEASE_STATUS.json. Setup SHA bf353ed17ccb7baf26fd2dc0e2a636bbe9102d97f0b1eba5d59b9117f5856af8; ZIP SHA e22e5302e95b326f9aa3f9948eb7be6c06642a8129322e2bed36580b9e38a2ab. Checkpoint1.8.0-PARTIAL preservado (86/116/3, payload/ZIP20 PASS);1.7/1.7.1 PARTIAL,1.4/1.5 ciclos completos anteriores.
- Distribuição completa parcial: desinstalador1.6 NotSigned bloqueado AppControl17/10 19:08:26 eventos3033/3077/3089. target/installation qa16-pdf, registro/atalhos/fixtures QA permanecem. Não bypass, mudar política, tentar nova rota ou remover registro manualmente. Testar só TESTMODE isolado sem registro/atalhos/uninstaller até assinatura aceita.
- Inventário363/22 famílias/362 offline:25 baseline/46 básico/26 parcial/3 a validar/259 pendente/3 referência/1 fora;42,00 créditos→11,6% interno, zero equivalentes. CDR leitura/escrita ausentes. Meta100% aberta; sem build executável original/corpus comparativo.
- Limites1.9: folha isolada/mestras/reparentamento e inversão entre camadas ausentes; qualquer raiz legada intercalada ou grupo plano multicamada na página impede alvo, mesmo não participante. Sem ciclo para alvo ocluído/destaque hover. Undo restaura Document integral; opostos não garantem interleaving bruto/dirty original. Comparar comportamento original; não normalizar content_eq sem auditoria.
- Páginas independentes, hierarquia/folhas, XML3 lê1/2, SVG e PDF1.4 RGB/texto em curvas permanecem. Checks PDF atuais PASS; homologação independente histórica1.6 preservada. Autorizações autônomas mantidas, nunca matar janela com possível trabalho não salvo.

## Mapa completo do projeto
Arquivos autorais:
- ARCHITECTURE_MEMORY.md — contrato, mapa, decisões, estado e registro.
- AGENTS.md — regras extraordinárias persistentes para memória, lacunas, fidelidade e CDR.
- scripts/update-parity.ps1 — valida IDs/campos/estados e gera/verifica os três documentos e bloco de progresso README a partir do CSV.
- docs/CORELDRAW_GAP_LEDGER.md — documento vivo gerado,363 itens com lacunas/aceite/evidência/fontes.
- docs/CDR_INTEROPERABILITY.md — critério de ida/volta sem perdas, subtarefas e fontes primárias; nada integrado ainda.
- Cargo.toml — eframe=0.31.1, Vello=0.5.1 (ambos wgpu24), rfd, roxmltree, svgtypes, ttf-parser, epaint_default_fonts e image/png. Release opt-level3, ThinLTO, 1 codegen unit, strip, panic abort.
- Cargo.lock — resolução reproduzível gerada pelo Cargo.
- .gitignore — exclui .tools/, target/, dist/ e logs.
- build.rs — recursos de ícone/versão/manifesto via windres COFF no GNU e rc.exe/.res no MSVC, com busca Windows SDK/ARTDRAW_RC.
- build.ps1 — modos Environment/Check/Test/Release/All, --locked e opção -Offline; aceita ambiente padrão ou ferramentas locais GNU. Incremental desativado no script.
- package.ps1 — licenças/fontes/Inno, assistente Inno Setup, portátil com todos os docs e SHA256SUMS.
- src/branding.rs — splash Win32 em thread própria com BMP incorporado e animação; ícone PNG para eframe.
- src/storage.rs — schema XML.artdraw v3, migração v1/v2, validação pré-save/checkpoint, validação, gravação atômica MoveFileExW e backup .bak.
- src/recovery.rs — checkpoints por sessão em worker, candidatos de processos encerrados e limpeza após ação confirmada.
- src/main.rs — janela nativa e modos --self-test, --gpu-smoke, --interaction-smoke, --recovery-smoke, --pdf-proof, --pdf-smoke e --splash-preview; log de falha em %TEMP%/ArtDraw-error.log.
- src/document.rs — Document/Page/Object/Geometry/Node, contornos, limites, hit-test, transformações, texto e testes.
- src/pdf.rs — export PDF1.4 vetorial, escopo documento/página, texto em curvas/DeviceRGB/alpha, limites e7 testes.
- docs/PDF_EXPORT.md — fluxo, matemática, corpus e perdas explícitas.
- scripts/verify-pdf.py — parser independente/PNGs/amostras PDF versus GPU; sem dependência de execução do app.
- src/ordering.rs — kernel planejado de ordem por unidades/camadas, transferências, rollback e27 unitários.
- docs/STACKING_ORDER.md — semântica/atalhos de ordem, limites e homologação.
- src/editing.rs — matemática pura de seleção por unidades/grupos, transformações coletivas, alinhamento, distribuição, cópia/ordem e formas; testes.
- src/app/relative.rs — escolha transitória por alvo, IDs estáveis, restauração de ferramenta, cancelamento e transação única.
- src/app/professional.rs — comandos coletivos, seleção isolada por ID, dock hierárquico/indexado de objetos/camadas/transformações/alinhamento.
- src/app/theme.rs — quatro presets próprios e preferência local.
- src/render.rs — Vello na GPU compartilhada com egui; cache da Page ativa/draft/visão/tema/tamanho, sem clonar páginas inativas.
- src/svg.rs — importação/exportação SVG, herança de estilo/alpha, transformações, texto portável/editável e testes.
- src/app.rs — painéis, menus, atalhos, propriedades, aparência, objetos, histórico, diálogos de arquivo/texto, captura de evidência.
- src/app/pages.rs — navegação/gerenciador e transações de páginas; confirmação e reset de estado transitório.
- tests/fixtures/legacy-v1.artdraw e legacy-v2.artdraw — fixtures próprias fixas para migração sem depender de target.
- tests/fixtures/reverse-interleaved.artdraw — variante própria com slots globais intercalados entre camadas; inversão dupla deve restaurar rawDoc/estado salvo exatos.
- tests/fixtures/reverse-order.artdraw — corpus próprio fixo com5 objetos/3 camadas/2 páginas para inversão, camada bloqueada e seleção entre camadas.
- tests/fixtures/nested-groups.svg — fixture própria3 folhas/2 níveis, página800×600, para QA externo.
- docs/GROUP_HIERARCHY.md — modelo/gestos/persistência/limites e evidência de hierarquia e folhas.
- scripts/tracked.cjs e scripts/write-tracked.ps1 — leem/sincronizam memória por arquivo autoral, inclusive documentação.
- src/app/interaction.rs — ponteiro, desenho, seleção, escala, F10, alças, divisão exata de curva, pan/zoom, grade e réguas.
- src/app/validation.rs — modo opcional de eventos egui simulados com verificações de desenho, movimento, escala, undo/redo, paleta, nós, caneta, texto e SVG; não ativa no uso normal.
- installer/ArtDraw.iss — instalador atual Inno6.7.3: splash/logo, licença, pasta/grupo/atalhos, progresso, conclusão e desinstalador padrão. /DQA=1 usa AppId distinto; /TESTMODE=1 não registra instalação.
- installer/TERMS.txt — apresentação da licença MIT existente; componentes mantêm suas licenças.
- installer/prepare_branding.py — variantes de tamanho/formato da imagem autorizada; usa Pillow, não redesenha arte.
- installer/test-payload.ps1 — teste TESTMODE sem registro/uninstaller, hashes de todos os arquivos e snapshots de registro antes/depois; não substitui QA completo.
- docs/WINDOWS_DISTRIBUTION.md — falha App Control1.6 e caminho legítimo de assinatura pendente.
- installer/test-installer.ps1 — QA completo com AppId separado em target; registro/payload/self-test/desinstalação/preservação.
- installer/setup.rs — histórico1.0/1.1, não usado por package.ps1≥1.2.
- installer/uninstall.rs — desinstalador legado1.0/1.1; não é o unins000.exe produzido pelo Inno atual.
- installer/assembler_bridge.rs — traduz --64 do GNU as para -c do driver LLVM.
- installer/dlltool_bridge.rs — chama dlltool GNU com o assembler bridge; evita colisões de import descriptors da variante LLVM.
- README.md — uso, atalhos, instalação, build e limitações.
- LICENSE — MIT do código original.
- docs/CORELDRAW_PARITY.md — referência pública, superfícies, workflows, lacunas e arquitetura independente.
- assets/app.rc — recursos Windows de ícone, versão e manifesto.
- assets/windows.manifest — DPI por monitor antes das janelas e execução asInvoker.
- assets/branding/logo-source.jpg — imagem fornecida pelo usuário, copiada sem alterações.
- assets/branding/splash.bmp, wizard.bmp, wizard-small.bmp, app.png, app.ico — variantes geradas e incorporadas; fonte original preservada.
- docs/VALIDATION_PROTOCOL.md — níveis de testes e pesos conservadores do indicador.
- docs/PARITY_STATUS.md — indicador, contagem de estados e limites.
- docs/FEATURE_MATRIX.csv —363 capacidades, estado, marco, aceite/evidência e fonte.
- docs/FEATURE_MATRIX.md — índice/estados/famílias da matriz.
- docs/ROADMAP.md — dez marcos, dependências e critérios.
- CHANGELOG.md — novidades1.9 e histórico1.8/1.7/1.2/1.1 e escopo pendente.
- scripts/tracked.cjs — read/edit/put por arquivo com sincronização da memória.
- VALIDATION.md — resultados finais observados, evidências e limites da verificação do instalador.
- scripts/write-tracked.ps1 — sincronização individual de memória e arquivos autorais.
Arquivos/árvores gerados:
- .tools/rustup-init.exe; .tools/rustup/; .tools/cargo/ — toolchain, rustfmt, registro e cache Cargo.
- .tools/llvm-mingw.zip; .tools/llvm-mingw-20261006-msvcrt-x86_64/ — ferramentas externas oficiais.
- .tools/windows-imports/libshlwapi.a — somente import lib de shlwapi, separada do CRT LLVM.
- .tools/innosetup-6.7.3.exe; .tools/inno/ — compilador oficial assinado, provisionado em modo portable; licença nos avisos.
- .tools/as-bridge.exe; .tools/dlltool-bridge.exe — adaptadores compilados.
- .tools/runtime-smoke.rs/.exe, .tools/debug-run.rs/.exe, .tools/import-smoke.def/.lib — diagnóstico temporário de runtime/linker.
- target/debug/ — cache e binários de verificação.
- target/release/coreldraw_clone.exe — destino obrigatório (existente e validado).
- dist/ArtDraw-Setup.exe; dist/Uninstall-ArtDraw.exe; dist/ArtDraw-Portable/; dist/ArtDraw-Portable.zip; dist/THIRD_PARTY_NOTICES.txt; dist/SHA256SUMS.txt — distribuição gerada; conferir estado por versão (1.6 parcial).

## Regras da UI e interação
Temas Claro/Cinza/Escuro/Preto próprios (persistência em %LOCALAPPDATA%/ArtDraw/settings.txt); menus/ações/propriedades no topo, ferramentas à esquerda, paleta à direita, inspector/lista de objetos junto à paleta, página branca central sobre fundo cinza, réguas e barra de estado. Seleção V, Forma F10, Caneta P, Retângulo F6/R, Elipse F7/E, Texto F8/T, Mão H. Esquerdo na paleta aplica fill; direito aplica stroke; primeira amostra remove cor. Sem seleção, define estilo dos próximos objetos. Shift+clique alterna unidades; Ctrl+A seleciona editáveis. Marquee esquerda→direita exige caixa integral do grupo; direita→esquerda cruza caixa inclusive linha sem área. Ctrl+G/Shift+G agrupa/desagrupa; Ctrl+C/X/V clipboard interno. Camadas/objetos têm visibilidade e bloqueio. Seleção coletiva move e redimensiona por quatro cantos; Shift restringe eixo/proporção. Caneta: clique cria canto, arrasto cria alças simétricas, Enter/direito termina aberto, primeiro nó fecha. Forma: arrastar nó/alças, Shift espelha alça, duplo clique divide segmento, Delete remove nó. Texto usa editor modal. Ctrl+roda zoom no ponteiro; espaço+arrasto/central navega. F4 ajusta página. Histórico de até100 snapshots, salvar padrão.artdraw e opção SVG com aviso de alterações ao fechar/novo/abrir.

## Matemática e dados
- Documento usa coordenadas próprias (px SVG); tela=origem+ponto*zoom; evento usa transformação inversa.
- Object conserva Geometry local e Affine separado. Mover pré-multiplica translação. Escala ancorada no canto oposto: T(destino)*S(destino/origem)*T(-origem)*transform_original; limites mínimos 0.1 evitam degeneração.
- Retângulos normalizados; elipses via kurbo (centro/raios). Nós contêm posição e alças absoluta de entrada/saída. Bézier B(t)=(1-t)^3P0+3(1-t)^2tP1+3(1-t)t²P2+t³P3. Mover nó translada alças; alças editam geometria local pela inversa de Affine.
- Limites usam extremos reais de curvas via kurbo; seleção usa winding nonzero/evenodd e distância a contorno flatten com tolerância convertida pelo zoom. Ordem reversa seleciona topo.
- Inserir nó usa nearest da cúbica e De Casteljau, preservando a curva exatamente.
- Texto: ttf-parser contorna Ubuntu Light incorporada; baseline=posição.y+tamanho, eixo Y invertido da fonte. Sem shaping complexo/seleção de fontes.
- Caminhos compostos preservam múltiplos subcontornos; F10 edita caminhos simples e converte primitivas preservando nome, estilo e Affine. Não explode texto/compostos em nós individuais.
- Vello compute real via wgpu/DX12 ou Vulkan; textura RGBA8, AA Area. Cache evita rasterizar novamente cenas estáticas; overlays são egui.

## SVG e limites explícitos
Primitivas, paths (cúbicas/quadráticas/arcos via kurbo), polígonos, grupos, affine, cores, transparência e estilo inline. Exporta texto em contornos com metadados ArtDraw para edição na reabertura. Mantém fill-rule evenodd e opacidade herdada. Limites: 16MiB de XML e50mil objetos. Adverte sobre recursos avançados não suportados: gradientes/padrões, raster, CSS global, filtros/recortes/máscaras, use, layout de texto avançado; CDR não é suportado. Fonte de texto externo adaptada para Ubuntu Light.

## Eventos nativos: regressões corrigidas
- Down/move/up podem chegar no mesmo redraw. Canvas inicia no pos do evento PointerButton pressionado (primário/central), com área e layer guard; drag/release usam posição final. Não regredir para âncora no pointer.interact_pos agregado.
- Ctrl/Shift podem ser soltos antes do redraw. Atalhos percorrem eventos Key pressionados em ordem e leem modifiers de cada evento, não i.modifiers final. Ctrl ou command detecta comandos; Shift por evento preserva redo/desagrupar. FastRect/FastKey cobrem esses casos; Ctrl+S externo também passou.

## Toolchain: caminho local GNU e caminho comunitário MSVC
O GNU mínimo do Rust contém GCC/ld/dlltool/CRT, mas não assembler nem shlwapi. LLVM-MinGW tem assembler e import libs, porém seu CRT não pode substituir o GNU. `build.ps1` usa GCC do Rust; `dlltool_bridge` preserva import libs GNU e direciona montagem a `assembler_bridge` (remove --64, adiciona -c, chama x86_64-w64-mingw32-as LLVM). Apenas libshlwapi.a entra no diretório isolado .tools/windows-imports. Não adicionar todo lib/ do LLVM ao link: isso mistura CRTs e gera referências indefinidas. O ensaio anterior com llvm-dlltool gerou IAT órfã para GetModuleHandleA/Sleep; LLD sozinho não resolveu. Diagnóstico mínimo Rust executou com sucesso.

## Decisões do incremento1.1 (históricas; hierarquia/SVG substituídos na1.4)
- Grupos são IDs simples por objeto; não há aninhamento. Cópias remapeiam grupo e usam camada ativa. Agrupar reúne numa camada e traz conjunto à frente; preservação sofisticada de hierarquia fica M2.
- Alinhamento/distribuição operam caixas de unidades (grupo ou objeto). Distribuição usa centros; extremos ficam fixos. Transformação comum pré-multiplica cada Affine. Giro/inclinação/espelho usam centro coletivo.
- Grupo com qualquer membro oculto/bloqueado não participa de seleção/transformação; evita editar parte involuntariamente.
- Paint order usa ordem de camadas e ordem de objetos, não apenas índices. Layer e Object controlam visibilidade/edição separadamente.
- Polygon/Star são caminhos fechados; espiral usa48 segmentos por volta; freehand amostra polilinha por distância em tela; não se promete suavização/parametrização posterior.
- SVG exporta arte normal + metadados ArtDraw de layers/grupo/estados/ordem e floats f64 exatos para retângulos/nós/affine. Importação nativa evita recálculo que alterava igualdade exata. Grupos externos aninhados são achatados; recursos SVG avançados continuam limitados.
- Tema altera painel/chrome/canvas e invalida cache; smoke inicia escuro, não sobrescreve preferências. Claro/escuro testados; cinza/preto/persistência ainda sem roteiro dedicado.

## Decisões de durabilidade/marca1.2
- Object.id é persistente; novos/copiar/importar alocam identidade distinta. Document.next_object_id persiste mesmo após excluir. Seleção e snapshots ainda usam índices; não afirmar migração completa para árvore por IDs.
- Nativo XML1 representa todas as geometrias atuais, layers/active, IDs/contador, grupos simples, visibilidade/bloqueio, RGBA, stroke/fill-rule e Affine. Leitura rejeita versão desconhecida, IDs repetidos, camada ausente, dados não finitos e limites32MiB/50mil objetos. Não é CDR.
- Save escreve temporário exclusivo no mesmo diretório, sync_all, backup anterior e MoveFileExW(REPLACE_EXISTING|WRITE_THROUGH). Falha não usa fallback de sobrescrita destrutiva. Fixtures ficam em target/test-work: TEMP da sandbox negava acesso; código de produto não foi afrouxado.
- Recuperação usa %LOCALAPPDATA%/ArtDraw/Recovery/recovery-PID-tempo.artdraw; worker após gesto, intervalo15s e comparação com último snapshot. Ignora sessões vivas; mantém original até salvamento explícito. Undo ao estado salvo retira checkpoint obsoleto. Testes usam ARTDRAW_RECOVERY_DIR, sempre em target.
- Splash GDI antes do GPU usa BMP incorporado; normal termina após área de trabalho pronta e mínimo1,1s. Falha fecha via Drop. Prévia45s usa janela detectável no taskbar apenas para QA. Ícone multirresolução no PE e PNG na janela. Manifesto DPI antes do splash evita depender da chamada posterior do winit.
- Inno é a camada de instalação; app continua Rust/egui/Vello. Install por usuário, pasta editável, licença MIT, atalhos opcionais, progresso real de cópia e unins000. QA usa identidade separada e preserva documentos. Instalador não contém rede em execução.
- Não substituir executável enquanto o usuário o usa nem matar janela com alterações. Fechamento normal/salvar é a alternativa segura; autorização para fechar foi dada e executada pelo usuário antes dos builds seguintes.

## Decisões de páginas1.3
- Document contém Vec<Page>, active_page ID estável, next_page_id e next_object_id globais. Page contém ID/nome/dimensões/objetos/camadas/camada ativa. Deref/DerefMut mantêm comandos de edição/render/hit-test restritos à página ativa; operações globais percorrem pages explicitamente.
- IDs de objetos são únicos no documento; duplicar página conserva geometria/estilos/estados/grupos locais/camadas e remapeia IDs. Nova página vazia herda dimensões/orientação, mas inicia camada básica. Nessa etapa grupos eram simples; a hierarquia1.4 abaixo substitui essa limitação.
- Navegar não chama remember e content_eq ignora somente active_page, preservando dirty neutro. Operações de conteúdo usam snapshot completo; undo/redo restaura páginas e seleção limpa, reajustando vista se página ou dimensão mudar.
- Schema2: root active-page/next-page-id/next-id e páginas completas. Leitor aceita1/2; schema1 vira página1 sem alterar geometria/IDs. Rejeita páginas vazias/repetidas, active/counter inválidos, IDs de objetos repetidos entre páginas, limites1000 páginas/50mil objetos globais/32MiB. ArtDraw1.2 não lê schema2; backup anterior conserva schema1 ao sobrescrever.
- validated_encode revalida pelo decoder antes de save/checkpoint, impedindo substituir origem por nativo ilegível. Atomicidade/backup anteriores preservados. Teste atualiza com IDs inválidos e confirma arquivo anterior intacto.
- SVG contém página ativa e seus metadados de ID/nome/contador. Salvar documento multipágina como SVG é recusado; exportação ativa explícita mantém documento atual. Importação de documento multipágina na página corrente é recusada com diagnóstico; abertura é completa.
- Canvas bloqueia interação sob gerenciador/confirmador de páginas; navegação limpa estados transitórios e ajusta vista. Excluir precisa confirmação e permite undo; última página protegida. Uma página é visível por vez, sem mestras/visão simultânea/árvore de grupos.
- Render cache guarda somente Page ativa para não comparar/clonar todo Document ao atualizar vista; formato de textura Vello/wgpu e conversão sRGB anteriores permanecem intactos.

## Decisões de ordenação1.5
- Unidades são raiz de grupo/objeto por camada, na ordem de pintura. Plano guarda índices originais+camada destino; selecionados/restantes preservam ordem relativa. Capturar blocos no limite antes das trocas locais e inserir transferências depois evita mover chegada duas vezes. Todo descendente adota camada destino; arestas/Affine/IDs não mudam.
- apply valida hierarquia após swap e restaura objetos em erro. Grupo legado não contíguo é recusado antes de mudar. app seleciona novamente pelos índices remapeados e só altera active_layer/histórico quando conteúdo muda. PageUp/Down sem modificadores continua navegação. Botões no limite ainda sem estado desabilitado equivalente à referência.

## Decisões de hierarquia/folhas1.4
- Page.group_parents:BTreeMap<child,parent> guarda apenas arestas; raiz implícita e Object.group imediato.64 níveis, ciclos/zero/órfãos recusados; raiz aninhada exige camada comum. Grupos planos legados entre camadas são aceitos para não quebrar SVG antigo. next_group considera chaves/valores/folhas e wrap u64::MAX sem colisão.
- Transformações ficam em cada folha Affine, sem transformar também o pai. Seleção normal/marquee/align/distribute usam raízes e indexam unidades uma vez; não refazer varredura integral por cada folha selecionada. Grupo com qualquer descendente oculto/bloqueado não é editado parcialmente.
- CtrlG conserva subgrupos, exige2 unidades e normaliza camada; CtrlU/alias CtrlShiftG separa um nível, Separar tudo remove todos os níveis selecionados. prune após delete/ungroup. Copiar/importar remapeia TODOS os pais/filhos, clipboard guarda arestas; duplicar página usa grupos locais e objetos globais novos.
- isolated_object:Option<stableID> transitório preserva seleção de folha enquanto válida. Ctrl+clique/canvas ou folha do docker isola; arrastar novamente a mesma folha sem Ctrl mantém isolamento. Cor, movimento/escala, texto,F10/delete só afetam a folha. Cópia/duplicação isolada vira objeto avulso. Grupos/ordem interna bloqueados até regras próprias; selecionar conjunto volta à raiz. Undo/redo/novo/abrir/páginas limpa escopo.
- ObjectTree adjacência/descendentes computados uma vez porframe; teste10mil folhas/5mil grupos é estrutural, não benchmark de latência. Docker toggles grupo afetam descendentes; estado parcial ainda não tri-state. Drag/reparentamento/foco/seleção progressiva pendentes.
- XML3 guarda groups/group id,parent; lê1/2/3, fixtures fixas legacy-v1.artdraw e legacy-v2.artdraw. validated_encode lê novamente antes de save/checkpoint. SVG escreve g padrão aninhado e metadados exatos; import nativo evita realocar wrappers, externo usa posição XML como identidade para montar arestas herdadas. Grupos legados intercalados podem virar wrappers separados para preservar pintura externa.

## Decisões PDF1.6
- px96dpi→0,75pt: CTM página[.75,0,0,-.75,0,height*.75], seguida de affine por objeto. Elevar quadrática exatamente: C1=P0+2/3(Q-P0), C2=P2+2/3(Q-P2). Preservar Move/Line/Cubic/Close e fill-rule, round caps/joins, ordem por layer+objeto, visibilidade mas não ignorar bloqueados. Clip na página branca.
- ExtGState ca/CA independentes reutilizados globalmente, recursos por página; DeviceRGB/Normal/transparency group, sem ICC. Texto somente outlines Ubuntu, título UTF16BE. Vello_encoding0.5.1 DrawColor converte Srgb e premultiplica8bit; seis amostras tolerância3/255 coincidiram exatamente, sem certificar antialias completo.
- Writer objetos indiretos/xref offsets reais, Length bytes; números sem expoente finitos abs<=1e9,64MiB/max1000 páginas/dimensões1..100000. save exige.pdf e serializa antes de atomic_write/backup. Exportar não altera doc/saved/history/file. Poppler/pypdf só em QA.
- --pdf-proof gera corpus3 páginas+fonte.artdraw/log; --pdf-smoke usa mesmo corpus na GPU. Outros modos permanecem: self-test, interaction-smoke, recovery-smoke, gpu-smoke, splash-preview. Marcador da habilidade PDF executado exatamente uma vez antes de autoria. Prova final output/pdf/ArtDraw-proof.pdf; PNGs intermediários tmp/pdfs, logs target.

## Histórico resumido, regressões e checkpoints
-1.0 base12 unitários;1.1 seleção coletiva/camadas/temas/formas;1.2 durabilidade/marca/installer40 unitários/44 UI/3 recovery;1.3 páginas48/57/3. Detalhes/versionamento em VALIDATION.md e CHANGELOG.md.
-1.4:59 unitários/80 UI98frames/3 recovery+selftest e desktop árvore/folha/cores/F10/save/reopen PASS. QA14-groups e14-groups-final completos; ZIP hash conferido. Arquivado dist/releases/1.4.0: exe f754f897e473ede865d1149c9202de3a8cb6ecae94434ebd5a7ad94591de96cb; setup2e64381c36573b89698c0ba9ed0679989f3a437b91f8f6359c5f3d521f9b455b; ZIP80d6eb8abaac2dd995340a4090578930ba1077b9eddcd6912f6ec4e2f610b774.
-1.4 corrigiu validação excessiva de grupo plano legado entre camadas, sem afrouxar árvore nova. ObjectTree test contava4 objetos, demo tinha5; corrigido teste. Dois cliques fixos do smoke ignoravam espelhoH anterior; alvo passou a usar geometria transformada real, critérios de irmãos/nó intactos preservados. Não esconder essas falhas antigas nem classificá-las como equivalência.
-1.5:67 unitários/93 UI117frames/3 recovery+selftest; desktop sobreposição/passo/skip camada2bloqueada/extremos/noop/history/save/reopen PASS. target/desktop-order15.artdraw IDs2,3,4,5,1, ID1camada3/página1; SHA44f76d2addd1790bb97cc0e155242fd3f8a923d8bff7eff9894e874c588b9e95. QA15-order e15-order-final completos+ZIP. Arquivo dist/releases/1.5.0: exe344eca70251fffb6c814e0f887ca228c172226d50070a817b03fa8e79a840f55; setup61c5c83d455a276a5cb13fc000c5b82bcae2bed52064c16169172fce5202949a; ZIP3e94e7219695af29ffd6e0a833ad482f2fe76de0367c94f26873a7131b914c1f.
-1.6:74/95/3+selftest/PDF externo/desktop PASS; uninstall bloqueado, portanto distribuição PARCIAL. PDF corpus e desktopdoc SHA5e1b61ea9843b7766432d75949b3b158b81ac2f93ca1f1b6b434346d1b776976; página2 SHA472faaec1863bb20e75123bbc370a678fcf5c70270c8918d0630b958f7d255dd. Executável1.6 SHA51d0cf48fbb6ee1f951e63aa98ce6257d1c7e585660b242fc0d2d895dac3a5f3. Novas descobertas PDF independentes: seleção, intervalos, crop por objetos, multidoc, senha, permissões (pendentes). Instalação parcial reduz indicador após11,5% preliminar para11,4%.
-1.6 documento de atualização em lote teve SyntaxError Node antes de executar; não houve edição parcial autoral. Comando separado corrigido e gerador confirmou363 IDs9 campos. Não tratar erro de ferramenta como falha de app.

## CDR — nenhum suporte anunciado
Pesquisa primária libcdr MPL2.0: CDRDocument detecção/parse, RIFF/CDR e Waldo, contêiner estruturado content/riffData.cdr ou root.dat, lista dataFileList.dat e content/data; perfis color/profiles/cmyk e rgb. Nada incorporado. Precisam corpus autorizado/build fixada, sondagem limitada, bytes/streams desconhecidos preservados, recursos/referências, parser semântico, adaptador e gravador real por versão. Detecção não significa abrir/editar; converter SVG/XML renomeado é proibido. Critérios e referências em docs/CDR_INTEROPERABILITY.md. Leitura e escrita continuam requisitos separados, pendentes, sem versões homologadas.

## Próximas operações e cuidados
1. Publicar fontes limpas e preview1.9.0 com notas/hashes preservados. Não reempacotar versões históricas nem presumir QA de uma máquina como CI de outra. Binários/publicação não significam fim da meta de fidelidade.
2. Assinatura de publicação aceita é dependência de uninstall na política observada. Não alterar configurações de segurança, usar outra rota para executar o arquivo bloqueado ou remover registro QA para mascarar falha. Não tratar TESTMODE como homologação completa.
3. Próximos recursos pelas lacunas: organização interna/reparentamento, seleção progressiva/recursos e kernel geométrico; inversão na mesma camada e alvo por clique têm subconjuntos1.7/1.9. PDF seleção/intervalos continuam pendentes. Mestras/visão simultânea/texto/raster/cor e CDR continuam abertos.
4. Ler CSV/ledger/referência antes de cada recurso e memória antes de cada arquivo. Atualizar aceite/evidência/lacuna, gerar e -Check. Não promover smoke a equivalência.
5. Todos os diretórios installation qa12/13/14/15 contêm fixtures preservadas; não apagar para reciclar RunId.1.6 QA ocupada mantém identidade separada; novas conferências payload não registram. Respeitar trabalho do usuário, fechamento normal.

## Complemento matemático e estado1.7–1.9
- Inverter ordem: >=2 unidades na mesma camada, permuta apenas seus slots; dupla inversão devolve Document/encode exatos inclusive vetor global intercalado. Não inverter entre camadas nem ordenar folha isolada.
- Projeção1.8: para cada slot original i, D[i] recebe a camada final da sua fonte no plano; Q(l) lista slots D[i]=l e P(l) lista fontes planejadas nessa camada. Atribuir P(l)[k] a Q(l)[k] preserva slots de camadas não envolvidas. Não guardar proveniência histórica; comando oposto pode recuperar pintura/camada sem raw interleaving/dirty originais. Undo restaura snapshot integral.
- Disponibilidade compartilha builder com planner;7 comandos×32 máscaras×5 cenas=1120 combinações passaram. Sem benchmark grande nem equivalência comparativa.
- Relativo1.9: fontes e alvo expandem raízes; retira selecionadas em ordem de pintura e insere bloco antes/depois da unidade alvo, mudando apenas layer planejado. finish_plan valida/projeta/rollback. Bloqueio/visibilidade são editabilidade. Estado transitório usa IDs/página/ferramenta anterior; cancelamento não escreve documento/histórico. Clique aceito consumido antes de movimento/duplo clique. Casos conservadores/gestos/provas detalhados em docs/STACKING_ORDER.md.

## Histórico adicional preservado em síntese
-1.7:79 unitários/101 GPU/3 recovery; inversão por unidades. Houve falha DirectoryNotEmpty no cleanup de teste; repetir somente remove_dir vazio até1s resolveu o harness sem mudar produto.
-1.7.1:81/107/3; dois testes de deslocamento sem transferência reproduziram normalização indevida antes e passaram depois da projeção sobre slots originais.
-1.8:86/116/3; quatro regressões de transferência reproduzidas antes/passaram após projeção por camada final. Mais1120 combinações de disponibilidade. Desktop e XML íntegros salvo camada planejada, payload/ZIP20 PASS.
-1.9:93/139/3 e138frames;7 novos unitários relativos,23 checks UI adicionais. Desktop painel/menu/cancelar/undo/redo/save/reabrir/fechar normal e XML integral PASS. RELEASE_STATUS.json no checkpoint local contém hashes; CDR ausente.
- Checkpoints1.7/1.7.1/1.8/1.9 são PARTIAL pela pendência de distribuição;1.4/1.5 ciclos completos históricos. Não executar uninstaller bloqueado por outra rota.
- Log de operações anterior foi resumido, mantendo mapa, contratos, matemática, UI, regressões e evidências. Cópia local integral em target/ARCHITECTURE_MEMORY-pre-community.md; não faz parte do Git público. Documentos de validação/ordem conservam resultados detalhados por versão.

## Publicação comunitária —19/10/2026
- Repositório remoto caio-vinicius-dmc/ArtDraw público inicialmente só LICENSE. .git local integrado à main remota sem reescrever histórico; atribuição MIT original preservada. Nenhum cache/toolchain/artifato/teste temporário deve ser publicado em Git. Binários validados1.9 devem ser anexos de pré-release com hashes existentes, sem chamar publicação de nova homologação.
- README refeito com motivação, screenshot real, progresso/restante gerados do CSV, limites/CDR, download, instalação, arquitetura e convite à comunidade. CONTRIBUTING e docs/BUILDING/USAGE detalham participação/caminho de build; .github contém CI Windows e modelos de issue/PR.
- Helpers scripts/tracked.cjs e scripts/write-tracked.ps1 entram no Git; .tools continua local e ignorada. .gitignore também exclui output/tmp e padrões de credenciais. .gitattributes mantém LF/bytes binários. Novos assets docs/images/workspace-v1.9.0.png e editing-v1.9.0.png são capturas GPU reais próprias sem dados pessoais; docs/images/README.md documenta origem.
- CI .github/workflows/windows.yml: clone, Rust MSVC estável, conferir paridade/README, fetch locked, check/test offline. Checkout fixado por SHA/contents read. Não executa GPU nem instalador. build.rs mantém GNU e acrescenta rc.exe SDK MSVC; ARTDRAW_RC override. build.ps1 -Offline explícito, somente host GNU ativa bridges locais. Validação local GNU dos ajustes: check sem avisos e93 unitários PASS;44 links/embeds locais e parser PowerShell PASS. Caminho MSVC conferido separadamente no CI após publicação. Binários/checkpoints existentes conservados; não anunciar novo ensaio GPU.

## Arquivos comunitários adicionais ao mapa
- CONTRIBUTING.md — contribuição, memória, issues/PRs, testes e clean-room.
- .gitattributes — normalização de texto/ativos.
- .github/workflows/windows.yml — CI de clone Windows MSVC sem GPU.
- .github/pull_request_template.md; .github/ISSUE_TEMPLATE/bug_report.md e feature_request.md — relatos e revisão.
- docs/BUILDING.md; docs/USAGE.md — build/QA/pacote e uso/formatos/atalhos.
- docs/images/README.md; docs/images/workspace-v1.9.0.png; docs/images/editing-v1.9.0.png — capturas reais/legendas.

## Registro ativo comunitário
- CONCLUÍDO: ARCHITECTURE_MEMORY.md.
- SINCRONIZAR: docs/CORELDRAW_GAP_LEDGER.md a partir da matriz; memória lida antes da gravação.
- CONCLUÍDO: docs/CORELDRAW_GAP_LEDGER.md; matriz SHA256 787a9b5cb0e69a2a1d53cce791070c474a12262e706300879d57c487dfd2dbad
- SINCRONIZAR: docs/FEATURE_MATRIX.md a partir da matriz; memória lida antes da gravação.
- CONCLUÍDO: docs/FEATURE_MATRIX.md; matriz SHA256 787a9b5cb0e69a2a1d53cce791070c474a12262e706300879d57c487dfd2dbad
- SINCRONIZAR: docs/PARITY_STATUS.md a partir da matriz; memória lida antes da gravação.
- CONCLUÍDO: docs/PARITY_STATUS.md; matriz SHA256 787a9b5cb0e69a2a1d53cce791070c474a12262e706300879d57c487dfd2dbad
- SINCRONIZAR: bloco de progresso README a partir da matriz, mantendo texto autoral.
- CONCLUÍDO: bloco de progresso README sincronizado.

- BUILD Check iniciado: Cargo pode atualizar Cargo.lock e as árvores geradas target/.tools/cargo. Memória lida antes da operação.
- BUILD Check concluído com sucesso.

- EM EXECUÇÃO: build.rs — Formatar apenas build.rs com rustfmt antes da suíte final.
- CONCLUÍDO: build.rs.

- BUILD Test iniciado: Cargo pode atualizar Cargo.lock e as árvores geradas target/.tools/cargo. Memória lida antes da operação.
- BUILD Test concluído com sucesso.

- EM EXECUÇÃO: docs/BUILDING.md — Precisar diretório versionado do resource compiler no Windows SDK.
- CONCLUÍDO: docs/BUILDING.md.

- EM EXECUÇÃO: docs/BUILDING.md — Exibir corretamente placeholder do SDK como código Markdown.
- CONCLUÍDO: docs/BUILDING.md.

- EM EXECUÇÃO: LICENSE — Conservar newline final padrão sem alterar termos MIT.
- CONCLUÍDO: LICENSE.

- EM EXECUÇÃO: VALIDATION.md — Registrar testes reais de preparação pública sem promover documentação a recurso ou GPU histórica a teste novo.
- CONCLUÍDO: VALIDATION.md.
- CONCLUÍDO: ARCHITECTURE_MEMORY.md.

- EM EXECUÇÃO: target/release-notes-public.md — Preparar notas de pré-release concretas com hashes/status históricos corretos e participação.
- CONCLUÍDO: target/release-notes-public.md.
