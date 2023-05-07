pub mod counter;
pub mod counter2;

fn style(css: &'static str) -> stylist::Style {
    stylist::Style::create("leptos-test", css).unwrap()
}
