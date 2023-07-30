# rust_spikes
Learning rust

## How to use cargo

### Create a new "binary" project
```
cargo new project_name
```

create a project with a different directory name

```
cargo new --name project_name dir_name
```

### Create a new "library" project
```
cargo new project_name --lib
```

create a project with a different directory name

```
cargo new --name project_name dir_name --lib
```


### Run/Build a project
```
cargo build
cargo run
```

### Clean a project
```
cargo clean
```

### Add a dependency to a project
```
cargo add crate_name
```

E.g: crate can be regex