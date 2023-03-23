use qt_widgets::{QApplication, QMainWindow};

fn main() {
    QApplication::init(|_| unsafe {
        let window = QMainWindow::new_0a();
        window.resize_2a(500, 300);
        window.show_maximized();
        QApplication::exec()
    })
}
