$ErrorActionPreference = "Stop"

$workflowPath = Join-Path $PSScriptRoot "..\workflows\harness-cli-release.yml"
$workflow = Get-Content -LiteralPath $workflowPath -Raw

$requiredPatterns = @{
    "Node.js 24 opt-in" = "(?m)^\s*FORCE_JAVASCRIPT_ACTIONS_TO_NODE24:\s*true\s*$"
    "checkout v6" = "(?m)^\s*uses:\s*actions/checkout@v6\s*$"
    "upload-artifact v7" = "(?m)^\s*uses:\s*actions/upload-artifact@v7\s*$"
    "download-artifact v8" = "(?m)^\s*uses:\s*actions/download-artifact@v8\s*$"
}

foreach ($entry in $requiredPatterns.GetEnumerator()) {
    if ($workflow -notmatch $entry.Value) {
        throw "Missing required workflow setting: $($entry.Key)"
    }
}

$forbiddenPatterns = @(
    "(?m)^\s*uses:\s*actions/checkout@v[1-5]\s*$",
    "(?m)^\s*uses:\s*actions/upload-artifact@v[1-6]\s*$",
    "(?m)^\s*uses:\s*actions/download-artifact@v[1-7]\s*$"
)

foreach ($pattern in $forbiddenPatterns) {
    if ($workflow -match $pattern) {
        throw "Found a GitHub action version that is not Node.js 24-ready: $($Matches[0].Trim())"
    }
}

Write-Output "GitHub Actions Node.js 24 readiness check passed."
