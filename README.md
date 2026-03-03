<div align="center">

<img src="assets/icons/wraith_logo.svg" width="300" alt="Wraith Logo">

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](#) [![Version](https://img.shields.io/badge/version-v0.1.0-blue)](#)

# Wraith: Secure Anonymous Messenger

Wraith is a secure anonymous messenger built with Rust and Dioxus. It provides a private and reliable communication platform for its users.

[Installation](#installation) • [Usage](#usage) • [Contributing](#contributing) • [Acknowledgements](#acknowledgements)

</div>

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- Android SDK and NDK (for Android builds)

### Build and Run for Android

1. Ensure your `ANDROID_HOME`, `ANDROID_NDK_HOME`, and `JAVA_HOME` environment variables are set correctly. For example:
   ```powershell
   $env:JAVA_HOME = "C:\Program Files\Android\Android Studio\jbr"
   $env:ANDROID_HOME = "C:\Users\<user>\AppData\Local\Android\Sdk"
   $env:ANDROID_NDK_HOME = "C:\Users\<user>\AppData\Local\Android\Sdk\ndk\<version>"
   ```

2. Install the Dioxus CLI:
   ```powershell
   cargo install dioxus-cli --version "^0.6"
   # Or using cargo-binstall:
   # cargo binstall dioxus-cli@0.6.3
   ```

3. Add Rust Android targets:
   ```powershell
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```

4. Build the application:
   ```powershell
   dx build --platform android
   ```

5. Serve the application on an emulator or connected device:
   ```powershell
   dx serve --platform android
   ```

## Usage

Once the application is running on your device or emulator, you can start using it to send secure and anonymous messages. Modifications to the code will automatically be hot-reloaded by Dioxus while `dx serve` is running.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any bugs or feature requests.

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Dioxus](https://dioxuslabs.com/)
