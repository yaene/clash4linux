use qt_core::qs;
use qt_widgets::{QApplication, QLabel, QMainWindow};

mod administrator;

fn main() {
    QApplication::init(|_| unsafe {
        let window = QMainWindow::new_0a();
        let proxies = administrator::read_selectors();
        let label = QLabel::from_q_string_q_widget(
            &qs(proxies.unwrap().concat()),
            &window,
        );
        label.set_fixed_width(500);
        label.set_word_wrap(true);
        window.resize_2a(500, 300);
        window.show_maximized();
        QApplication::exec()
    })
}
