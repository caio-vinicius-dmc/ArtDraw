# Compatibilidade CDR — requisito de preservação bidirecional

Estado em17/10/2026: leitura CDR pendente, escrita CDR pendente, versões CDR homologadas: nenhuma. O formato nativo ArtDraw XML3 (com leitura de XML1/2) não é CDR. A meta solicitada é abrir um arquivo feito no CorelDRAW, editar no ArtDraw e reabrir no CorelDRAW preservando aparência, estrutura e edição, além do caminho inverso.

## Referência e fatos verificados

Referência principal: CorelDRAW2026 para Windows; build exata e corpus comparativo ainda não registrados. A ajuda oficial descreve CDR como formato padrão, escolha de versão, incorporação de fontes condicionada às permissões e opções de preservação para versões anteriores. Isso obriga testes por versão e recurso. [Corel: salvar desenhos](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Save-drawings.html).

A libcdr se apresenta como biblioteca de leitura/conversão de arquivos CorelDRAW, sob MPL2.0, com dependências nativas. É uma candidata a investigação para importação; a descrição pública consultada não demonstra um gravador CDR nem compatibilidade integral com o CorelDRAW2026. Nenhuma integração foi feita no ArtDraw. [README oficial no espelho LibreOffice](https://github.com/LibreOffice/libcdr).

Não há evidência nesta etapa de uma especificação pública completa de todas as versões/recursos CDR. Isso é uma lacuna da pesquisa, não prova de impossibilidade. Uma biblioteca que abre alguns arquivos não satisfaz o requisito de ida e volta sem perdas.

## Subtarefas de FORMATOS-004 e FORMATOS-005

| ID | Trabalho e critério | Estado |
| --- | --- | --- |
| CDR-R01 | Identificar contêiner/versão e rejeitar estruturas inválidas/truncadas com limites de recursos | Pendente |
| CDR-R02 | Mapear páginas, dimensões/unidades, camadas mestras, ordem, grupos aninhados, IDs e vínculos | Pendente |
| CDR-R03 | Preservar curvas, primitivas paramétricas, tipos de nós, transformações e precisão | Pendente |
| CDR-R04 | Texto artístico/parágrafo, trechos, shaping/fontes, layout e permissões de incorporação | Pendente |
| CDR-R05 | Degradês, malhas, padrões/texturas, contornos, transparências e modos de composição | Pendente |
| CDR-R06 | Bitmaps, vínculos, perfis ICC, CMYK/spot, efeitos, clipping e símbolos | Pendente |
| CDR-R07 | Recursos compartilhados, metadados, previews e registros desconhecidos sem descarte silencioso | Pendente |
| CDR-W01 | Especificar gravador real por versão e validar estrutura com leitor independente e CorelDRAW | Pendente |
| CDR-W02 | Serializar modelo completo e recursos; salvar de forma atômica, mantendo original e backup | Pendente |
| CDR-W03 | Política de versão anterior: identificar exatamente quais recursos mudam e impedir alegação de preservação integral nesses casos | Pendente |
| CDR-Q01 | Corpus próprio/autorizado com build, versão do formato, fontes/perfis/imagens e licença/origem | Pendente |
| CDR-Q02 | CorelDRAW → CDR → ArtDraw → editar/salvar → CorelDRAW → editar/salvar → ArtDraw | Pendente |
| CDR-Q03 | ArtDraw → CDR → CorelDRAW → editar/salvar → ArtDraw; comparar com origem e histórico | Pendente |
| CDR-Q04 | Arquivos vazios/grandes, Unicode, múltiplas páginas, grupos profundos, duplicatas, dados corrompidos e cancelamento | Pendente |
| CDR-Q05 | Comparação visual e semântica com tolerâncias explícitas por recurso; todas as perdas catalogadas | Pendente |

Estas subtarefas detalham duas linhas da matriz e não acrescentam créditos ao indicador.

## O que significa sem perdas

Comparar aparência por página em resolução/perfil fixados, com tolerância documentada e inspeção dos desvios. Comparar também árvore/ordem de objetos, geometria, texto/estilos/fontes, recursos, cores/perfis, efeitos e vínculos. Os objetos devem continuar editáveis no tipo original; achatar em bitmap ou converter todo texto/efeito em curvas não atende à preservação integral. Igualdade de bytes não é necessária quando as aplicações reserializam o documento, mas diferenças semânticas precisam ser explicadas.

Registros desconhecidos podem ser retidos para investigação e preservação, mas copiar blocos brutos não prova que referências/offsets continuam válidos após edição. O escritor precisa atualizar as relações ou recusar o salvamento com diagnóstico, preservando o arquivo de origem.

## Sequência técnica

1. Preparar corpus e fixar CorelDRAW/build/versão CDR; não declarar suporte genérico a todas as gerações.
2. Completar o modelo ArtDraw para representar dados que hoje nem cabem nele: páginas/hierarquia, texto/estilos, recursos, cor e efeitos. Importar descartando esses dados violaria a meta.
3. Avaliar leitores abertos licenciados e/ou parser independente, com testes de limites. Separar parser de adaptação ao modelo e relatório de perdas.
4. Implementar leitura e escrita como adaptadores separados. Uma ponte opcional que dependa de CorelDRAW instalado não equivale a suporte nativo independente e precisa ser identificada como tal.
5. Homologar versões/subconjuntos com resultados explícitos, ampliando apenas quando os testes sustentarem. O requisito final completo continua pendente enquanto houver recurso perdido ou não comparado.

## Evidência exigida por fixture

Identificador, origem/licença, CorelDRAW versão/build/plataforma, versão CDR, SHA256 de entrada/saídas, fontes/perfis/recursos, operações executadas, árvore e propriedades esperadas, capturas/renders, diferenças observadas, logs de import/export e resultado de edição no consumidor. Nenhuma fixture CDR comparativa foi homologada nesta entrega.

## Pesquisa e preparação na1.3

O modelo ArtDraw agora possui páginas com IDs estáveis, nomes/dimensões e conteúdo/camadas isolados; schema2 conserva todas e migra schema1. Isso prepara parte de CDR-R02, sem implementar leitura/escrita CDR. Grupos aninhados, mestras, recursos, texto/cor/efeitos continuam faltando.

Foi inspecionada a [API pública CDRDocument](https://raw.githubusercontent.com/LibreOffice/libcdr/master/inc/libcdr/CDRDocument.h): oferece detecção via isSupported e leitura via parse para uma RVNGDrawingInterface. Essa superfície pode orientar um adaptador de importação; não demonstra um gravador nem preservação de toda a semântica. O [README](https://raw.githubusercontent.com/LibreOffice/libcdr/master/README) confirma a finalidade de leitura/conversão e dependências Boost/ICU/lcms2/librevenge/zlib. Nenhum código da biblioteca foi incorporado nesta etapa.

Próxima investigação: mapear os eventos/propriedades emitidos pela interface para o modelo sem achatar tipos editáveis; identificar perdas por recurso e preparar corpus autorizado com build/versão definidos. Não existe fixture CDR homologada, adaptador integrado ou escrita CDR nesta versão.

## Preparação estrutural1.4

Páginas independentes e árvore de grupos por arestas já existem no modelo e no schema3. A edição de uma folha preserva identidade/estilos/Affine e irmãos; copiar um conjunto remapeia pais e filhos. Isso remove parte de uma limitação do modelo, sem implementar leitura ou escrita CDR. Ainda faltam estilos/recursos compartilhados, efeitos, texto profissional, raster, gestão de cor e representações de recursos desconhecidos, entre outras capacidades. Nenhuma versão CDR foi homologada.

## Pesquisa de envelopes e recursos1.6

O [fluxo público CDRDocument](https://raw.githubusercontent.com/LibreOffice/libcdr/master/src/lib/CDRDocument.cpp) distingue cabeçalhos RIFF/CDR e família antiga Waldo. Contêiner estruturado pode fornecer content/riffData.cdr ou content/root.dat; o segundo caminho pode listar streams externos em content/dataFileList.dat e content/data/. Perfis aparecem em color/profiles/cmyk/ e rgb/. Isso é pesquisa em biblioteca aberta MPL2.0, sem incorporar código ou integrar leitor. Não basta procurar a extensão ou descompactar o primeiro stream.

Próximo adaptador deve separar sondagem de envelope com limites, preservação de bytes/streams desconhecidos, resolução de referências, coleta semântica e adaptação ao modelo. Nenhuma dessas camadas está implementada. Antes de converter, inventariar recursos e produzir relatório de perdas; sem representação fiel, recusar salvar CDR e conservar origem. CDR-R01/R07 e leitura/escrita permanecem pendentes; não há fixture autorizada homologada nas duas aplicações.
