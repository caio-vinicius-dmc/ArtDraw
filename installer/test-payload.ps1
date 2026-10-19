param([ValidatePattern('^[a-zA-Z0-9-]+$')][string]$RunId='17-reverse-payload')
$ErrorActionPreference='Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
& ./scripts/update-parity.ps1 -Check
$memory=Get-Content -Raw ARCHITECTURE_MEMORY.md
Add-Content ARCHITECTURE_MEMORY.md ("- QA payload TESTMODE sem registro/uninstall: "+$RunId+"; gerar árvore target e relatório.")
$root=[IO.Path]::GetFullPath($PWD.Path)
$dest=[IO.Path]::GetFullPath((Join-Path $root ('target/payload qa'+$RunId)))
$allowed=[IO.Path]::GetFullPath((Join-Path $root 'target'))+[IO.Path]::DirectorySeparatorChar
if(!$dest.StartsWith($allowed,[StringComparison]::OrdinalIgnoreCase)){throw 'Destino fora de target'}
if(Test-Path -LiteralPath $dest){throw 'Destino já existe; use RunId inédito'}
$group='ArtDraw Payload '+$RunId
$menu=Join-Path ([Environment]::GetFolderPath('Programs')) $group
if(Test-Path -LiteralPath $menu){throw 'Grupo já existe; use RunId inédito'}
$keys=@('HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\{D9281462-263E-4BCE-90CA-567A9FA921BD}_is1','HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\{D9281462-263E-4BCE-90CA-567A9FA921BD}-QA_is1')
function Registry-Snapshot {
    @($keys | ForEach-Object {
        if(Test-Path -LiteralPath $_){$r=Get-ItemProperty -LiteralPath $_;[ordered]@{Key=$_;Version=$r.DisplayVersion;Location=$r.InstallLocation;Uninstall=$r.UninstallString}}else{[ordered]@{Key=$_;Absent=$true}}
    }) | ConvertTo-Json -Depth 4 -Compress
}
$before=Registry-Snapshot
$log=Join-Path $root ('target/payload-'+$RunId+'-install.log')
$p=Start-Process (Join-Path $root 'dist/ArtDraw-Setup.exe') -ArgumentList '/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART','/TESTMODE=1','/TASKS=',('/GROUP="'+$group+'"'),('/DIR="'+$dest+'"'),('/LOG="'+$log+'"') -PassThru -Wait -WindowStyle Hidden
if($p.ExitCode -ne 0){throw "Instalação TESTMODE falhou: $($p.ExitCode)"}
if((Registry-Snapshot) -cne $before){throw 'Registro anterior alterado indevidamente'}
if(Test-Path -LiteralPath $menu){throw 'TESTMODE criou atalhos'}
if(Get-ChildItem -LiteralPath $dest -Filter 'unins*' -File){throw 'TESTMODE criou desinstalador'}
$entries=@(@('target/release/coreldraw_clone.exe','coreldraw_clone.exe'),@('README.md','README.md'),@('AGENTS.md','AGENTS.md'),@('VALIDATION.md','VALIDATION.md'),@('CHANGELOG.md','CHANGELOG.md'),@('LICENSE','LICENSE'),@('installer/TERMS.txt','TERMS.txt'),@('dist/THIRD_PARTY_NOTICES.txt','THIRD_PARTY_NOTICES.txt'))
foreach($f in (Get-ChildItem docs -File | Where-Object {$_.Extension -in @('.md','.csv')})){$entries+=,@(('docs/'+$f.Name),('docs/'+$f.Name))}
foreach($e in $entries){if((Get-FileHash -LiteralPath (Join-Path $dest $e[1])).Hash -ne (Get-FileHash -LiteralPath $e[0]).Hash){throw ('Hash divergente: '+$e[1])}}
$selftest=Join-Path $root ('target/payload-'+$RunId+'-selftest.txt')
$p=Start-Process (Join-Path $dest 'coreldraw_clone.exe') -ArgumentList '--self-test',('"'+$selftest+'"') -PassThru -Wait -WindowStyle Hidden
if($p.ExitCode -ne 0 -or !(Get-Content -Raw -LiteralPath $selftest).StartsWith('PASS')){throw 'Self-test do payload falhou'}
if((Registry-Snapshot) -cne $before){throw 'Registro alterado após self-test'}
$report=[ordered]@{Result='PASS';RunId=$RunId;Mode='TESTMODE payload only';Files=$entries.Count;ExeHash=(Get-FileHash (Join-Path $dest 'coreldraw_clone.exe')).Hash;RegistryUnchanged=$true;NoShortcuts=$true;NoUninstaller=$true;SelfTest=$true;FullInstallationValidated=$false;UninstallValidated=$false;Destination=$dest}
$report | ConvertTo-Json | Set-Content -Encoding utf8 -LiteralPath ('target/payload-'+$RunId+'-report.json')
$memory=Get-Content -Raw ARCHITECTURE_MEMORY.md
Add-Content ARCHITECTURE_MEMORY.md ('- QA payload PASS: '+$entries.Count+' arquivos, self-test, registros inalterados; não testa uninstall. '+$RunId)
$report | ConvertTo-Json
