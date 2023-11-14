// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use crate::{
    scenes::{HeaderView, MainView},
    states::{
        local_state::{LocalState, LocalStateDelta},
        remote_state::RemoteState,
    },
    widget::{Context, FromDelta, Pod, Widget},
};

pub struct App {}

#[derive(Clone)]
pub enum Msg {
    Redraw,
}

impl FromDelta<LocalState> for Msg {
    fn from_delta(_delta: &LocalStateDelta) -> Option<Self> {
        Some(Self::Redraw)
    }
}

impl FromDelta<RemoteState> for Msg {
    fn from_delta(_delta: &Reaction) -> Option<Self> {
        Some(Self::Redraw)
    }
}

impl Widget for App {
    type Msg = Msg;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    fn on_event(&mut self, msg: Self::Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        match msg {
            Msg::Redraw => {},
        }
        ctx.redraw();
        Ok(())
    }

    fn view_opt(&self, _ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="app_view" data-tauri-drag-region="">
                // TODO: Render a global header
                <Pod<HeaderView> />
                <Pod<MainView> />
                // TODO: Render `ExpertView`
                // TODO: Render `Settings`
                // TODO: Render `Modal`
            </div>
        })
    }
}