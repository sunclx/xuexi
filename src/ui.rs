use druid::widget::prelude::*;
use druid::widget::{
    Align, Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, Switch, WidgetExt,
};
use druid::{AppLauncher, Data, Key, Lens, LocalizedString, WindowDesc};
use std::thread;

#[derive(Clone, Data, Lens)]
pub struct ArgsState {
    pub auto: bool,
    pub local: bool,
    pub article: bool,
    pub video: bool,
    pub challenge: bool,
    pub daily: bool,
}

fn build_ui() -> impl Widget<ArgsState> {
    let switch = Flex::row()
        .with_child(Label::new("自动获取"))
        .with_child(Switch::new().lens(ArgsState::auto))
        .padding(5.0);
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
    let button_start = Button::new("开始").on_click(|_ctx, data: &mut ArgsState, _env| {
        let data = data.clone();
        thread::spawn(move || super::xuexi(data));
    });
    let button_end = Button::new("结束").on_click(|ctx, _, _env| {
        ctx.submit_command(druid::commands::QUIT_APP, druid::Target::Global);
    });
    let buttons = Flex::row()
        .with_child(button_start)
        .with_spacer(10.)
        .with_child(button_end)
        .padding(5.0);
    let layout = Flex::column()
        .with_child(Label::new("设置：     "))
        .with_child(switch)
        .with_spacer(10.)
        .with_child(either)
        .with_spacer(20.)
        .with_child(buttons)
        .padding(5.0);
    Align::centered(layout)
}
pub fn run_ui() {
    // create the initial app state
    let initial_state = ArgsState {
        auto: true,
        local: true,
        article: true,
        video: true,
        challenge: true,
        daily: true,
    };
    // describe the main window
    let main_window = WindowDesc::new(build_ui)
        .title(LocalizedString::new("学习强国"))
        .window_size((400.0, 400.0));

    // start the application
    AppLauncher::with_window(main_window)
        .configure_env(|env: &mut _, &_| {
            env.set(Key::<&str>::new("font_name"), "Kai");
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}
