# Publicação comunitária — 19/10/2026

Produto continua na1.9.0 experimental, indicador interno11,6%, sem nova equivalência ou suporte CDR. Nesta etapa foram preparados README com capturas reais e progresso/restante sincronizados pelo CSV, CONTRIBUTING, manuais de build/uso, helpers de memória versionados, modelos de issues/PRs e CI Windows MSVC. Nenhuma ferramenta nova do editor foi implementada; documentação/infraestrutura de colaboração não elevam o indicador.

- Build GNU local após ajustes de build.rs/build.ps1: cargo check PASS, zero avisos;93 unitários PASS, zero falhas. Build.rs agora seleciona windres/GNU ou rc.exe do SDK/MSVC; os recursos e a lógica do editor não mudaram.
- Windows MSVC em clone limpo PASS: execução [39327529588](https://github.com/caio-vinicius-dmc/ArtDraw/actions/runs/39327529588), commit c1db036, cargo check e93 testes unitários sem falhas. O primeiro CI teve anotação Node20/checkoutv4, não um aviso do compilador Rust; ação atualizada para versão oficialv7.0.1/Node24 fixada por SHA. O resultado de cada commit permanece em Actions. Esse CI não executa GPU, UI real, DPI ou instalador.
- Conferidos44 links/embeds locais em5 documentos públicos, parser PowerShell de build/paridade e -Check do inventário/README. Revisão inicial de78 arquivos elegíveis (~5,63MiB) não detectou padrões de token/chave privada/caminho pessoal; não constitui auditoria de segurança completa.
- .gitignore mantém target, dist, .tools, tmp, output e credenciais fora do Git; Cargo.lock, fixtures, assets próprios e helpers necessários ao clone são versionados. Licença MIT original do repositório/atribuição do autor preservadas. Histórico remoto inicial preservado; nenhum force-push planejado.
- [Pré-release v1.9.0 publicada](https://github.com/caio-vinicius-dmc/ArtDraw/releases/tag/v1.9.0), cinco anexos baixados novamente e conferidos por SHA256 contra o checkpoint1.9.0-PARTIAL. Binários não recompilados só para alterar documentação. Hashes exe b2fdebaab82d968eb48e041630e90da38b710720f21c0f9358c538fd1c51eb5b, setup bf353ed17ccb7baf26fd2dc0e2a636bbe9102d97f0b1eba5d59b9117f5856af8 e ZIP e22e5302e95b326f9aa3f9948eb7be6c06642a8129322e2bed36580b9e38a2ab reconferidos localmente. As capturas GPU são da1.9.0 em18/10/2026, não novos ensaios nesta data.
- Assinatura/instalação/desinstalação completa continuam parciais pela falha App Control já registrada. Publicação no GitHub não resolve esse bloqueio. Recomendação pública: avaliar pelo portátil, sem instrução para contornar proteções.

# Validação do ArtDraw 1.9.0

18/10/2026, Windows x64. Implementação clean-room; sem comparação interativa com CorelDRAW e sem leitor/gravador CDR.

| Verificação | Resultado |
| --- | --- |
| cargo check --locked --offline | PASS, zero erros/avisos |
| cargo test --locked --offline | 93 PASS, zero falhas; sete novos testes de alvo |
| release | PASS, opt3/ThinLTO/1 codegen unit |
| --interaction-smoke | 139 PASS,138 frames Vello |
| --recovery-smoke | 3 PASS |
| --self-test | PASS1.9.0 |
| Desktop externo | Frente pelo painel, atrás pelo menu, cancelar, transferência, undo/redo, SaveAs/XML integral/reabrir e fechar normal PASS |
| Distribuição1.9 | Payload19-relative-payload PASS20 hashes/self-test/registros inalterados; instalação/desinstalação completa permanece parcial |

## Critérios e cobertura

Sete novos unitários sobre86 anteriores cobrem frente/atrás imediatos, seleção em bloco na ordem atual de pintura, transferência de várias camadas, preservação de slots alheios/atributos completos, grupos aninhados de fonte e alvo sem reparentamento, recusas sem mutação, disponibilidade/noop exato e nativo/SVG mantendo página2. Suite completa93 passou sem falha de teste nesta etapa. O kernel de projeção final é compartilhado com os comandos anteriores; nenhum schema/dependência foi alterado.

GPU:116 verificações anteriores mais23, total139/138frames no Intel Iris Xe/Vulkan1800×1080. O roteiro injeta eventos egui na janela com render Vello real; não substitui o teste externo do desktop. Inclui cliques em branco/próprio/bloqueado sem perder modo/seleção/histórico, Esc/direito, fonte não arrastada pelo clique do alvo, frente/atrás, transferência, undo/redo, cancelar por ferramenta/página, restaurar ferramenta anterior e salvar/reabrir nativo/SVG. Logs/capturas target/release-interaction19.txt/.png, release-recovery19.txt/.png e release-self-test19.txt; captura GPU inspecionada. Checks PDF existentes passaram no roteiro, sem nova homologação independente de parser/raster PDF neste ciclo.

## Desktop e persistência

Sessão isolada com ARTDRAW_RECOVERY_DIR=target/desktop-relative19-recovery e ARTDRAW_SETTINGS_PATH=target/desktop-relative19-settings.txt, executável release atual. Abriu tests/fixtures/reverse-interleaved.artdraw; selecionou Vermelho ID1. Modo À frente do alvo pelo painel, Esc preservou seleção e documento limpo. Nova ativação e clique no verde colocou vermelho à frente dele, mantendo seleção/posição/dimensões. Um CtrlZ restaurou o documento sem dirty, sem transação de cancelamento adicional.

Depois Objeto → Ordem → Atrás de um alvo e clique no amarelo transferiram vermelho da camada1 à3 imediatamente atrás do ID5. Inspector mostrou Superior acima de Vermelho; a seleção permaneceu no vermelho. CtrlZ restaurou arte/estado limpo, CtrlY refez transferência. SaveAs pelo diálogo nativo criou target/desktop-relative19.artdraw, reaberto sem dirty e com aparência/camada corretas. AltF4 fechou normalmente após salvar; inventário posterior confirmou zero janelas ArtDraw QA. Nenhum encerramento forçado.

Conferência integral do XML contra a fixture: somente page1.active-layer e object1.layer mudaram para3. Raiz/contadores, todos os outros atributos e geometria, definições de camadas/grupos, flags e página2 idênticos. Vetor bruto1,4,2,5,3; pintura2,3,4,1,5. SHA256 fec2530c488c015f347f521511dad0a42cd2bf0475799c0f6d25e4b3738d2265. Relatório target/desktop-relative19-report.json.

Observação do instrumento de QA: capturas1440×864 e cliques apresentaram desvio vertical; foram reobservados antes de adaptar as coordenadas. Cliques UIA em Abrir/Restaurar não produziram ação; a observação confirmou estado intacto, e Return no diálogo com caminho verificado completou Abrir/Salvar. Não foi atribuída causa ao produto nem alegada homologação de DPI. Não houve alteração de código para compensar hipótese sobre o instrumento.

## Limites e distribuição

Alvo próprio/oculto/bloqueado ou raiz com membro não editável recusado. Folha isolada, mestras, reparentamento e inversão entre camadas não implementados. Qualquer raiz legada intercalada ou grupo plano multicamada na página impede o modo, mesmo fora da operação. Não há ciclo para alvo ocluído nem destaque de hover. Comandos opostos não garantem a intercalação bruta inicial; undo restaura Document integral. Grupos/camadas/estados/UI/UX exigem ensaio comparativo na build original.

Exe SHA256 b2fdebaab82d968eb48e041630e90da38b710720f21c0f9358c538fd1c51eb5b; FileVersion1.9.0.0/ProductVersion1.9.0 e metadado SVG1.9 conferidos. TESTMODE19-relative-payload aprovou20 arquivos idênticos às fontes, self-test1.9.0 do executável copiado e registros produção/QA intactos, sem atalhos/desinstalador. Relatório target/payload-19-relative-payload-report.json. Reconferência após consolidar estes documentos usa RunId19-relative-payload-final; o resultado fica no relatório correspondente e no checkpoint, sem antecipá-lo aqui. Checkpoint1.8.0-PARTIAL preservado. Distribuição completa continua parcial: uninstaller QA1.6 NotSigned bloqueado pelo App Control, eventos3033/3077/3089; registro/atalhos/pasta e fixtures permanecem. Nenhuma nova tentativa, mudança de política ou remoção manual. Payload TESTMODE testa cópia/execução isoladas sem registro/atalhos/uninstaller; não demonstra desinstalação. Ver WINDOWS_DISTRIBUTION.md.

Inventário363 itens/362 offline,42,00 créditos, indicador interno11,6%, zero equivalente_verificado. O indicador mede maturidade parcial do inventário, não percentual certificado de cópia. CDR leitura/escrita ausentes;100% ainda distante e a meta permanece aberta.

## Histórico1.8

# Validação do ArtDraw1.8.0

18/10/2026 Windows x64, clean-room. Sem comparação interativa com CorelDRAW e sem suporte CDR.

| Verificação | Resultado |
| --- | --- |
| cargo check --locked --offline | PASS, zero erros/avisos |
| cargo test --locked --offline |86 PASS, zero falhas |
| release | PASS, opt3/ThinLTO/1 codegen unit |
| --interaction-smoke |116 PASS,130 frames Vello |
| --recovery-smoke |3 PASS |
| --self-test | PASS1.8.0 |
| Desktop externo | Transferência, bloqueio, undo/redo/dirty, menu/painel desabilitados, clique neutro, SaveAs/XML/reabrir e fechar normal PASS |
| Distribuição | Payload18-transfers-payload PASS20 hashes/self-test/registros intactos; ciclo completo parcial pelo App Control/assinatura |

## Regressões e critérios

Quatro novos testes de transferência falharam antes da correção: subida, descida, destino vazio com camada oculta e grupo raiz aninhado movido à frente da página. O plano antigo normalizava slots de camadas alheias. Após projetar fontes sobre slots de suas camadas finais, os quatro passaram; atributos completos, seleção por identidade, ordem interna/arestas e persistência foram conferidos. Primeiro build do harness falhou E0609 por presumir tupla em storage::decode; corrigido para Document antes de executar as regressões. Não era erro do produto.

O quinto teste compara disponibilidade e resultado do planner em7 comandos×32 máscaras×5 cenas=1120 combinações, incluindo seleção vazia, camadas bloqueadas, grupo aninhado e legado intercalado recusado. Não são1120 ferramentas nem equivalência CorelDRAW. Suite completa86 PASS, incluindo81 anteriores. Builder de unidades compartilhado; não clona Document por botão. Não constitui benchmark de grandes sessões.

## Interface e persistência

GPU:107 checks anteriores mais nove sobre transferência, atributos/slots, salvar/reabrir, undo/redo, disponibilidade dos controles reais e cliques/atalho neutros. Total116/130 frames Intel Iris Xe/Vulkan1800×1080. Captura final inspecionada. Arquivos target/release-interaction18.txt/.png/.transfer.artdraw, release-recovery18.txt/.png e release-self-test18.txt.

Desktop: tests/fixtures/reverse-interleaved.artdraw abriu sem dirty; Verde selecionado e CtrlPgUp transferiu ID3 da camada1 à3 pulando2 bloqueada. CtrlZ retornou ao salvo; CtrlY restaurou. Amarelo no limite superior mostrou frente de página/camada e avançar indisponíveis no menu, frente/subir indisponíveis no painel. Clique no item desabilitado não criou histórico: um undo restaurou o salvo; redo repôs transferência.

SaveAs nativo criou target/desktop-transfer18.artdraw. XML raw1,4,2,3,5, pintura1,2,4,3,5; somente a associação de ID3 mudou para layer3. Objetos completos por ID, camadas/flags, atributos raiz e página2 intactos; camada ativa3 esperada. SHA25636c1b5423ffd51f9bfae34e8049c46e6648d21a03ace56be9bef5b1930aebfbc. Reabertura manteve aparência/camada/estado limpo; fechamento normal e inventário sem janelas QA. Grupos e destino vazio novos têm provas de código, sem roteiro desktop dedicado neste ciclo.

Exe SHA2568902773c5565b67d91b0b27ece749fce16e5b612289ae3ef6dd31b8750e5d8eb. Metadados/schema3 e dependências conservados salvo versão1.8.0. Payload TESTMODE18-transfers-payload PASS20 arquivos idênticos às fontes e self-test1.8.0 do exe copiado, com registros produção/QA intactos e sem atalhos/uninstaller. Metadado nativo1.8.0/SVG1.8 conferidos. Relatório target/payload-18-transfers-payload-report.json. A reconferência com os documentos consolidados usa18-transfers-payload-final, registrada na memória e no respectivo relatório target. Checkpoint1.7.1 preservado. Não houve nova tentativa do uninstaller bloqueado nem mudança de política/registro QA.

## Limites

Projeção preserva slots de camadas não envolvidas, sem guardar proveniência histórica. Um comando oposto pode recuperar pintura/camada sem restaurar intercalação bruta; comparação estrita de dirty continua exigindo auditoria semântica. Undo restaura o Document exato. Mestras, folha interna, posição por alvo e inversão entre camadas continuam pendentes. Estados de menu foram observados no ArtDraw, sem comparação da build executável CorelDRAW.

Inventário363/362 offline,41,50 créditos, indicador interno11,5%, zero equivalente_verificado. Indicador não elevado por correção/estados já incluídos em capacidades básicas. CDR leitura/escrita inexistentes; demais ferramentas, temas, texto, cor e produção permanecem no documento vivo. Distribuição completa depende de assinatura aceita e ensaio de instalação/desinstalação aprovado.

## Histórico1.7.1

# Validação do ArtDraw1.7.1

Windows x64, clean-room. Sem comparação interativa com CorelDRAW e sem leitura/escrita CDR.

| Verificação | Resultado |
| --- | --- |
| cargo check --locked --offline | PASS, zero erros/avisos |
| cargo test --locked --offline |81 PASS, zero falhas |
| release | PASS, opt3/ThinLTO/1 codegen unit |
| --interaction-smoke |107 PASS,126 frames Vello |
| --recovery-smoke |3 PASS |
| --self-test | PASS1.7.1 |
| Desktop externo | CtrlPgUp/Down, ShiftPgUp/Down, estado salvo, SaveAs/XML/reabrir e fechar normal PASS |
| Distribuição | Payload171-slots-payload PASS20 hashes/self-test/registros intactos; ciclo completo segue parcial pelo bloqueio App Control1.6 |

## Correção e falha reproduzida

Dois testes novos reproduziram o defeito na1.7 (0/2 PASS): ordenações sem transferência de camada normalizavam o vetor global e deslocavam slots de camadas alheias, deixando representação/dirty diferente ao voltar à pintura original. Ambos passaram após a correção, junto aos79 anteriores (81 total). Cada teste cobre três pares de comandos: subir/descer, frente/trás da camada e frente/trás da página quando as demais camadas estão bloqueadas. O segundo usa grupo raiz aninhado, Affine e alpha próprios. Verifica seleção por identidade, todos os atributos/arestas, slots alheios e retorno ao Document/encode originais.

Regra matemática: se cada fonte no plano continua associada à camada original, projetar a sequência desejada de cada camada sobre seus slots globais originais. Não canonizar camadas alheias. Planos com transferência entre camadas mantêm a implementação anterior e continuam uma lacuna explícita. Nativo schema3/dependências inalterados. Não é mudança no comportamento visual desejado nem equivalência comparativa com o original.

## UI GPU e desktop

As101 verificações anteriores passaram, mais seis sobre a fixture intercalada: abertura sem normalização/dirty, passo com slots/atributos intactos, comando inverso restaurando documento salvo/histórico, extrema de camada, salvar/reabrir representação exata e inverso restaurando novamente. Total107 checks, não107 ferramentas; Intel Iris Xe/Vulkan1800×1080,126 frames. Captura final inspecionada. Evidências target/release-interaction171.txt/.png/.interleaved.artdraw, release-recovery171.txt/.png e release-self-test171.txt.

Desktop abriu tests/fixtures/reverse-interleaved.artdraw, selecionou Vermelho em área exposta. CtrlPgUp alterou sobreposição e dirty; CtrlPgDown restaurou arte e limpou dirty. ShiftPgUp/Down repetiu frente/trás somente na camada1 sem trocar Página1. Novo CtrlPgUp e SaveAs nativo criou target/desktop-order171.artdraw; XML confirmou raw2,4,1,5,3 e pintura2,1,3,4,5. Objetos completos por ID, camadas, atributos raiz e página2 idênticos à origem. SHA256ee3900b834d0d9b0e64f0368f9da2d568e73f045a04dd98d86d6a67299be727e. Reabertura manteve aparência e documento limpo; fechamento normal, inventário sem janelas ArtDraw QA. Extremos de página sem transferência e grupos novos foram testados no código, sem novo roteiro desktop dedicado.

Exe final SHA256b96ae5f1db7d6c958057bea3b5455c5ae45825e14bb46cf3a8b12b9a4505e14a. Pacote1.7.0 anterior preservado em dist/releases/1.7.0-PARTIAL com seu próprio payload e ZIP aprovados. TESTMODE171-slots-payload PASS20 arquivos idênticos às fontes e self-test1.7.1 do exe copiado; registros intactos, nenhum atalho/uninstaller. Relatório target/payload-171-slots-payload-report.json. A reconferência do pacote com estes documentos consolidados usa RunId171-slots-payload-final e fica registrada na memória e no respectivo relatório target. Não houve nova execução do desinstalador bloqueado.

Inventário363/362 offline,41,50 créditos, indicador interno11,5%, zero equivalente_verificado. Pontuação não foi aumentada por correção de regressão. Ainda longe da meta de100%: CDR, ferramentas/efeitos/texto/cor/UI completos e comparação com build original continuam abertos. Instalação/desinstalação completas permanecem parciais até assinatura aceita e ciclo aprovado.

## Histórico1.7.0

# Validação do ArtDraw1.7.0

Observado em17/10/2026 neste Windows x64. Clean-room. Sem comparação interativa com CorelDRAW e sem leitura/escrita CDR.

| Verificação | Resultado |
| --- | --- |
| cargo check --locked --offline | PASS, zero erros/avisos |
| cargo test --locked --offline |79 PASS, zero falhas |
| release | PASS, opt-level3/ThinLTO/1 codegen unit |
| --interaction-smoke |101 PASS,121 frames Vello |
| --recovery-smoke |3 PASS |
| --self-test | PASS1.7.0 |
| Desktop | Menu/botão, involução, undo/redo, seleção desabilitada, SaveAs/XML/reabrir PASS |
| Distribuição | Payload TESTMODE17-reverse-payload PASS20 hashes/self-test/registros intactos; ciclo completo não homologado por pendência de assinatura/QA1.6 ocupada |

## Inversão e testes de regressão

Cinco testes novos validam inversão de slots selecionados não adjacentes com objeto intermediário intacto; seleção duplicada/índice inválido; involução; grupo raiz aninhado com ordem interna e todos os atributos/Affine conservados; recusa entre camadas sem mutação; camada bloqueada ignorada; zero/uma unidade neutra; recusa de grupo legado intercalado. O quinto teste usa vetor global intercalado entre camadas e confirma Document/encode exatos após duas inversões, preservando slots globais dos objetos não selecionados. Os74 testes anteriores também passaram.

Roteiro GPU conservou as95 verificações anteriores, inclusive PDF, e acrescentou seis verificações: uma unidade não ativa o botão nem adiciona histórico; duas unidades não adjacentes invertem só seus slots; salvar/reler nativo e SVG; undo, redo e undo até o estado salvo. Total101 verificações, não101 ferramentas distintas. Intel Iris Xe/Vulkan1800×1080,121 frames Vello. Captura final inspecionada. Arquivos target/release-interaction17.txt/.png/.svg/.reverse.artdraw; release-recovery17.txt/.png e release-self-test17.txt.

## Desktop externo e persistência

Fixture fixa tests/fixtures/reverse-order.artdraw: duas páginas, três camadas, cinco objetos. Clique em Vermelho mostrou Inverter ordem desabilitado; clique no botão não mudou dirty/arte. Marquee selecionou3 retângulos da camada1 sem incluir amarelo/cinza. Menu Objeto → Ordem → Inverter ordem levou Vermelho acima do Azul/Verde. Botão Inverter ordem restaurou aparência e estado salvo. CtrlZ/Y alternou corretamente e limpou seleção transitória.

CtrlA selecionou4 objetos editáveis em duas camadas, excluindo a camada bloqueada; Inverter ordem desabilitado, conforme limite declarado. Marquee voltou a selecionar3 da camada1 e botão inverteu novamente. CtrlShiftS salvou target/desktop-reverse17.artdraw; XML confirmou IDs3,2,1,4,5, todos os objetos integralmente idênticos à origem por ID, camadas/flags e página2 intactas. Página ativa1. SHA2565f0c93e2b796b6cc8cdb32e848cb062d85fed61018f8fbb930a1ef699ad71bd0. Reabertura no app manteve a sobreposição e nome do arquivo sem dirty. Fechamento normal depois de salvar.

## Revisão e falha intermitente registrada

A revisão encontrou que o plano genérico canonizava a ordem global de objetos entre camadas. Isso poderia deixar um documento válido marcado dirty depois de duas inversões, embora sua pintura voltasse ao original. Corrigido especificamente em Reverse: projetar a sequência invertida de cada camada sobre seus slots globais originais. O novo teste de involução rawDoc/encode passou. A mesma variante dos seis comandos de deslocamento foi mantida como lacuna, sem alegar correção.

Na primeira revalidação de79 testes houve78 PASS e1 FAIL em recovery::live_instance_is_not_offered_as_crash_recovery, na linha de limpeza fs::remove_dir com DirectoryNotEmpty/código145. A asserção de sessão viva já havia passado. A pasta de teste estava vazia na inspeção seguinte. O harness agora repete somente a remoção de pasta vazia por até1s; não remove arquivos nem usa exclusão recursiva e continua falhando se algum conteúdo persistir. Código de recuperação de produção não mudou. Suite79 PASS e três repetições adicionais dos4 testes recovery PASS, log target/recovery-repeat17.txt. O mecanismo de [DeleteFile no Windows](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-deletefilew) permite remoção pendente até fechamento de handles; é compatível com o atraso observado, sem identificar qual handle causou o caso.

## Interrupção e retomada do controle visual

Após correção do interleaving/harness, o release final compilou e passou79 unitários,101 checks de edição GPU/121 frames,3 de recuperação e self-test1.7. Os4 recovery também passaram em3 repetições adicionais. O desktop canônico descrito acima foi homologado antes da revisão; sua ordem resultante permanece coberta pelo roteiro final.

A janela final abriu sem documento. Ao iniciar a abertura da fixture reverse-interleaved.artdraw, Computer Use detectou Escape físico e informou que o usuário interrompeu o controle. Nenhuma outra ação de interface foi enviada. Naquele ponto a prova visual específica de interleaving ficou pendente; seus testes de kernel/rawDoc/encode passaram. Não encerrar à força a janela nem presumir seu estado após a intervenção humana.

Retomada autorizada: abriu tests/fixtures/reverse-interleaved.artdraw. Marquee selecionou3 objetos, botão inverteu a sobreposição, segunda inversão restaurou aparência e limpou dirty, terceira inverteu novamente. SaveAs nativo criou target/desktop-reverse-interleaved17.artdraw; XML confirmou vetor3,4,2,5,1 e pintura3,2,1,4,5. Cada objeto completo por ID, camadas, atributos raiz e página2 ficaram idênticos à origem. SHA256872f74806a0732107c2c8d27468e44e4d898827cf37ed8d64bad284b5ed13bd0. Reabertura manteve aparência/arquivo limpo; fechamento normal e inventário sem janelas QA.

Durante retomada, houve deslocamento de alvo em coordenadas na janela restaurada e pan após menu de sistema; não se identificou causa no produto. Nova observação, maximização pelo botão nativo, normalização de Space e F4 permitiram concluir. Não é evidência de homologação entre monitores/DPI; esse escopo permanece aberto.

O executável final1.7 tem SHA25672cd941c51924e28bfcee23b303127a9774157e928c18a2052731d6eb78c41b0. Reempacotamento atualizado e TESTMODE17-reverse-payload PASS:20 arquivos idênticos às fontes e self-test1.7 do executável copiado PASS. Registros produção/QA inalterados, nenhum atalho/uninstaller criado. Relatório target/payload-17-reverse-payload-report.json. Este relatório consolidado será incluído no pacote final e reconferido com RunId17-reverse-payload-final; não executar o uninstaller bloqueado ou modificar registro QA.

## Limites e distribuição

Inversão somente de duas ou mais unidades da mesma camada, incluindo raízes de grupos inteiros. Entre camadas e edição da ordem interna de uma folha ainda indisponíveis; grupos legados intercalados recusados. Os seis comandos de deslocamento no limite ainda ficam clicáveis e neutros, sem equivalência de estados UI certificada. Schema continua3 e lê1/2; dependências não mudaram.

Não houve nova tentativa de executar o uninstaller bloqueado nem alteração de política Windows. Instalação QA1.6 segue registrada em target/installation qa16-pdf, com documentos preservados no local. Homologação completa da distribuição permanece parcial até assinatura aceita e ciclo próprio aprovado. TESTMODE verifica apenas payload/execução, sem registro/atalhos/uninstaller; a primeira conferência1.7 passou, conforme relatório acima. A conferência do pacote com os documentos consolidados é registrada na memória e em target/payload-17-reverse-payload-final-report.json quando concluída. Detalhes em docs/WINDOWS_DISTRIBUTION.md.

Inventário363/362 offline. 41,50 créditos/362 itens, indicador interno11,5% após promoção conservadora de ORGANIZACAO-022 para parcial; nenhum equivalente_verificado. CDR leitura/escrita continuam inexistentes.1.6 arquivada como PARTIAL,1.4/1.5 com ciclos completos anteriores. Pesquisa e testes internos não certificam fidelidade integral.

## Histórico de validação1.6

# Validação do ArtDraw1.6.0

Observado em17/10/2026 neste Windows x64. Implementação independente; nenhuma comparação interativa com CorelDRAW e nenhum suporte CDR.

| Verificação | Resultado |
| --- | --- |
| cargo check | PASS, zero erros/avisos |
| cargo test |74 PASS, zero falhas |
| release | PASS, opt-level3/ThinLTO/1 codegen unit |
| --interaction-smoke |95 PASS,117 frames Vello |
| --recovery-smoke |3 PASS |
| --self-test | PASS1.6.0 |
| PDF externo | pypdf6.10 strict + Poppler96dpi,3 páginas e inspeção visual PASS |
| PDF versus GPU |6 amostras RGB/transparência/ocultos idênticas no corpus; não certifica todas as bordas |
| Desktop | Exportar doc3 páginas/página2 única, cancelar, nativo sem dirty e fechar normalmente PASS |
| Instalador1.6 | PARCIAL: instalação/registro/atalhos/hashes/self-test PASS; uninstall bloqueado por App Control |

## PDF e regressão

Sete unitários PDF novos somam-se aos67 anteriores: escopo/dimensões sem mutação, xref com offsets exatos, elevação de quadrática, alpha/affine, ocultos, evenodd/texto em curvas/Unicode e falha NaN sem substituir arquivo anterior. Roteiro GPU contém as93 verificações anteriores mais exportação PDF completa/ativa do desenho editado sem alterar nativo. Intel Iris Xe/Vulkan1800×1080. Arquivos target/release-interaction16.*, release-recovery16.* e release-self-test16.txt.

Corpus autoral em output/pdf:3 páginas800×600,400×300 e300×450. Poppler96dpi gerou as três imagens em tmp/pdfs/proof-1..3.png, todas inspecionadas. Parser confirmou dimensões600×450pt,300×225pt e225×337,5pt, xref válido, operações cúbicas/evenodd, alpha fill/contorno independente, ausência de Font/OutputIntents e título Unicode. target/pdf-qa-report.json contém cores observadas; a página vazia é branca. --pdf-smoke capturou o mesmo corpus no Vello; seis amostras coincidiram exatamente.

Desktop externo abriu o.artdraw do corpus, exportou todas as páginas em target/desktop-pdf16.pdf: bytes idênticos ao CLI, SHA2565e1b61ea9843b7766432d75949b3b158b81ac2f93ca1f1b6b434346d1b776976. PageDown navegou à segunda; exportar página ativa gerou target/desktop-pdf16-page2.pdf com1 página300×225pt e título correto, SHA256472faaec1863bb20e75123bbc370a678fcf5c70270c8918d0630b958f7d255dd. Raster dessa saída idêntico à segunda página do corpus. Nova exportação cancelada com Escape; sem dirty, caminho nativo preservado. Fechamento normal confirmado por inventário de janelas vazio para ArtDraw.

## Falha de distribuição1.6

RunId16-pdf instalou normalmente com identidade QA separada, versão1.6.0, atalhos corretos, hashes do payload/documentos e self-test instalado PASS. Depois de criar fixtures de preservação, Start-Process do unins000.exe foi bloqueado pelo Windows App Control. Log CodeIntegrity eventos3033/3077/3089 de19:08:26 registra requisito de assinatura/política0283ac0f-fff1-49ae-ada1-8a933130cad6; Authenticode NotSigned. Desinstalação e preservação após remoção não homologadas. A pasta target/installation qa16-pdf, registro QA e atalhos permanecem; nenhuma política foi alterada nem registro removido manualmente. Ver docs/WINDOWS_DISTRIBUTION.md.

TESTMODE16-final-payload PASS:20 arquivos com hashes iguais às fontes, self-test do executável instalado PASS, registros produção/QA anteriores inalterados e nenhum atalho/desinstalador criado. Relatório target/payload-16-final-payload-report.json. Esse ensaio não substitui o ciclo completo reprovado.1.4/1.5 foram preservadas com suas evidências anteriores; não atribuir esse sucesso ao uninstall1.6.

## Limites1.6

Texto somente em curvas da fonte atual, sem editabilidade/pesquisa/font embedding. Saída DeviceRGB/Normal, sem ICC/CMYK/spot/PDFX/PDFA, presets, seleção/intervalos/lote, segurança, raster ou preflight. Exportação não implica importação PDF. Não há testes comparativos no CorelDRAW, logo nenhuma equivalência. CDR leitura e escrita continuam não implementados. Detalhes em docs/PDF_EXPORT.md e documento vivo gerado.

Inventário363 itens/362 offline,41,25 créditos e indicador interno11,4%; não é percentual certificado de clone.1.4 e1.5 estáveis arquivadas em dist/releases, com instalador/ZIP conferidos. Relatórios anteriores preservados abaixo.

## Histórico de validação1.5

# Validação do ArtDraw1.5.0

Observado em17/10/2026 neste Windows x64. Resultado do subconjunto implementado, sem comparação interativa com CorelDRAW e sem suporte CDR.

| Verificação | Resultado |
| --- | --- |
| cargo check | PASS, zero erros/avisos |
| cargo test |67 PASS, zero falhas |
| release | PASS, opt-level3/ThinLTO/1 codegen unit |
| --interaction-smoke |93 PASS,117 frames Vello |
| --recovery-smoke |3 PASS |
| --self-test | PASS1.5.0 |
| GPU/captura | Intel Iris Xe, Vulkan,1800×1080 |
| Desktop | Sobreposição, passo, camada bloqueada, extrema página/camada, Save As, XML e reabertura PASS |
| Instalador1.5 | PASS: RunId15-order, registro1.5/atalhos/hash do payload e docs/self-test instalado/uninstall/documentos preservados |

## Ordenação homologada

Oito unitários novos cobrem passo dentro da camada, cruzamento que pula bloqueada/oculta, entrada em camada vazia, extrema página versus camada, bloco selecionado sem inversão, grupo aninhado com atributos intactos, limite neutro, recusa de grupo legado intercalado sem mutação e unidade recebida que não cruza outra camada no mesmo comando. Os59 testes anteriores também passaram.

O roteiro GPU abre fixture de2 páginas/3 camadas/5 objetos; verifica CtrlPgUp/PgDown, CtrlHome/End, ShiftPgUp/PgDown, ordem e camada exatas, página ativa conservada, histórico neutro no limite, salvar/reabrir e nove undos até a origem salva. Depois executa todas as80 verificações anteriores de desenho/grupos/folhas/arquivos. Total93 passos, não93 ferramentas diferentes. Arquivos target/release-interaction15.txt/.png/.svg/.order-input.artdraw/.order.artdraw; target/release-recovery15.txt/.png e release-self-test15.txt.

Desktop externo: abriu a fixture de2 páginas. Retângulo vermelho selecionado em área visível; primeiro CtrlPgUp passou vermelho à frente do azul, segundo à frente do verde, terceiro transferiu para Superior, pulando camada2 bloqueada. CtrlEnd restaurou ordem/camada original e removeu dirty; CtrlHome levou à frente da página. ShiftPgDown/Up alterou somente ordem da camada, mantendo Página1.

CtrlShiftS criou target/desktop-order15.artdraw. XML confirmou ordem de IDs2,3,4,5,1, vermelho ID1 na camada3, camada ativa3 e página1 de2. Comparação por ID conservou todos os atributos/geometrias exceto a camada esperada; definições/flags de camadas ficaram idênticas à origem. SHA25644f76d2addd1790bb97cc0e155242fd3f8a923d8bff7eff9894e874c588b9e95. Reabertura manteve sobreposição/camada. Fechamento normal já salvo; nenhuma janela QA restante.

## Limites1.5

Kernel src/ordering.rs calcula o plano antes da mutação e restaura em falha de validação. Schema permanece3. Visibilidade e bloqueio definem destino editável no ArtDraw; essa correspondência precisa comparação com o original. Ordenação de folha isolada dentro de grupo, grupos planos legados intercalados, reversão e posição relativa a alvo ainda ausentes. Botões no limite seguem clicáveis, mas ação não altera conteúdo/histórico. Páginas mestras não implementadas.

Inventário357 itens/356 offline, indicador interno11,3% (40,25 créditos); não é percentual certificado de clone. Nenhuma equivalência verificada. CDR leitura e escrita continuam inexistentes; demais lacunas permanecem no documento vivo.

A versão1.4 foi arquivada em dist/releases/1.4.0 com seu próprio executável/instalador/ZIP/manifesto e passou QA final. O instalador1.5 também passou o ensaio isolado. Logs target/installer-qa15-install.log, installer-qa15-uninstall.log e installed-self-test15.txt; nova execução15-order-final confere o pacote com este relatório consolidado.

## Histórico de validação1.4

# Relatório preservado1.4.0

Observado em17/10/2026 neste Windows x64. Implementação clean-room; comparação interativa com CorelDRAW não realizada. Nenhum item equivalente_verificado.

| Verificação | Resultado |
| --- | --- |
| cargo check --locked --offline | PASS, sem erros/avisos |
| cargo test --locked --offline |59 PASS, zero falhas |
| cargo build --release --locked --offline | PASS, opt-level3/ThinLTO/1 codegen unit |
| --interaction-smoke |80 verificações PASS,98 frames Vello |
| --recovery-smoke |3 verificações PASS |
| --self-test | PASS1.4.0, SVG/Unicode/curvas/seleção |
| GPU e captura | Intel Iris Xe, Vulkan,1800×1080 |
| Desktop externo | Abrir SVG, árvore de2 níveis, folha individual, arrasto sem Ctrl, paleta, Save As, Ctrl+U, separar tudo, undo, salvar/backup/reabrir, fechamento normal |
| Instalador1.4 | PASS: instalação real com identidade QA separada, versão1.4 no registro, hashes do payload/documentos, atalhos, self-test instalado, desinstalação e documentos preservados |

## Casos novos e evidências

Grupos: agrupar unidades existentes mantém subgrupos; agrupar uma unidade não cria nível/histórico. Seleção/marquee/alinhamento/transformação usam raiz e preservam offsets. Descendente bloqueado impede edição parcial involuntária do conjunto. Separar um nível mantém filhos, separar tudo remove árvores selecionadas. Copiar/importar remapeia todos os IDs de grupos, incluindo pais sem folha direta, e objetos; teste com u64::MAX evita colisão após wrap.

Persistência: schema3 aninhado multipágina, fixture schema2 de3 páginas e migração1; ciclos, arestas órfãs, profundidade acima64 e árvores aninhadas entre camadas rejeitados. Grupos planos legados entre camadas permanecem legíveis. SVG exporta wrappers g padrão e metadados ArtDraw; arquivos ativos e nativos são salvos/reabertos no roteiro GPU. Atomicidade/backup e validação pré-save/checkpoint continuam cobertos.

Folhas: Ctrl+arrasto com modificador solto no mesmo frame move apenas a folha; remover fill, copiar avulso e excluir preservam irmãos. F10 converte só o retângulo interno. Arrasto move exatamente nó0 em10 unidades nos eixos, conserva os outros3 nós e todos os irmãos. Undo restaura documento e encerra isolamento transitório.

Painel: índice de adjacência calculado uma vez por quadro. Teste estrutural verifica10mil folhas/5mil grupos, sem duplicações. Não constitui benchmark de latência, render ou memória de uma sessão grande.

Arquivos do release: target/release-interaction14-final.txt/.png/.svg/.nested.artdraw; target/release-recovery14.txt/.png; target/release-self-test14.txt. Os80 passos incluem repetições e não são80 ferramentas distintas.

## Ensaio real com teclado/mouse Windows

Fixture própria tests/fixtures/nested-groups.svg: página800×600, retângulo vermelho e elipse azul no subgrupo69, retângulo verde no pai66. Pelo painel, somente o verde foi selecionado. Arrasto sem Ctrl alterou sua translação para46.86069924949652 /35.61412427924219 e paleta alterou fill para249/115/22/255. Vermelho/azul conservaram identidade, geometria, transformação identidade e cores.

Ctrl+Shift+S criou target/desktop-groups14.artdraw em diálogo nativo. Selecionar conjunto→Ctrl+U removeu raiz66 e conservou subgrupo69 de2 folhas. Ctrl+S gerou backup com a árvore anterior. Undo restaurou raiz; botão Separar todos os níveis removeu todos os grupos, confirmado no XML. Undo/save/reabrir restaurou os dois níveis e a aparência. A janela foi fechada normalmente já salva, sem processo de QA remanescente.

Arquivo final desktop-groups14.artdraw SHA256: 918894a82c5b19321685d703df80b4eaf40e5411b3b008dc81907c6dc336220f. Seu .bak contém3 objetos avulsos. A fixture de origem não foi alterada.

## Falhas detectadas e resolvidas

- Validação nova rejeitava grupo plano entre camadas na fixture SVG anterior (55/56). Corrigida compatibilidade sem alterar fixture; testes voltaram a passar.
- Novo teste do índice assumia4 objetos no demo que contém5 (57/58). Corrigida montagem/contagem;59 testes finais passam.
- Primeiro roteiro de folhas teve2 falhas entre80 passos: cliques fixos não consideravam espelho H anterior da fixture. Alvos agora usam contorno e nó0 transformados; critérios de exclusão/irmãos e movimento exato mantidos. Roteiro final80/80 aprovado.

## Limitações e aproximação

Inventário conhecido354 itens/22 famílias,353 no alvo offline, indicador interno11,3%; não é percentual certificado de cópia. Granularidade e esforço variam. Zero equivalentes verificados. Sem auditoria completa da versão/build original, corpus CDR ou homologação nas duas aplicações. CDR leitura/escrita não implementado.

Grupos limitados64 níveis; sem seleção progressiva intermediária, reparentamento por drag, Focus mode ou ordenação interna. Texto interno implementado mas sem cenário dedicado; flags agregados V/B e cadeias extensas precisam mais UI QA. ArtDraw1.3 não lê schema3; arquivos anteriores são preservados em backup ao sobrescrever. Outros editores podem descartar metadados SVG; grupos legados intercalados podem virar vários wrappers externos para conservar z-order.

Booleanos/offsets, fills avançados, fontes/shaping, raster/efeitos, mestras/visão simultânea, ICC/CMYK/PDF e demais lacunas continuam na matriz. Não foram repetidas interativamente todas as opções do instalador/splash1.2, que não mudaram; a receita e o payload1.4 passaram QA isolado de instalação/desinstalação.

QA do instalador: RunId14-groups aprovado. Logs target/installer-qa14-install.log e installer-qa14-uninstall.log; target/installed-self-test14.txt. Diretório de QA foi desinstalado e manteve somente os documentos de teste previstos. Nova execução com RunId14-groups-final valida o pacote com este relatório consolidado.
