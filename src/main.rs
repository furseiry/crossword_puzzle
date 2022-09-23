mod board;
mod puzzle_parser;

use yew::prelude::*;

use board::Cell;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            {
                if !puzzle_parser::is_complete() {
                    html!{<>
                        <div class="board">
                            {for (0..225).map(|_| html!{<Cell/>})}
                        </div>
                        <div class="clues">
                            <div>
                                <font size="5">{"Across"}</font>
                                <hr/>
                                {for crossword_puzzle::ACROSS_CLUES.iter().map(|clue| {
                                    html!{<div class="clue">
                                        <span class="clue_number">{clue.0}</span>
                                        <span class="clue_text">{clue.1}</span>
                                    </div>}
                                })}
                            </div>
                            <div>
                                <font size="5">{"Down"}</font>
                                <hr/>
                                {for crossword_puzzle::DOWN_CLUES.iter().map(|clue| {
                                    html!{<div class="clue">
                                        <span class="clue_number">{clue.0}</span>
                                        <span class="clue_text">{clue.1}</span>
                                    </div>}
                                })}
                            </div>
                        </div>
                        <button class="check_button" onclick={ctx.link().callback(|_| ())}>{"Check puzzle"}</button>
                    </>}
                } else {
                    html!{<div>
                        {"Congratulations, you did it!"}
                    </div>}
                }
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
