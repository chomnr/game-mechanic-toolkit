/// Represents the different states of movement or action a user can be in during the game.
/// Each variant of this enum corresponds to a specific type of movement or lack thereof,
/// affecting gameplay mechanics such as the consumption or regeneration of stamina.
#[derive(PartialEq)]
enum UserMovement {
    Running,
    Walking,
    Resting,
}

impl Default for UserMovement {
    fn default() -> Self {
        Self::Resting
    }
}
// This constant defines the maximum stamina capacity that any individual can have in the game.
// It's a universal cap, meaning no individual's stamina can exceed this value under normal game conditions.
// This is used to set boundaries for stamina-related calculations and ensure game balance.
static MAX_STAMINA: usize = 100;

// This variable represents the current stamina of an individual within the game.
// It starts at the maximum value but can decrease or increase based on game events (like running, resting, etc.).
// Access and modify this value carefully to reflect the individual's stamina changes over time.
static mut YOUR_STAMINA: usize = 100;

// whenever the user moves.
fn on_user_move(/* e: UserEvent */) {
    /* let current_movement = e.get_movement_state() */
    let current_movement = UserMovement::default();
    if current_movement == UserMovement::Running {
        unsafe {
            // Check if the user has the required amount of stamina if not then deny the transition to running.
            if YOUR_STAMINA > 3 {
                YOUR_STAMINA = YOUR_STAMINA - 3
            }
            change_movement(UserMovement::Walking)
        }
    }
    if current_movement == UserMovement::Walking {
        unsafe {
            // stop the user's current stamina from increasing.
            if YOUR_STAMINA < MAX_STAMINA {
                YOUR_STAMINA = YOUR_STAMINA + 1
            }
        }
    }
    if current_movement == UserMovement::Resting {
        unsafe {
            // stop the user's current stamina from increasing.
            if YOUR_STAMINA < MAX_STAMINA {
                YOUR_STAMINA = YOUR_STAMINA + 2
            }
        }
    }
}

fn change_movement(movement: UserMovement) {}
