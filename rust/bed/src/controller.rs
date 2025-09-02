use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    sync::{mpsc, Arc, Mutex},
};

use encrypted_backup::{
    miniscript::{Descriptor, DescriptorPublicKey},
    EncryptedBackup,
};

use crate::{
    bed::{Mode, Notification, Screen},
    decrypt::{Decrypt, InnerDecrypt},
    encrypt::{Encrypt, InnerEncrypt},
    generic::XpubScreen,
    lock,
};

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
        let encrypt = InnerEncrypt::new(notif_sender.clone(), error_sender.clone());
        let encrypt = Encrypt {
            inner: Arc::new(Mutex::new(encrypt)),
        };
        let decrypt = InnerDecrypt::new(notif_sender.clone(), error_sender.clone());
        let decrypt = Decrypt { inner: decrypt };
        Self {
            encrypt,
            decrypt,
            notif,
            error,
            notif_sender,
            error_sender,
        }
    }

    pub fn encrypt(&mut self) -> Box<Encrypt> {
        Box::new(self.encrypt.clone())
    }
    pub fn decrypt(&mut self) -> Box<Decrypt> {
        Box::new(self.decrypt.clone())
    }

    pub fn poll(&self) -> Notification {
        self.notif.try_recv().ok().into()
    }

    pub fn error(&self) -> String {
        self.error.try_recv().unwrap_or_default()
    }
    pub fn encrypt_screen(&mut self) -> Screen {
        (*self.encrypt()).into()
    }
    pub fn decrypt_screen(&mut self) -> Screen {
        (*self.decrypt()).into()
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
            let trimmed = str.trim();
            if self.try_drop_key(trimmed, Mode::Encrypt) {
                let _ = self.notif_sender.send(Notification::UpdateEncrypt);
                return;
            }
            if Descriptor::<DescriptorPublicKey>::from_str(trimmed).is_ok() {
                self.encrypt().set_descriptor(trimmed.to_string());
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
                        self.encrypt()
                            .inner
                            .lock()
                            .expect("poisoned")
                            .xpubs_mut()
                            .push((dpk_str, true, true));
                    }
                }
                Mode::Decrypt => {
                    let decrypt = *self.decrypt();
                    let mut lock = lock!(decrypt);
                    if !lock.contains(&dpk_str) {
                        lock.xpubs_mut().push((dpk_str, true, true));
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
        if let Ok(backup) = EncryptedBackup::new().set_encrypted_payload(&bytes) {
            let decrypt = *self.decrypt();
            let mut lock = lock!(decrypt);
            lock.set_ciphertext(bytes);
            lock.set_derivation_paths(backup.get_derivation_paths());
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
