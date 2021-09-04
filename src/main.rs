use std::collections::HashMap;

struct Widget {}
impl Widget {
    pub(crate) fn set_label(&self, format: &str) -> () {
        println!("label: {}", format);
    }
}
impl WidgetExt for Widget {
    fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, _cb: F)
    where
        Self: Sized,
    {
        // nothing
    }
}
pub trait WidgetExt {
    fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F)
    where
        Self: Sized;
}
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
        Self: WidgetExt,
    {
        CALLBACKS.insert(self as *const dyn WidgetExt, callback);
        self.set_callback(move |widget| CALLBACKS.get(widget)());
    }
}

static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();

pub fn main() {
    let mut b = Widget {};
    let mut click = 0;
    b.add_callback(move |b| {
        b.set_label(&format!("click {}", click));
        click += 1;
    });
    b.add_callback(move |_b| {
        println!("click {}", click);
    });
}
