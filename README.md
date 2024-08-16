#  A Simple API in Rust
##  Using Warp

#### Warp is straightfoward except for the filter deal.  
If you open this in VSCode ( and I am sure other IDE's...) the repeated <AND<AND<AND business
can be intimidating but it's simple chained .and or .map() or .or()...

- cargo add warp
- cargo add serde --features derive
- cargo add chrono --features serde
- cargo add tokio --features full
- cargo add pretty_env_logger
- cargo add uuid --features v4

- cargo install cargo-watch  


### cargo r

#### When running - http://localhost:8000/api/ping -> Pong

You can see the rest of the routes in the thunder-collection_Koyzon.json file.
