use crate::gui::GuiDrawCmd;

#[derive(Default)]
pub struct ScreenRenderState {
    pub draw_cmds: Vec<GuiDrawCmd>,
}

impl ScreenRenderState {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait Screen {
    fn extract_render_state(&self, out: &mut ScreenRenderState) {}
}

pub trait InMenuPhaseScreen: Screen {}
pub trait ConnectingPhaseScreen: Screen {}
pub trait InGamePhaseScreen: Screen {}
