[![Build Status](https://dev.azure.com/theswiftfox/mud-saver/_apis/build/status/mud-saver-build?branchName=develop)](https://dev.azure.com/theswiftfox/mud-saver/_build/latest?definitionId=1&branchName=develop)

# mud-saver

A savegame manager for MudRunner™ and SnowRunner™.  
Backend written in [rust](https://www.rust-lang.org/) with a web-based frontend.  

## Build  
Requires rust nightly  

Check out repository and build with cargo:  
```
git clone https://github.com/theswiftfox/mud-saver
cd mud-saver
cargo run
```
Open your favorite browser at 'localhost:8000'

> [rocket-rs](https://rocket.rs/) supports hot reloading of templates and static files in debug mode -> no need to restart the application when chaning frontend and testing via browser  

### Run with native ui
The application can be compiled with its own ui utilizing [web-view](https://github.com/Boscop/web-view) by enabling a feature switch:  
```
cargo run --features embed_ui
```
or in release mode  
```
cargo run --release --features embed_ui
```
