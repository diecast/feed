use std::path::PathBuf;

use rss_;

use diecast::{self, Handle, Bind, Item};

pub struct Builder<H>
where H: Fn(&str, &str, &Bind) -> Vec<rss_::Item> {
    path: PathBuf,
    title: String,
    url: String,
    handler: H
}

pub fn create<P, T, U, H>(path: P, title: T, url: U, handler: H) -> Builder<H>
where P: Into<PathBuf>,
      T: Into<String>, U: Into<String>,
      H: Fn(&str, &str, &Bind) -> Vec<rss_::Item> {
    Builder {
        path: path.into(),
        title: title.into(),
        url: url.into(),
        handler: handler,
    }
}

impl<H> Handle<Bind> for Builder<H>
where H: Fn(&str, &str, &Bind) -> Vec<rss_::Item> {
    fn handle(&self, bind: &mut Bind) -> diecast::Result<()> {
        let channel = rss_::Channel {
            title: self.title.clone(),
            link: self.url.clone(),
            items: (self.handler)(&self.title, &self.url, bind),
            .. Default::default()
        };

        let mut item = Item::writing(self.path.clone());
        // TODO
        // make it possible to leverage versions::load?
        item.body = channel.to_string();
        bind.attach(item);

        Ok(())
    }
}
