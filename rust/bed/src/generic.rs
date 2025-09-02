use crate::controller::is_xpub_valid;

pub type XpubList = Vec<(String, bool /* valid */, bool /* selected */)>;

#[macro_export]
macro_rules! lock {
    ($s: ident) => {
        $s.inner.lock().expect("poisoned")
    };
}

#[macro_export]
macro_rules! wrap {
    ($strct: ident) => {
        impl $strct {
            pub fn set_selected(&mut self, index: usize, selected: bool) {
                lock!(self)._set_selected(index, selected);
            }
            pub fn edit_xpub(&mut self, index: usize, xpub: String) {
                lock!(self)._edit_xpub(index, xpub);
            }
            pub fn add_xpub(&mut self) {
                lock!(self)._add_xpub();
            }
            pub fn remove_xpub(&mut self, index: usize) {
                lock!(self)._remove_xpub(index);
            }
        }

        impl From<$strct> for Screen {
            fn from(value: $strct) -> Screen {
                Screen::from(&*lock!(value))
            }
        }
    };
}

pub trait XpubScreen {
    fn xpubs_mut(&mut self) -> &mut XpubList;
    fn update(&self);
    fn _set_selected(&mut self, index: usize, selected: bool) {
        let xpubs = self.xpubs_mut();
        if index >= xpubs.len() {
            return;
        }
        let entry = xpubs.get_mut(index).expect("checked");
        entry.2 = selected;
    }
    fn _edit_xpub(&mut self, index: usize, xpub: String) {
        let xpubs = self.xpubs_mut();
        let valid = is_xpub_valid(&xpub);
        if index >= xpubs.len() {
            log::error!("Decrypt::edit_xpub(): index out of bound.");
            return;
        }
        let entry = xpubs.get_mut(index).expect("checked");
        entry.0 = xpub;
        entry.1 = valid;
        if !valid {
            entry.2 = false;
        }
        self.update();
    }
    fn _add_xpub(&mut self) {
        self.xpubs_mut().push((String::new(), false, false));
        self.update();
    }
    fn _remove_xpub(&mut self, index: usize) {
        let xpubs = self.xpubs_mut();
        if index < xpubs.len() {
            xpubs.remove(index);
        }
        self.update();
    }
    fn contains(&mut self, key: &str) -> bool {
        for (k, _, _) in self.xpubs_mut() {
            if k == key {
                return true;
            }
        }
        false
    }
}
