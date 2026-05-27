use winit::dpi::PhysicalSize;

use crate::gui::in_game::{InGameGui, InGameGuiCmd, InGameGuiRenderCtx, InGameGuiRenderState};
use crate::gui::screen::{ConnectingPhaseScreen, InMenuPhaseScreen, ScreenRenderState};
use crate::renderer::camera::CameraMode;

pub mod color;
mod in_game;
mod screen;
mod screens;

pub enum GuiDrawCmd {
    InGame(InGameGuiCmd),
}

#[derive(Default)]
pub struct GuiRenderState {
    /// Before blur draw commands
    pub bb_draw_cmds: Vec<GuiDrawCmd>,
    /// After blur draw commands
    pub ab_draw_cmds: Vec<GuiDrawCmd>,
}

impl GuiRenderState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_in_game_gui_render_state(&mut self, state: InGameGuiRenderState) {
        self.bb_draw_cmds
            .extend(state.draw_cmds.into_iter().map(GuiDrawCmd::InGame));
    }

    pub fn push_screen_render_state(&mut self, state: ScreenRenderState) {
        self.ab_draw_cmds.extend(state.draw_cmds);
    }
}

#[derive(Clone, Copy)]
pub struct GuiRenderCtx {
    vignette: bool,
    camera_mode: CameraMode,
}

pub enum GuiPhase {
    Setup,
    InMenu {
        screen: Box<dyn InMenuPhaseScreen>,
    },
    Connecting {
        screen: Box<dyn ConnectingPhaseScreen>,
    },
    InGame(InGameGui),
}

pub struct Gui {
    hidden: bool,
    phase: GuiPhase,
    height: u32,
    width: u32,
}

impl Gui {
    pub fn new(window_size: PhysicalSize<u32>, gui_scale_setting: u32) -> Self {
        let gui_height = window_size.height / gui_scale_setting;
        let gui_width = window_size.width / gui_scale_setting;

        Self {
            hidden: false,
            phase: GuiPhase::Setup,
            height: gui_height,
            width: gui_width,
        }
    }

    pub fn extract_render_state(&self, ctx: GuiRenderCtx) -> GuiRenderState {
        let mut render_state = GuiRenderState::new();

        match &self.phase {
            GuiPhase::Setup => {}
            GuiPhase::InMenu { screen } => {
                let mut screen_state = ScreenRenderState::new();
                screen.extract_render_state(&mut screen_state);
                render_state.push_screen_render_state(screen_state);
            }
            GuiPhase::Connecting { screen } => {
                let mut screen_state = ScreenRenderState::new();
                screen.extract_render_state(&mut screen_state);
                render_state.push_screen_render_state(screen_state);
            }
            GuiPhase::InGame(gui) => gui.extract_render_state(
                &mut render_state,
                InGameGuiRenderCtx {
                    gui_height: self.height,
                    gui_width: self.width,
                    vignette: ctx.vignette,
                    camera_mode: ctx.camera_mode,
                },
            ),
        }

        render_state
    }
}
