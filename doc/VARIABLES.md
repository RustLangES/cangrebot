# Configuración del Bot

Para ejecutar el bot, crea un archivo `.env` con las siguientes variables:

> Existe el archivo `.env.example` que puedes renombrar

### ¿Dónde obtener los valores?

- `DISCORD_TOKEN`: Obtenlo desde el [Token del bot](https://discord.com/developers/applications)
- `GUILD_ID`: Activa el Modo Desarrollador en Discord, haz clic derecho en tu servidor y selecciona 'Copiar ID'
- `BOT_APIKEY`: Autorizacion para canal cifrado entre el bot y el servidor (Puede contener cualquier texto)
- `CHANNEL_DAILY` & `CHANNEL_SUGGEST`: Haz clic derecho en el canal de Discord y selecciona 'Copiar ID'
- `LAVALINK_PASSWORD`: Contraseña Lavalink

**Opcionales**

- `STATIC_ROOT`: corresponde a la ubicacion del contenido `static`, por defecto es `./static`

#### Formato para `.env`

```toml
DISCORD_TOKEN = "Bot token"
GUILD_ID = "Server ID"
BOT_APIKEY = "API key for secure communication"
CHANNEL_DAILY = "Channel ID for daily challenges"
CHANNEL_SUGGEST = "Channel ID for suggestions"
```
