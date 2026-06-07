param(
    [Parameter(Mandatory = $true)]
    [string]$Version,
    [string]$Manifest,
    [string]$OutDir
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
$Arguments = @(
    (Join-Path $PSScriptRoot "build-production-payload.py"),
    "--version",
    $Version
)

if ($Manifest) {
    $Arguments += @("--manifest", $Manifest)
}
if ($OutDir) {
    $Arguments += @("--out-dir", $OutDir)
}

Push-Location $RepoRoot
try {
    & python @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "Production payload build failed with exit code $LASTEXITCODE."
    }
}
finally {
    Pop-Location
}
