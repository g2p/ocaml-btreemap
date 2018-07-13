#[macro_use]
extern crate ocaml;
use ocaml::ToValue;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::mem;

struct OCamlString(ocaml::Value);

impl Ord for OCamlString {
    fn cmp(&self, other: &Self) -> Ordering {
        // XXX Will segfault if these are not really strings
        let selfs = ocaml::Str::from(self.0.clone());
        let others = ocaml::Str::from(other.0.clone());
        return selfs.as_str().cmp(others.as_str());
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
            dest = ocaml::Value::some(val.clone());
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
            callback.call2(k.0.clone(), v.clone())
                .expect("Callback failure");
        }
    });
});

caml!(btreemap_exists, |handle, callback|, <dest>, {
    load_btreemap!(handle, btreemap, {
        let found = btreemap.iter().any(
            |(ref k, ref v)|
                callback.call2(k.0.clone(), (*v).clone())
                .expect("Callback failure").usize_val() != 0
            );
        dest = ocaml::Value::bool(found);
    });
} -> dest);

caml!(btreemap_remove, |handle, index|, {
    modify_btreemap!(handle, btreemap, {
        btreemap.remove(&OCamlString(index));
    });
});

caml!(btreemap_min_binding, |handle|, <dest>, {
    load_btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.iter().next() {
            let tuple : ocaml::Tuple = tuple!(k.0.clone(), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_max_binding, |handle|, <dest>, {
    load_btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.iter().next_back() {
            let tuple : ocaml::Tuple = tuple!(k.0.clone(), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_mem, |handle, index|, <dest>, {
    load_btreemap!(handle, btreemap, {
        dest = ocaml::Value::bool(
            btreemap.contains_key(&OCamlString(index)));
    });
} -> dest);

caml!(btreemap_fold, |handle, callback, acc|, <dest>, {
    load_btreemap!(handle, btreemap, {
        for (k, v) in btreemap.iter() {
            acc = callback.call3(k.0.clone(), v.clone(), acc)
                .expect("Callback failure");
        }
    });
    dest = acc;
} -> dest);

caml!(btreemap_find_first_opt, |handle, start_inclusive|, <dest>, {
    load_btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.range(OCamlString(start_inclusive)..).next() {
            let tuple : ocaml::Tuple = tuple!(k.0.clone(), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_find_last_opt, |handle, end_exclusive|, <dest>, {
    load_btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.range(..OCamlString(end_exclusive)).next_back() {
            let tuple : ocaml::Tuple = tuple!(k.0.clone(), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_iter_range,
      |handle, start_inclusive, end_exclusive, callback|,
{
    load_btreemap!(handle, btreemap, {
        for (k, v) in btreemap.range(
            OCamlString(start_inclusive)..OCamlString(end_exclusive))
        {
            callback.call2(k.0.clone(), v.clone())
                .expect("Callback failure");
        }
    });
});

caml!(btreemap_iter_inclusive_range,
      |handle, start_inclusive, end_inclusive, callback|,
{
    load_btreemap!(handle, btreemap, {
        for (k, v) in btreemap.range(
            OCamlString(start_inclusive)..=OCamlString(end_inclusive))
        {
            callback.call2(k.0.clone(), v.clone())
                .expect("Callback failure");
        }
    });
});
