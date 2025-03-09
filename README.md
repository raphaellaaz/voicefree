## VoiceFree - Alternativa a Voicemeeter (Proyecto en Rust)

VoiceFree es una herramienta de código abierto para la gestión avanzada de audio en sistemas Linux, desarrollada con Rust, utilizando **cpal** para el manejo de las entradas y salidas de audio y **PipeWire** para la redirección de audio. Su propósito es ofrecer funcionalidades similares a Voicemeeter.

Este proyecto está diseñado para ser multiplataforma (con un enfoque inicial en Linux), y se utilizará para redirigir las entradas y salidas de audio, así como gestionar dispositivos virtuales de audio, con el objetivo de aprender y experimentar con el desarrollo en Rust, así como con las bibliotecas de **cpal**, **PipeWire** y **Slint**.

## Características

- **Redirección de Entradas y Salidas de Audio**: Capacidad de seleccionar y redirigir dispositivos de entrada y salida de audio en tiempo real, utilizando **cpal** para interactuar con las interfaces de audio y **PipeWire** para gestionar el enrutamiento del audio.
- **Dispositivos Virtuales**: Crear y gestionar dispositivos de audio virtuales, como lo haría Voicemeeter.
- **Interfaz Gráfica con Slint**: Una interfaz simple y amigable para configurar y controlar las opciones de audio.
- **Multiplataforma**: Compatible con Linux (en fase inicial), con planes de expansión a otras plataformas como Windows.
- **Desarrollo con Rust**: Utilización de Rust como lenguaje de programación para garantizar seguridad, velocidad y facilidad de mantenimiento.

## Dependencias

- **Rust**: Este proyecto está desarrollado en Rust. Para comenzar a trabajar con el código, debes tener instalado Rust en tu máquina.
  - [Instalar Rust](https://www.rust-lang.org/tools/install)

- **cpal**: Biblioteca para la captura y reproducción de audio. Se utiliza para interactuar directamente con las interfaces de audio.
  - [cpal en crates.io](https://crates.io/crates/cpal)

- **PipeWire**: Es la biblioteca principal para el manejo de audio y multimedia en Linux.
  - [Sitio web de PipeWire](https://pipewire.org/)

- **Slint**: Para la creación de la interfaz gráfica de usuario (GUI).
  - [Sitio web de Slint](https://slint-ui.com/)

## Instalación

### Requisitos previos

1. **Instalar Rust**: Si aún no lo tienes instalado, puedes hacerlo con el siguiente comando:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Instalar PipeWire**: Asegúrate de tener PipeWire instalado en tu sistema. En sistemas basados en Debian, puedes hacerlo con:
   ```bash
   sudo apt install pipewire pipewire-audio-client-libraries
   ```

3. **Instalar las dependencias de Slint**: Si planeas trabajar en la interfaz gráfica, debes instalar Slint:
   ```bash
   cargo add slint
   ```

4. **Instalar las dependencias de cpal**: Para manejar el audio con **cpal**, agrega la dependencia al proyecto:
   ```bash
   cargo add cpal
   ```

### Clonar el repositorio

Primero, clona el repositorio en tu máquina local:

```bash
https://github.com/raphaellaaz/voicefree.git
cd voicefree
```

### Compilar y ejecutar

Para compilar el proyecto, usa Cargo:

```bash
cargo build
```

Y para ejecutar el proyecto:

```bash
cargo run
```

## Releases

Para ver las últimas versiones del proyecto, puedes visitar la sección de **Releases** en GitHub:

[Releases de VoiceFree](https://github.com/raphaellaaz/voicefree/releases)

## Uso

- **Configuración de Audio**: Al iniciar el programa, se mostrará una interfaz gráfica donde podrás seleccionar los dispositivos de entrada y salida, así como redirigir el audio entre ellos.
- **Crear Dispositivos Virtuales**: Desde la interfaz gráfica, podrás configurar dispositivos virtuales de audio y enrutar el audio a través de ellos.
- **Control de Retardo de Audio**: Optimiza las rutas de audio para reducir el retardo y mejorar la calidad en tiempo real.

## Contribuciones

Este proyecto es de código abierto y las contribuciones son bienvenidas. Si deseas contribuir, sigue estos pasos:

1. Haz un fork del repositorio.
2. Crea una rama para tus cambios (`git checkout -b feature/new-feat`).
3. Realiza los cambios y haz commit de ellos (`git commit -am 'Añadir nueva funcionalidad'`).
4. Empuja los cambios a tu fork (`git push origin feature/new-feat`).
5. Crea un pull request desde tu fork a este repositorio.

## Licencia

Este proyecto está bajo la Licencia GPLv3. Consulta el archivo [LICENSE](LICENSE) para más detalles.

## Contacto

Si tienes alguna pregunta o sugerencia, no dudes en abrir un issue en GitHub o contactar al autor.

- **Autor**: Raphael Laaz Muñoz
- **Correo**: raphaellaaz01@gmail.com
- **GitHub**: [@raphaellaaz](https://github.com/raphaellaaz)

¡Gracias por interesarte en VoiceFree! Saludos