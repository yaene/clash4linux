use qt_core::{cpp_core::CppBox, qs, QListOfQString, QString, QStringList};
use qt_widgets::{QApplication, QLabel, QListWidget, QMainWindow};

mod administrator;

fn main() {
    QApplication::init(|_| unsafe {
        let window = QMainWindow::new_0a();
        let proxies = administrator::read_selectors();
        let selector_select = QListWidget::new_1a(&window);
        selector_select.add_items(&to_qstring_list(&proxies.unwrap()));
        selector_select.resize_2a(500, 300);
        window.resize_2a(500, 300);
        window.show_maximized();
        QApplication::exec()
    })
}

unsafe fn to_qstring_list(input: &Vec<String>) -> CppBox<QStringList> {
    let list = QStringList::new();
    for str in input.iter() {
        list.append_q_string(&qs(str));
    }
    list
}
