# SYNOPSIS
An event emtter for Rust.

# USAGE

```rust
fn main () {
    let mut e = EventEmitter::new();

    e.on("click", |data: &mut dyn Any| {
        let d = &mut data.downcast_mut::<Args>().unwrap();
        assert_eq!(d.x, 1);
    });

    e.on("click", |data: &mut dyn Any| {
        let d = &mut data.downcast_mut::<Args>().unwrap();
        assert_eq!(d.y, 2);
    });

    struct Args {
        pub x: usize,
        pub y: usize,
    }

    e.emit("click", &mut Args { x: 1, y: 2 });
}
```
