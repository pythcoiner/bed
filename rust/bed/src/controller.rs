use std::{
    collections::BTreeSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    sync::mpsc,
};

use encrypted_backup::{
    descriptor::dpk_to_pk,
    miniscript::{Descriptor, DescriptorPublicKey, ForEachKey},
    Decrypted, EncryptedBackup,
};

use crate::bed::{Mode, Notification, Screen};

type XpubList = Vec<(String, bool /* valid */, bool /* selected */)>;

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

#[derive(Debug, Clone)]
pub struct Encrypt {
    xpubs: XpubList,
    descriptor: String,
    descriptor_valid: bool,
    ciphertext: Vec<u8>,
    notif: mpsc::Sender<Notification>,
    error: mpsc::Sender<String>,
}

impl From<Encrypt> for Screen {
    fn from(value: Encrypt) -> Screen {
        Screen {
            keys: value.xpubs.iter().map(|(k, _v, _s)| k.clone()).collect(),
            valid: value.xpubs.iter().map(|(_k, v, _s)| *v).collect(),
            selected: value.xpubs.iter().map(|(_k, _v, s)| *s).collect(),
            descriptor: value.descriptor.clone(),
            descriptor_valid: value.descriptor_valid,
            ciphertext: value.ciphertext.clone(),
            btn_enabled: value.descriptor.is_empty(),
            mode: Mode::Decrypt,
        }
    }
}

impl XpubScreen for Encrypt {
    fn xpubs_mut(&mut self) -> &mut XpubList {
        &mut self.xpubs
    }
    fn update(&self) {
        let _ = self.notif.send(Notification::UpdateEncrypt);
    }
}

