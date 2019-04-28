use std::io;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::thread;

use serde_json;

use glib;
use gtk;
use gtk::prelude::*;

use super::store::PlugInfo;

pub fn call<F>(query: Option<String>, cb: F)
where
    F: FnOnce(io::Result<DescriptionList>) + Send + 'static,
{
    thread::spawn(move || {
        let mut result = Some(request(query.as_ref().map(|s| s.as_ref())));
        let mut cb = Some(cb);

        glib::idle_add(move || {
            cb.take().unwrap()(result.take().unwrap());
            Continue(false)
        })
    });
}

fn request(query: Option<&str>) -> io::Result<DescriptionList> {
    let child = Command::new("curl")
        .arg("-s")
        .arg(format!(
            "https://vimawesome.com/api/plugins?query={}&page=1",
            query.unwrap_or("")
        ))
        .stdout(Stdio::piped())
        .spawn()?;

    let out = child.wait_with_output()?;

    if out.status.success() {
        if out.stdout.is_empty() {
            Ok(DescriptionList::empty())
        } else {
            let description_list: DescriptionList = serde_json::from_slice(&out.stdout)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            Ok(description_list)
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "curl exit with error:\n{}",
                match out.status.code() {
                    Some(code) => format!("Exited with status code: {}", code),
                    None => "Process terminated by signal".to_owned(),
                }
            ),
        ))
    }
}

pub fn build_result_panel<F: Fn(PlugInfo) + 'static>(
    list: &DescriptionList,
    add_cb: F,
) -> gtk::ScrolledWindow {
    let scroll = gtk::ScrolledWindow::new(
        Option::<&gtk::Adjustment>::None,
        Option::<&gtk::Adjustment>::None,
    );
    scroll.get_style_context().add_class("view");
    let panel = gtk::ListBox::new();

    let cb_ref = Rc::new(add_cb);
    for plug in list.plugins.iter() {
        let row = create_plug_row(plug, cb_ref.clone());

        panel.add(&row);
    }

    scroll.add(&panel);
    scroll.show_all();
    scroll
}

fn create_plug_row<F: Fn(PlugInfo) + 'static>(
    plug: &Description,
    add_cb: Rc<F>,
) -> gtk::ListBoxRow {
    let row = gtk::ListBoxRow::new();
    let row_container = gtk::Box::new(gtk::Orientation::Vertical, 5);
    row_container.set_border_width(5);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let label_box = create_plug_label(plug);

    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    button_box.set_halign(gtk::Align::End);

    let add_btn = gtk::Button::new_with_label("Install");
    button_box.pack_start(&add_btn, false, true, 0);

    row_container.pack_start(&hbox, true, true, 0);
    hbox.pack_start(&label_box, true, true, 0);
    hbox.pack_start(&button_box, false, true, 0);

    row.add(&row_container);

    add_btn.connect_clicked(clone!(plug => move |btn| {
        if let Some(ref github_url) = plug.github_url {
            btn.set_sensitive(false);
            add_cb(PlugInfo::new(plug.name.clone(), github_url.clone()));
        }
    }));

    row
}

fn create_plug_label(plug: &Description) -> gtk::Box {
    let label_box = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let name_lbl = gtk::Label::new(None);
    name_lbl.set_markup(&format!(
        "<b>{}</b> by {}",
        plug.name,
        plug.author
            .as_ref()
            .map(|s| s.as_ref())
            .unwrap_or("unknown",)
    ));
    name_lbl.set_halign(gtk::Align::Start);
    let url_lbl = gtk::Label::new(None);
    if let Some(url) = plug.github_url.as_ref() {
        url_lbl.set_markup(&format!("<a href=\"{}\">{}</a>", url, url));
    }
    url_lbl.set_halign(gtk::Align::Start);

    label_box.pack_start(&name_lbl, true, true, 0);
    label_box.pack_start(&url_lbl, true, true, 0);
    label_box
}

#[derive(Deserialize, Debug)]
pub struct DescriptionList {
    pub plugins: Box<[Description]>,
}

impl DescriptionList {
    fn empty() -> DescriptionList {
        DescriptionList {
            plugins: Box::new([]),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Description {
    pub name: String,
    pub github_url: Option<String>,
    pub author: Option<String>,
    pub github_stars: Option<i64>,
}
