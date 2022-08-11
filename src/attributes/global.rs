/// non-empty no-spaces
#[derive(Default)]
pub struct Id(String);

#[derive(Default)]
pub struct BCP47(String);
#[derive(Default)]
pub struct Class(String);
#[derive(Default)]
pub struct AccessKey(String);

#[derive(Default)]
pub enum Boolish {
    True,
    False,
    #[default]
    Inherit,
}

pub enum Dir {
    LeftToRight,
    RightToLeft,
    Auto,
}

#[derive(Default)]
pub struct Common {
    id: Id,
    lang: Option<BCP47>,
    title: String,
    dir: Option<Dir>,
    class: Class,
    style: String,
    draggable: bool,
    hidden: bool,
    access_key: AccessKey,
    content_editable: Boolish,
    context_menu: Id,
    tabindex: Option<i32>,
    spellcheck: Boolish,
}

#[derive(Default)]
pub struct Events {
    abort: String,
    blur: String,
    can_play: String,
    can_play_through: String,
    change: String,
    click: String,
    context_menu: String,
    cue_change: String,
    double_click: String,
    drag: String,
    drag_end: String,
    drag_enter: String,
    drag_leave: String,
    drag_over: String,
    drag_start: String,
    drop: String,
    duration_change: String,
    emptied: String,
    ended: String,
    error: String,
    focus: String,
    input: String,
    invalid: String,
    keydown: String,
    keypress: String,
    keyup: String,
    load: String,
    loaded_data: String,
    loaded_metadata: String,
    load_start: String,
    mouse_down: String,
    mouse_move: String,
    mouse_out: String,
    mouse_over: String,
    mouse_up: String,
    mouse_wheel: String,
    pause: String,
    play: String,
    playing: String,
    progress: String,
    ratechange: String,
    ready_state_change: String,
    reset: String,
    scroll: String,
    seeked: String,
    seeking: String,
    select: String,
    show: String,
    stalled: String,
    submit: String,
    suspend: String,
    time_update: String,
    volume_change: String,
    waiting: String,
}
