# AeroPress timer 
Small website for following an AeroPress recipe along with a timer.

This is built with Rust and WebAssembly.

I've used the [Yew](https://github.com/DenisKolodin/yew) framework, which uses many familiar concepts from React, Elm, and Redux.

### Running
For development:
```sh 
cargo web start
```

For production:
```sh
cargo web deploy
```

Deploy to `now`
```sh
sh deploy.sh
```

###### TODO
- [ ] Experiment with adding a bundler to further optimise the static bundle 
- [ ] Add a full recipe view 
- [ ] Add a way of creating more recipes
    - [ ] Use local storage to save the recipes
- [ ] Try adding some test

