# Auto reloading of code and template changes
# install cargo-watch and systemfd
# cargo install systemfd cargo-watch
systemfd --no-pid -s http::8000 -- cargo watch -x run