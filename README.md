# CangreBot

Bot de la comunidad de Discord de RustLang en Español.

## Desarrollo

### 1. Modificar variables de entorno:

Para que el bot se ejecute tenemos que crear el archivo `Secrets.toml` con sus respectivas variables:

- `DISCORD_TOKEN` Token del bot
- `GUILD_ID` Id del Servidor
- `BOT_APIKEY` Autorizacion para canal cifrado entre el bot y el servidor (Puede contener cualquier texto)
- `LAVALINK_PASSWORD` Contraseña Lavalink

> Existe el archivo `.env.example` que puedes renombrar

### 2. Instalar shuttle

Recarga automática al guardar un archivo usando `shuttle`:

Para instalar `shuttle` deberiamos usar `cargo-binstall`.

Esto instalara cargo-binstall en nuestro sistema.

- Para Mac y Linux:
```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
```

- Para Windows:

```powershell
Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content
```

Una vez instalado instalamos shuttle:

```
cargo binstall cargo-shuttle
```

> Mira [Shuttle](https://docs.shuttle.rs/getting-started/installation) para más información.

### 3. Ejecutar el bot

Luego ejecuta el siguiente comando para ejecutar de modo local el bot:
```bash
cargo shuttle run
```

Producción:

Para ejecutar el bot en modo producción debemos ejecutar el siguiente comando:

```bash
cargo shuttle deploy
```

Esto deployara en Shuttle el bot.

## Autores

-   [@sergiomeneses](https://github.com/sergiomeneses) - Contribuidor
-   [@shiftrtech](https://github.com/shiftrtech) - Contribuidor
-   [@danielsolartech](https://github.com/danielsolartech) - Contribuidor
-   [@Phosphorus-M](https://github.com/Phosphorus-M) - Contribuidor
-   [@SergioRibera](https://github.com/SergioRibera) - Contribuidor
