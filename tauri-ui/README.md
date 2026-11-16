# ROG Ally Controller

Tauri v2 app for controlling ASUS ROG Ally X hardware settings. Alternative to Armoury Crate SE.

## Prerequisites

- Rust (rustup.rs)
- Tauri CLI v2: `cargo install tauri-cli@^2`

## Build & Run

```bash
cd src-tauri
cargo tauri dev    # development
cargo tauri build  # production .msi
```

## Notes

- **Mock data**: All hardware calls currently return simulated values. Real ACPI/WMI integration requires the ASUS System Control Interface driver on Windows.
- **Admin required**: Real hardware control (TDP, fan curves, charge limit) needs administrator privileges to access ACPI WMI via `\\.\ATKACPI` and Ryzen SMU.
- **ACPI codes**: See comments in `src/main.rs` for the DSTS/DEVS codes mapped from G-Helper analysis.
- **Gamepad nav**: D-pad/stick navigates, A selects, B goes home, LB/RB switch tabs.
