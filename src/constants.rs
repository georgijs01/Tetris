use std::time::Duration;

// The different types of input available to the player
pub const INPUT_TYPES: [&str; 5] = ["left", "right", "rotate", "descend", "drop"];

// The time after which holding down a key will register as multiple key presses
pub const KEY_REPEAT_THRESHOLD: Duration = Duration::from_millis(300);

// How often a key press will be registered while held down
pub const KEY_REPEAT_TIME: Duration = Duration::from_millis(70);

// Size of the camera view, should have the same ratio as window size
pub const VIEW_WIDTH: f32 = 320.;
pub const VIEW_HEIGHT: f32 = 640.;

// The spawn point on the grid
pub const SPAWN_POINT: (i32, i32) = (8, 42);

pub const ARENA_WIDTH: i32 = 20;
pub const ARENA_HEIGHT: i32 = 46;

// An array of all wall kicks to try and perform
pub const WALL_KICKS: [[[(i32, i32); 5]; 8]; 2] = [
    // Wall Kicks for pieces: J, L, S, T, Z
    [
        // 0 >> 1
        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        // 1 >> 0
        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        // 1 >> 2
        [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
        // 2 >> 1
        [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        // 2 >> 3
        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
        // 3 >> 2
        [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
        // 0 >> 3
        [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
        // 3 >> 0
        [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    ],
    // Wall Kicks for pieces: I
    [
        // 0 >> 1
        [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
        // 1 >> 0
        [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        // 1 >> 2
        [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
        // 2 >> 1
        [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
        // 2 >> 3
        [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
        // 3 >> 2
        [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
        // 0 >> 3
        [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
        // 3 >> 0
        [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    ]
];