use qt5qml::core::QApplicationFactory;
use qt5qml::cstr;
use sailfishapp::SailfishApp;

mod model;
mod process;
mod unsafe_send;

pub use unsafe_send::UnsafeSend;

fn main() {
    env_logger::init();

    let app = SailfishApp::new_from_env_args();

    model::ResultListModel::register_type(
        cstr!("de.richardliebscher.harbour_finder"),
        0,
        1,
        cstr!("ResultListModel"),
    );
    process::Process::register_type(
        cstr!("de.richardliebscher.harbour_finder"),
        0,
        1,
        cstr!("Process"),
    );

    let mut view = SailfishApp::create_view();
    view.set_source(&SailfishApp::path_to_main_qml());
    view.show_full_screen();
    app.exec();
}
