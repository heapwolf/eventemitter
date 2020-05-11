# SYNOPSIS
An event emtter for Rust.

# USAGE

```rust
mod events;

fn main () {
    let mut e = events::EventEmitter::new();

    e.on("click", |data: &mut dyn Any| {

        //
        // Listen for any data structure and re-cast it.
        //
        let d = &mut data.downcast_mut::<Args>().unwrap();
        assert_eq!(d.x, 1);
    });

    struct Args {
        pub x: usize,
        pub y: usize,
    }

    //
    // Emit any data structure
    //
    e.emit("click", &mut Args { x: 1, y: 2 });
}
```
