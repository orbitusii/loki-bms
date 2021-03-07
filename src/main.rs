use imgui::*;

mod support;

fn main() {
    println!("Hello, world!");

    let mut current_item: usize = 0;
    let items = [1.0, 2.0, 3.0, 4.0];
    let flags = WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE | WindowFlags::NO_TITLE_BAR;

    let system = support::init(file!());

    system.main_loop(move |_, ui| {
        MenuItem::new(im_str!("File")).build(ui);
        MenuItem::new(im_str!("File/something")).build(ui);

        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .position([0.0, 0.0], Condition::FirstUseEver)
            .flags(flags)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
                if ui.button(im_str!("Some Button"), [65.0, 30.0]) {
                    ui.text(im_str!("The button is pressed!"));
                }
                else {
                    ui.text(im_str!("The button is idle"));
                }
            });

        ComboBox::new(im_str!("Combo Box"))
            .build_simple(ui, &mut current_item, &items, &label_from_f32);
    });
}

fn label_from_f32<'b> (float: &'b f32) -> std::borrow::Cow<'b, ImStr> {
    std::borrow::Cow::Owned(im_str!("{}", float))
}
