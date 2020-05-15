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
The application can be run with its own ui utilizing [web-view](https://github.com/Boscop/web-view) by passing some argument to cargo:  
However, as there is no reloading on demand of the view implemented, the application has to be restarted everytime changes are made to the base UI.  
```
cargo run -- --with-ui
```
or in release mode  
```
cargo run --release -- --with-ui
```
