use crate::Mode;
use std::{str::FromStr, sync::mpsc};

use encrypted_backup::{
    descriptor::dpk_to_pk, miniscript::DescriptorPublicKey, Decrypted, EncryptedBackup,
};

use crate::{
    bed::{Notification, Screen},
    controller::{write_to_file, XpubList, XpubScreen},
};

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
    pub fn set_ciphertext(&mut self, ciphertext: Vec<u8>) {
        self.ciphertext = ciphertext;
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
        write_to_file(self.descriptor.as_bytes(), path, self.error.clone());
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
