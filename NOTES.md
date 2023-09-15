
```
rm -rf target Cargo.lock
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/lib/x86_64-linux-gnu/pkgconfig/
echo $PKG_CONFIG_PATH
cargo run --features bevy/dynamic_linking
```

[bevy book](https://bevyengine.org/learn/book/getting-started/ecs/)

[bevy cheatbook](https://bevy-cheatbook.github.io/features/coords.html)

[bevy examples](https://github.com/bevyengine/bevy/tree/latest/examples#examples)

[rust book](https://doc.rust-lang.org/book/)
