use qt_widgets::{QApplication, QWidget};

fn main() {
    QApplication::init(|_| unsafe {
        let window = QWidget::new_0a();
        window.resize_2a(500, 500);
        window.show();
        QApplication::exec()
    })
}
