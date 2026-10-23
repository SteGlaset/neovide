param([string]$OutputDirectory = (Join-Path $PSScriptRoot '..\..\target\release'))
$ErrorActionPreference = 'Stop'
Get-Command cl.exe -ErrorAction Stop | Out-Null
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$outputRoot = (Resolve-Path -LiteralPath $OutputDirectory).Path
$source = Join-Path $PSScriptRoot 'neovide-fast-launch.c'
$executable = Join-Path $outputRoot 'neovide-fast-launch.exe'
$object = Join-Path $outputRoot 'neovide-fast-launch.obj'
& cl.exe /nologo /O2 /W4 /WX /MT $source "/Fo:$object" "/Fe:$executable" /link /SUBSYSTEM:WINDOWS user32.lib
if ($LASTEXITCODE -ne 0) { throw 'Native launcher compilation failed' }
Write-Output $executable
