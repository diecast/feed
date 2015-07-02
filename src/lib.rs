extern crate diecast;

#[cfg(feature = "rss")]
extern crate rss as rss_;

#[cfg(feature = "atom")]
extern crate atom_syndication as atom_;

#[cfg(feature = "rss")]
pub mod rss;

#[cfg(feature = "atom")]
pub mod atom;
