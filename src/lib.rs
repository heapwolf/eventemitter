use std::any::Any;
use std::collections::HashMap;

type Callback = Box<dyn FnMut(&mut dyn Any) + 'static>;

pub struct EventEmitter {
    events: HashMap<String, Vec<Callback>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn on<F>(&mut self, name: impl Into<String>, cb: F)
    where
        F: FnMut(&mut dyn Any) + 'static,
    {
        self.events
            .entry(name.into())
            .or_default()
            .push(Box::new(cb));
    }

    pub fn off(&mut self, name: &str) {
        self.events.remove(name);
    }

    pub fn emit(&mut self, name: &str, data: &mut dyn Any) {
        if let Some(list) = self.events.get_mut(name) {
            for cb in list.iter_mut() {
                cb(data);
            }
        }
    }

    pub fn listener_count(&self, name: &str) -> usize {
        self.events.get(name).map(|v| v.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_data() {
        let mut e = EventEmitter::new();

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            d.x = 10;
        });

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            d.y = 20;
        });

        struct Args {
            pub x: usize,
            pub y: usize,
        }

        let args = &mut Args { x: 1, y: 2 };

        assert_eq!(args.x, 1);
        assert_eq!(args.y, 2);

        e.emit("click", args);

        assert_eq!(args.x, 10);
        assert_eq!(args.y, 20);
        assert_eq!(e.listener_count("click"), 2);
    }

    #[test]
    fn emit_inline() {
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

    #[test]
    fn off() {
        let mut e = EventEmitter::new();

        e.on("click", |data: &mut dyn Any| {
            let d = &mut data.downcast_mut::<Args>().unwrap();
            d.x = d.x + 1;
        });

        struct Args {
            pub x: usize,
        }

        let args = &mut Args { x: 0 };

        e.emit("click", args);
        assert_eq!(args.x, 1);
        e.emit("click", args);
        assert_eq!(args.x, 2);
        e.off("click");
        e.emit("click", args);
        assert_eq!(args.x, 2);
    }
}
