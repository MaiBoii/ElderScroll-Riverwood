use bevy::prelude::*;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(show_characters)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Character {
        name: "위대한 야를 발그루프".to_string(),
        race: Race::Nord,
        level: 20,
    });
    commands.spawn(Character {
        name: "이릴레스".to_string(),
        race: Race::Dunmer,
        level: 13,
    });
    commands.spawn(Character {
        name: "나짐".to_string(),
        race: Race::Redguard,
        level: 7,
    });
}

pub fn show_characters(query: Query<&Character>) {
    for character in query.iter() {
        println!("{}는 레벨 {}입니다.", character.name, character.level);
    }
}

#[derive(Component)]
pub struct Character {
    name: String,
    race: Race,
    level: u32,
}

#[derive(Component)]
pub struct Class {

}

#[derive(Debug)]
pub enum Race {
    Nord,
    Breton,
    Imperial,
    Redguard,
    Altmer,
    Bosmer,
    Dunmer,
    Orsimer,
    Argonian,
    Khajiit,
}