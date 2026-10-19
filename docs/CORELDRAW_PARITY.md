# Referência e paridade funcional — ArtDraw

Referência consultada em 17/10/2026: CorelDRAW Graphics Suite 2026 para Windows e atualização de setembro de 2026. Este é um inventário de engenharia versionado, não uma afirmação de cobertura completa ou de paridade alcançada. O manual possui opções, exceções e combinações que exigem cenários de teste adicionais. Não calcular “percentual de clone” contando botões.

## Evidência e fronteiras

A página oficial confirma o produto 2026 e novidades de setembro, incluindo geração de vetores e revisão do PowerTRACE. Recursos variam entre assinatura, licença permanente, plataforma e aplicativos da suíte. O alvo principal é o editor CorelDRAW desktop Windows; PHOTO-PAINT, Font Manager, CAPTURE e serviços Web são trilhas separadas. Não foram copiados binários, código, ícones, fontes comerciais ou conteúdo da Corel. Não foi inspecionada uma sessão executável do original. Descrições abaixo são especificações independentes baseadas em documentação pública. [Produto oficial](https://www.coreldraw.com/en/product/coreldraw/).

## Superfícies e operação

A caixa de ferramentas organiza famílias em menus expansíveis; a barra de propriedades acompanha a ferramenta/seleção. Dockers reúnem propriedades e estrutura do documento. Página, área de trabalho, réguas, paletas, navegação e barra de estado compõem o fluxo principal. A referência documenta personalização das ferramentas; no ArtDraw, menus devem anunciar somente comandos executáveis. Catálogo nominal de ferramentas na matriz anexa, sem reutilizar os pictogramas originais. [Toolbox](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Toolbox.html).

Aparência deve ter presets claro/escuro, contraste legível em todos os painéis, escala DPI, ícones vetoriais próprios, atalhos configuráveis e preferência persistida. Os presets ArtDraw não implicam correspondência visual exata aos temas Corel. Tamanho/forma/cor dos nós e feedback de controle precisam ser configuráveis; a referência distingue tipos de nós. [Nós e alças](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Options-for-nodes-handles-previews.html).

## Áreas de capacidade e lacunas

| Área | Comportamento a especificar/testar | Baseline ArtDraw 1.0 |
| --- | --- | --- |
| Seleção e transformação | Seleção coletiva, hierarquia, pivô, escala, giro, inclinação, espelho, repetição e precisão numérica | Um objeto, mover e escala por cantos |
| Organização | Grupos aninhados, edição interna, ordem, visibilidade, bloqueio, camadas e páginas mestras | Lista plana e ordem individual |
| Geometria | Caminhos abertos/compostos, ferramentas paramétricas, tipos de nó, divisão/junção, operações booleanas e offsets | Retângulo, elipse, caneta, F10 e divisão de cúbica |
| Aparência | Preenchimento uniforme, degradês, padrões, malhas; contornos detalhados; transparência e estilos | RGBA uniforme, contorno simples e evenodd |
| Texto | Seleção direta, fontes, shaping, estilos por trecho, parágrafos, colunas, fluxo, caminho, tabelas e substituição | Texto artístico em editor com Ubuntu Light |
| Layout | Documentos multipágina, unidades físicas, guias, snapping, grade, margens, sangria e imposição | Uma página ajustável, régua e grade visual |
| Efeitos | Sombras, contornos, mistura, envelope, distorção, extrusão, perspectiva, simetria e recorte editável | Não implementados |
| Raster | Imagens incorporadas/vinculadas, recorte, cor, reamostragem, filtros e vetorização | Não implementados |
| Produção | PDF, imagens, lotes, seleção/páginas, impressão, ICC/CMYK/spot, prova, separações e preflight | SVG básico |
| Infraestrutura | Salvamento sem perda, migrações, recovery, histórico, clipboard interoperável, busca, plugins, acessibilidade e desempenho | SVG e snapshots de histórico |

Grupos devem preservar atributos dos filhos e comportar níveis internos. Alinhamento exige referência explícita; distribuição precisa definir centros ou intervalos e tratar grupos como unidades. O primeiro incremento implementa grupos simples, alinhamento pela caixa coletiva/página e distribuição por centros: essas diferenças ficam visíveis na matriz. [Grupos](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Group-objects.html), [alinhamento](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Align-and-distribute-objects.html).

Camadas precisam separar visibilidade, edição e saída, além de hierarquia de páginas. Layout requer presets, orientação e regras de publicação próprias. Implementar ocultar/bloquear não basta para declarar todo o sistema de camadas concluído. [Camadas](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Layers.html), [layout](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Page-layout.html).

Preenchimentos devem ser recursos estruturados e editáveis; uma cor média não substitui um degradê. O mesmo vale para transparência, malhas e padrões. [Fills](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Fills.html). Operações de curva, fechamento, corte, booleans e recortes requerem precisão e preservação de estilo/histórico. [Shape objects](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Shape-objects.html).

Tipografia profissional exige resolução de fontes, agrupamento de caracteres, kerning, ligaduras, scripts complexos, bidi e texto por parágrafos. Desenhar um glifo por caractere, como no baseline, não fornece esses comportamentos. Os recursos OpenType precisam depender das tabelas da fonte e ser testados com fixtures livres. [OpenType](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-OpenType-features.html) — fonte conceitual Mac; confirmar disposição e gestos no Windows.

Snapping deve ter tolerância em pixels da tela e prioridade determinística, com indicação do alvo e desativação temporária. Simetria precisa manter relação editável entre matriz e instâncias; duplicar espelhado não equivale a simetria viva. [Snap](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Snap-objects.html), [simetria](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Create-symmetry.html).

Importação e exportação exigem uma matriz por formato, direção e versão. “Abrir arquivo” não prova preservação de fontes, clipping, cores e efeitos. SVG básico já é suportado. CDR bidirecional sem perdas é requisito obrigatório do usuário e tem trilha própria de pesquisa, representação, leitura, escrita e homologação desde agora, sem alegar compatibilidade antes de validação. Ver [critérios CDR](CDR_INTEROPERABILITY.md) e [documento vivo](CORELDRAW_GAP_LEDGER.md). [Formatos](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Supported-file-formats.html), [exportação](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-files.html).

Cor para produção é uma trilha própria: perfis, políticas, intenções, prova e conversão de espaço não podem ser simulados por simples fórmula RGB–CMYK. A referência documenta Microsoft ICM como CMM padrão, sem tornar público todo o motor de renderização. [Gestão de cor](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-management.html), [conversão](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Color-conversion-settings.html). PDF e preflight devem verificar a saída efetiva, não apenas produzir um arquivo com essa extensão. [PDF](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Export-documents-as-PDF-files.html), [preflight](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-PDF-preflight-summaries.html).

Vetorização demanda comparação com bitmap, parâmetros de simplificação, cores, estrutura e repetição. O equivalente ArtDraw deverá ter implementação e avaliação próprias; não usar o nome PowerTRACE para um algoritmo diferente. [Trace](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-Trace.html). Recursos recentes de IA/Cloud são mapeados, mas o requisito offline exige modelos locais ou alternativas locais; serviços da Corel não serão invocados nem imitados com botões sem função. [Atualização setembro 2026](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Mac/CorelDRAW-en/CorelDRAW-CorelDRAW-Graphics-Suite-September-2026-Update.html).

## Motor: o que sabemos e a arquitetura proposta

O comportamento documentado inclui objetos vetoriais, texto, raster, transformações, efeitos, cor e saída. Isso não revela estruturas internas, algoritmos, APIs gráficas ou código proprietário. Não há evidência nesta análise para afirmar “Corel usa Vello/Skia” ou reproduzir seu motor exatamente.

A arquitetura ArtDraw proposta, de nossa autoria:

1. Modelo de documento com IDs estáveis, páginas/camadas, árvore de grupos, estilos e recursos compartilhados. A migração de índices planos para IDs deve preceder animação de histórico, referências de efeitos e múltiplas páginas.
2. Comandos transacionais para uma ação por gesto, histórico com orçamento de memória, redo e recuperação. Snapshots atuais continuam válidos no incremento, mas não escalam para imagens grandes.
3. Kernel geométrico f64 para curvas, interseções, offsets, booleanos, tesselação e transformação. Tolerâncias dependem da escala e devem ter testes de degeneração/autointerseção.
4. Shaping de texto com fonte livre/licenciada e pipeline bidi/linha/layout; cache por fonte, texto e estilo.
5. Grafo de renderização com preenchimentos, imagens, clipping, composição e efeitos. Vello/wgpu permanece o backend GPU; cenas e recursos devem ser cacheados e possuir caminho de tratamento de falha.
6. Serviço de cor separado, com perfis e saída gerenciada. Não interpretar a codificação sRGB da textura como gerenciamento ICC completo.
7. Adaptadores de formatos com avisos explícitos de perda, fixtures e testes de ida/volta. Um formato nativo versionado será necessário para recursos que SVG não representa integralmente.
8. Serviços Windows para clipboard, fontes, impressão, diálogo, instalação, DPI e acessibilidade. Plugins e serviços online ficam desacoplados.

## Uso profissional: cenários de aceite

- Identidade visual: importar SVG, montar marca com formas, alinhar, agrupar, aplicar cores, salvar/reabrir e exportar sem perder estrutura suportada.
- Material impresso: unidades físicas, páginas, texto, imagens e sangria; publicar PDF e validar dimensões/perfis/fontes. Pendente até a trilha de impressão.
- Sinalização/corte: curvas fechadas, escala física, união/diferença, contorno e exportação adequada; validar continuidade e tamanho. Pendente kernel booleano/saída.
- Diagrama técnico: conectores, cotas, snapping, referências e atualização após mover. Pendente conectores/cotas.
- Ilustração: desenho livre, nós, estilos, grupos, clipping, efeitos e transformação. Cobertura parcial, registrada por item.

Cada recurso só muda para validado após teste de geometria/persistência e, quando interativo, evidência por eventos de UI no executável. A matriz não representa uma auditoria completa de todas as versões, licenças e plugins do produto original.

Temas: a [orientação oficial de aparência](https://learn.corel.com/tips/how-to-change-the-coreldraw-ui-color/) foi consultada; não foi obtido neste levantamento o tópico completo2026 Windows de personalização. Confirmar presets e opções por edição antes de alegar correspondência exata. A [automação oficial](https://help.coreldraw.com/CorelDRAW/540111192/Documentation-Windows/CorelDRAW-en/CorelDRAW-Use-macros-and-scripts-to-automate-tasks.html) descreve VBA/VSTA/JavaScript; o ArtDraw ainda não possui integração equivalente.
