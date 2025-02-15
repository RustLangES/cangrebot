# Gestión de Submódulos en Cangrebot

## Introducción

Este proyecto utiliza submódulos de Git para gestionar dependencias externas dentro del repositorio. Un submódulo es un repositorio dentro de otro repositorio, lo que permite incluir código de terceros sin mezclarlo directamente con el código principal.

## Inicialización y Actualización de Submódulos

Si has clonado el repositorio y el submódulo no aparece, debes inicializarlo y actualizarlo manualmente con el siguiente comando:

```sh
 git submodule update --init --recursive
```

Esto descargará el contenido del submódulo y lo sincronizará con la versión esperada en el repositorio principal.

> El archivo `build.rs` detectará automáticamente si el submódulo existe. Si no está inicializado, mostrará un mensaje indicando que es necesario ejecutar: `git submodule update --init --recursive`


## Verificación del Estado del Submódulo

Puedes verificar el estado del submódulo con:

```sh
git submodule status
```

Esto mostrará si el submódulo está correctamente sincronizado o si requiere una actualización.

## Consideraciones

- Si trabajas en una rama donde el submódulo ha cambiado, ejecuta `git submodule update --remote` para traer la última versión.
- Al clonar un nuevo repositorio, recuerda siempre inicializar y actualizar los submódulos.

## Conclusión

El uso de submódulos permite gestionar dependencias de manera eficiente sin mezclar código externo en el repositorio principal. 
Con la automatización en `build.rs`, se puede reducir la fricción al compilar el proyecto sin necesidad de pasos adicionales
