use super::android::connect;
use super::config::{CFG, KEY};
use druid::widget::prelude::*;
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, RadioGroup, Switch, TextBox,
    WidgetExt,
};
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
    device: String,
    port: String,
    host: String,
}

fn build_ui() -> impl Widget<ArgsState> {
    let radios = RadioGroup::new(
        CFG.device_configs
            .keys()
            .map(|key| (key.to_string(), key.to_string())),
    );
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("设置"))
                .with_child(radios.lens(ArgsState::device))
                .padding(5.0),
        )
        .with_child(
            Flex::row()
                .with_child(
                    TextBox::new()
                        .with_placeholder("host")
                        .lens(ArgsState::host),
                )
                .with_spacer(10.)
                .with_child(
                    TextBox::new()
                        .with_placeholder("port")
                        .lens(ArgsState::port),
                )
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
                        println!("{:?}", data.device);
                        connect(&data.host, &data.port);
                        let key = Arc::clone(&KEY);
                        let mut b = key.lock().unwrap();
                        *b = data.device.clone();
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
        device: "mumu".to_string(),
        host: CFG.device_configs["mumu"].host.to_string(),
        port: CFG.device_configs["mumu"].port.to_string(),
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
