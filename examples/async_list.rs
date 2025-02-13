use dioxus::prelude::*;
use dioxus_lazy::{lazy, List};
use dioxus_logger::tracing::Level;

fn app() -> Element {
    rsx! {
        List {
            len: 100,
            size: 400.,
            item_size: 20.,
            make_item: move |idx: &usize| rsx! { "Async item {*idx}" },
            make_value: lazy::from_async_fn(|idx| async move { idx })
        }
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).unwrap();
    console_error_panic_hook::set_once();

    #[cfg(feature = "examples")]
    {
        dioxus::launch(app)
    }
    #[cfg(not(feature = "examples"))]
    {
        println!(r#"run with "--features example" to see the output"#)
    }
}
