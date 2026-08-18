use bevy::prelude::*;
use fonts::FontHandles;
use settings::Settings;

pub fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    info!("Loading fonts");

    let loading_screen_tooltip_font =
        asset_server.load(&settings.fonts.loading_screen_tooltip_font);

    let loading_screen_loading_text_font =
        asset_server.load(&settings.fonts.loading_screen_loading_text_font);

    let main_font = asset_server.load(&settings.fonts.main_font);

    let button_font = asset_server.load(&settings.fonts.button_font);

    let fonts = FontHandles {
        loading_screen_tooltip_font,
        loading_screen_loading_text_font,
        main_font,
        button_font,
    };
    commands.insert_resource(fonts);
}
