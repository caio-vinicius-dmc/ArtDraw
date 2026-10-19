param(
    [ValidateSet('Check','Test','Release','All','Environment')][string]$Mode = 'All',
    [switch]$Offline
)
$ErrorActionPreference = 'Stop'
Set-Location $PSScriptRoot
$env:CARGO_INCREMENTAL = '0'
$memoryPath = Join-Path $PSScriptRoot 'ARCHITECTURE_MEMORY.md'
$memory = Get-Content -Raw -LiteralPath $memoryPath
Add-Content -LiteralPath $memoryPath -Value "`n- BUILD $Mode iniciado: Cargo pode atualizar Cargo.lock e as árvores geradas target/.tools/cargo. Memória lida antes da operação." -Encoding utf8
$localCargo = Join-Path $PSScriptRoot '.tools/cargo/bin'
if (Test-Path (Join-Path $localCargo 'cargo.exe')) {
    $env:CARGO_HOME = Join-Path $PSScriptRoot '.tools/cargo'
    $env:RUSTUP_HOME = Join-Path $PSScriptRoot '.tools/rustup'
    $env:PATH = "$localCargo;$env:PATH"
}
$llvm = Get-ChildItem -LiteralPath (Join-Path $PSScriptRoot '.tools') -Directory -Filter 'llvm-mingw-*-msvcrt-x86_64' -ErrorAction SilentlyContinue | Sort-Object Name -Descending | Select-Object -First 1
$taskRustHost = (& rustc -vV | Select-String '^host:').ToString()
if ($LASTEXITCODE -ne 0) { throw 'Rust não encontrado; veja docs/BUILDING.md' }
if ($llvm -and $taskRustHost -match 'windows-gnu') {
    $bin = Join-Path $llvm.FullName 'bin'
    $env:PATH = "$bin;$env:PATH"
    $env:ARTDRAW_WINDRES = Join-Path $bin 'x86_64-w64-mingw32-windres.exe'
    $env:ARTDRAW_WINDOWS_IMPORT_LIBS = Join-Path $PSScriptRoot '.tools/windows-imports'
    New-Item -ItemType Directory -Force $env:ARTDRAW_WINDOWS_IMPORT_LIBS | Out-Null
    $importLib = Join-Path $env:ARTDRAW_WINDOWS_IMPORT_LIBS 'libshlwapi.a'
    if (!(Test-Path -LiteralPath $importLib)) {
        $memory = Get-Content -Raw -LiteralPath $memoryPath
        Add-Content -LiteralPath $memoryPath -Value '- Gerar .tools/windows-imports/libshlwapi.a: biblioteca de importação Windows isolada; evitar misturar CRTs GNU e LLVM.' -Encoding utf8
        Copy-Item -LiteralPath (Join-Path $llvm.FullName 'x86_64-w64-mingw32/lib/libshlwapi.a') -Destination $importLib
    }
    $taskRustRoot = (& rustc --print sysroot).Trim()
    $rustGnuBin = Join-Path $taskRustRoot 'lib/rustlib/x86_64-pc-windows-gnu/bin/self-contained'
    $env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = Join-Path $rustGnuBin 'x86_64-w64-mingw32-gcc.exe'
    $env:ARTDRAW_GNU_DLLTOOL = Join-Path $rustGnuBin 'dlltool.exe'
    $env:ARTDRAW_ASSEMBLER = Join-Path $bin 'x86_64-w64-mingw32-as.exe'
    $env:ARTDRAW_AS_BRIDGE = Join-Path $PSScriptRoot '.tools/as-bridge.exe'
    foreach ($entry in @(@('installer/assembler_bridge.rs','.tools/as-bridge.exe'),@('installer/dlltool_bridge.rs','.tools/dlltool-bridge.exe'))) {
        $source=Join-Path $PSScriptRoot $entry[0]; $output=Join-Path $PSScriptRoot $entry[1]
        if (!(Test-Path $output) -or (Get-Item $source).LastWriteTimeUtc -gt (Get-Item $output).LastWriteTimeUtc) {
            $memory=Get-Content -Raw -LiteralPath $memoryPath
            Add-Content -LiteralPath $memoryPath -Value "- Gerar adaptador de toolchain $($entry[1])." -Encoding utf8
            & rustc $source --edition 2021 -C ('linker='+$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER) -C ('dlltool='+(Join-Path $bin 'llvm-dlltool.exe')) -o $output
            if($LASTEXITCODE -ne 0){throw 'Falha ao compilar adaptador de toolchain'}
        }
    }
    $env:CARGO_ENCODED_RUSTFLAGS = '-C' + [char]31 + ('dlltool=' + (Join-Path $PSScriptRoot '.tools/dlltool-bridge.exe'))
}
function Run-Cargo([string[]]$CargoArgs) {
    $networkArgs = @('--locked')
    if ($Offline) { $networkArgs += '--offline' }
    & cargo @CargoArgs @networkArgs
    if ($LASTEXITCODE -ne 0) { throw "cargo $($CargoArgs -join ' ') falhou com código $LASTEXITCODE" }
}
if ($Mode -in @('Check','Test','Release','All')) { & (Join-Path $PSScriptRoot 'scripts/update-parity.ps1') -Check }
if ($Mode -in @('Check','All')) { Run-Cargo @('check') }
if ($Mode -in @('Test','All')) { Run-Cargo @('test') }
if ($Mode -in @('Release','All')) { Run-Cargo @('build','--release') }
$memory = Get-Content -Raw -LiteralPath $memoryPath
Add-Content -LiteralPath $memoryPath -Value "- BUILD $Mode concluído com sucesso." -Encoding utf8