impl Encrypt {
    pub fn new(notif: mpsc::Sender<Notification>, error: mpsc::Sender<String>) -> Self {
        Self {
            xpubs: vec![],
            descriptor: String::new(),
            descriptor_valid: false,
            ciphertext: vec![],
            notif,
            error,
        }
    }
    pub fn set_descriptor(&mut self, descriptor: String) {
        let mut keys = BTreeSet::new();
        match Descriptor::<DescriptorPublicKey>::from_str(&descriptor) {
            Ok(d) => {
                self.descriptor_valid = true;
                d.for_each_key(|k| {
                    keys.insert(k.clone());
                    true
                });
            }
            Err(_) => {
                self.descriptor_valid = false;
            }
        };
        for k in keys {
            let key_str = k.to_string();
            if !self.contains(&key_str) {
                self.xpubs.push((key_str, true, true));
            }
        }
        self.descriptor = descriptor;
    }
    pub fn reset(&mut self) {
        self.xpubs.clear();
        self.descriptor.clear();
        self.ciphertext.clear();
        self.update();
    }
    pub fn save(&self, path: String) {
        // TODO:
    }
    pub fn try_encrypt(&mut self) {
        let keys = self
            .xpubs
            .iter()
            .filter_map(|(k, _, selected)| {
                if !selected {
                    None
                } else {
                    DescriptorPublicKey::from_str(k)
                        .ok()
                        .map(|dpk| dpk_to_pk(&dpk))
                }
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        if keys.is_empty() {
            let _ = self
                .error
                .send("No key(s) selected for encrypt!".to_string());
            return;
        }

        let descriptor = match Descriptor::<DescriptorPublicKey>::from_str(&self.descriptor) {
            Ok(d) => d,
            Err(_) => {
                let _ = self.error.send("Invalid descriptor".to_string());
                return;
            }
        };

        let backup = match EncryptedBackup::new().set_payload(&descriptor) {
            Ok(b) => b,
            Err(e) => {
                let _ = self
                    .error
                    .send(format!("Invalid descriptor payload: {e:?}"));
                return;
            }
        };
        self.ciphertext = match backup.set_keys(keys).encrypt() {
            Ok(c) => c,
            Err(e) => {
                let _ = self.error.send(format!("Fail to encrypt: {e:?}"));
                return;
            }
        };
        self.update();
    }
}

impl Encrypt {
    pub fn set_selected(&mut self, index: usize, selected: bool) {
        self._set_selected(index, selected);
    }
    pub fn edit_xpub(&mut self, index: usize, xpub: String) {
        self._edit_xpub(index, xpub);
    }
    pub fn add_xpub(&mut self) {
        self._add_xpub();
    }
    pub fn remove_xpub(&mut self, index: usize) {
        self._remove_xpub(index);
    }
}

#[derive(Debug, Clone)]
pub struct Decrypt {
    xpubs: XpubList,
    descriptor: String,
    ciphertext: Vec<u8>,
    notif: mpsc::Sender<Notification>,
    error: mpsc::Sender<String>,
}

impl From<Decrypt> for Screen {
    fn from(value: Decrypt) -> Screen {
        Screen {
            keys: value.xpubs.iter().map(|(k, _v, _s)| k.clone()).collect(),
            valid: value.xpubs.iter().map(|(_k, v, _s)| *v).collect(),
            selected: value.xpubs.iter().map(|(_k, _v, s)| *s).collect(),
            descriptor: value.descriptor.clone(),
            descriptor_valid: !value.descriptor.is_empty(),
            ciphertext: value.ciphertext.clone(),
            btn_enabled: value.descriptor.is_empty(),
            mode: Mode::Decrypt,
        }
    }
}

impl XpubScreen for Decrypt {
    fn xpubs_mut(&mut self) -> &mut XpubList {
        &mut self.xpubs
    }
    fn update(&self) {
        let _ = self.notif.send(Notification::UpdateDecrypt);
    }
}

impl Decrypt {
    pub fn new(notif: mpsc::Sender<Notification>, error: mpsc::Sender<String>) -> Self {
        Self {
            xpubs: vec![],
            descriptor: String::new(),
            ciphertext: vec![],
            notif,
            error,
        }
    }
    pub fn reset(&mut self) {
        self.xpubs.clear();
        self.descriptor.clear();
        self.ciphertext.clear();
        self.update();
    }
    pub fn try_decrypt(&mut self) {
        let dpks: Vec<_> = self
            .xpubs
            .iter()
            .filter_map(|(s, _, _)| DescriptorPublicKey::from_str(s).ok())
            .collect();
        let mut pks = Vec::with_capacity(dpks.len());
        for dpk in dpks {
            let pk = dpk_to_pk(&dpk);
            pks.push(pk);
        }
        let backup = match EncryptedBackup::new()
            .set_keys(pks)
            .set_encrypted_payload(&self.ciphertext)
        {
            Ok(b) => b,
            Err(e) => {
                let msg = format!("Fail to decode backup: {e:?}");
                log::error!("{msg}");
                let _ = self.error.send(msg);
                return;
            }
        };
        let descriptor = match backup.decrypt() {
            Ok(d) => d,
            Err(e) => {
                let msg = format!("Fail to decrypt backup: {e:?}");
                log::error!("{msg}");
                let _ = self.error.send(msg);
                return;
            }
        };

        if let Decrypted::Descriptor(descr) = descriptor {
            self.descriptor = descr.to_string();
            self.update();
        } else {
            let msg = "Backup decrypted but do not contains a descriptor".to_string();
            log::error!("{msg}");
            let _ = self.error.send(msg);
        }
    }
    pub fn save(&self, path: String) {
        if self.descriptor.is_empty() {
            let _ = self.error.send("Descriptor not decrypted.".to_string());
            return;
        }

        let file_path = match PathBuf::from_str(&path) {
            Ok(p) => p,
            Err(_) => {
                let _ = self.error.send("Descriptor not decrypted.".to_string());
                return;
            }
        };

        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(e) => {
                let _ = self.error.send(format!("fail to open {path}: {e:?}"));
                return;
            }
        };

        if file.write_all(self.descriptor.as_bytes()).is_err() {
            let _ = self
                .error
                .send("Fails to write descriptor to file.".to_string());
        }
    }
}

impl Decrypt {
    pub fn set_selected(&mut self, index: usize, selected: bool) {
        self._set_selected(index, selected);
    }
    pub fn edit_xpub(&mut self, index: usize, xpub: String) {
        self._edit_xpub(index, xpub);
    }
    pub fn add_xpub(&mut self) {
        self._add_xpub();
    }
    pub fn remove_xpub(&mut self, index: usize) {
        self._remove_xpub(index);
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
                        self.encrypt().xpubs.push((dpk_str, true, true));
                    }
                }
                Mode::Decrypt => {
                    if !self.decrypt().contains(&dpk_str) {
                        self.decrypt().xpubs.push((dpk_str, true, true));
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
            self.decrypt().ciphertext = bytes;
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
