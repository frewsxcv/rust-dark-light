use zbus::names::MemberName;
use zvariant::{OwnedValue, Value};
use std::{sync::mpsc::{self, Sender, Receiver}, thread, ops::Deref, convert::TryFrom};

use crate::Mode;

use super::{get_freedesktop_color_scheme, detect::detect};

pub fn notify(callback: &dyn Fn(Mode)) -> anyhow::Result<()> {
    let (tx, rx): (Sender<Mode>, Receiver<Mode>) = mpsc::channel();
    if get_freedesktop_color_scheme().is_ok() {
        freedesktop_watch(tx)?;
    } else {
        non_freedesktop_watch(tx)?;
    }
    loop {
        match rx.recv() {
            Ok(mode) => callback(mode),
            Err(_) => {},
        }
    }
}

fn non_freedesktop_watch(tx: Sender<Mode>) -> anyhow::Result<()> {
    let mut mode = detect();
    thread::spawn(move || {
        // TODO: Remove and replace for something like `notify`.
        loop {
            let new_mode = detect();
            if mode != new_mode {
                mode = new_mode;
                tx.send(mode).unwrap();
            }
        }
    });
    Ok(())
}

fn freedesktop_watch(tx: Sender<Mode>) -> anyhow::Result<()> {
    let connection = zbus::blocking::Connection::session()?;
    let proxy = zbus::blocking::Proxy::new(
        &connection,
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        "org.freedesktop.portal.Settings",
    )?;
    thread::spawn(move || -> anyhow::Result<()> {
        for signal in proxy.receive_signal(&MemberName::try_from("SettingChanged")?)? {
            let msg = signal.deref();
            let msg_header = signal.header()?;
            if msg_header.message_type()? == zbus::MessageType::Signal && msg_header.member()? == Some(&MemberName::try_from("SettingChanged")?) {
                let response = msg.body::<(String, String, OwnedValue)>()?;
                let mode = match response.2.downcast_ref::<Value>().unwrap().downcast_ref::<u32>().unwrap() {
                    1 => Mode::Dark,
                    2 => Mode::Light,
                    _ => Mode::Default,
                };
                tx.send(mode).unwrap();
            }
        }
        Ok(())
    });
    Ok(())
}