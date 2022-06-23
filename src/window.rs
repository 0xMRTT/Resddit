use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/io/github/OxMRTT/ressdit/window.ui")]
    pub struct ResdditWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ResdditWindow {
        const NAME: &'static str = "ResdditWindow";
        type Type = super::ResdditWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ResdditWindow {}
    impl WidgetImpl for ResdditWindow {}
    impl WindowImpl for ResdditWindow {}
    impl ApplicationWindowImpl for ResdditWindow {}
}

glib::wrapper! {
    pub struct ResdditWindow(ObjectSubclass<imp::ResdditWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ResdditWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create ResdditWindow")
    }
}
