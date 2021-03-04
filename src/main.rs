#![recursion_limit = "512"]

use std::cell::RefCell;
use std::path::PathBuf;

use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct App {
    num_words: RefCell<String>,
    capitalization_enabled: bool,
    punctuations_enabled: bool,
    digits_enabled: bool,
    wordlist: RefCell<PathBuf>,
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
                if num_words.parse::<usize>().is_ok() || num_words.is_empty() {
                    self.num_words.replace(num_words);
                }
                UpdateAction::Render
            }
            Message::ToggleCapitalization => {
                self.capitalization_enabled = !self.capitalization_enabled;
                UpdateAction::None
            }
            Message::TogglePunctuations => {
                self.punctuations_enabled = !self.punctuations_enabled;
                UpdateAction::None
            }
            Message::ToggleDigits => {
                self.digits_enabled = !self.digits_enabled;
                UpdateAction::None
            }
            Message::SetWordlist { wordlist } => {
                if let Some(wordlist) = wordlist {
                    self.wordlist.replace(wordlist);
                }
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
                                <Entry Box::pack_type=PackType::End input_purpose=InputPurpose::Digits text=self.num_words.borrow().clone() on changed=|entry| {
                                    Message::SetNumOfWords {
                                        num_words: entry.get_text().to_string(),
                                    }
                                }/>
                            </Box>
                            <Box>
                                <Label label="Include capitalization?" />
                                <CheckButton Box::pack_type=PackType::End active=self.capitalization_enabled on toggled=|_| Message::ToggleCapitalization />
                            </Box>
                            <Box>
                                <Label label="Include punctuation?" />
                                <CheckButton Box::pack_type=PackType::End active=self.punctuations_enabled on toggled=|_| Message::TogglePunctuations />
                            </Box>
                            <Box>
                                <Label label="Include number?" />
                                <CheckButton Box::pack_type=PackType::End active=self.digits_enabled on toggled=|_| Message::ToggleDigits />
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
                                <Label Box::pack_type=PackType::End label=self.wordlist.borrow().to_string_lossy().to_string() />
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
