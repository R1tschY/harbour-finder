use qobject_compiler::moc::MocConfig;
use qobject_compiler::{
    CcBuild, Include, QObjectBuild, QObjectMethod, QObjectProp, QObjectSignal, TypeRef,
};
use qt5qml::core::{QHashIntQByteArray, QModelIndex, QString, QVariant};

fn main() {
    // Qt
    let core = pkg_config::probe_library("Qt5Core").unwrap();
    let qml = pkg_config::probe_library("Qt5Qml").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    cpp.flag("-std=gnu++11");
    for include in &core.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }
    for include in &qml.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    let abstract_list_model = TypeRef::new(
        "QAbstractListModel",
        "std::ffi::c_void",
        false,
        Some(Include::System("QAbstractListModel".to_string())),
    );
    QObjectBuild::new("ResultListModel")
        .inherit(abstract_list_model)
        .property(
            QObjectProp::new::<QString>("query")
                .read("query")
                .write("setQuery")
                .notify("queryChanged"),
        )
        .method(QObjectMethod::new("query").const_().ret::<QString>())
        .method(QObjectMethod::new("setQuery").arg::<&QString>("value"))
        .signal(QObjectSignal::new("queryChanged").arg::<&QString>("query"))
        // methods
        .method(
            QObjectMethod::new("data")
                .const_()
                .override_()
                .arg::<&QModelIndex>("index")
                .arg::<i32>("role")
                .ret::<QVariant>(),
        )
        .method(
            QObjectMethod::new("rowCount")
                .const_()
                .override_()
                .arg::<&QModelIndex>("parent")
                .ret::<i32>(),
        )
        .method(
            QObjectMethod::new("roleNames")
                .const_()
                .override_()
                .ret::<QHashIntQByteArray>(),
        )
        // proxies
        .method(
            QObjectMethod::new("beginRemoveRows")
                .proxy("QAbstractListModel")
                .arg::<&QModelIndex>("parent")
                .arg::<i32>("first")
                .arg::<i32>("last"),
        )
        .method(QObjectMethod::new("endRemoveRows").proxy("QAbstractListModel"))
        .method(
            QObjectMethod::new("beginInsertRows")
                .proxy("QAbstractListModel")
                .arg::<&QModelIndex>("parent")
                .arg::<i32>("first")
                .arg::<i32>("last"),
        )
        .method(QObjectMethod::new("endInsertRows").proxy("QAbstractListModel"))
        .qml(true)
        .build(&cpp, &moc);
}
