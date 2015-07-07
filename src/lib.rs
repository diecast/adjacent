extern crate diecast;
extern crate typemap;

use std::sync::Arc;

use diecast::{Bind, Item};

/// Stores the previous and next `Item` relative
/// to the `Item`.

#[derive(Clone, Debug)]
pub struct Adjacent {
    previous: Option<Arc<Item>>,
    next: Option<Arc<Item>>,
}

impl typemap::Key for Adjacent {
    type Value = Adjacent;
}

/// Inserts a copy of the previous and next `Item` relative
/// to the current `Item` for each `Item`.

pub fn adjacent(bind: &mut Bind) -> diecast::Result<()> {
    let count = bind.items().len();

    let last_num = if count == 0 {
        0
    } else {
        count - 1
    };

    // TODO: yet another reason to have Arc<Item>?
    // FIXME
    // the problem with this is that unlike Paginate,
    // it'll contain copies of the item Should probably
    // instead insert an index?
    let cloned: Vec<Arc<Item>> =
        bind.items().iter()
        .map(|i| Arc::new(i.clone()))
        .collect();

    for (idx, item) in bind.items_mut().iter_mut().enumerate() {
        let prev =
            if idx == 0 { None }
            else { Some(cloned[idx - 1].clone()) };
        let next =
            if idx == last_num { None }
            else { Some(cloned[idx + 1].clone()) };

        item.extensions.insert::<Adjacent>(Adjacent {
            previous: prev,
            next: next,
        });
    }

    Ok(())
}

