param([switch]$Check)
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
Set-Location $root
$rows=@(Import-Csv -LiteralPath docs/FEATURE_MATRIX.csv)
if(!$rows.Count){throw 'Matriz vazia'}
$weights=@{baseline=0.5;implementado_basico=0.5;parcial=0.25;implementado_a_validar=0;pendente=0;referencia_a_validar=0;fora_do_alvo_offline=0;equivalente_verificado=1}
$seen=[Collections.Generic.HashSet[string]]::new()
foreach($r in $rows){
    if(!$seen.Add($r.id)){throw "ID duplicado: $($r.id)"}
    foreach($field in @('id','familia','recurso','estado','marco','aceite','evidencia','fonte','lacuna')){if([string]::IsNullOrWhiteSpace($r.$field)){throw "Campo ausente: $($r.id).$field"}}
    if(!$weights.ContainsKey($r.estado)){throw "Estado inválido: $($r.estado)"}
}
$groups=@($rows | Group-Object familia)
$counts=@{};foreach($state in $weights.Keys){$counts[$state]=@($rows | Where-Object estado -eq $state).Count}
$target=@($rows | Where-Object estado -ne 'fora_do_alvo_offline').Count
$credits=0.0;foreach($r in $rows){$credits+=$weights[$r.estado]}
$score=if($target){100.0*$credits/$target}else{0}
$culture=[Globalization.CultureInfo]::GetCultureInfo('pt-BR')
$scoreText=$score.ToString('0.0',$culture)
$creditText=$credits.ToString('0.00',$culture)
$hash=(Get-FileHash -LiteralPath docs/FEATURE_MATRIX.csv -Algorithm SHA256).Hash.ToLowerInvariant()
function Publish([string]$Path,[Collections.Generic.List[string]]$Lines){
    $content=($Lines -join [char]10)+[char]10
    if($Check){
        if(!(Test-Path -LiteralPath $Path) -or [IO.File]::ReadAllText((Join-Path $root $Path)) -cne $content){throw "Documento desatualizado: $Path. Execute scripts/update-parity.ps1."}
    }else{
        $memory=Get-Content -LiteralPath ARCHITECTURE_MEMORY.md -Raw
        Add-Content -LiteralPath ARCHITECTURE_MEMORY.md -Value ("- SINCRONIZAR: "+$Path+" a partir da matriz; memória lida antes da gravação.")
        [IO.File]::WriteAllText((Join-Path $root $Path),$content,[Text.UTF8Encoding]::new($false))
        $memory=Get-Content -LiteralPath ARCHITECTURE_MEMORY.md -Raw
        Add-Content -LiteralPath ARCHITECTURE_MEMORY.md -Value ("- CONCLUÍDO: "+$Path+"; matriz SHA256 "+$hash)
    }
}
$ledger=[Collections.Generic.List[string]]::new()
$ledger.Add('# Documento vivo — o que falta para a fidelidade ao CorelDRAW')
$ledger.Add('')
$ledger.Add('Referência de pesquisa: CorelDRAW2026 desktop Windows, documentação consultada em17/10/2026. Build executável e comparação integral ainda pendentes. O inventário conhecido não é uma auditoria exaustiva de tudo o que existe no produto; novas descobertas devem ser acrescentadas.')
$ledger.Add('')
$ledger.Add("Inventário: $($rows.Count) itens, $($groups.Count) famílias; $target no alvo offline. Indicador interno: **$scoreText%**. Equivalentes verificados: **$($counts.equivalente_verificado)**. Todos os itens sem equivalência comparativa continuam com trabalho em aberto, inclusive os básicos.")
$ledger.Add('')
$ledger.Add('A meta inclui funções, ferramentas, gestos, atalhos, foco, cancelamento, descoberta, temas, disposição/medidas de painéis e preservação de documentos. Presets próprios não demonstram temas idênticos. O motor interno proprietário não é conhecido; a comparação deve avaliar comportamento e saída observáveis.')
$ledger.Add('')
$ledger.Add('## Regra extraordinária de atualização')
$ledger.Add('')
$ledger.Add('Leia este documento antes de implementar. A cada descoberta/implementação/teste, atualize estado, aceite, evidência e lacuna em FEATURE_MATRIX.csv; regenere com scripts/update-parity.ps1. Antes de encerrar a interação ou empacotar, rode o mesmo comando com -Check. Regras persistentes em ../AGENTS.md; memória obrigatória em ../ARCHITECTURE_MEMORY.md.')
$ledger.Add('')
$ledger.Add('Este arquivo é gerado. Não editar manualmente. Para detalhar um item, altere sua lacuna/aceite na matriz. Para CDR, veja [plano e critérios](CDR_INTEROPERABILITY.md). Para testes/pesos, veja [protocolo](VALIDATION_PROTOCOL.md).')
$ledger.Add('')
$ledger.Add('## Resumo por estado')
$ledger.Add('')
$ledger.Add('| Estado | Itens |')
$ledger.Add('| --- | ---: |')
foreach($state in @('equivalente_verificado','baseline','implementado_basico','parcial','implementado_a_validar','pendente','referencia_a_validar','fora_do_alvo_offline')){$ledger.Add("| $state | $($counts[$state]) |")}
$ledger.Add('')
$ledger.Add('## Prioridades e dependências')
$ledger.Add('')
$ledger.Add('1. Completar modelo durável: páginas mestras, reparentamento de grupos, recursos/estilos e identidades em comandos/histórico. Páginas independentes e hierarquia/folhas têm implementação básica nas1.3/1.4.')
$ledger.Add('2. Compatibilidade CDR: corpus por versão, leitor, gravador e testes nos dois sentidos; nenhuma versão homologada hoje. A pesquisa começa em paralelo ao modelo, sem chamar pesquisa de suporte.')
$ledger.Add('3. Geometria e aparência: booleanos/offsets, nós tipados, fills/contornos/recortes e efeitos editáveis.')
$ledger.Add('4. Texto/imagens/cor/produção: fontes/shaping/layout, raster/vetorizar, ICC/CMYK/spot, PDF e impressão.')
$ledger.Add('5. Fidelidade de UI/UX: inventariar cada diálogo, flyout, estado, atalho, tema e fluxo na build de referência, comparando cenas e medidas. Auditar todas as famílias, não apenas criar ferramentas novas.')
$ledger.Add('')
foreach($g in $groups){
    $ledger.Add("## $($g.Name) — $($g.Count) itens")
    $ledger.Add('')
    foreach($r in $g.Group){
        $ledger.Add("### $($r.id) — $($r.recurso)")
        $ledger.Add('')
        $ledger.Add("- Estado: **$($r.estado)**; marco: $($r.marco).")
        $ledger.Add("- Aceite: $($r.aceite)")
        $ledger.Add("- Evidência atual: $($r.evidencia)")
        $ledger.Add("- **Falta / diferença:** $($r.lacuna)")
        $ledger.Add("- Referência: $($r.fonte)")
        $ledger.Add('')
    }
}
$ledger.Add("Matriz de origem SHA256: $hash")
Publish 'docs/CORELDRAW_GAP_LEDGER.md' $ledger
$index=[Collections.Generic.List[string]]::new()
$index.Add('# Matriz de capacidades')
$index.Add('')
$index.Add("Inventário conhecido: $($rows.Count) itens em $($groups.Count) famílias. Dados em [FEATURE_MATRIX.csv](FEATURE_MATRIX.csv); detalhes e lacunas em [documento vivo](CORELDRAW_GAP_LEDGER.md). Contagens podem se sobrepor; não medem equivalência.")
$index.Add('')
$index.Add('Estados: baseline/básico representam subconjuntos; parcial é incompleto; implementado_a_validar carece de teste específico; pendente não está implementado; referencia_a_validar precisa confirmar escopo/edição; fora_do_alvo_offline permanece registrado. equivalente_verificado exige comparação com a build de referência, critérios e corpus. Nenhum estado básico significa fidelidade integral.')
$index.Add('')
$index.Add('| Família | Itens |')
$index.Add('| --- | ---: |')
foreach($g in $groups){$index.Add("| $($g.Name) | $($g.Count) |")}
$index.Add('')
$index.Add('Fontes Mac são conceituais e não comprovam gestos Windows; entradas ArtDraw de infraestrutura não revelam internals Corel. Arquivo gerado por scripts/update-parity.ps1; alterar a matriz e regenerar.')
$index.Add('')
$index.Add("Matriz de origem SHA256: $hash")
Publish 'docs/FEATURE_MATRIX.md' $index
$status=[Collections.Generic.List[string]]::new()
$status.Add('# Indicador de aproximação — ArtDraw')
$status.Add('')
$status.Add("**Indicador interno conservador: $scoreText%**. Base: $($rows.Count) itens conhecidos/$($groups.Count) famílias, $target no alvo offline. Isso não certifica esse percentual de equivalência ao CorelDRAW. Ainda estamos longe de100%; não houve comparação interativa completa com a referência.")
$status.Add('')
$status.Add('| Estado | Itens | Peso |')
$status.Add('| --- | ---: | ---: |')
foreach($state in @('baseline','implementado_basico','parcial','implementado_a_validar','pendente','referencia_a_validar','equivalente_verificado','fora_do_alvo_offline')){$weight=if($state -eq 'fora_do_alvo_offline'){'excluído'}else{$weights[$state].ToString($culture)};$status.Add("| $state | $($counts[$state]) | $weight |")}
$status.Add('')
$status.Add("Cálculo: $creditText créditos / $target itens ×100 = $scoreText%. Granularidade/esforço variam e capacidades podem se sobrepor. Pesos fixos no [protocolo](VALIDATION_PROTOCOL.md). A versão1.1 tinha9,3% pelas mesmas regras.")
$status.Add('')
$status.Add('A versão1.9 acrescenta comandos à frente/atrás de alvo por clique com bloco de unidades, transferência de camada e cancelamento seguro.93 unitários/139 checks GPU e desktop próprio passaram; não há comparação CorelDRAW. Folha interna, mestras, legados intercalados/multicamadas, inversão entre camadas e semântica de dirty após transferências opostas continuam pendentes. Ver STACKING_ORDER.md e VALIDATION.md.')
$status.Add('')
$status.Add('A versão1.6 exporta documento/página ativa em PDF1.4 vetorial RGB com texto em curvas. Parser strict, raster Poppler, amostras GPU e desktop passaram no corpus próprio; fontes editáveis, ICC/CMYK, PDF/X/A, presets, seleção/intervalos/lote e segurança permanecem pendentes. Ver PDF_EXPORT.md.')
$status.Add('')
$status.Add('A versão1.5 distingue frente/trás de página/camada e passo com transferência entre camadas editáveis, preservando grupos e histórico. Inversão entre camadas e ordenar folhas internas ainda faltam; alvo por clique recebeu subconjunto1.9 e estados no limite implementação1.8, sem equivalência comparativa. Detalhes em STACKING_ORDER.md.')
$status.Add('')
$status.Add('A versão1.4 acrescenta grupos aninhados, edição de folha, separação por nível/tudo, cópia/importação de árvore e schema3 com leitura1/2, sobre as páginas independentes da1.3. Continuam pendentes reparentamento/foco/seleção progressiva, visão simultânea/mestras, booleanos/offsets, fills avançados, tipografia, raster/efeitos, ICC/CMYK/PDF profissional e CDR bidirecional. Testes do subconjunto atual não eliminam essas lacunas.')
$status.Add('')
$status.Add('Próximo marco: organização interna de grupos/recursos, completar páginas, kernel geométrico e ampliar pesquisa/corpus CDR. Leitor/gravador CDR e qualquer alegação sem perdas exigem o [protocolo CDR](CDR_INTEROPERABILITY.md). Lista detalhada em [documento vivo](CORELDRAW_GAP_LEDGER.md).')
$status.Add('')
$status.Add("Gerado por scripts/update-parity.ps1. Matriz SHA256: $hash")
Publish 'docs/PARITY_STATUS.md' $status

