extern crate rand;
#[macro_use]
extern crate glayout;

mod resource;
mod cover;
mod level;
mod levels;

extern {
    pub fn play_audio(id: i32);
    pub fn get_audio_current_time(id: i32) -> f32;
}

fn loading() {
    let mut canvas = glayout::canvas::Canvas::new(0);
    canvas.ctx(|ctx| {
        ctx.set_clear_color(0., 0., 0., 1.);
        let pixel_ratio = ctx.device_pixel_ratio();
        ctx.set_canvas_size(1280, 720, pixel_ratio);
    });

    // load resource
    let mut loader = {
        let context = canvas.context();
        let mut ctx = context.borrow_mut();
        resource::ResourceLoader::new(ctx.canvas_config())
    };
    loader.load_image("snake_normal_1", "resource/snake_normal_1.png");
    loader.load_image("snake_normal_2", "resource/snake_normal_2.png");
    loader.load_image("snake_normal_3", "resource/snake_normal_3.png");
    loader.load_image("snake_fail", "resource/snake_fail.png");
    loader.load_image("flower", "resource/flower.png");
    resource::ResourceLoader::ended(loader, move |resource| {
        let mut c = cover::Cover::new(canvas.context(), resource);
        c.show();
    });
}

fn main() {
    glayout::init();
    glayout::set_log_level_num(-1);
    glayout::main_loop(loading);
}
