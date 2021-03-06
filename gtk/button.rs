// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A widget that emits a signal when clicked on

use std::libc::c_void;
use std::{ptr, cast};

use traits::{GtkWidget, GtkButton, GtkBin, GtkContainer, Signal};
use ffi;
use gtk::enums::GtkIconSize;

/** 
* Button — A widget that emits a signal when clicked on
*
* # Availables signals :
* * `activate` : Action
* * `clicked`: Action
* * `enter` : Run First
* * `leave` : Run First
* * `pressed` : Run First
* * `released` : Run First
*/
pub struct Button {
    priv pointer:           *ffi::C_GtkWidget,
    priv can_drop:          bool,
    priv signal_handlers:   ~[~SignalHandler]
}

impl Button {
    pub fn new() -> Option<Button> {
        let tmp_pointer = unsafe { ffi::gtk_button_new() };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_with_label(label: &str) -> Option<Button> {
        let tmp_pointer = unsafe { 
            label.with_c_str(|c_str| { 
                ffi::gtk_button_new_with_label(c_str) 
            }) 
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_with_menmonic(mnemonic: &str) -> Option<Button> {
        let tmp_pointer = unsafe { 
            mnemonic.with_c_str(|c_str| {
                ffi::gtk_button_new_with_mnemonic(c_str)
            })
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_from_icon_name(icon_name: &str, size: GtkIconSize) -> Option<Button> {
        let tmp_pointer = unsafe { 
            icon_name.with_c_str(|c_str| {
                ffi::gtk_button_new_from_icon_name(c_str, size) 
            })
        };
        check_pointer!(tmp_pointer, Button)
    }

    pub fn new_from_stock(stock_id: &str) -> Option<Button> {
        let tmp_pointer = unsafe { 
            stock_id.with_c_str(|c_str| {
                ffi::gtk_button_new_from_stock(c_str) 
            }) 
        };  
        check_pointer!(tmp_pointer, Button)
    }
}

impl_GtkWidget!(Button)
redirect_callback!(Button)
redirect_callback_widget!(Button)
struct_signal!(Button)
impl_signals!(Button)

impl GtkContainer for Button {}
impl GtkButton for Button {}
impl GtkBin for Button {}

