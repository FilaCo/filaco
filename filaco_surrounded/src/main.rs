use bevy::prelude::App;
use filaco_surrounded::feature::SurroundedPlugin;

fn main() {
    App::new().add_plugins(SurroundedPlugin).run()
}
