use std::path::PathBuf;

use atom_;

use diecast::{self, Handle, Bind, Item};

pub struct Builder<H>
where H: Fn(&str, &str, &Bind) -> Vec<atom_::Entry> {
    path: PathBuf,
    title: String,
    url: String,
    handler: H
}

pub fn create<P, T, U, H>(path: P, title: T, url: U, handler: H) -> Builder<H>
where P: Into<PathBuf>,
      T: Into<String>, U: Into<String>,
      H: Fn(&str, &str, &Bind) -> Vec<atom_::Entry> {
    Builder {
        path: path.into(),
        title: title.into(),
        url: url.into(),
        handler: handler,
    }
}

impl<H> Handle<Bind> for Builder<H>
where H: Fn(&str, &str, &Bind) -> Vec<atom_::Entry> {
    fn handle(&self, bind: &mut Bind) -> diecast::Result<()> {
        let feed = atom_::Feed {
            // TODO: configurable?
            id: self.title.clone(),
            title: self.title.clone(),
            entries: (self.handler)(&self.title, &self.url, bind),
            links: vec![atom_::Link {
                href: self.url.clone(),
                .. Default::default()
            }],
            .. Default::default()
        };

        let mut item = Item::writing(self.path.clone());
        item.body = feed.to_string();
        bind.attach(item);

        Ok(())
    }
}

