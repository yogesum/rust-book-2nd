## Rust Programing Language Book - Projects
This repository contains 2 working projects built in process of learning rust with *[Rust Book - second edition](https://doc.rust-lang.org/book/second-edition/)*.

1. **[minigrep](./minigrep)** - Basic version of `grep` command
[`chapter-12`](https://doc.rust-lang.org/book/second-edition/ch12-00-an-io-project.html)

```sh
cd minigrep

# case sensitive search
cargo run To poem.txt

# to search in case insensitive mode
CASE_INSENSITIVE=1 cargo run to poem.txt
```

2. **[webserver](./webserver)** - A multi-threaded HTTP Web server
[`chapter-20`](https://doc.rust-lang.org/book/second-edition/ch20-00-final-project-a-web-server.html)
```sh
cd webserver

cargo run # open http://127.0.0.1:7878/ in browser
```

## License

Licensed under [MIT license](./LICENSE)