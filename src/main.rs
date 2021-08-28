use std::fmt::{self, format};

use fltk::{app::App, button::Button, prelude::*, window::Window};

pub trait Callbacks {
    fn add_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F)
    where
        Self: Sized;
}
impl<T> Callbacks for T
where
    T: WidgetExt,
    T: Sized,
{
    fn add_callback<F: FnMut(&mut Self) + 'static>(&mut self, mut callback: F)
    where
        Self: Sized,
    {
        let mut cb = self.callback();
        println!("add {}", cb.is_some());
        self.set_callback(move |widget| {
            callback(widget);
            if let Some(func) = cb.as_mut() {
                (*func)();
            }
        });
    }
}

pub fn main() {
    let application = App::default();
    let mut window = Window::default();
    window.set_size(110, 110);
    let mut b = Button::new(5, 5, 100, 100, "click me");
    let mut click = 0;
    b.add_callback(move |b| {
        b.set_label(&format!("click {}", click));
        click += 1;
    });
    b.add_callback(move |b| {
        println!("click {}", click);
    });
    window.end();
    window.show();
    application.run();
}
