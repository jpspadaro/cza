// chirperjax, a demo game built using the "gate" game library.
// Copyright (C) 2017-2019  Matthew D. Michelotti
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate gate;
extern crate collider;

gate_header!();

mod level_loader;
mod game_input;
mod game;
mod asset_id { include!(concat!(env!("OUT_DIR"), "/asset_id.rs")); }

use gate::{App, AppContext, AppInfo, KeyCode};
use gate::renderer::{Renderer, Affine};

use crate::game_input::{GameInput, InputEvent};
use crate::game::GameBoard;
use crate::asset_id::{AssetId, MusicId, SpriteId};
use crate::level_loader::LEVEL_COUNT;

/// Kicks off GameApp with appropriate window info.
fn main() {
    // TODO allow some flexibility in the app height
    let info = AppInfo::with_max_dims(game::SCREEN_PIXELS_HEIGHT * 16. / 9., game::SCREEN_PIXELS_HEIGHT)
                       .min_dims(game::SCREEN_PIXELS_HEIGHT * 4. / 3., game::SCREEN_PIXELS_HEIGHT)
                       .tile_width(8)
                       .title("Conrad's Zany Adventure")
                       .print_workload_info()
                       .print_gl_info();
    gate::run(info, GameApp::new());
}

/// Primary struct for game logic.
struct GameApp { input: GameInput, level: usize, board: GameBoard }

impl GameApp {
    /// GameApp constructor. Takes in GameInput, the starting level, and loads the game board.
    pub fn new() -> GameApp {
        GameApp { input: GameInput::new(), level: 0, board: level_loader::load(0) }
    }
    /// Advances the level and then calls level_loader::load to put it in the board.
    fn load_next_level(&mut self) {
        self.level = (self.level + 1) % LEVEL_COUNT;
        self.board = level_loader::load(self.level);
        if let Some(held_dir) = self.input.held_dir() {
            self.board.input(InputEvent::UpdateMovement(Some(held_dir)));
        }
    }
}

impl App<AssetId> for GameApp {
    /// Used when the game starts. Just loads music for now.
    fn start(&mut self, ctx: &mut AppContext<AssetId>) { ctx.audio.loop_music(MusicId::BgMusic); }
    /// Renders the gameboard to the window. Also includes the logic for additional graphics overlays on a per-level basis.
    fn render(&mut self, renderer: &mut Renderer<AssetId>, ctx: &AppContext<AssetId>) {
        self.board.draw(renderer, ctx);
        
		// INSERT NEW LEVEL SPLASHES HERE
		if self.level == 0 {
            let affine = &Affine::translate(0.5 * ctx.dims().0, 0.5 * ctx.dims().1).pre_scale(2.);
            renderer.sprite_mode().draw(affine, SpriteId::Instructions);
        }
		if self.level == 1 {
			let affine = &Affine::translate(0.5 * ctx.dims().0, 0.5 * ctx.dims().1).pre_scale(2.);
            renderer.sprite_mode().draw(affine, SpriteId::Level1Splash);
		}
		if self.level == 2 {
			let affine = &Affine::translate(0.5 * ctx.dims().0, 0.5 * ctx.dims().1).pre_scale(2.);
            renderer.sprite_mode().draw(affine, SpriteId::Level2Splash);
		}
    }
    /// Advances board and checks if the level is done.
    fn advance(&mut self, seconds: f64, ctx: &mut AppContext<AssetId>) {
        self.board.advance(seconds, &mut ctx.audio);
        if self.board.is_done() {
            self.load_next_level();
        }
    }
    /// Checks for key down events.
    fn key_down(&mut self, key: KeyCode, _: &mut AppContext<AssetId>) {
        if let Some(event) = self.input.key_down(key) {
            self.board.input(event);
        }
    }
    /// Checks for key up events.
    fn key_up(&mut self, key: KeyCode, _: &mut AppContext<AssetId>) {
        if let Some(event) = self.input.key_up(key) {
            self.board.input(event);
        }
    }
}
