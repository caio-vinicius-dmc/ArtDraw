# Hierarquia de grupos — ArtDraw1.4

Referência de comportamento: [CorelDRAW2026 Windows, Group objects](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html). Implementação matemática própria. Não descreve o motor interno proprietário.

- Ctrl+G agrupa duas ou mais unidades. Grupos existentes tornam-se filhos; os atributos e Affine das folhas são preservados. Todos os membros adotam uma camada e o conjunto vai à frente. Agrupar uma unidade é neutro, inclusive no histórico.
- Ctrl+U, alias Ctrl+Shift+G, ou Separar remove somente o nível externo. Subgrupos sobrevivem. Separar todos os níveis no painel/Objeto remove toda a árvore selecionada. Undo/redo usa snapshots completos.
- Clique e seleção por área trabalham com a raiz. Um membro oculto/bloqueado impede seleção parcial involuntária do conjunto. Transformar/alinha/distribuir usa limites coletivos da raiz, aplicando uma matriz a cada folha.
- O painel mostra a árvore expansível, controles de visibilidade/bloqueio dos descendentes e seleção externa. Ctrl+clique no canvas ou clique na folha da árvore isola esse objeto. Mover, redimensionar, cor, texto, nós e excluir operam só na folha. Selecionar conjunto encerra o escopo e volta à raiz. Não percorre subgrupos intermediários por Ctrl+cliques sucessivos; Focus mode e arrastar para mudar pai continuam pendentes na matriz.
- Cópia interna captura objetos e arestas. Colar, duplicar e importar remapeiam todos os grupos, incluindo pais sem folhas diretas, e atribuem IDs de objeto novos. Pode copiar entre páginas. Uma folha isolada é copiada/duplicada sem grupo. Importação preserva estilos/estados; duplicação torna cópias visíveis e desbloqueadas, na camada ativa.

Ordenação interna de folhas ainda não está disponível; o comando informa essa limitação e não altera a árvore. Desagrupar exige seleção do conjunto. O modo individual é transitório, identificado pelo ID persistente da folha; navegar/abrir/undo encerra esse escopo. F10 exige uma folha para evitar converter um membro arbitrário do conjunto.

## Persistência e limites

Schema nativo3 guarda arestas filho→pai por página; raízes implícitas, grupo imediato em cada Object. Lê1/2 como árvores planas sem modificar IDs/geometria. ArtDraw1.3 não lê3; backup conserva o original ao sobrescrever. Máximo64 níveis,50mil objetos e50mil arestas por página,1000 páginas,32MiB de arquivo. O limite global de objetos continua50mil.

Ciclos, IDs de grupo zero, arestas sem descendentes e árvores aninhadas entre camadas são recusados. Grupos planos legados entre camadas são aceitos para não quebrar arquivos anteriores. Agrupar novo reúne numa camada.

SVG exporta wrappers g padrão aninhados com transformações nas folhas e metadados ArtDraw de arestas/IDs/camadas/estados. Importa g externo aninhado com herança de estilo/transformação. Grupos legados intercalados na ordem de pintura podem gerar wrappers separados: aparência e identidade ArtDraw são preservadas, mas sua estrutura em outro editor pode diferir. Texto é exportado em contornos; outros editores podem descartar metadados editáveis. SVG não oferece garantia de roundtrip CorelDRAW. CDR continua sem leitor/escritor.

## Verificação

Unitários: criação/não reagrupamento, separar nível/tudo, cópia entre páginas, independência de IDs, bloqueio de descendente, alinhamento/transformação, ciclos/arestas órfãs/profundidade/camadas, schema3 e fixture schema2. Regressão encontrada e corrigida: validação nova rejeitava grupos planos legados entre camadas; a fixture anterior foi mantida. Resultados finais e ensaio da janela em VALIDATION.md.

O painel usa um índice de filhos/descendentes por quadro, com teste estrutural de10mil folhas. Isso não substitui benchmark de interação/renderização de documentos grandes, ainda pendente.
