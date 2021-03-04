#![recursion_limit = "1024"]

mod config;

use std::cell::RefCell;
use std::path::PathBuf;

use rand::seq::SliceRandom;
use vgtk::ext::*;
use vgtk::lib::gio::ApplicationFlags;
use vgtk::lib::gdk::SELECTION_CLIPBOARD;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

use config::Config;

#[derive(Clone, Debug)]
pub struct App {
    clipboard: Clipboard,
    config: Config,
    password: RefCell<String>,
    pass2: String,
    step: Step,
}

impl Default for App {
    fn default() -> Self {
        App {
            clipboard: Clipboard::get(&SELECTION_CLIPBOARD),
            config: Config::default(),
            password: RefCell::new(String::new()),
            pass2: String::new(),
            step: Step::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Exit,
    SetStep { step: Step },
    SetNumOfWords { num_words: String },
    ToggleCapitalization,
    TogglePunctuations,
    ToggleDigits,
    SetWordlist { wordlist: Option<PathBuf> },
    GeneratePassword,
    CopyToClipboard,
}

#[derive(Clone, Debug)]
pub enum Step {
    Configuration,
    PasswordGeneration,
}

impl Default for Step {
    fn default() -> Self {
        Step::Configuration
    }
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
            Message::SetStep { step } => {
                self.step = step;
                UpdateAction::Render
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
            Message::GeneratePassword => {
                let candidates = vec!["hello", "foo", "bar", "omega", "delta"];
                self.password.replace(candidates.choose(&mut rand::thread_rng()).unwrap().to_string());
                UpdateAction::Render
            }
            Message::CopyToClipboard => {
                self.clipboard.set_text(&self.password.borrow());
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Application::new_unwrap(Some("com.pchan.pice"), ApplicationFlags::empty())>
                <Window default_width=500 default_height=250 border_width=20 on destroy=|_| Message::Exit>
                    {
                        match self.step {
                            Step::Configuration => {
                                gtk! {
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
                                            <Button Box::pack_type=PackType::End label="Next" on clicked=|_| Message::SetStep {
                                                step: Step::PasswordGeneration,
                                            }/>
                                        </Box>
                                    </Box>
                                }
                            }
                            Step::PasswordGeneration => {
                                gtk! {
                                    <ButtonBox orientation=Orientation::Vertical layout=ButtonBoxStyle::Spread>
                                        <Label markup="<big><b>Your Password Is:</b></big>" />
                                        <Label text=self.password.borrow().clone() />
                                        <ButtonBox layout=ButtonBoxStyle::Spread spacing=20>
                                            <Button label="Back" on clicked=|_| Message::SetStep {
                                                step: Step::Configuration,
                                            } />
                                            <Button label="Regenerate password" on clicked=|_| Message::GeneratePassword />
                                            <Button label="Copy" on clicked=|_| Message::CopyToClipboard />
                                        </ButtonBox>
                                    </ButtonBox>
                                }
                            }
                        }
                    }
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<App>());
}
