use std::rc::Rc;

use log::info;
use qt5qml::core::{
    QByteArray, QHashIntQByteArray, QModelIndex, QString, QVariant, ToQString, QT_USER_ROLE,
};

use harbour_finder::{DesktopEntryIndex, Entry};

include!(concat!(env!("OUT_DIR"), "/qffi_ResultListModel.rs"));

const NAME_ROLE: i32 = QT_USER_ROLE + 0;
const ICON_ROLE: i32 = QT_USER_ROLE + 1;
const EXEC_ROLE: i32 = QT_USER_ROLE + 2;
const FILENAME_ROLE: i32 = QT_USER_ROLE + 3;

pub struct ResultListModelPrivate {
    qobject: *mut ResultListModel,
    query: String,
    result_set: Vec<Rc<Entry>>,
    index: DesktopEntryIndex,
}

impl ResultListModelPrivate {
    pub fn new(qobject: *mut ResultListModel) -> Self {
        let mut index = DesktopEntryIndex::new();
        index.index();
        Self {
            qobject,
            query: String::new(),
            result_set: index.get_all(),
            index,
        }
    }

    fn q(&self) -> &ResultListModel {
        unsafe { &*self.qobject }
    }

    fn q_mut(&mut self) -> &mut ResultListModel {
        unsafe { &mut *self.qobject }
    }

    pub fn query(&self) -> QString {
        self.query.to_qstring()
    }

    pub fn set_query(&mut self, value: &QString) {
        let query = value.to_string();
        if self.query != query {
            info!("Query: {}", query);
            self.query = query;
            self.q_mut().query_changed(value);
            self.update();
        }
    }

    fn update(&mut self) {
        self.q_mut().begin_reset_model();

        let result_set = if self.query.is_empty() {
            self.index.get_all()
        } else {
            self.index.search_for(&self.query)
        };

        self.result_set = result_set;
        self.q_mut().end_reset_model();
    }

    // #[slot]
    pub fn index(&mut self) {}

    // #[override]
    pub fn role_names(&self) -> QHashIntQByteArray {
        let mut roles = QHashIntQByteArray::new();
        roles.insert(NAME_ROLE, QByteArray::from_bytes(b"name"));
        roles.insert(ICON_ROLE, QByteArray::from_bytes(b"icon"));
        roles.insert(EXEC_ROLE, QByteArray::from_bytes(b"exec"));
        roles.insert(FILENAME_ROLE, QByteArray::from_bytes(b"fileName"));
        roles
    }

    // #[override]
    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        if !index.is_valid() {
            return Default::default();
        }

        let row = index.row();
        if row < 0 || row as usize > self.result_set.len() {
            return Default::default();
        }
        let row = row as usize;

        match role {
            NAME_ROLE => (&self.result_set[row].name as &str).into(),
            ICON_ROLE => self.result_set[row].icon.as_ref().map(|s| s as &str).into(),
            EXEC_ROLE => (&self.result_set[row].exec as &str).into(),
            FILENAME_ROLE => (&self.result_set[row].file_name as &str).into(),
            _ => Default::default(),
        }
    }

    // #[override]
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        if parent.is_valid() {
            0
        } else {
            self.result_set.len() as i32
        }
    }

    // #[provide]
    // fn model_changed(&mut self) {}
}
