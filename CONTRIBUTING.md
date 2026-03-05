# Guía de contribución

Antes que todo, la comunidad de RustLangEs agradece de antemano el querer apoyar de alguna manera en el desarrollo de este proyecto, cada aporte es de gran valor para nosotros y nos impulsa a continuar desarrollando herramientas que apoyen de alguna manera a la comunidad.

## Flujo de trabajo general

Este proyecto sigue un flujo basado en ramas:

- `main` — rama de produccion. Solo recibe merges desde ramas externas.

### Pasos para contribuir

1. Haz un fork del repositorio en GitHub.
2. Clona tu fork localmente:
   ```
   git clone https://github.com/RustLangES/cangrebot.git
   cd cangrebot
   ```
3. Crea una rama descriptiva desde `main`:

   ```
   git switch -c feat/nombre-de-tu-cambio
   ```
4. Realiza tus cambios y una vez que tengas la resolución prueba el comando `just`, una vez que pase con exito el comando puedes comitear con total seguridad y subir tus cambios.
5. Sube tu rama a tu fork:
   ```
   git push origin feat/nombre-de-tu-cambio
   ```


## Como probar tus cambios

Actualmente el projecto contiene un justfile que simula las diferentes stages del archivo CI de GitHub, dentro de este archivo existen principalmente tres comandos:

- `check-fmt`: ejecuta el comando: `cargo fmt --all --check`, el cual valida que el codigo de Rust este formateado.
- `clippy`: ejecuta el comando: `cargo clippy --all-targets --all-features -- -D warnings`, ejecuta Clippy en todos los targets y con todas las features habilitadas, tratando las advertencias como errores para garantizar una calidad de código estricta.
- `test`: ejecuta el comando: `cargo test --verbose`, el cual ejecuta los test existentes en el proyecto dando una descripción mas detallada.

Cada uno de estos comandos se ejecuta por defecto si dentro de la terminal ejecutas solamente el comando 

`just`

Recomendamos ampliamente que cuando se realice el fork del proyecto, se cree una rama de la feature (o bug) a resolver en modo "Draft".

De esta manera el proceso de CI no se ejecuta constantemente en cada commit y solamente se ejecutara una vez que se marque la pull request como "Ready to review".

Es importante mencionar que se puede linkear la issue existente con la pull request para que esta se cierre una vez aprobada la pull request, esto se puede realizar facilmente siguiendo la estructura `Closes #numIssue`, ejemplo:
`Closes #45`