# Only this marked block is generated; the rest of README remains authored.
$readmePath=Join-Path $root 'README.md'
$readme=[IO.File]::ReadAllText($readmePath)
$start='<!-- parity:start -->';$end='<!-- parity:end -->'
if(([regex]::Matches($readme,[regex]::Escape($start))).Count -ne 1 -or ([regex]::Matches($readme,[regex]::Escape($end))).Count -ne 1){throw 'README deve conter um par único de marcadores parity:start/end'}
$first=$readme.IndexOf($start);$last=$readme.IndexOf($end)
if($last -lt $first){throw 'Marcadores de progresso invertidos'}
$remainingText=(100.0-$score).ToString('0.0',$culture)
$unverified=$target-$counts.equivalente_verificado
$progress=@(
    $start,
    '| Medida | Estado atual |',
    '| --- | ---: |',
    "| Indicador interno de maturidade | **$scoreText%** |",
    "| Restante no indicador (não é estimativa de esforço) | **$remainingText%** |",
    "| Capacidades inventariadas | $($rows.Count) em $($groups.Count) famílias |",
    "| Itens pendentes / implementados a validar / escopo a confirmar | $($counts.pendente) / $($counts.implementado_a_validar) / $($counts.referencia_a_validar) |",
    "| Equivalência comparativa comprovada | **$($counts.equivalente_verificado)** |",
    "| Itens do alvo offline ainda sem equivalência comprovada | **$unverified de $target** |",
    $end
) -join [char]10
$updatedReadme=$readme.Substring(0,$first)+$progress+$readme.Substring($last+$end.Length)
if($Check){
    if($updatedReadme -cne $readme){throw 'Progresso do README desatualizado; execute scripts/update-parity.ps1'}
}elseif($updatedReadme -cne $readme){
    $memory=Get-Content -LiteralPath ARCHITECTURE_MEMORY.md -Raw
    Add-Content ARCHITECTURE_MEMORY.md '- SINCRONIZAR: bloco de progresso README a partir da matriz, mantendo texto autoral.'
    [IO.File]::WriteAllText($readmePath,$updatedReadme,[Text.UTF8Encoding]::new($false))
    $memory=Get-Content -LiteralPath ARCHITECTURE_MEMORY.md -Raw
    Add-Content ARCHITECTURE_MEMORY.md '- CONCLUÍDO: bloco de progresso README sincronizado.'
}
"PASS: $($rows.Count) IDs únicos, $($groups.Count) famílias, lacunas obrigatórias; documentos $(if($Check){'sincronizados'}else{'gerados'}); indicador $scoreText%."
