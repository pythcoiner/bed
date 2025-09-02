use crate::{
    generic::{XpubList, XpubScreen},
    lock, wrap, Mode,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use encrypted_backup::{
    descriptor::dpk_to_pk,
    miniscript::{
        bitcoin::{bip32::DerivationPath, Network},
        DescriptorPublicKey,
    },
    tokio, Decrypted, EncryptedBackup,
};
use tokio::runtime::Runtime;

use crate::{
    bed::{Notification, Screen},
    controller::write_to_file,
};

#[derive(Debug, Clone)]
pub struct Decrypt {
    pub inner: Arc<Mutex<InnerDecrypt>>,
}

wrap!(Decrypt);

impl Decrypt {
    pub fn reset(&mut self) {
        lock!(self).reset();
    }
    pub fn try_decrypt(&mut self) {
        lock!(self).try_decrypt();
    }
    pub fn save(&self, path: String) {
        lock!(self).save(path);
    }
}

impl From<&InnerDecrypt> for Screen {
    fn from(value: &InnerDecrypt) -> Screen {
        Screen {
            keys: value.xpubs.iter().map(|(k, _v, _s)| k.clone()).collect(),
            valid: value.xpubs.iter().map(|(_k, v, _s)| *v).collect(),
            selected: value.xpubs.iter().map(|(_k, _v, s)| *s).collect(),
            descriptor: value.descriptor.clone(),
            descriptor_valid: !value.descriptor.is_empty(),
            ciphertext: value.ciphertext.clone(),
            devices: value.devices.len(),
            mode: Mode::Decrypt,
        }
    }
}

#[derive(Debug)]
pub struct InnerDecrypt {
    xpubs: XpubList,
    descriptor: String,
    ciphertext: Vec<u8>,
    derivation_paths: Vec<DerivationPath>,
    devices: Vec<String>,
    devices_keys: BTreeMap<DescriptorPublicKey, bool /* deleted */>,
    notif: mpsc::Sender<Notification>,
    error: mpsc::Sender<String>,
    _rt: Runtime,
    stop: bool,
    poller: Option<JoinHandle<()>>,
}

impl XpubScreen for InnerDecrypt {
    fn xpubs_mut(&mut self) -> &mut XpubList {
        &mut self.xpubs
    }
    fn update(&self) {
        let _ = self.notif.send(Notification::UpdateDecrypt);
    }
}

impl Drop for InnerDecrypt {
    fn drop(&mut self) {
        self.stop = true;
        let mut stop = false;
        loop {
            if let Some(handle) = &self.poller {
                if stop {
                    return;
                }
                if handle.is_finished() {
                    stop = true;
                } else {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}

impl InnerDecrypt {
    pub fn new(notif: mpsc::Sender<Notification>, error: mpsc::Sender<String>) -> Arc<Mutex<Self>> {
        let _rt = Runtime::new().expect("Failed to create Tokio runtime");
        let new = Arc::new(Mutex::new(Self {
            xpubs: vec![],
            descriptor: String::new(),
            ciphertext: vec![],
            derivation_paths: vec![],
            devices: vec![],
            devices_keys: BTreeMap::new(),
            notif,
            error,
            _rt,
            stop: false,
            poller: None,
        }));
        let poller = poll_devices(new.clone());
        new.lock().expect("poisoned").poller = Some(poller);
        new
    }
    pub fn set_ciphertext(&mut self, ciphertext: Vec<u8>) {
        self.ciphertext = ciphertext;
    }
    pub fn set_derivation_paths(&mut self, paths: Vec<DerivationPath>) {
        self.derivation_paths = paths;
    }
    pub fn reset(&mut self) {
        self.xpubs.clear();
        self.descriptor.clear();
        self.ciphertext.clear();
        self.derivation_paths.clear();
        self.devices_keys.clear();
        self.update();
    }
    pub fn try_decrypt(&mut self) {
        let keys: Vec<_> = self
            .xpubs
            .iter()
            .filter_map(|(s, _, selected)| {
                DescriptorPublicKey::from_str(s).ok().filter(|_| *selected)
            })
            .collect();
        if keys.is_empty() {
            let msg = "Fail to decode backup: no keys".to_string();
            log::error!("{msg}");
            let _ = self.error.send(msg);
            return;
        }
        let mut pks = Vec::with_capacity(keys.len());
        for dpk in keys {
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

impl InnerDecrypt {
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
        if let Some((xpub_str, _, _)) = self.xpubs_mut().get(index) {
            if let Ok(dpk) = DescriptorPublicKey::from_str(xpub_str) {
                if let Some(deleted) = self.devices_keys.get_mut(&dpk) {
                    // we mark the device key deleted
                    *deleted = true;
                }
            }
        }
        self._remove_xpub(index);
    }
}

pub fn poll_devices(decrypt: Arc<Mutex<InnerDecrypt>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let stop_requested = decrypt.lock().expect("poisoned").stop;
        if stop_requested {
            return;
        }

        // we try to fetch keys only if we have derivation paths
        let (paths, rt) = {
            let lock = decrypt.lock().expect("poisoned");
            let paths = lock.derivation_paths.clone();
            let rt = lock._rt.handle().clone();
            (paths, rt)
        }; // <- drop the lock here

        if !paths.is_empty() {
            let keys = rt.block_on(encrypted_backup::signing_devices::collect_xpubs(paths));
            {
                let mut lock = decrypt.lock().expect("poisoned");
                for k in keys {
                    if !lock.devices_keys.contains_key(&k) {
                        lock.devices_keys.insert(k.clone(), false);
                        lock.xpubs_mut().push((k.to_string(), true, false));
                    }
                }
            } // <- drop lock here
        }

        // we always list devices
        let mut connected_devices = vec![];
        for network in [Network::Bitcoin, Network::Signet] {
            let mut dev = rt
                .block_on(encrypted_backup::signing_devices::list(network))
                .unwrap_or_default();
            connected_devices.append(&mut dev);
        }
        let mut conn_devices = BTreeSet::new();
        for d in connected_devices {
            let name = d.device_kind().to_string();
            conn_devices.insert(name);
        }
        {
            let mut lock = decrypt.lock().expect("poisoned");
            lock.devices = conn_devices.into_iter().collect();
            let _ = lock.notif.send(Notification::UpdateDecrypt);
        } // <- drop the lock here
        thread::sleep(Duration::from_secs(3));
    })
}
