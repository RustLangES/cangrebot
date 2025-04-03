# CangreBot

Bot de la comunidad de Discord de RustLang en Español.

## Desarrollo

### 1. Configurar variables de entorno

Las variables de entorno necesarias están documentadas en [`doc/VARIABLES.md`](doc/VARIABLES.md). Asegúrate de configurarlas antes de ejecutar el bot.

### 2. Gestión de Submódulos

Para gestionar los submódulos del proyecto, consulta la documentación en [`doc/SUBMODULOS.md`](doc/SUBMODULOS.md).

### 3. Ejecutar el bot

Luego ejecuta el siguiente comando para ejecutar de modo local el bot:
```bash
cargo run
```
## Configuración de Variables Secretas para GitHub Actions

Para que el flujo de trabajo de GitHub Actions funcione correctamente, es necesario configurar las siguientes variables secretas en el repositorio. Estas variables se utilizan para la autenticación y despliegue del binario Rust en el servidor remoto.

### Variables necesarias

1. **`REMOTE_USER`**: El nombre de usuario para acceder al servidor remoto.
2. **`REMOTE_HOST`**: La dirección IP o el nombre de host del servidor remoto.
3. **`REMOTE_PATH`**: La ruta en el servidor remoto donde se copiará el binario.
4. **`PROGRAM_NAME`**: El nombre del binario generado por el proyecto Rust.
5. **`SSH_PRIVATE_KEY`**: La clave privada SSH para autenticarte en el servidor remoto.
6. **`KNOWN_HOSTS`**: La lista de hosts conocidos para evitar advertencias de autenticidad SSH.

### Pasos para configurar las variables para el despliege en github actions

1. Ve a la página del repositorio en GitHub.
2. Haz clic en la pestaña **Settings**.
3. En el menú lateral, selecciona **Secrets and variables** > **Actions**.
4. Haz clic en el botón **New repository secret** para agregar cada una de las variables mencionadas anteriormente.
5. Ingresa el nombre de la variable (por ejemplo, `REMOTE_USER`) y su valor correspondiente.
6. Repite este proceso para todas las variables necesarias.

### Ejemplo de configuración

Si estás desplegando un proyecto llamado `my-rust-app`, las variables podrían configurarse de la siguiente manera:

- `REMOTE_USER`: `deploy_user`
- `REMOTE_HOST`: `192.168.1.100`
- `REMOTE_PATH`: `/home/deploy_user/apps`
- `PROGRAM_NAME`: `my-rust-app`
- `SSH_PRIVATE_KEY`: (contenido de tu clave privada SSH)

Una vez configuradas las variables, el flujo de trabajo de GitHub Actions se encargará de construir y desplegar automáticamente el binario Rust en el servidor remoto al hacer un push a la rama `main`.

## Autores

-   [@sergiomeneses](https://github.com/sergiomeneses) - Contribuidor
-   [@shiftrtech](https://github.com/shiftrtech) - Contribuidor
-   [@danielsolartech](https://github.com/danielsolartech) - Contribuidor
-   [@Phosphorus-M](https://github.com/Phosphorus-M) - Contribuidor
-   [@SergioRibera](https://github.com/SergioRibera) - Contribuidor
-   [@memw](https://github.com/stifskere) - Contribuidor
