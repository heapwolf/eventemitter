# SYNOPSIS
An event emtter for Rust.

# USAGE

```rs
struct Args {
    pub x: usize,
    pub y: usize,
}

fn main () {
    let mut e = EventEmitter::new();

    e.on("click", |data: &mut dyn Any| {
        let d = &mut data.downcast_mut::<Args>().unwrap();
        d.x = 10;
    });

    e.on("click", |data: &mut dyn Any| {
        let d = &mut data.downcast_mut::<Args>().unwrap();
        d.y = 20;
    });

    e.emit("click", &mut Args { x: 1, y: 2 });
}
```
