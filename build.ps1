$pluginLIB = "target/aarch64-skyline-switch/release/libsmashline_icies_visualizer.nro"
$pluginNRO = "target/aarch64-skyline-switch/release/plugin.nro"
if (Test-Path $pluginNRO) {
    Remove-Item $pluginNRO
}

cargo skyline build --release
Move-Item -Path $pluginLIB -Destination $pluginNRO

Invoke-Item "target/aarch64-skyline-switch/release"