#![windows_subsystem = "windows"] // disable the console window when running the release version

use hostboi::{get_favorites, favorite, swap};
extern crate native_windows_gui as nwg;
use nwg::NativeUi;


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}


#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    box_edit: nwg::TextInput,
    swap_button: nwg::Button,
    fav_combo: nwg::ComboBox<String>,
}

impl BasicApp {
    fn swap_to_box(&self) {
        let box_num_parse = self.box_edit.text().parse::<i32>(); // is this the only way to get the actual integer out?
        match box_num_parse {
            Ok(box_num) => swap(box_num).expect("failedl to swap"),
            Err(_) => ()  // TODO handle this
        };
    }

    fn switch_to_favorite(&self) {
        let selection = self.fav_combo.selection_string();
        println!("selected something");
        match selection {
            Some(selection) => favorite(selection).expect("failed to switch to favorite"),
            None => ()
        };
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct BasicAppUi {
        inner: Rc<BasicApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>
    }

    impl nwg::NativeUi<BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
            use nwg::Event as E;


            
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((200, 90))
                .position((300, 300))
                .title("HostBoi")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .size((85, 25))
                .position((10, 10))
                .parent(&data.window)
                .build(&mut data.box_edit)?;

            nwg::Button::builder()
                .size((85, 25))
                .position((105, 10))
                .text("Swap")
                .parent(&data.window)
                .build(&mut data.swap_button)?;

            nwg::ComboBox::builder()
                .size((180, 25))
                .position((10, 45))
                .parent(&data.window)
                .collection(get_favorites().expect("failed to get favorites")) // TODO handle this better!
                .build(&mut data.fav_combo)?;

            // Wrap-up
            let ui = BasicAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &ui.swap_button {
                                BasicApp::swap_to_box(&ui);
                            }
                        },
                        E::OnComboxBoxSelection => {
                            if &handle == &ui.fav_combo {
                                BasicApp::switch_to_favorite(&ui);
                            }
                        },
                        E::OnWindowClose => 
                            if &handle == &ui.window {
                                nwg::stop_thread_dispatch();
                            },
                        _ => {}
                    }
                }
            };

           *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&ui.window.handle, handle_events));

            return Ok(ui);
        }
    }

    impl Drop for BasicAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }
}
