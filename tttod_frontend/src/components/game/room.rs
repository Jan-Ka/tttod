use super::PlayerList;
use std::collections::HashMap;
use tttod_data::{Challenge, ChallengeResult, Player, FAILURES_NEEDED, SUCCESSES_NEEDED};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use ybc::{HeaderSize, TileCtx, TileSize};
use yew::prelude::*;

pub struct Room {
    link: ComponentLink<Self>,
    props: Props,
    dismissed_gm_modal: bool,
    modal_bridge: yew::agent::Dispatcher<ybc::ModalCloser>,
    rejected_secret: Option<String>,
    show_gm_notification: NodeRef,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub room_idx: usize,
    pub gm: Uuid,
    pub successes: usize,
    pub failures: usize,
    pub state: RoomState,
    pub reject_secret_callback: yew::Callback<()>,
}

pub enum Msg {
    DismissGMModal,
    RejectSecret,
}

impl Component for Room {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            dismissed_gm_modal: false,
            modal_bridge: ybc::ModalCloser::dispatcher(),
            rejected_secret: None,
            show_gm_notification: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DismissGMModal => {
                self.dismissed_gm_modal = true;
                self.modal_bridge
                    .send(ybc::ModalCloseMsg("gm-notification".to_owned()));
                true
            }
            Msg::RejectSecret => {
                self.rejected_secret = self.props.state.clue.clone();
                self.props.reject_secret_callback.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.rejected_secret.is_some() && props.state.clue != self.rejected_secret {
            self.rejected_secret = None;
        }
        if self.props.gm == self.props.player_id && !self.dismissed_gm_modal {
            if let Some(show) = self.show_gm_notification.get() {
                show.unchecked_ref::<web_sys::HtmlElement>().click();
            }
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        log::debug!("self.dismissed_gm_modal = {:?}", self.dismissed_gm_modal);
        log::debug!("gm: {:?}", self.props.gm == self.props.player_id);
        let dismiss_modal = self.link.callback(|_| Msg::DismissGMModal);
        let reject_secret_handler = self.link.callback(|_| Msg::RejectSecret);
        // let player = self.props.players.get(&self.props.player_id);
        html! {
            <ybc::Tile vertical=true ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Parent>
                    <ybc::Tile ctx=TileCtx::Child size=TileSize::Nine>
                        <ybc::Title size=HeaderSize::Is1>{format!("Room {} of {}", self.props.room_idx + 1, self.props.players.len())}</ybc::Title>
                    </ybc::Tile>
                    <ybc::Tile classes="button-with-player-list" ctx=TileCtx::Child size=TileSize::Three>
                        <PlayerList player_id=self.props.player_id players=&self.props.players/>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Three>
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{"Failures:"}</label>
                        </div>
                        <div class="field-body is-align-items-center">
                            <div class="field">
                                <p class="control">
                                    <ybc::Progress classes="is-danger" max={ FAILURES_NEEDED as f32 } value={ self.props.failures as f32 }/>
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="field is-horizontal">
                        <div class="field-label is-normal">
                            <label class="label">{"Successes:"}</label>
                        </div>
                        <div class="field-body is-align-items-center">
                            <div class="field">
                                <p class="control">
                                <ybc::Progress classes="is-primary" max={ SUCCESSES_NEEDED as f32 } value={ self.props.successes as f32 }/>
                                </p>
                            </div>
                        </div>
                    </div>
                </ybc::Tile>
                // <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Twelve>
                // </ybc::Tile>
                {
                    if self.props.gm == self.props.player_id {
                        html! {
                            <ybc::ModalCard id="gm-notification" trigger={
                                html! {
                                    <div class="is-invisible" ref=self.show_gm_notification.clone()></div>
                                }
                            } title="You Are the Game Master Now!" body={
                                html! {
                                    <>
                                        <ybc::Box>
                                            <ybc::Title size=HeaderSize::Is5>{"Use this secret to build your room:"}</ybc::Title>
                                            {
                                                if let Some(clue) = &self.props.state.clue {
                                                    html! {
                                                        <p>{clue}</p>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </ybc::Box>
                                        <ybc::Title size=HeaderSize::Is4>{"Help With Creating Rooms"}</ybc::Title>
                                        <p class="block">
                                            {"Use these for inspiration! Or pick three and combine with flair to create a chamber that reflects the secret shown above."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Puzzles & Riddles"}</ybc::Title>
                                        <p class="block">
                                            {"Strange runic patterns, carefully arranged gems of power, statues \
                                            with rotating heads, movable dials, a chessboard floor, countless \
                                            levers, whispered rhymes sung by a thousand lipless mouths."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Environmental Obstacles"}</ybc::Title>
                                        <p class="block">
                                            {"Spike pits, lava stream, walls closing in on each other, rapidly \
                                            rising water, narrow ledges, decaying or invisible bridges, unnatural \
                                            snow or sandstorms."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Traps"}</ybc::Title>
                                        <p class="block">
                                            {"Flaming jets, poison darts, trapped chests, fake floors, cursed \
                                            altars, rolling boulders, deadly illusions, reverse or shifting \
                                            gravity, cursed magical items."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Enemies"}</ybc::Title>
                                        <p class="block">
                                            {"Venomous snakes, roaming mummies, dark spirits, swarm of scarab \
                                            beetles or scorpions, Nazis, cult members, and of course the most \
                                            terrifying of all: evil archeologists."}
                                        </p>
                                    </>
                                }
                            } footer={
                                html! {
                                    <>
                                        {
                                            if self.props.room_idx > 0 {
                                                html! {
                                                    <ybc::Button loading=self.rejected_secret.is_some() onclick=reject_secret_handler>{"This clue conficts with established lore"}</ybc::Button>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        <ybc::Button onclick=dismiss_modal.clone()>{"My room is ready!"}</ybc::Button>
                                    </>
                                }
                            }/>
                        }
                    } else {
                        html! {}
                    }
                }
            </ybc::Tile>
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RoomState {
    pub challenge: Option<Challenge>,
    pub challenge_result: Option<ChallengeResult>,
    pub clue: Option<String>,
}
