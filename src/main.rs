use std::rc::Rc;

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
    cpp_core::CppBox, qs, slot, MatchFlag, QBox, QListOfQString, QObject, QPtr,
    QString, QStringList, SlotNoArgs,
};
use qt_widgets::{QApplication, QLabel, QListView, QListWidget, QMainWindow};

mod administrator;

fn main() {
    QApplication::init(|_| unsafe {
        let proxy_widget = ProxyWidget::new();
        proxy_widget.show();
        QApplication::exec()
    })
}
struct ProxyWidget {
    window: QBox<QMainWindow>,
    selector: QBox<QListWidget>,
    proxies: QBox<QListWidget>,
}

impl StaticUpcast<QObject> for ProxyWidget {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.window.as_ptr().static_upcast()
    }
}

impl ProxyWidget {
    fn new() -> Rc<Self> {
        unsafe {
            let window = QMainWindow::new_0a();
            let selector = QListWidget::new_1a(&window);
            let proxies = QListWidget::new_1a(&window);
            let this = Rc::new(ProxyWidget {
                window,
                selector,
                proxies,
            });
            this.init();
            this
        }
    }
    unsafe fn init(self: &Rc<Self>) {
        self.selector.add_items(&to_qstring_list(
            &administrator::read_selectors().unwrap(),
        ));
        self.selector
            .current_item_changed()
            .connect(&self.slot_on_selector_selection_changed());
        // todo yb: error handling when no selectors
        self.selector.set_current_row_1a(0);
        self.selector.resize_2a(500, 300);
        let current_selector =
            &self.selector.current_item().text().to_std_string();

        let proxies =
            &administrator::read_proxies_for_selector(current_selector)
                .unwrap();

        self.proxies.add_items(&to_qstring_list(proxies));
        let current_proxy =
            administrator::read_current_proxy(current_selector).unwrap();
        let current_proxy = current_proxy.trim_matches('"');
        let current_proxy = self
            .proxies
            .find_items(&qs(current_proxy), MatchFlag::MatchExactly.into())
            .first();
        self.proxies.set_current_item_1a(*current_proxy);
        self.proxies.resize_2a(500, 300);
        self.proxies.move_2a(0, 300);
    }

    unsafe fn show(self: &Self) {
        self.window.show_maximized();
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_selector_selection_changed(self: &Self) {
        let current_selector =
            &self.selector.current_item().text().to_std_string();

        self.proxies.clear();
        self.proxies.add_items(&to_qstring_list(
            &administrator::read_proxies_for_selector(current_selector)
                .unwrap(),
        ));
        let current_proxy =
            administrator::read_current_proxy(current_selector).unwrap();
        let current_proxy = current_proxy.trim_matches('"');
        let current_proxy = self
            .proxies
            .find_items(&qs(current_proxy), MatchFlag::MatchExactly.into())
            .first();
        self.proxies.set_current_item_1a(*current_proxy);
    }
}

unsafe fn to_qstring_list(input: &Vec<String>) -> CppBox<QStringList> {
    let list = QStringList::new();
    for str in input.iter() {
        list.append_q_string(&qs(str));
    }
    list
}
