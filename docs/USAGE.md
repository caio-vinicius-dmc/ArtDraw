# Usar o ArtDraw1.9

Versão experimental. Abra arquivos próprios ou cópias; salve o documento completo em .artdraw. CDR ainda não é suportado. Download e instalação estão no [README](../README.md).

## Ferramentas e navegação

| Atalho | Ação |
| --- | --- |
| V | Selecionar, mover e redimensionar pelos cantos |
| F6/R; F7/E | Retângulo; elipse |
| P | Caneta: clique cria canto, arrasto cria alças Bézier |
| Enter/direito; primeiro nó | Terminar caminho aberto; fechar caminho |
| F10 | Editar nós/alças; duplo clique divide a curva; Delete remove nó |
| F8/T | Texto simples em editor modal; duplo clique reabre |
| Y; S; I; L; F5 | Polígono; estrela; espiral; linha; desenho livre |
| Shift+clique; Ctrl+A | Alternar unidade; selecionar editáveis |
| Ctrl+G; Ctrl+U | Agrupar; separar um nível |
| Ctrl+C/X/V; Ctrl+D | Clipboard interno; duplicar |
| H; Espaço+arrasto; botão central | Pan |
| Ctrl+roda; F4 | Zoom no ponteiro; ajustar página |
| Ctrl+N/O/I | Novo; abrir; importar |
| Ctrl+S; Ctrl+Shift+S | Salvar; salvar como |
| Ctrl+Z/Y | Desfazer/refazer |
| PageUp/PageDown | Página anterior/próxima |
| Escape | Cancelar gesto/estado transitório; seleção depende do modo |

Marquee esquerda→direita exige conter a caixa; direita→esquerda permite cruzamento. Shift restringe proporção/eixo; na caneta limita a direção a45 graus e na Forma espelha a alça. Paleta: esquerdo fill, direito stroke, primeira amostra remove cor. Sem seleção, define estilo futuro.

Camadas/objetos têm visibilidade e bloqueio. O painel permite transformações numéricas, alinhamentos e distribuição por centros. Grupos contam como unidades; Ctrl+clique ou folha da árvore isola um objeto. A seleção progressiva entre subgrupos ainda falta. Veja [hierarquia](GROUP_HIERARCHY.md) e [ordem/atalhos/limites](STACKING_ORDER.md).

Quatro temas próprios ficam em Exibir; não são cópias homologadas dos temas CorelDRAW. Ctrl+roda/pan e a barra inferior navegam o documento. O asterisco no título indica alterações. Fechar/novo/abrir oferece salvar; não encerre o processo à força.

## Formatos e preservação

.artdraw XMLv3 guarda todas as páginas/hierarquia e lêv1/v2. Save valida, grava atomicamente e mantém backup .bak ao sobrescrever. Recuperação por sessão em %LOCALAPPDATA%/ArtDraw/Recovery, com checkpoints após gestos/intervalo15s; não garante cada alteração antes do checkpoint.

SVG importa primitivas/paths/grupos/affine/estilos inline. Exporta página ativa, texto em curvas e metadados próprios para reedição no ArtDraw. Outros editores podem descartar metadados. Gradientes, raster, filtros/máscaras/recortes, CSS global e use não estão implementados. Texto externo é adaptado para a fonte incorporada; não há fidelidade tipográfica ampla. Salvar documento multipágina como SVG é recusado; exporte página ativa e mantenha o nativo.

PDF exporta documento ou página ativa, com RGB/alpha e texto em curvas. Sem ICC/CMYK/spot, fontes editáveis, PDF/X/A, importação, seleção/intervalos/lote ou impressão profissional. Detalhes em [PDF_EXPORT.md](PDF_EXPORT.md).

Texto usa Ubuntu Light incorporada. Ainda não há fontes do sistema, shaping complexo, edição direta por trechos ou layout profissional. F10 edita primitivas convertidas/caminho simples; texto e caminhos compostos não têm edição individual completa de nós.

## Falhas e relatos

Em erro de inicialização, o log pode estar em %TEMP%/ArtDraw-error.log. Relate versão, Windows, GPU/driver e passos mínimos; remova caminhos pessoais antes de publicar logs. [Abrir issue](https://github.com/caio-vinicius-dmc/ArtDraw/issues/new/choose). A lista completa de lacunas está em [CORELDRAW_GAP_LEDGER.md](CORELDRAW_GAP_LEDGER.md).
