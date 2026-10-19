param([Parameter(Mandatory=$true)][string]$Path, [Parameter(Mandatory=$true)][string]$Content, [string]$Description = 'Atualização de implementação')
$root = Split-Path $PSScriptRoot -Parent
$memoryPath = Join-Path $root 'ARCHITECTURE_MEMORY.md'
$memory = [IO.File]::ReadAllText($memoryPath)
[IO.File]::WriteAllText($memoryPath, $memory + "`n- EM EXECUÇÃO: $Path — $Description.`n", [Text.UTF8Encoding]::new($false))
$destination = Join-Path $root $Path
[IO.Directory]::CreateDirectory((Split-Path $destination -Parent)) | Out-Null
[IO.File]::WriteAllText($destination, $Content, [Text.UTF8Encoding]::new($false))
$memory = [IO.File]::ReadAllText($memoryPath)
[IO.File]::WriteAllText($memoryPath, $memory + "- CONCLUÍDO: $Path ($($Content.Length) caracteres).`n", [Text.UTF8Encoding]::new($false))
