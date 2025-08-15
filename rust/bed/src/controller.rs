use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    sync::mpsc,
};

use encrypted_backup::{
    miniscript::{Descriptor, DescriptorPublicKey},
    EncryptedBackup,
};

use crate::{
    bed::{Mode, Notification, Screen},
    decrypt::Decrypt,
    encrypt::Encrypt,
};

pub type XpubList = Vec<(String, bool /* valid */, bool /* selected */)>;

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

#[derive(Debug)]
pub struct Controller {
    encrypt: Encrypt,
    decrypt: Decrypt,
    notif: mpsc::Receiver<Notification>,
    error: mpsc::Receiver<String>,
    notif_sender: mpsc::Sender<Notification>,
    error_sender: mpsc::Sender<String>,
}

impl Controller {
    pub fn new() -> Self {
        let (notif_sender, notif) = mpsc::channel();
        let (error_sender, error) = mpsc::channel();
        let encrypt = Encrypt::new(notif_sender.clone(), error_sender.clone());
        let decrypt = Decrypt::new(notif_sender.clone(), error_sender.clone());
        Self {
            encrypt,
            decrypt,
            notif,
            error,
            notif_sender,
            error_sender,
        }
    }

    pub fn encrypt(&mut self) -> &mut Encrypt {
        &mut self.encrypt
    }
    pub fn decrypt(&mut self) -> &mut Decrypt {
        &mut self.decrypt
    }

    pub fn poll(&self) -> Notification {
        self.notif.try_recv().ok().into()
    }

    pub fn error(&self) -> String {
        self.error.try_recv().unwrap_or_default()
    }
    pub fn encrypt_screen(&mut self) -> Screen {
        self.encrypt().clone().into()
    }
    pub fn decrypt_screen(&mut self) -> Screen {
        self.decrypt().clone().into()
    }
    fn read_file(&self, file_path: String) -> Option<Vec<u8>> {
        let path = match PathBuf::from_str(&file_path) {
            Ok(p) => p,
            Err(_) => {
                let _ = self.error_sender.send(format!("Invalid path: {file_path}"));
                return None;
            }
        };
        if !path.exists() || !path.is_file() {
            let _ = self
                .error_sender
                .send(format!("File {file_path} do not exists!"));
            return None;
        };

        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                let _ = self
                    .error_sender
                    .send(format!("fail to open {file_path}: {e:?}"));
                return None;
            }
        };

        let mut bytes = vec![];
        let read = match file.read_to_end(&mut bytes) {
            Ok(r) => r,
            Err(e) => {
                let _ = self
                    .error_sender
                    .send(format!("fail to open {file_path}: {e:?}"));
                return None;
            }
        };
        if read == 0 {
            let _ = self.error_sender.send(format!("File {file_path} empty"));
            return None;
        }
        Some(bytes)
    }
    pub fn drag_n_drop(&mut self, file_path: String, mode: Mode) {
        let bytes = match self.read_file(file_path) {
            Some(b) => b,
            None => return,
        };
        match mode {
            Mode::Encrypt => self.drag_n_drop_encrypt(bytes),
            Mode::Decrypt => self.drag_n_drop_decrypt(bytes),
            _ => unreachable!(),
        }
    }
    pub fn drag_n_drop_encrypt(&mut self, bytes: Vec<u8>) {
        if let Ok(str) = String::from_utf8(bytes) {
            if self.try_drop_key(&str, Mode::Encrypt) {
                let _ = self.notif_sender.send(Notification::UpdateEncrypt);
                return;
            }
            if Descriptor::<DescriptorPublicKey>::from_str(&str).is_ok() {
                self.encrypt().set_descriptor(str);
                let _ = self.notif_sender.send(Notification::UpdateEncrypt);
                return;
            }
        }
        let _ = self
            .error_sender
            .send("Fail to import file: key or descriptor expected!".to_string());
    }
    fn try_drop_key(&mut self, key_str: &str, mode: Mode) -> bool {
        let dpk_str = key_str.trim().to_string();
        if DescriptorPublicKey::from_str(&dpk_str).is_ok() {
            match mode {
                Mode::Encrypt => {
                    if !self.encrypt().contains(&dpk_str) {
                        self.encrypt().xpubs_mut().push((dpk_str, true, true));
                    }
                }
                Mode::Decrypt => {
                    if !self.decrypt().contains(&dpk_str) {
                        self.decrypt().xpubs_mut().push((dpk_str, true, true));
                    }
                }
                _ => unreachable!(),
            };
            return true;
        }
        false
    }
    pub fn drag_n_drop_decrypt(&mut self, bytes: Vec<u8>) {
        // check if it's a valid encrypted payload
        if EncryptedBackup::new().set_encrypted_payload(&bytes).is_ok() {
            self.decrypt().set_ciphertext(bytes);
            let _ = self.notif_sender.send(Notification::UpdateDecrypt);
            return;
        }

        // check if it's a valid dpk
        if let Ok(key_str) = String::from_utf8(bytes) {
            if self.try_drop_key(&key_str, Mode::Decrypt) {
                let _ = self.notif_sender.send(Notification::UpdateDecrypt);
                return;
            }
        }

        let _ = self
            .error_sender
            .send("File content cannot be decoded.".to_string());
    }
}

pub fn write_to_file(bytes: &[u8], path: String, error: mpsc::Sender<String>) {
    let file_path = match PathBuf::from_str(&path) {
        Ok(p) => p,
        Err(_) => {
            let _ = error.send(format!("Invalid path: {path}"));
            return;
        }
    };

    let mut file = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => {
            let _ = error.send(format!("Fails to open {path}: {e:?}"));
            return;
        }
    };

    if file.write_all(bytes).is_err() {
        let _ = error.send("Fails to write file.".to_string());
    }
}

pub fn is_xpub_valid(xpub: &str) -> bool {
    DescriptorPublicKey::from_str(xpub).is_ok()
}
pub fn is_descriptor_valid(xpub: &str) -> bool {
    Descriptor::<DescriptorPublicKey>::from_str(xpub).is_ok()
}

#[allow(clippy::unnecessary_box_returns)]
pub fn init_controller() -> Box<Controller> {
    Box::new(Controller::new())
}
