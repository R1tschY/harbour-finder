use qt5qml::core::QApplicationFactory;
use qt5qml::cstr;
use sailfishapp::SailfishApp;

use crate::model::ResultListModel;

mod model;

fn main() {
    env_logger::init();

    let app = SailfishApp::new_from_env_args();

    ResultListModel::register_type(
        cstr!("de.richardliebscher.harbour_finder"),
        0,
        1,
        cstr!("ResultListModel"),
    );

    let mut view = SailfishApp::create_view();
    view.set_source(&SailfishApp::path_to_main_qml());
    view.show_full_screen();
    app.exec();
}
