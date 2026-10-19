# Contribuir com o ArtDraw

Obrigado por considerar participar. O ArtDraw está em fase experimental e precisa de olhos, ideias e testes tanto quanto de código. Não é necessário saber Rust para abrir uma issue útil.

## Escolha um começo pequeno

- Relate um bug com passos, versão, ambiente e um arquivo próprio mínimo.
- Melhore um texto ou documente uma tarefa na interface.
- Estude uma lacuna da [matriz](docs/FEATURE_MATRIX.csv) e proponha critérios de aceite.
- Implemente uma correção ou um fluxo completo. Para alterações grandes, abra uma issue antes para alinhar escopo e evitar trabalho duplicado.

Não é necessário ter acesso ao CorelDRAW para contribuir. Quem o tiver deve informar edição/build/plataforma e usar conteúdo próprio/autorizado. Sem essa comparação, a capacidade continua parcial/básica; testes internos não provam equivalência.

## Preparar o ambiente

Siga [BUILDING.md](docs/BUILDING.md). Faça um fork, clone seu fork e crie uma branch descritiva:

```powershell
git switch -c melhoria/descricao-curta
cargo fetch --locked
.\build.ps1 -Mode Check -Offline
```

## Regra de memória e documento vivo

Leia [AGENTS.md](AGENTS.md) e [ARCHITECTURE_MEMORY.md](ARCHITECTURE_MEMORY.md). Antes de cada arquivo autoral, registre intenção na memória; depois registre o resultado. Há helpers no repositório para não depender de ferramentas locais:

```javascript
// Exemplo executado com Node.js, na raiz do projeto:
const {edit} = require('./scripts/tracked.cjs');
edit('docs/USAGE.md', texto => texto + '\nUma explicação útil.\n', 'Explicar um fluxo verificado');
```

Sem Node, use scripts/write-tracked.ps1 com -Path, -Content e -Description. A memória pode ser mantida manualmente seguindo o mesmo contrato. Preserve mapa, decisões matemáticas, UI, evidências e limitações ao resumi-la.

Antes de implementar um recurso, leia sua linha em docs/FEATURE_MATRIX.csv, a lacuna em docs/CORELDRAW_GAP_LEDGER.md e a referência correspondente. Atualize estado, aceite, evidência e lacuna conforme o que realmente mudou. Descobertas independentes ganham linhas próprias; pendências difíceis não desaparecem para elevar a pontuação.

```powershell
.\scripts\update-parity.ps1
.\scripts\update-parity.ps1 -Check
```

O gerador mantém os três documentos de paridade e o bloco de progresso do README. Não edite essas saídas manualmente. Uma correção de documentação não exige inventar uma capacidade nova na matriz.

## Implementação e verificação

O fluxo esperado é UI → modelo → histórico → render → persistência. Considere seleção, bloqueio, cancelamento, undo/redo, erro e salvar/reabrir. Preserve documentos de origem; operações recusadas não devem perder dados.

```powershell
.\build.ps1 -Mode Check -Offline
.\build.ps1 -Mode Test -Offline
```

Quando alterar interface/render/interação, teste também na aplicação e anote Windows, GPU/driver, passos, resultado e limites. Os roteiros GPU estão em [BUILDING.md](docs/BUILDING.md). Uma nova captura não substitui os testes de dados; um teste unitário não substitui verificar usabilidade. Não crie testes que apenas repitam a implementação sem critério independente.

Atualize VALIDATION.md com resultados reais; preserve regressões e falhas. Nunca declare suporte CDR, exportação sem perdas ou equivalência somente porque um arquivo foi gerado. Siga [CDR_INTEROPERABILITY.md](docs/CDR_INTEROPERABILITY.md).

## Enviar a pull request

1. Descreva o problema, o comportamento resultante e a issue/ID da matriz.
2. Inclua como foi testado e as limitações restantes; capturas quando úteis.
3. Inclua somente fontes/assets necessários. Não envie target, dist, .tools, logs, credenciais ou documentos pessoais.
4. Execute o gerador com -Check. O CI Windows verifica clone, compilação e testes unitários; GPU e instalação precisam de evidência separada.

PRs pequenas e completas são mais fáceis de revisar. Documentação, testes manuais e estudos também são contribuições válidas. Use uma linguagem respeitosa: critique o comportamento ou a solução, sem atacar quem contribui.

## Clean-room e licenças

Envie apenas código original ou dependências com licença compatível e atribuição. Não copie código nem ativos proprietários da Corel. Não publique documentos de clientes; use fixtures próprias ou explicitamente autorizadas. Ao enviar uma contribuição, você concorda que seu código original seja distribuído sob a licença MIT do projeto; material de terceiros deve indicar sua licença e origem.

Problemas de instalação/assinatura são registrados em docs/WINDOWS_DISTRIBUTION.md. Não proponha desativar proteções do Windows para fazer um teste passar.
