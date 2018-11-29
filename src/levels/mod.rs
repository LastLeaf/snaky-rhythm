use super::level::LevelStates;

pub fn level(index: usize) -> LevelStates {
    match index {
        1 => LevelStates {
            position: (0, 4),
            audio_id: 0,
            beats_per_min: 130.,
            patterns: vec![
                "x---x-x-",
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
                "x---x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-------",
            ],
        },
        2 => LevelStates {
            position: (0, 4),
            audio_id: 1,
            beats_per_min: 140.,
            patterns: vec![
                "x---x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-xxxxx-",
                "x---x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-xxxxx-",
                "x---x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-xxxxx-",
                "x---x-x-",
                "x-x-x-x-",
                "x---x-x-",
                "x-xxxxx-",
            ],
        },
        0 => LevelStates {
            position: (0, 4),
            audio_id: 2,
            beats_per_min: 110.,
            patterns: vec![
                "x-x-x-x-",
                "x-x-x-x-",
                "x-x-x-x-",
                "x-xxxxx-",
                "x-x-x-x-",
                "x-x-x-x-",
                "x-x-x-x-",
                "x-xxxxx-",
                "x-x-x-x-",
                "x-x-x-x-",
                "x-x-x-x-",
                "x-xxxxx-",
                "x-x-x-x-",
                "x-x-x-x-",
                "x-x-x-x-",
                "x-xxxxx-",
            ],
        },
        _ => {
            panic!();
        }
    }
}
