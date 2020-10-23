# mud-saver

A savegame manager for MudRunner™ and SnowRunner™.  
Backend written in [rust](https://www.rust-lang.org/) with a web-based frontend.  

## Install

### Windows
Currently no installer is provided. You can grab the latest release from the [releases](https://github.com/theswiftfox/mud-saver/releases) page. Extract the zip file and run mud-saver.exe. The release builds are built with embedded ui.

### Linux
We don't have automated linux builds for now. On linux you will have to follow the instructions below to manually build the application.

## Build  
Requires rust stable  

Check out repository and build with cargo:  
```
git clone https://github.com/theswiftfox/mud-saver
cd mud-saver
cargo run
```
Open your favorite browser at 'localhost:8000' 

### Run with native ui
The application can be compiled with its own ui utilizing [web-view](https://github.com/Boscop/web-view) by enabling a feature switch:  
```
cargo run --features embed_ui
```
or in release mode  
```
cargo run --release --features embed_ui
```
