# Stop on errors and show commands being executed
$ErrorActionPreference = "Stop"

# Check if required tools exist
function Test-CommandExists {
    param ($command)
    $oldPreference = $ErrorActionPreference
    $ErrorActionPreference = 'stop'
    try {
        Get-Command $command
        return $true
    }
    catch {
        return $false
    }
    finally {
        $ErrorActionPreference = $oldPreference
    }
}

if (-not (Test-CommandExists "chiptool")) {
    Write-Host "chiptool could not be found. Install it with the following command:`n"
    Write-Host "    cargo install --git https://github.com/embassy-rs/chiptool --locked`n"
    exit 1
}

if (-not (Test-CommandExists "form")) {
    Write-Host "form could not be found. Install it with the following command:`n"
    Write-Host "    cargo install form`n"
    exit 1
}

# Remove existing generated files
if (Test-Path "src/sf*") {
    Remove-Item -Recurse -Force src/sf*
}

# Process each chip
$chips = @("SF32LB52x")
foreach ($chip in $chips) {
    $chip_lower = $chip.ToLower()
    # chiptool generate --svd "svd/SF32LB52x.svd" --transform "transform/SF32LB52x.yaml"
    chiptool generate --svd "svd/$chip.svd" --transform "transform/$chip.yaml"
    rustfmt lib.rs
    (Get-Content lib.rs) | Where-Object { $_ -notmatch '#!\[no_std\]' } | Set-Content lib.rs
    
    # Create directory if it doesn't exist
    New-Item -ItemType Directory -Force -Path "src/$chip_lower" | Out-Null
    
    form -i lib.rs -o "src/$chip_lower"
    Move-Item "src/$chip_lower/lib.rs" "src/$chip_lower/mod.rs" -Force
    Remove-Item lib.rs -Force
}

# Run cargo commands
cargo fmt
cargo check --features sf32lb52x