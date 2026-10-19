param([ValidatePattern('^[a-zA-Z0-9-]+$')][string]$RunId='17-reverse')
$ErrorActionPreference='Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
& ./scripts/update-parity.ps1 -Check
$memory=Get-Content -Raw ARCHITECTURE_MEMORY.md
Add-Content ARCHITECTURE_MEMORY.md '- QA instalador: compilar identidade separada, instalar em target, validar payload/registro e desinstalar preservando documentos.'
$root=[IO.Path]::GetFullPath($PWD.Path)
$taskInstall=[IO.Path]::GetFullPath((Join-Path $root ("target/installation qa"+$RunId)))
$allowed=[IO.Path]::GetFullPath((Join-Path $root 'target'))+[IO.Path]::DirectorySeparatorChar
if(!$taskInstall.StartsWith($allowed,[StringComparison]::OrdinalIgnoreCase)){throw 'Destino fora de target'}
if(Test-Path -LiteralPath $taskInstall){throw 'Pasta de QA já existe; não sobrescrever'}
$taskGroup="ArtDraw Homologacao"+$RunId
$taskMenu=Join-Path ([Environment]::GetFolderPath('Programs')) $taskGroup
if(Test-Path -LiteralPath $taskMenu){throw 'Grupo do menu QA já existe; não alterar atalhos existentes'}
$key='HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\{D9281462-263E-4BCE-90CA-567A9FA921BD}-QA_is1'
if(Test-Path -LiteralPath $key){throw 'Identidade QA já registrada; não alterar instalação existente'}
& .tools/inno/ISCC.exe /Qp /DQA=1 /Otarget /FArtDraw-QA-Setup installer/ArtDraw.iss
if($LASTEXITCODE -ne 0){throw 'Compilação QA falhou'}
$p=Start-Process (Join-Path $root 'target/ArtDraw-QA-Setup.exe') -ArgumentList '/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART','/TASKS=',('/GROUP="'+$taskGroup+'"'),('/DIR="'+$taskInstall+'"'),('/LOG="'+$root+'/target/installer-qa17-install.log"') -PassThru -Wait -WindowStyle Hidden
if($p.ExitCode -ne 0){throw "Instalação falhou: $($p.ExitCode)"}
$verificationFailure=$null
$uninstallFailure=$null
try {
    foreach($name in @('ArtDraw.lnk','Desinstalar ArtDraw.lnk')){if(!(Test-Path -LiteralPath (Join-Path $taskMenu $name))){throw "Atalho ausente: $name"}}
    $shortcut=(New-Object -ComObject WScript.Shell).CreateShortcut((Join-Path $taskMenu 'ArtDraw.lnk'))
    if($shortcut.TargetPath -ne (Join-Path $taskInstall 'coreldraw_clone.exe')){throw 'Destino do atalho incorreto'}
    if((Get-ItemProperty -LiteralPath $key).DisplayVersion -ne '1.9.0'){throw 'Registro de versão incorreto'}
    if((Get-FileHash -LiteralPath (Join-Path $taskInstall 'coreldraw_clone.exe')).Hash -ne (Get-FileHash target/release/coreldraw_clone.exe).Hash){throw 'Payload divergente'}
    foreach($name in @('VALIDATION.md','CHANGELOG.md','AGENTS.md')){if(!(Test-Path -LiteralPath (Join-Path $taskInstall $name))){throw "Relatório ausente: $name"}}
    foreach($name in @('CORELDRAW_PARITY.md','FEATURE_MATRIX.csv','VALIDATION_PROTOCOL.md','CORELDRAW_GAP_LEDGER.md','CDR_INTEROPERABILITY.md')){if(!(Test-Path -LiteralPath (Join-Path $taskInstall ('docs/'+$name)))){throw "Documentação ausente: $name"}}
    foreach($name in @('FEATURE_MATRIX.csv','CORELDRAW_GAP_LEDGER.md','PARITY_STATUS.md','CDR_INTEROPERABILITY.md')){
        if((Get-FileHash -LiteralPath (Join-Path $taskInstall ('docs/'+$name))).Hash -ne (Get-FileHash -LiteralPath ('docs/'+$name)).Hash){throw "Documento desatualizado instalado: $name"}
    }
    $p=Start-Process (Join-Path $taskInstall 'coreldraw_clone.exe') -ArgumentList '--self-test',($root+'/target/installed-self-test17.txt') -PassThru -Wait -WindowStyle Hidden
    if($p.ExitCode -ne 0){throw 'Autoteste instalado falhou'}
    Set-Content -LiteralPath (Join-Path $taskInstall 'keep-me.svg') -Value '<svg xmlns="http://www.w3.org/2000/svg"/>'
    Set-Content -LiteralPath (Join-Path $taskInstall 'docs/user-notes.txt') -Value 'Documento do usuário para validar preservação'
} catch {
    $verificationFailure=$_
} finally {
    try {
    $p=Start-Process (Join-Path $taskInstall 'unins000.exe') -ArgumentList '/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART',('/LOG="'+$root+'/target/installer-qa17-uninstall.log"') -PassThru -Wait -WindowStyle Hidden
    if($p.ExitCode -ne 0){throw 'Desinstalação falhou'}
    } catch {$uninstallFailure=$_}
}
$summary=[ordered]@{RunId=$RunId;InstallSucceeded=$true;VerificationSucceeded=($null -eq $verificationFailure);UninstallProcessSucceeded=($null -eq $uninstallFailure);VerificationError=$(if($verificationFailure){$verificationFailure.Exception.Message}else{$null});UninstallError=$(if($uninstallFailure){$uninstallFailure.Exception.Message}else{$null});PostUninstallChecksSucceeded=$false}
$summary | ConvertTo-Json | Set-Content -Encoding utf8 -LiteralPath ('target/installer-'+$RunId+'-report.json')
if($verificationFailure){if($uninstallFailure){Write-Warning ('Falha adicional ao desinstalar: '+$uninstallFailure.Exception.Message)};throw $verificationFailure}
if($uninstallFailure){throw $uninstallFailure}
Start-Sleep -Seconds 2
if(Test-Path -LiteralPath $key){throw 'Registro QA não foi removido'}
if(Test-Path -LiteralPath $taskMenu){throw 'Atalhos/grupo QA não foram removidos'}
if(Test-Path -LiteralPath (Join-Path $taskInstall 'coreldraw_clone.exe')){throw 'Aplicativo não removido'}
if(!(Test-Path -LiteralPath (Join-Path $taskInstall 'keep-me.svg'))){throw 'Documento SVG não preservado'}
if(!(Test-Path -LiteralPath (Join-Path $taskInstall 'docs/user-notes.txt'))){throw 'Anotações não preservadas'}
$summary.PostUninstallChecksSucceeded=$true
$summary | ConvertTo-Json | Set-Content -Encoding utf8 -LiteralPath ('target/installer-'+$RunId+'-report.json')
$memory=Get-Content -Raw ARCHITECTURE_MEMORY.md
Add-Content ARCHITECTURE_MEMORY.md '- QA instalador PASS: versão1.9 no registro, hash do payload, self-test instalado, desinstalação/registro e preservação de documentos.'
'PASS: instalação normal com identidade QA isolada, versão, payload, self-test, desinstalação, registro e documentos preservados'
