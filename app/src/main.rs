extern crate piston_window;
extern crate shared;
extern crate sprite;
use std::{collections::HashMap, rc::Rc};

#[cfg(feature = "hot_reload_libs")]
extern crate hot_reload_lib;

#[cfg(not(feature = "hot_reload_libs"))]
extern crate update;

#[cfg(not(feature = "hot_reload_libs"))]
extern crate view;

#[cfg(feature = "hot_reload_libs")]
use hot_reload_lib::HotReloadLib;
use piston_window::*;
use sprite::{Scene, Sprite};

use std::path::PathBuf;

#[cfg(feature = "hot_reload_libs")]
struct HotReloadLibs {
    update: HotReloadLib,
    view: HotReloadLib,
}

#[cfg(feature = "hot_reload_libs")]
impl HotReloadLibs {
    fn new(hot_reload_libs_folder: &str) -> Self {
        Self {
            update: HotReloadLib::new(hot_reload_libs_folder, "update"),
            view: HotReloadLib::new(hot_reload_libs_folder, "view"),
        }
    }

    fn update_libs(&mut self) {
        if self.update.update() {
            println!("Reloaded update lib");
        }
        if self.view.update() {
            println!("Reloaded view lib");
        }
    }
}

struct Application {
    state: shared::State,

    #[cfg(feature = "hot_reload_libs")]
    libs: HotReloadLibs,
}

impl Application {
    fn new(_hot_reload_libs_folder: &str) -> Application {
        Application {
            state: shared::State::new(),

            #[cfg(feature = "hot_reload_libs")]
            libs: HotReloadLibs::new(_hot_reload_libs_folder),
        }
    }

    #[cfg(feature = "hot_reload_libs")]
    fn update_state(&mut self, delta: f64) {
        self.libs
            .update
            .load_symbol::<fn(f64, &mut shared::State)>("update_state")(
            delta, &mut self.state
        );
    }

    #[cfg(not(feature = "hot_reload_libs"))]
    fn update_state(&mut self, delta: f64) {
        update::update_state(delta, &mut self.state);
    }

    #[cfg(feature = "hot_reload_libs")]
    fn view_state(&mut self, res: &mut shared::GameAssets, ctx: Context, g: &mut G2d) {
        self.libs
            .view
            .load_symbol::<fn(&shared::State, &mut shared::GameAssets, Context, &mut G2d)>(
                "view_state",
            )(&self.state, res, ctx, g);
    }

    #[cfg(not(feature = "hot_reload_libs"))]
    fn view_state(&mut self, res: &mut shared::GameAssets, ctx: Context, g: &mut G2d) {
        view::view_state(&self.state, res, ctx, g);
    }
}

fn main() {
    println!("Creating window");
    use piston_window::*;
    let mut window: PistonWindow = WindowSettings::new("Hello piston!", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = Application::new("target/debug");

    println!("Locating assets");
    let assets_path = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut glyph_cache_manager = shared::GlyphCacheManager::new(
        &mut window,
        &assets_path,
        vec!["Kenney Pixel.ttf", "FiraCode-Regular.ttf"],
    );

    let mut res = shared::GameAssets {
        glyphs: glyph_cache_manager,
        textures: HashMap::new(),
    };

    let mut tex_ctx = window.create_texture_context();

    let texture = Rc::new(
        Texture::from_path(
            &mut tex_ctx,
            "./assets/textures/playerShip1_blue.png",
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Failed to load spaceship texture"),
    );
    res.textures.insert("Ship1_blue".into(), texture);
    // let mut sprite = Sprite::from_texture(texture);
    // sprite.set_position(300., 400.);

    // let mut scene = Scene::new();

    // scene.add_child(sprite);

    println!("Starting application loop");

    let mut counter = 0;

    while let Some(event) = window.next() {
        // println!("event: {:?}", event);
        #[cfg(feature = "hot_reload_libs")]
        app.libs.update_libs();

        // match event {
        //     Event::Loop(lEvent) => match lEvent {
        //         Loop::Update(UpdateArgs { dt }) => {
        //             app.update_state(dt);
        //         }
        //         Loop::Render(RenderArgs {
        //             ext_dt,
        //             window_size,
        //             draw_size,
        //         }) => {
        //             let w = &mut window;

        //             w.draw_2d(&event, |ctx, g, _d| {
        //                 // move what into where now
        //                 view::view_state_2(ctx, g, &app.state);
        //             });
        //             // app.view_state(&event);
        //         }
        //         _ => (),
        //     },
        //     _ => (),
        // };
        if let Some(args) = event.update_args() {
            app.update_state(args.dt);
        }
        window.draw_2d(&event, |ctx, graphics, device| {
            clear([1.0; 4], graphics);
            // rectangle(
            //     [1.0, 0.0, 0.0, 0.5], //red
            //     [0.0, 0.0, 100.0, 100.0],
            //     ctx.transform.clone().trans(50., 20.),
            //     graphics,
            // );
            // scene.draw(ctx.transform, graphics);
            counter += 1;
            let test_str = "aabcc<^&><*&()A0123456789abcdefghijklmnopqrstuvwxyz000->><";
            let str_to_draw = format!("Counter: {}", counter);
            // println!("'{}'", str_to_draw.as_str());

            res.glyphs.get_glyphs_cache(0, |glyphs| {
                text(
                    [0.0, 0.0, 0.0, 1.0],
                    20,
                    // str_to_draw.as_str(),
                    &test_str,
                    glyphs,
                    ctx.transform.clone().trans(0., 50.),
                    graphics,
                )
                .unwrap();

                text(
                    [0.0, 0.0, 0.0, 1.0],
                    20,
                    str_to_draw.as_str(),
                    glyphs,
                    ctx.transform.clone().trans(0., 140.),
                    graphics,
                )
                .unwrap();
            });

            res.glyphs.get_glyphs_cache(1, |glyphs| {
                text(
                    [0.0, 0.0, 0.0, 1.0],
                    20,
                    // str_to_draw.as_str(),
                    &test_str,
                    glyphs,
                    ctx.transform.clone().trans(0., 80.),
                    graphics,
                )
                .unwrap();
                text(
                    [0.0, 0.0, 0.0, 1.0],
                    20,
                    str_to_draw.as_str(),
                    glyphs,
                    ctx.transform.clone().trans(0., 120.),
                    graphics,
                )
                .unwrap();
            });
            // println!("before view_state");

            app.view_state(&mut res, ctx, graphics);
            // view::view_state(&app.state, ctx, graphics);
            // println!("after view_state");
            // Update glyphs before rendering.
            res.glyphs.flush_factory_encoder(device);
        });
    }
}
