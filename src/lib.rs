use std::any::Any;
use std::collections::HashMap;

type Callback<'a> = Box<(dyn FnMut(&mut dyn Any) + 'static)>;

pub struct EventEmitter<'a> {
    pub events: HashMap<&'a str, Vec<Callback<'a>>>,
}

impl<'a> EventEmitter<'a> {
    pub fn new() -> Self {
        EventEmitter {
            events: HashMap::new(),
        }
    }

    pub fn create_callback<'x, F>(f: F) -> Callback<'x>
    where
        F: FnMut(&mut dyn Any) + 'static,
    {
        Box::new(f) as Callback
    }

    pub fn on(&mut self, name: &'a str, cb: impl Fn(&mut dyn Any) + 'static) {
        if !self.events.contains_key(name) {
            self.events.insert(name, Vec::new());
        }

        let list = &mut self.events.get_mut(name).unwrap();
        list.push(EventEmitter::<'a>::create_callback(cb));
    }

    pub fn off(&mut self, name: &'a str) {
        if self.events.contains_key(name) {
            self.events.remove(name);
        }
    }

    pub fn emit(&mut self, name: &'a str, data: &mut dyn Any) {
        if !self.events.contains_key(name) {
            return
        }

        let list = &mut self.events.get_mut(name).unwrap();

        for cb in list.iter_mut() {
            cb(data);
        }
    }

    pub fn listeners(&mut self, name: &'a str) -> Option<&Vec<Callback<'a>>> {
        let listeners = self.events.get(name);

        if listeners.is_some() {
            return listeners;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_data () {
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
        assert_eq!(e.listeners("click").unwrap().len(), 2);
    }

    #[test]
    fn emit_inline () {
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
    fn off () {
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
