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
use shared::Actor;
use sprite::{Scene, Sprite};

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

    let glyph_cache_manager = shared::GlyphCacheManager::new(
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
    res.textures.insert("Ship1_blue".into(), texture.clone());

    let mut sprite = Sprite::from_texture(texture.clone());
    sprite.set_position(100., 400.);

    let mut scene = Box::new(Scene::new());
    let id = scene.add_child(sprite);
    app.state.actors.insert(id, Box::new(Actor::new(id)));

    app.state.scenes.push(scene);

    println!("Starting application loop");

    while let Some(event) = window.next() {
        // println!("event: {:?}", event);
        #[cfg(feature = "hot_reload_libs")]
        app.libs.update_libs();

        if let Some(args) = event.update_args() {
            app.update_state(args.dt);
        }
        window.draw_2d(&event, |ctx, graphics, device| {
            app.view_state(&mut res, ctx, graphics);
            res.glyphs.flush_factory_encoder(device);
        });
    }
}
