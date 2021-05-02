# CangreBot

Bot de la comunidad de Discord de RustLang en Español.

## Desarrollo

Para ejecutar el código en modo desarrollo tienes dos opciones:

1. Recarga automática al guardar un archivo usando `cargo-watch`:

Para instalar `cargo-watch` usa:
```bash
cargo install cargo-watch
```

> Mira [https://crates.io/crates/cargo-watch](https://crates.io/crates/cargo-watch) para más información.

Luego ejecuta el siguiente comando:
```bash
cargo watch -c -x 'run --bin cangrebot'
```

2. Ejecutar el código sin recargar al guardar:

```bash
cargo run --bin cangrebot
```

## Producción

1. Compilamos el código en modo producción:
```bash
cargo build --release --bin cangrebot
```

2. Ejecutamos el archivo binario resultante:

Linux o macOS:
```bash
DISCORD_TOKEN=token,BOT_PREFIX=! target/release/cangrebot
```

Windows PowerShell:
```powershell
$env:DISCORD_TOKEN="token" & BOT_PREFIX="!"; target/release/cangrebot.exe
```

## Autores

-   [@sergiomeneses](https://github.com/sergiomeneses) - Contribuidor Inicial
-   [@shiftrtech](https://github.com/shiftrtech) - Contribuidor Inicial
-   [@danielsolartech](https://github.com/danielsolartech) - Contribuidor Inicial
