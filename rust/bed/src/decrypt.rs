use crate::Mode;
use std::{
    collections::BTreeSet,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
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
use tokio::runtime::{Handle, Runtime};

use crate::{
    bed::{Notification, Screen},
    controller::{write_to_file, XpubList, XpubScreen},
};

impl From<&mut Decrypt> for Screen {
    fn from(value: &mut Decrypt) -> Screen {
        Screen {
            keys: value.xpubs.iter().map(|(k, _v, _s)| k.clone()).collect(),
            valid: value.xpubs.iter().map(|(_k, v, _s)| *v).collect(),
            selected: value.xpubs.iter().map(|(_k, _v, s)| *s).collect(),
            descriptor: value.descriptor.clone(),
            descriptor_valid: !value.descriptor.is_empty(),
            ciphertext: value.ciphertext.clone(),
            devices: value.devices.lock().expect("poisoned").len(),
            mode: Mode::Decrypt,
        }
    }
}

#[derive(Debug)]
pub struct Decrypt {
    xpubs: XpubList,
    descriptor: String,
    ciphertext: Vec<u8>,
    derivation_paths: Arc<Mutex<Vec<DerivationPath>>>,
    devices: Arc<Mutex<Vec<String>>>,
    devices_keys: Arc<Mutex<Vec<DescriptorPublicKey>>>,
    notif: mpsc::Sender<Notification>,
    error: mpsc::Sender<String>,
    _rt: Runtime,
    stop: Arc<AtomicBool>,
    poller: JoinHandle<()>,
}

impl XpubScreen for Decrypt {
    fn xpubs_mut(&mut self) -> &mut XpubList {
        &mut self.xpubs
    }
    fn update(&self) {
        let _ = self.notif.send(Notification::UpdateDecrypt);
    }
}

impl Drop for Decrypt {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        while !self.poller.is_finished() {
            thread::sleep(Duration::from_millis(100));
        }
    }
}

impl Decrypt {
    pub fn new(notif: mpsc::Sender<Notification>, error: mpsc::Sender<String>) -> Self {
        let _rt = Runtime::new().expect("Failed to create Tokio runtime");
        let handle = _rt.handle().clone();
        let stop = Arc::new(AtomicBool::new(false));
        let derivation_paths = Arc::new(Mutex::new(vec![]));
        let devices = Arc::new(Mutex::new(vec![]));
        let devices_keys = Arc::new(Mutex::new(vec![]));
        let poller = poll_devices(
            handle,
            derivation_paths.clone(),
            devices.clone(),
            devices_keys.clone(),
            notif.clone(),
            stop.clone(),
        );
        Self {
            xpubs: vec![],
            descriptor: String::new(),
            ciphertext: vec![],
            derivation_paths,
            devices,
            devices_keys,
            notif,
            error,
            _rt,
            stop,
            poller,
        }
    }
    pub fn set_ciphertext(&mut self, ciphertext: Vec<u8>) {
        self.ciphertext = ciphertext;
    }
    pub fn set_derivation_paths(&mut self, paths: Vec<DerivationPath>) {
        *self.derivation_paths.lock().expect("poisoned") = paths;
    }
    pub fn reset(&mut self) {
        self.xpubs.clear();
        self.descriptor.clear();
        self.ciphertext.clear();
        self.derivation_paths.lock().expect("poisoned").clear();
        self.update();
    }
    pub fn try_decrypt(&mut self) {
        let mut keys = vec![];
        let mut dpks: Vec<_> = self
            .xpubs
            .iter()
            .filter_map(|(s, _, _)| DescriptorPublicKey::from_str(s).ok())
            .collect();
        keys.append(&mut dpks);
        keys.append(&mut self.devices_keys.lock().expect("poisoned").clone());
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

pub fn poll_devices(
    rt: Handle,
    derivation_paths: Arc<Mutex<Vec<DerivationPath>>>,
    devices: Arc<Mutex<Vec<String>>>,
    devices_keys: Arc<Mutex<Vec<DescriptorPublicKey>>>,
    notif: mpsc::Sender<Notification>,
    stop: Arc<AtomicBool>,
) -> JoinHandle<()> {
    thread::spawn(move || loop {
        let stop_requested = stop.load(Ordering::Relaxed);
        if stop_requested {
            return;
        }

        // we try to fetch keys only if we have derivation paths
        let paths = derivation_paths.lock().expect("poisoned").clone();
        if !paths.is_empty() {
            let keys = rt.block_on(encrypted_backup::signing_devices::collect_xpubs(paths));
            *devices_keys.lock().expect("poisoned") = keys;
        } else {
            *devices_keys.lock().expect("poisoned") = vec![];
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
        *devices.lock().expect("poisoned") = conn_devices.into_iter().collect();
        let _ = notif.send(Notification::UpdateDecrypt);
        thread::sleep(Duration::from_secs(3));
    })
}
