use std::{
    collections::BTreeSet,
    str::FromStr,
    sync::{mpsc, Arc, Mutex},
};

use encrypted_backup::{
    descriptor::dpk_to_pk,
    miniscript::{Descriptor, DescriptorPublicKey, ForEachKey},
    EncryptedBackup,
};

use crate::{
    bed::{Mode, Notification, Screen},
    controller::write_to_file,
    generic::{XpubList, XpubScreen},
    lock, wrap,
};

#[derive(Debug, Clone)]
pub struct Encrypt {
    pub inner: Arc<Mutex<InnerEncrypt>>,
}

wrap!(Encrypt);

impl Encrypt {
    pub fn reset(&mut self) {
        lock!(self).reset();
    }
    pub fn try_encrypt(&mut self) {
        lock!(self).try_encrypt();
    }
    pub fn save(&self, path: String) {
        lock!(self).save(path);
    }
    pub fn set_descriptor(&mut self, descriptor: String) {
        lock!(self).set_descriptor(descriptor);
    }
    pub fn contains(&mut self, key: &str) -> bool {
        lock!(self).contains(key)
    }
}

#[derive(Debug, Clone)]
pub struct InnerEncrypt {
    xpubs: XpubList,
    descriptor: String,
    descriptor_valid: bool,
    ciphertext: Vec<u8>,
    notif: mpsc::Sender<Notification>,
    error: mpsc::Sender<String>,
}

impl From<&InnerEncrypt> for Screen {
    fn from(value: &InnerEncrypt) -> Screen {
        Screen {
            keys: value.xpubs.iter().map(|(k, _v, _s)| k.clone()).collect(),
            valid: value.xpubs.iter().map(|(_k, v, _s)| *v).collect(),
            selected: value.xpubs.iter().map(|(_k, _v, s)| *s).collect(),
            descriptor: value.descriptor.clone(),
            descriptor_valid: value.descriptor_valid,
            ciphertext: value.ciphertext.clone(),
            devices: 0,
            mode: Mode::Decrypt,
        }
    }
}

impl XpubScreen for InnerEncrypt {
    fn xpubs_mut(&mut self) -> &mut XpubList {
        &mut self.xpubs
    }
    fn update(&self) {
        let _ = self.notif.send(Notification::UpdateEncrypt);
    }
}

impl InnerEncrypt {
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
        if self.ciphertext.is_empty() {
            let _ = self.error.send("Descriptor not encrypted.".to_string());
            return;
        }
        write_to_file(&self.ciphertext, path, self.error.clone());
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
