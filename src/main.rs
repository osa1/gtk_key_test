use glib::clone;
use gtk::prelude::*;

fn main() {
    let app = gtk::Application::new(Some("net.osa1.gtk-key-test"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("gtk-key-test");
    window.set_border_width(8);

    let frame = gtk::Frame::new(None);
    frame.set_shadow_type(gtk::ShadowType::In);

    window.add(&frame);

    let label = gtk::Label::new(None);

    window.connect_key_press_event(clone!(
        @strong label =>
        move |_, key| gtk_key_press(&label, key)
    ));

    frame.add(&label);

    window.show_all();
}

fn gtk_key_press(label: &gtk::Label, key: &gdk::EventKey) -> gtk::Inhibit {
    println!("key pressed");

    let keyval = key.keyval();
    let keyval_name = keyval.name();
    let unicode = keyval.to_unicode();
    let modifier_type = key.state();

    label.set_markup(&format!(
        "<span size=\"large\">\
            keyval = {:?}, \
            keyval name = {:?}, \
            unicode = {:?}, \
            is modifier = {}, \
            modifier type = {}\
         </span>",
        keyval,
        keyval_name,
        unicode,
        key.is_modifier(),
        modifier_type
    ));

    label.queue_draw();

    gtk::Inhibit(true)
}
