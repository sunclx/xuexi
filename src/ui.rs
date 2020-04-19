use super::config::KEY;
use super::config::OUT;
use druid::widget::prelude::*;
use druid::widget::{Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, Switch, WidgetExt};
use druid::{AppLauncher, Data, Key, Lens, LocalizedString, WindowDesc};
use std::sync::Arc;
use std::thread;

#[derive(Clone, Data, Lens)]
pub struct ArgsState {
    pub auto: bool,
    pub local: bool,
    pub article: bool,
    pub video: bool,
    pub challenge: bool,
    pub daily: bool,
    start: bool,
    mumu: bool,
}

fn build_ui() -> impl Widget<ArgsState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new(|_: &_, _: &_| {
                    let clone = OUT.clone();
                    let io = clone.lock().unwrap();
                    let out = io.to_string();
                    out
                }))
                .with_child(Checkbox::new("mumu").lens(ArgsState::mumu))
                .padding(5.0),
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("自动获取"))
                .with_child(Switch::new().lens(ArgsState::auto))
                .padding(5.0),
        )
        .with_spacer(10.)
        .with_child(Either::new(
            |data, _| data.auto,
            Flex::row(),
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Checkbox::new("本地频道").lens(ArgsState::local))
                .with_spacer(10.)
                .with_child(Checkbox::new("视听学习").lens(ArgsState::video))
                .with_spacer(10.)
                .with_child(Checkbox::new("阅读文章").lens(ArgsState::article))
                .with_spacer(10.)
                .with_child(Checkbox::new("挑战答题").lens(ArgsState::challenge))
                .with_spacer(10.)
                .with_child(Checkbox::new("每日答题").lens(ArgsState::daily))
                .padding(5.0),
        ))
        .with_spacer(20.)
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("开始").on_click(|_ctx, data: &mut ArgsState, _env| {
                        if data.mumu {
                            let key = Arc::clone(&KEY);
                            let mut b = key.lock().unwrap();
                            *b = true;
                        }
                        if !data.start {
                            data.start = true;
                            let data = data.clone();
                            thread::spawn(move || super::xuexi(data.clone()));
                        }
                    }),
                )
                .with_spacer(10.)
                .with_child(Button::new("结束").on_click(|ctx, _, _env| {
                    ctx.submit_command(druid::commands::QUIT_APP, druid::Target::Global);
                }))
                .padding(5.0),
        )
        .padding(5.0)
        .center()
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
        start: false,
        mumu: false,
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
