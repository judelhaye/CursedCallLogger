extern crate chrono;
extern crate cursive;

use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView,
                     LinearLayout, SelectView};
use cursive::traits::*;
use chrono::Utc;


fn main() {
    let mut siv = Cursive::default();

    // Call list view
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_id("select")
        .fixed_size((80, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("*Dring !*", add_call))
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Forget it", delete_call))
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(Dialog::around(LinearLayout::vertical()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("cursed CRM"));

    siv.run();
}

fn add_call(s: &mut Cursive) {
    // on success put the smc in the SelectView
    fn ok(s: &mut Cursive, smc: &str) {
        s.call_on_id("select", |view: &mut SelectView<String>| {
            view.add_item_str(smc)
        });
        s.pop_layer();
    }
    // popup to fill
    s.add_layer(Dialog::around(LinearLayout::vertical()
    .child(EditView::new()
            .on_submit(ok)
            .with_id("smc")
            .fixed_width(10)))
        .title("Who is calling ?")
        .button("Ok", |s| {
            let now  = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let call_info =
                s.call_on_id("smc", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            let smc = format!("{:?} - {}", now, call_info);
            ok(s, &smc);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}


fn delete_call(s: &mut Cursive) {
    let mut select = s.find_id::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No record to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}


fn on_submit(s: &mut Cursive, smc: &String) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Call: {}\n", smc))
        .title(format!("{}'s info", smc))
        .button("Quit", Cursive::quit));
}
