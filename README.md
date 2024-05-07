# CangreBot

Bot de la comunidad de Discord de RustLang en Español.

## Desarrollo

> [!IMPORTANT]
> Antes de compilar el proyecto necesitas definir algunas variables de entorno para que compile correctamente
> las variables a definir las tienes en el archivo [`./.env.example`](./.env.example)

Para ejecutar el código en modo desarrollo tienes dos opciones:

1. Recarga automática al guardar un archivo usando `shuttle`:

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

2. Modificar variables de entorno:

Para que el bot se ejecute tenemos que crear el archivo `Secret.toml` con sus respectivas variables, un archivo de ejemplo esta en el repositorio, puedes renombrarlo.

3. Ejecutar el bot:

Luego ejecuta el siguiente comando para ejecutar de modo local el bot:
```bash
cargo shuttle run
```

## Producción

Para ejecutar el bot en modo producción debemos ejecutar el siguiente comando:

```bash
cargo shuttle deploy
```

Esto deployara en Shuttle el bot.

## Autores

-   [@sergiomeneses](https://github.com/sergiomeneses) - Contribuidor Inicial
-   [@shiftrtech](https://github.com/shiftrtech) - Contribuidor Inicial
-   [@danielsolartech](https://github.com/danielsolartech) - Contribuidor Inicial
-   [@Phosphorus-M](https://github.com/Phosphorus-M) - Contribuidor
