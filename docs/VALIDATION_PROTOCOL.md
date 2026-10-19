# Protocolo de aproximação e homologação

A meta é equivalência funcional observável, com implementação independente. Contar controles visíveis não demonstra essa equivalência. A documentação pública orienta o inventário; testes comparativos com o original ainda precisam de uma instalação/corpus autorizado e versão fixada.

## Ciclo por capacidade

1. Registrar gatilho, estado inicial, parâmetros, atalhos, mouse, referência oficial e edição/plataforma.
2. Definir resultado geométrico/documental e resultado visível, inclusive erros e cancelamento.
3. Implementar uma operação completa: UI → modelo → histórico → render → salvar/reabrir/exportar.
4. Testar geometria/dados e casos degenerados; verificar que falhas não destroem o documento anterior.
5. Abrir a aplicação real e exercitar gestos. Os modos de smoke usam eventos egui e renderização GPU na janela; testes manuais no desktop complementam esses modos.
6. Comparar com o original usando o mesmo arquivo/ação quando houver acesso. Guardar versão, fixture, resultado e captura. Sem comparação não classificar como equivalente.
7. Atualizar matriz, limitações, relatório e memória; compilar o release e testar o pacote.

## Pacote mínimo de cenários por família

| Família | Cenários de regressão |
| --- | --- |
| Seleção/transformação | Shift alterna unidade, marquee nas duas direções, sobreposição, bloqueio, grupo, pivô/escala, undo/redo e persistência |
| Organização | Camada ativa, ordem de pintura, esconder/bloquear sem apagar, copiar sem colisão de identidade, salvar/reabrir |
| Curvas | Cúbicas com extremos, tangências, nós coincidentes, subcontornos, fechamento, alças, topologia e precisão após várias transformações |
| Aparência | Fill/stroke/alpha, zoom, bordas, ordem de composição, mistura e saída em consumidor externo |
| Texto | Acentos, quebras, fontes ausentes, ligaduras/bidi, seleção/edição, métricas, curvas e PDF |
| Documento | Salvar novo, sobrescrever com backup, caminho com espaços/Unicode, arquivo bloqueado/inválido, versão futura, encerramento inesperado e recuperação |
| UI/UX | Descoberta, estados habilitados, foco, teclado, cancelar, DPI, redimensionamento, temas/contraste e feedback de operações demoradas |
| Instalação | Abertura com marca, licença bloqueia avanço, pasta/atalhos/resumo, progresso real, payload correto, iniciar/desinstalar e preservar documentos |
| Produção | Tamanho físico, ICC/CMYK/spot, sangria, fontes/imagens, PDF/preflight e comparação da saída |
| Desempenho |10/100/1mil/10mil objetos, importação, hit-test, pan/zoom, memória, latência de gesto e recuperação de GPU |

## O que o indicador significa

Há três medidas separadas: presença de implementação, cobertura de testes e equivalência comparativa. Não somar essas medidas como se fossem a mesma coisa. Infraestrutura ArtDraw também aparece na matriz e não prova comportamento interno do CorelDRAW.

Para acompanhar incrementos usaremos um **indicador interno conservador**, com regras fixas: baseline/implementado_basico=0,5; parcial=0,25; pendente/implementado_a_validar/referencia_a_validar=0; equivalente_verificado=1. Serviços fora do alvo offline saem do denominador. A pontuação é a soma dividida pelo número de itens no alvo. Ela mede maturidade do inventário atual, não um percentual certificado de clone; granularidade e esforço dos itens variam. Os pesos deliberadamente reservam metade do crédito dos recursos básicos à equivalência ainda não demonstrada.

O progresso não poderá ser declarado100% sem critérios/corpus por recurso e comparação de todos os fluxos exigidos com a versão de referência. O inventário pode crescer conforme exceções e recursos forem encontrados. Perdas conhecidas não podem ser escondidas por arredondamento ou por adicionar itens fáceis.
