//! Contains a debus session object.

use std::sync::mpsc::{Receiver};

use dbus::{Connection, BusType, NameFlag};
use dbus::tree::{Factory};

use super::{DBusTree};

use super::dbus_message::DBusMessage;

/// Dbus session object.
///
/// Contains all of the horrors of the dbus library within.
/// Way Cooler's IPC is split up into files for different interfaces
/// which all come together
#[allow(dead_code)]
pub struct DBusSession {
    tree: DBusTree,
    connection: Connection,
    receiver: Receiver<DBusMessage>
}

impl DBusSession {
    pub fn create(receiver: Receiver<DBusMessage>) -> DBusSession {
        let connection = Connection::get_private(BusType::Session)
            .expect("Unable to create dbus session");
        connection.register_name("org.way-cooler", NameFlag::AllowReplacement as u32)
            .expect("Unable to register 'org.way-cooler' on dbus");

        let mut factory = Factory::new_fn::<()>();

        let tree = factory.tree()
            .add(super::layout::setup(&mut factory))
            .add(super::screen::setup(&mut factory))
            .add(super::theme::setup(&mut factory));

        //super::keybindings::setup(&mut factory);

        DBusSession {
            tree: tree,
            connection: connection,
            receiver: receiver
        }
    }

    pub fn run_thread(&mut self) {
        self.tree.set_registered(&self.connection, true)
            .expect("Could not register connect");

        for _ in self.tree.run(&self.connection,
                               self.connection.iter(1000)) {
        }
    }
}
