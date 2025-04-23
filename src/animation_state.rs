//Holds various information on the various animations
pub struct AnimationState {
    pub start_screen_y : f32, 
    pub velocity : f32,
    pub acceleration : f32,
    pub going_down_start_screen : bool, 
    pub selected_item_main_menu : i32, 
    pub selected_item_gamemode_selector : i32,

}

impl AnimationState{
    pub fn new () -> Self {
        Self { 
            // Bouncing text in start_screen.rs
            start_screen_y: -50., 
            going_down_start_screen : false, 
            velocity : 0.,
            acceleration : 0.05,

            // Arrow Main Menu
            selected_item_main_menu : 0, // (0 - START GAME)....(1 - SETTINGS)

            // Arrow GameMode Selector
            selected_item_gamemode_selector : 0, // (0 - 1v1)....(1 - default) .....(2 - vs bots)
            

        }
    }
}