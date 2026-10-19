# Regras obrigatórias do ArtDraw

## Memória antes de qualquer arquivo
Antes de criar ou modificar cada arquivo autoral, leia ARCHITECTURE_MEMORY.md, registre a intenção e salve. Após a alteração, registre o resultado. Use scripts/tracked.cjs ou scripts/write-tracked.ps1 (as cópias locais antigas em .tools/ também funcionam). Não codifique com a memória desatualizada. Preserve mapa, matemática, UI e decisões ao resumir.

## Regra extraordinária do documento de lacunas
A meta do usuário é fidelidade funcional, visual e de uso ao CorelDRAW Windows, incluindo temas e troca bidirecional CDR sem perdas. Essa meta permanece aberta até evidência comparativa suficiente.

1. Antes de iniciar um recurso, leia docs/CORELDRAW_GAP_LEDGER.md, sua linha em docs/FEATURE_MATRIX.csv e a referência correspondente. Use as lacunas para escolher o próximo trabalho.
2. A cada recurso descoberto, iniciado, alterado ou testado, atualize a matriz: estado, aceite, evidência e lacuna restante. Acrescente detalhes/variantes como novas linhas quando forem capacidades independentes; não esconda casos difíceis nem remova pendências para elevar o indicador.
3. Execute scripts/update-parity.ps1 após atualizar a matriz. Ele sincroniza o documento de lacunas, índice e indicador. Nunca edite as três saídas geradas nem o bloco entre <!-- parity:start --> e <!-- parity:end --> do README manualmente; o script sincroniza esse bloco também.
4. Antes de encerrar uma interação, compilar uma entrega ou empacotar, execute scripts/update-parity.ps1 -Check. Se falhar, sincronize e corrija a causa. Atualize VALIDATION.md e ARCHITECTURE_MEMORY.md com os resultados reais.
5. Todos os estados, inclusive básico/baseline/parcial, conservam uma lacuna até comparação. Nenhum recurso é equivalente_verificado só porque existe um botão ou um smoke passa. A equivalência exige versão/build/plataforma, fixture, passos, critérios visuais/semânticos e evidência do CorelDRAW e ArtDraw.
6. Registre regressões e testes reprovados. Não confunda pesquisa ou documentação com implementação. Reporte indicador interno e suas limitações em cada entrega; não afirme 100% sem auditoria de todo o alvo.
7. CDR leitura e escrita são requisitos obrigatórios separados, ainda pendentes. Siga docs/CDR_INTEROPERABILITY.md. Não renomeie SVG/XML como CDR; não anuncie compatibilidade sem abrir, editar, salvar e reabrir nas duas aplicações. Preservar aparência, estrutura e editabilidade são critérios separados. Dados desconhecidos exigem preservação/relatório e não podem desaparecer silenciosamente.
8. Implemente de forma clean-room com código próprio e dependências licenciadas. A marca ArtDraw/logomarca do usuário é própria; não copie código ou ativos proprietários da Corel.
9. Não encerre à força janelas que possam conter trabalho não salvo. Use fechamento normal e salvamento. O usuário autorizou desenvolvimento/testes/empacotamento autônomos; não crie confirmações adicionais para trabalho já autorizado.

## Fonte de verdade e comandos
- Dados: docs/FEATURE_MATRIX.csv.
- Documento vivo: docs/CORELDRAW_GAP_LEDGER.md.
- Critérios: docs/VALIDATION_PROTOCOL.md e docs/CDR_INTEROPERABILITY.md.
- Gerar: ./scripts/update-parity.ps1.
- Conferir: ./scripts/update-parity.ps1 -Check.
- Código: ./build.ps1 -Mode Check|Test|Release.
- Pacote: ./package.ps1.
- QA isolado: ./installer/test-installer.ps1 -RunId <identificador-inedito>.
