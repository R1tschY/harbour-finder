use std::process::{Command, ExitStatus};
use std::{io, mem, thread};

use log::*;

use crate::UnsafeSend;
use qt5qml::core::{ConnectionTypeKind, QMetaObject, QObjectRef, QString, QStringList, ToQString};
use qt5qml::cstr;

include!(concat!(env!("OUT_DIR"), "/qffi_Process.rs"));

struct ProcessPrivate {
    q: *mut Process,
    program: Option<String>,
    arguments: QStringList,
    thread: Option<std::thread::JoinHandle<io::Result<ExitStatus>>>,
    result: Option<io::Result<ExitStatus>>,
}

impl ProcessPrivate {
    pub fn new(q: *mut Process) -> Self {
        Self {
            q,
            program: None,
            arguments: QStringList::new(),
            thread: None,
            result: None,
        }
    }

    pub fn running(&self) -> bool {
        self.thread.is_some()
    }

    pub fn program(&self) -> QString {
        self.program.to_qstring()
    }

    pub fn set_program(&mut self, value: &QString) {
        let string = value.to_string();
        if Some(&string) == self.program.as_ref() {
            return;
        }

        self.program = Some(string);
        unsafe { &mut *self.q }.program_changed();
    }

    pub fn arguments(&self) -> QStringList {
        self.arguments.clone()
    }

    pub fn set_arguments(&mut self, value: &QStringList) {
        self.arguments = QStringList::clone(value);
    }

    pub fn start(&mut self) {
        let arguments = self.arguments.clone();
        let q = UnsafeSend::new(self.q);

        if let Some(program) = self.program.clone() {
            self.thread = Some(thread::spawn(move || {
                let args = arguments.as_slice();
                info!("Process starting ...: {:?} {:?}", &program, &args);
                let result = Command::new(program)
                    .args(args.iter().map(|arg| arg.to_string()))
                    .status();
                unsafe {
                    QMetaObject::build_invoke_method(
                        (&mut *q.unwrap()).as_qobject_mut(),
                        cstr!("_poll"),
                    )
                    .type_(ConnectionTypeKind::Queued)
                    .invoke();
                }
                info!("Process exited: {:?}", &result);
                result
            }));
            unsafe { &mut *self.q }.running_changed();
        }
    }

    pub fn _poll(&mut self) {
        if let Some(result) = mem::replace(&mut self.thread, None) {
            self.result = Some(result.join().unwrap());

            unsafe { &mut *self.q }.running_changed();
        }
    }
}
