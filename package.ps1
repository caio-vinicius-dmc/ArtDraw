$ErrorActionPreference='Stop'
Set-Location $PSScriptRoot
& (Join-Path $PSScriptRoot 'scripts/update-parity.ps1') -Check
& .\build.ps1 -Mode Environment
function Artifact([string]$Path,[scriptblock]$Action) {
    $memory=Get-Content -Raw ARCHITECTURE_MEMORY.md
    Add-Content ARCHITECTURE_MEMORY.md "`n- EMPACOTAMENTO: gerar $Path." -Encoding utf8
    & $Action
    if($LASTEXITCODE -and $LASTEXITCODE -ne 0){throw "Falha ao gerar $Path"}
    $memory=Get-Content -Raw ARCHITECTURE_MEMORY.md
    Add-Content ARCHITECTURE_MEMORY.md "- CONCLUÍDO: $Path." -Encoding utf8
}
if(!(Test-Path target/release/coreldraw_clone.exe)){throw 'Compile o release antes de empacotar.'}
New-Item -ItemType Directory -Force dist/ArtDraw-Portable | Out-Null
$metadata=(& cargo metadata --locked --offline --format-version 1 --filter-platform x86_64-pc-windows-gnu | ConvertFrom-Json)
if($LASTEXITCODE -ne 0){throw 'Falha ao consultar dependências.'}
$nodes=@{};foreach($node in $metadata.resolve.nodes){$nodes[$node.id]=$node}
$ids=[Collections.Generic.HashSet[string]]::new()
function Visit([string]$id){if($ids.Add($id)){foreach($dep in $nodes[$id].dependencies){Visit $dep}}}
Visit $metadata.resolve.root
$notices=[Text.StringBuilder]::new()
[void]$notices.AppendLine('ArtDraw 1.9.0 — Third-party software and font notices')
[void]$notices.AppendLine('Resolved dependency licenses, copied from the distributed crate sources. Rust runtime: MIT OR Apache-2.0. Native Windows libraries are supplied by the OS.')
foreach($pkg in ($metadata.packages | Where-Object {$ids.Contains($_.id) -and $_.name -ne 'coreldraw_clone'} | Sort-Object name)) {
    [void]$notices.AppendLine("`n========================================`n$($pkg.name) $($pkg.version)`nSPDX: $($pkg.license)`nSource: $($pkg.repository)")
    $folder=Split-Path $pkg.manifest_path -Parent
    $licenses=@(Get-ChildItem -LiteralPath $folder -File | Where-Object {$_.Name -match '^(LICEN[CS]E|COPYING|NOTICE|UNLICENSE)'})
    if($pkg.name -eq 'epaint_default_fonts'){$licenses+=Get-ChildItem -LiteralPath (Join-Path $folder 'fonts') -File | Where-Object {$_.Extension -eq '.txt'}}
    if($pkg.license_file){$extra=Join-Path $folder $pkg.license_file;if(Test-Path -LiteralPath $extra){$licenses+=Get-Item -LiteralPath $extra}}
    foreach($file in ($licenses | Sort-Object FullName -Unique)){[void]$notices.AppendLine("--- $($file.Name) ---");[void]$notices.AppendLine([IO.File]::ReadAllText($file.FullName))}
}
$inno=Join-Path $PSScriptRoot '.tools/inno/ISCC.exe'
if(!(Test-Path -LiteralPath $inno)){$inno=(Get-Command ISCC.exe -ErrorAction Stop).Source}
[void]$notices.AppendLine("`nInno Setup — installer engine, Jordan Russell and Martijn Laan; https://jrsoftware.org/")
[void]$notices.AppendLine([IO.File]::ReadAllText((Join-Path (Split-Path $inno -Parent) 'License.txt')))
Artifact 'dist/THIRD_PARTY_NOTICES.txt' {[IO.File]::WriteAllText((Join-Path $PWD 'dist/THIRD_PARTY_NOTICES.txt'),$notices.ToString(),[Text.UTF8Encoding]::new($false))}
Artifact 'dist/ArtDraw-Setup.exe' {& $inno '/Qp' 'installer/ArtDraw.iss';if($LASTEXITCODE -ne 0){throw 'Falha no instalador Inno Setup'}}
foreach($entry in @(@('target/release/coreldraw_clone.exe','coreldraw_clone.exe'),@('README.md','README.md'),@('AGENTS.md','AGENTS.md'),@('VALIDATION.md','VALIDATION.md'),@('CHANGELOG.md','CHANGELOG.md'),@('LICENSE','LICENSE'),@('installer/TERMS.txt','TERMS.txt'),@('dist/THIRD_PARTY_NOTICES.txt','THIRD_PARTY_NOTICES.txt'))) {
    $source=$entry[0];$target=Join-Path 'dist/ArtDraw-Portable' $entry[1]
    Artifact $target {Copy-Item -LiteralPath $source -Destination $target -Force}
}
New-Item -ItemType Directory -Force dist/ArtDraw-Portable/docs | Out-Null
foreach($name in (Get-ChildItem -LiteralPath docs -File | Where-Object {$_.Extension -in @('.md','.csv')} | Select-Object -ExpandProperty Name)) {
    $source=Join-Path 'docs' $name; $target=Join-Path 'dist/ArtDraw-Portable/docs' $name
    Artifact $target {Copy-Item -LiteralPath $source -Destination $target -Force}
}
Artifact 'dist/ArtDraw-Portable.zip' {Compress-Archive -Path dist/ArtDraw-Portable/* -DestinationPath dist/ArtDraw-Portable.zip -Force}
Artifact 'dist/SHA256SUMS.txt' {
    $lines=Get-FileHash target/release/coreldraw_clone.exe,dist/ArtDraw-Setup.exe,dist/ArtDraw-Portable.zip -Algorithm SHA256 | ForEach-Object {$_.Hash.ToLower()+'  '+(Split-Path $_.Path -Leaf)}
    [IO.File]::WriteAllLines((Join-Path $PWD 'dist/SHA256SUMS.txt'),$lines,[Text.UTF8Encoding]::new($false))
}
Get-Item target/release/coreldraw_clone.exe,dist/ArtDraw-Setup.exe,dist/ArtDraw-Portable.zip | Select-Object FullName,Length