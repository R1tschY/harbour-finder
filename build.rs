use qobject_compiler::moc::MocConfig;
use qobject_compiler::{
    CcBuild, Include, QObjectBuild, QObjectMethod, QObjectProp, QObjectSignal, TypeRef,
    TypeRefTrait,
};
use qt5qml::core::{QHashIntQByteArray, QModelIndex, QObject, QString, QVariant};

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
    let string_list = TypeRef::new(
        "QStringList",
        "qt5qml::core::QStringList",
        false,
        Some(Include::System("QStringList".to_string())),
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
        .method(QObjectMethod::new("beginResetModel").proxy("QAbstractListModel"))
        .method(QObjectMethod::new("endResetModel").proxy("QAbstractListModel"))
        .qml(true)
        .build(&cpp, &moc);

    QObjectBuild::new("Process")
        .inherit(QObject::type_ref())
        .property(
            QObjectProp::new::<QString>("program")
                .read("program")
                .write("setProgram")
                .notify("programChanged"),
        )
        .method(QObjectMethod::new("program").ret::<QString>())
        .method(QObjectMethod::new("setProgram").arg::<&QString>("value"))
        .signal(QObjectSignal::new("programChanged"))
        .property(
            QObjectProp::new_with_type(string_list.clone(), "arguments")
                .read("arguments")
                .write("setArguments")
                .notify("argumentsChanged"),
        )
        .method(QObjectMethod::new("arguments").ret_type(string_list.clone()))
        .method(
            QObjectMethod::new("setArguments").arg_with_type("value", string_list.with_const_ref()),
        )
        .signal(QObjectSignal::new("argumentsChanged"))
        .property(
            QObjectProp::new::<bool>("running")
                .read("running")
                .notify("runningChanged"),
        )
        .method(QObjectMethod::new("running").ret::<bool>())
        .signal(QObjectSignal::new("runningChanged"))
        .slot(QObjectMethod::new("start"))
        .slot(QObjectMethod::new("_poll"))
        .qml(true)
        .build(&cpp, &moc);
}
