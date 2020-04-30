use super::android::connect;
use super::config::{Config, Configuration, Rules};
use druid::widget::prelude::*;
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, RadioGroup, Switch, TextBox,
    WidgetExt,
};
use druid::{commands, AppLauncher, Command, Data, Key, Lens, LocalizedString, Target, WindowDesc};
use std::thread;
lazy_static! {
    pub static ref CONFIG_TOML:String =  std::fs::read_to_string("./config.toml").unwrap();
    pub static ref DEVICES: std::collections::HashMap<String, Rules> = {
        let c: Configuration = toml::from_str(CONFIG_TOML.as_str()).unwrap();
        c.device_configs
    };
}

#[derive(Clone, Data, Lens)]
pub struct ArgsState {
    pub auto: bool,
    pub local: bool,
    pub article: bool,
    pub video: bool,
    pub challenge: bool,
    pub daily: bool,
    
    pub config: Config,
    pub rules: Rules,
    device: String,
    start: bool,
    port: String,
    host: String,
}

fn build_ui() -> impl Widget<ArgsState> {
    let radios = RadioGroup::new(DEVICES.keys().map(|key| (key.to_string(), key.to_string())));
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("设置").on_click(|ctx, _data: &mut ArgsState, _env| {
                        let new_win = WindowDesc::new(build_ui).window_size((300.0, 500.0));
                        let command = Command::one_shot(commands::NEW_WINDOW, new_win);
                        ctx.submit_command(command, Target::Global);
                    }),
                )
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
                .with_child(Button::new("开始").on_click(
                    move |_ctx, data: &mut ArgsState, _env| {
                        println!("{:?}", data.config.device);
                        connect(&data.host, &data.port);
                        data.rules = DEVICES[&data.config.device].clone();
                        if !data.start {
                            data.start = true;
                            let data = data.clone();
                            thread::spawn(move || super::xuexi(data.clone()));
                        }
                    },
                ))
                .with_spacer(10.)
                .with_child(Button::new("结束").on_click(|ctx, _, _env| {
                    ctx.submit_command(commands::QUIT_APP, Target::Global);
                }))
                .padding(5.0),
        )
        .padding(5.0)
        .center()
}
pub fn run_ui() {
    let config: Config = toml::from_str(CONFIG_TOML.as_str()).unwrap();
    // create the initial app state
    let initial_state = ArgsState {
        auto: true,
        local: true,
        article: true,
        video: true,
        challenge: true,
        daily: true,
        config: config,
        device: "mumu".to_string(),
        rules: DEVICES["mumu"].clone(),
        start: false,
        host: DEVICES["mumu"].host.to_string(),
        port: DEVICES["mumu"].port.to_string(),
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
