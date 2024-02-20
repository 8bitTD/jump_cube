pub mod common{
    pub const TOOLNAME: &str = "jump_cube";
}

pub mod assets{
    pub const DEFAULTFONT: &str = "fonts/NotoSansJP-Bold.ttf";
    pub const SOUNDJUMP: &str = "sound/jump.mp3";
    pub const SOUNDLANDING: &str = "sound/landing.wav";
    pub const SOUNDSIDELANDING: &str = "sound/side_landing.wav";
    pub const SOUNDGETNUMBER: &str = "sound/get_number.wav";
    pub const BGM: &str = "bgm/dungeon.mp3";
    pub const BGMENDING: &str = "bgm/ending.mp3";
}

pub mod value{
    //pub const DEFAULTWINDOWPOSX: i32 = -1220;
    pub const DEFAULTWINDOWPOSX: i32 = 450;
    pub const DEFAULTPOSX: f32 = 500.0;
    pub const DEFAULTPOSY: f32 = 19.0;
    pub const MAXSPEED: f32 = 7.5;
    pub const MAXSTAGE: u32 = 3;
    pub const DEFAULTTEXTSTAGEALPHA: f32 = 3.0;
    pub const BLOCKSIZE: f32 = 20.0;
    pub const ENDINGTEXTMOVE: f32 = 130.0;
    pub const DEFAULTCAMERAPOSX: f32 = 500.0;
    pub const DEFAULTCAMERAPOSY: f32 = 0.0;
    pub const VOLUME: f32 = 0.05;
    //pub const VOLUME: f32 = 0.00;
    pub const ISDEBUG: bool = false;
    pub const PER60FPS: f32 = 0.016;
    pub const FADETIME: f32 = 0.5;
}