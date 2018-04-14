
extern crate failure;

#[cfg(feature="futures")]
extern crate futures;

#[cfg(feature="gfx-hal")]
extern crate gfx_hal as hal;

#[cfg(feature="gfx-mesh")]
extern crate gfx_mesh as mesh;

#[cfg(feature="gfx-render")]
extern crate gfx_render as render;

#[cfg(feature="obj")]
extern crate obj;

#[cfg(feature="png")]
extern crate png;

#[cfg(any(test, feature="ron"))]
extern crate ron;

#[cfg(any(test, feature="serde"))]
#[macro_use]
extern crate serde;

#[cfg(feature="gfx-hal")]
pub mod gfx;

pub mod asset;
pub mod handle;
pub mod manager;
pub mod sprite;
pub mod store;

#[cfg(test)]
mod tests;
