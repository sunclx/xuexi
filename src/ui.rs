use super::android::connect;
use super::config::{Config, Configuration, Rules};
use druid::widget::prelude::*;
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, MainAxisAlignment, RadioGroup,
    Switch, TextBox, WidgetExt,
};
use druid::{commands, AppLauncher, Command, Data, Key, Lens, LocalizedString, Target, WindowDesc};
use std::thread;
lazy_static! {
    pub static ref CFG: Configuration = {
        let s = std::fs::read_to_string("./config.toml").unwrap();
        let c: Configuration = toml::from_str(&s).unwrap();
        c
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
    start: bool,
}
fn row<L: Lens<Config, String> + 'static>(label: &'static str, lens: L) -> impl Widget<Config> {
    Flex::row()
        .with_child(Label::new(label).fix_width(160.0))
        .with_child(TextBox::new().fix_width(200.0).lens(lens))
        .padding(4.0)
}
fn row_parse<
    S: Data + std::str::FromStr + std::fmt::Display,
    L: Lens<Config, Option<S>> + 'static,
>(
    label: &'static str,
    lens: L,
) -> impl Widget<Config> {
    Flex::row()
        .with_child(Label::new(label).fix_width(160.0))
        .with_child(TextBox::new().fix_width(200.0).parse().lens(lens))
        .padding(4.0)
}
fn row_bool<L: Lens<Config, bool> + 'static>(label: &'static str, lens: L) -> impl Widget<Config> {
    Flex::row()
        .with_child(Label::new(label).fix_width(160.0))
        .with_child(Checkbox::new("").lens(lens))
        .padding(4.0)
}
fn setting_ui() -> impl Widget<ArgsState> {
    let device = row("device", Config::device);
    let database_uri = row("database_uri", Config::database_uri);
    let database_json = row("database_json", Config::database_json);
    let db_wrong_json = row("db_wrong_json", Config::db_wrong_json);
    let daily_json = row("daily_json", Config::daily_json);
    let challenge_json = row("challenge_json", Config::challenge_json);
    let comments_json = row("comments_json", Config::comments_json);
    let is_user = row_bool("is_user", Config::is_user);
    let daily_forever = row_bool("daily_forever", Config::daily_forever);
    let daily_delay = row_parse("daily_delay", Config::daily_delay);
    let challenge_count = row_parse("challenge_count", Config::challenge_count);
    let challenge_delay = row_parse("challenge_delay", Config::challenge_delay);
    let video_column_name = row("video_column_name", Config::video_column_name);
    let video_count = row_parse("video_count", Config::video_count);
    let video_delay = row_parse("video_delay", Config::video_delay);
    let enable_article_list = row_bool("enable_article_list", Config::enable_article_list);
    let article_column_name = row("article_column_name", Config::article_column_name);
    let local_column_name = row("local_column_name", Config::local_column_name);
    let article_count = row_parse("article_count", Config::article_count);
    let article_delay = row_parse("article_delay", Config::article_delay);
    let star_share_comment = row_parse("star_share_comment", Config::star_share_comment);
    let keep_star_comment = row_bool("keep_star_comment", Config::keep_star_comment);
    let flex = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(device)
        .with_child(database_uri)
        .with_child(database_json)
        .with_child(db_wrong_json)
        .with_child(daily_json)
        .with_child(challenge_json)
        .with_child(comments_json)
        .with_child(is_user)
        .with_child(daily_forever)
        .with_child(daily_delay)
        .with_child(challenge_count)
        .with_child(challenge_delay)
        .with_child(video_column_name)
        .with_child(video_count)
        .with_child(video_delay)
        .with_child(enable_article_list)
        .with_child(article_column_name)
        .with_child(local_column_name)
        .with_child(article_count)
        .with_child(article_delay)
        .with_child(star_share_comment)
        .with_child(keep_star_comment);
    flex.lens(ArgsState::config)
}

fn build_ui() -> impl Widget<ArgsState> {
    let radios = RadioGroup::new(
        CFG.devices
            .keys()
            .map(|key| (key.to_string(), key.to_string())),
    );
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("设置").on_click(|ctx, _data: &mut ArgsState, _env| {
                        let new_win = WindowDesc::new(setting_ui)
                            .title(LocalizedString::new("设置"))
                            .window_size((400.0, 700.0));
                        let command = Command::one_shot(commands::NEW_WINDOW, new_win);
                        ctx.submit_command(command, Target::Global);
                    }),
                )
                .with_child(radios.lens(Config::device).lens(ArgsState::config))
                .padding(5.0),
        )
        .with_child(
            Flex::row()
                .with_child(TextBox::new().with_placeholder("host").lens(Rules::host))
                .with_spacer(10.)
                .with_child(TextBox::new().with_placeholder("port").lens(Rules::port))
                .padding(5.0)
                .lens(ArgsState::rules),
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
                .with_child(
                    Flex::row()
                        .with_child(Checkbox::new("本地频道").lens(ArgsState::local))
                        .with_child(Button::new("运行").on_click(
                            move |_ctx, data: &mut ArgsState, _env| {
                                connect(&data.rules.host, &data.rules.port);
                                data.rules = CFG.devices[&data.config.device].clone();
                                let data = data.clone();
                                thread::spawn(move || super::local::Local::start(data));
                            },
                        )),
                )
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
                        connect(&data.rules.host, &data.rules.port);
                        data.rules = CFG.devices[&data.config.device].clone();
                        if !data.start {
                            data.start = true;
                            let data = data.clone();
                            thread::spawn(move || super::xuexi(data));
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
    let config = CFG.config.clone();
    let rules = CFG.devices[&config.device].clone();
    let initial_state = ArgsState {
        auto: true,
        local: true,
        article: true,
        video: true,
        challenge: true,
        daily: true,
        config: config,
        rules: rules,
        start: false,
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
