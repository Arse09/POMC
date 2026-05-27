use glam::{UVec2, Vec2, uvec2, vec2};

use crate::gui::GuiRenderState;
use crate::gui::color::Color;
use crate::gui::screen::{InGamePhaseScreen, ScreenRenderState};
use crate::renderer::camera::CameraMode;
use crate::renderer::pipelines::menu_overlay::SpriteId;

pub enum InGameGuiCmd {
    Sprite {
        sprite_id: SpriteId,
        pos: Vec2,
        size: UVec2,
        tint: Color,
    },
}

#[derive(Default)]
pub struct InGameGuiRenderState {
    pub draw_cmds: Vec<InGameGuiCmd>,
}

impl InGameGuiRenderState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, cmd: InGameGuiCmd) {
        self.draw_cmds.push(cmd);
    }
}

#[derive(Clone, Copy)]
pub struct InGameGuiRenderCtx {
    pub gui_height: u32,
    pub gui_width: u32,

    pub vignette: bool,
    pub camera_mode: CameraMode,
}

pub struct InGameGui {
    hidden: bool,

    camera_overlays: CameraOverlays,
    crosshair: Crosshair,

    hotbar_and_decors: HotbarAndDecors,
    effects: Effects,
    boss_overlay: BossOverlay,

    sleep_overlay: SleepOverlay,

    scoreboard_sidebar: ScoreboardSidebar,
    overlay_message: OverlayMessage,
    title: Title,
    tab_list: TabList,
    subtitle_overlay: SubtitleOverlay,

    screen: Option<Box<dyn InGamePhaseScreen>>,
}

impl InGameGui {
    pub fn extract_render_state(&self, out: &mut GuiRenderState, ctx: InGameGuiRenderCtx) {
        let mut render_state = InGameGuiRenderState::new();

        if !self.hidden {
            self.camera_overlays
                .extract_render_state(&mut render_state, ctx);
            self.crosshair.extract_render_state(&mut render_state, ctx);

            self.hotbar_and_decors
                .extract_render_state(&mut render_state, ctx);
            self.effects.extract_render_state(&mut render_state, ctx);
            self.boss_overlay
                .extract_render_state(&mut render_state, ctx);
        }

        self.sleep_overlay
            .extract_render_state(&mut render_state, ctx);

        if !self.hidden {
            self.scoreboard_sidebar
                .extract_render_state(&mut render_state, ctx);
            self.overlay_message
                .extract_render_state(&mut render_state, ctx);
            self.title.extract_render_state(&mut render_state, ctx);
            self.tab_list.extract_render_state(&mut render_state, ctx);
            self.subtitle_overlay
                .extract_render_state(&mut render_state, ctx);
        } else if self.screen.is_some() {
            self.subtitle_overlay
                .extract_render_state(&mut render_state, ctx);
        }

        if let Some(screen) = self.screen.as_deref() {
            let mut render_state = ScreenRenderState::new();
            screen.extract_render_state(&mut render_state);
            out.push_screen_render_state(render_state);
        }
    }
}

pub struct CameraOverlays {}

impl CameraOverlays {
    const POWDERED_SNOW_OUTLINE_SPRITE: SpriteId = SpriteId::PowderedSnowOutline;

    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {
        if ctx.vignette {
            self.extract_vignette(out, ctx);
        }

        if ctx.camera_mode == CameraMode::FirstPerson {
            if false
            /* Java: player.isScoping() */
            {
                self.extract_spyglass_overlay(out, ctx)
            } else {
            }
        }
    }

    fn extract_vignette(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {
        // TODO: vignette rendering
        // Java: private void extractVignette()
        // (net.minecraft.client.gui/Gui:1027)
    }

    fn extract_spyglass_overlay(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {
        out.add(InGameGuiCmd::Sprite {
            sprite_id: Self::POWDERED_SNOW_OUTLINE_SPRITE,
            pos: vec2(0.0, 0.0),
            size: uvec2(ctx.gui_height, ctx.gui_width),
            tint: Color::default(),
        });
    }
}

pub struct Crosshair {}

impl Crosshair {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct HotbarAndDecors {}

impl HotbarAndDecors {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct Effects {}

impl Effects {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct BossOverlay {}

impl BossOverlay {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct SleepOverlay {}

impl SleepOverlay {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct ScoreboardSidebar {}

impl ScoreboardSidebar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct OverlayMessage {}

impl OverlayMessage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct Title {}

impl Title {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct TabList {}

impl TabList {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}

pub struct SubtitleOverlay {}

impl SubtitleOverlay {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_render_state(&self, out: &mut InGameGuiRenderState, ctx: InGameGuiRenderCtx) {}
}
