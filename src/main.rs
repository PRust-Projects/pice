#![recursion_limit = "512"]

mod config;

use std::path::PathBuf;

use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

use config::Config;

#[derive(Clone, Debug, Default)]
pub struct App {
    config: Config,
}

#[derive(Clone, Debug)]
pub enum Message {
    Exit,
    SetNumOfWords { num_words: String },
    ToggleCapitalization,
    TogglePunctuations,
    ToggleDigits,
    SetWordlist { wordlist: Option<PathBuf> },
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::SetNumOfWords { num_words } => {
                self.config.set_num_words(num_words);
                UpdateAction::Render
            }
            Message::ToggleCapitalization => {
                self.config.toggle_capitalization();
                UpdateAction::None
            }
            Message::TogglePunctuations => {
                self.config.toggle_punctuations();
                UpdateAction::None
            }
            Message::ToggleDigits => {
                self.config.toggle_digits();
                UpdateAction::None
            }
            Message::SetWordlist { wordlist } => {
                self.config.set_wordlist(wordlist);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Application::new_unwrap(Some("com.pchan.pice"), ApplicationFlags::empty())>
                <Window default_width=500 default_height=250 border_width=20 on destroy=|_| Message::Exit>
                    <Box orientation=Orientation::Vertical spacing=30>
                        <Box orientation=Orientation::Vertical spacing=10>
                            <Box>
                                <Label label="Number of words" />
                                <Entry Box::pack_type=PackType::End input_purpose=InputPurpose::Digits text=self.config.get_num_words() on changed=|entry| {
                                    Message::SetNumOfWords {
                                        num_words: entry.get_text().to_string(),
                                    }
                                }/>
                            </Box>
                            <Box>
                                <Label label="Include capitalization?" />
                                <CheckButton Box::pack_type=PackType::End active=self.config.get_capitalization() on toggled=|_| Message::ToggleCapitalization />
                            </Box>
                            <Box>
                                <Label label="Include punctuation?" />
                                <CheckButton Box::pack_type=PackType::End active=self.config.get_punctuations() on toggled=|_| Message::TogglePunctuations />
                            </Box>
                            <Box>
                                <Label label="Include number?" />
                                <CheckButton Box::pack_type=PackType::End active=self.config.get_digits() on toggled=|_| Message::ToggleDigits />
                            </Box>
                            <Box spacing=10>
                                <Label label="Which wordlist?" />
                                <Button Box::pack_type=PackType::End label="..." on clicked=|_| {
                                    let dialog = FileChooserNative::new(Some("Select File"), vgtk::current_window().as_ref(), FileChooserAction::Open, Some("_Select"), Some("_Cancel"));
                                    dialog.run();
                                    Message::SetWordlist {
                                        wordlist: dialog.get_filename(),
                                    }
                                }/>
                                <Label Box::pack_type=PackType::End label=self.config.get_wordlist() />
                            </Box>
                        </Box>
                        <Box>
                            <Button label="Reset" />
                            <Button Box::pack_type=PackType::End label="Next" />
                        </Box>
                    </Box>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<App>());
}
