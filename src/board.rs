use yew::prelude::*;
use super::puzzle_parser;

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for String {
    fn is_letter(&self) -> bool {
        let letters = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
        letters.contains(&self.to_uppercase().as_str())
    }
}

pub struct Cell {
    index: usize,
    cell_number: usize,
    correct_value: String,
    display_value: String
}

impl Component for Cell {
    type Message = String;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            index: puzzle_parser::get_index(),
            cell_number: puzzle_parser::get_cell_number(),
            correct_value: puzzle_parser::next(),
            display_value: String::new()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if msg.is_letter() {
            self.display_value = msg.to_uppercase();
        } else if msg == String::from("Backspace") {
            self.display_value = String::new();
        }
        puzzle_parser::update_square(self.index, self.display_value == self.correct_value);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            {
                if self.correct_value != String::new() {
                    html!{
                        <button class="letter_square" onkeydown={ctx.link().callback(|k: KeyboardEvent| k.key())}>
                            <div class="cell_number">{
                                if self.cell_number != 0 {
                                    html!{self.cell_number}
                                } else {
                                    html!{}
                                }
                            }</div>
                            <div class="cell_value">{
                                if self.display_value != String::new() {
                                    html!{&self.display_value}
                                } else {
                                    html!{<br/>}
                                }
                            }</div>
                        </button>
                    }
                } else {
                    html!{
                        <div class="blank_square"/>
                    }
                }
            }
        }
    }
}
