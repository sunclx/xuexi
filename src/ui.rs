use druid::widget::prelude::*;
use druid::widget::{
    Align, Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, Switch, WidgetExt,
};
use druid::{AppLauncher, Data, Key, Lens, LensWrap, LocalizedString, WindowDesc};

#[derive(Clone, Data, Lens)]
struct ArgsState {
    auto: bool,
    local: bool,
    article: bool,
    video: bool,
    challenge: bool,
    daily: bool,
}
const WINDOW_TITLE: LocalizedString<ArgsState> = LocalizedString::new("学习强国");
pub const FONT_NAME: Key<&str> = Key::new("font_name");
fn build_ui() -> impl Widget<ArgsState> {
    let switch = Flex::row()
        .with_child(Label::new("自动获取"))
        .with_child(LensWrap::new(Switch::new(), ArgsState::auto));
    let checks = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Checkbox::new("本地频道").lens(ArgsState::local))
        .with_spacer(10.)
        .with_child(Checkbox::new("阅读文章").lens(ArgsState::article))
        .with_spacer(10.)
        .with_child(Checkbox::new("视听学习").lens(ArgsState::video))
        .with_spacer(10.)
        .with_child(Checkbox::new("挑战答题").lens(ArgsState::challenge))
        .with_spacer(10.)
        .with_child(Checkbox::new("每日答题").lens(ArgsState::daily))
        .padding(5.0);
    let either = Either::new(|data, _| data.auto, Flex::row(), checks);
    let button = Button::new("开始").on_click(|_, _, _env| {
        use std::thread;

        thread::spawn(move || {
            super::xuexi()
            // some work here
        });
        println!("button：开始");
    });
    let button2 = Button::new("结束").on_click(|ctx, _, _env| {
        ctx.submit_command(druid::commands::QUIT_APP, druid::Target::Global);
        println!("button：结束");
    });
    let layout2 = Flex::row()
        .with_child(button)
        .with_spacer(10.)
        .with_child(button2);
    let layout = Flex::column()
        .with_child(Label::new("设置：     ").padding((0., 0., 0., 10.)))
        .with_child(switch)
        .with_spacer(10.)
        .with_child(either)
        .with_spacer(20.)
        .with_child(layout2);
    Align::centered(layout)
}
pub fn run_ui() {
    // describe the main window
    let main_window = WindowDesc::new(build_ui)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = ArgsState {
        auto: true,
        local: true,
        article: true,
        video: true,
        challenge: true,
        daily: true,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .configure_env(|env: &mut _, &_| {
            env.set(FONT_NAME, "Kai");
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}
