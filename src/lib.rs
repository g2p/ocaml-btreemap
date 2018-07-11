#[macro_use]
extern crate ocaml;
use ocaml::ToValue;

#[macro_use]
extern crate lazy_static;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::mem;

struct OCamlString(ocaml::Value);

lazy_static! {
    static ref COMPARE: ocaml::Value =
        ocaml::named_value("compare").unwrap();
}

impl Ord for OCamlString {
    fn cmp(&self, other: &Self) -> Ordering {
        let cr = COMPARE.call2(self.0.clone(), other.0.clone()).unwrap().isize_val();
        if cr < 0 { return Ordering::Less; }
        if cr > 0 { return Ordering::Greater; }
        return Ordering::Equal;
    }
}

impl Eq for OCamlString {
}

impl PartialEq for OCamlString {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl PartialOrd for OCamlString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

extern "C" fn finalize(value: ocaml::core::Value) {
    let handle = ocaml::Value(value);
    let ptr = handle.field(0).mut_ptr_val();

    let btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = unsafe {
        Box::from_raw(ptr)
    };

    mem::drop(btreemap)
}

macro_rules! load_btreemap {
    ($v:ident, $btreemap:ident, $block:block) => {
        let ptr = $v.field(0).mut_ptr_val();
        let mut $btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = Box::from_raw(ptr);
        $block
        mem::forget($btreemap);
    }
}

macro_rules! modify_btreemap {
    ($v:ident, $btreemap:ident, $block:block) => {
        let ptr = $v.field(0).mut_ptr_val();
        let mut $btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = Box::from_raw(ptr);
        $block
        let new_ptr = Box::into_raw($btreemap);
        let _ = $v.store_field(0, ocaml::Value::ptr(new_ptr));
    }
}

caml!(btreemap_create, |unit|, <dest>, {
    let mut btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = Box::new(BTreeMap::new());
    let ptr = Box::into_raw(btreemap);
    dest = tuple!(ocaml::Value::ptr(ptr); finalize);
} -> dest);

caml!(btreemap_length, |handle|, <dest>, {
    load_btreemap!(handle, btreemap, {
        dest = ocaml::Value::usize(btreemap.len());
    });
} -> dest);

caml!(btreemap_is_empty, |handle|, <dest>, {
    load_btreemap!(handle, btreemap, {
        dest = ocaml::Value::bool(btreemap.is_empty());
    });
} -> dest);

caml!(btreemap_clear, |handle|, {
    modify_btreemap!(handle, btreemap, {
        btreemap.clear()
    });
});

caml!(btreemap_find_opt, |handle, index|, <dest>, {
    load_btreemap!(handle, btreemap, {
        if let Some(val) = btreemap.get(&OCamlString(index)) {
            dest = ocaml::Value::some(val.clone())
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_add, |handle, index, x|, {
    modify_btreemap!(handle, btreemap, {
        btreemap.insert(OCamlString(index), x);
    });
});

caml!(btreemap_iter, |handle, callback|, {
    load_btreemap!(handle, btreemap, {
        for (k, v) in btreemap.iter() {
            callback.call2(k.0.clone(), v.clone()).expect("Callback failure");
        }
    });
});
