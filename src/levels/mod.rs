use super::level::LevelStates;

pub fn level(index: usize) -> LevelStates {
    match index {
        0 => LevelStates {
            position: (0, 4),
            audio_id: 0,
            beats_per_min: 130.,
            patterns: vec![
                "x-x-x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-x-x---",
                "x-x-x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-x-x---",
                "x---x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-x-x---",
                "x---x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-x-x---",
            ],
        },
        _ => {
            panic!();
        }
    }
}
