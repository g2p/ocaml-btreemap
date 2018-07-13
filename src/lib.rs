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
    let ptr = handle.custom_ptr_val_mut::<BTreeMap<OCamlString, ocaml::Value>>();

    let btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = unsafe {
        Box::from_raw(ptr)
    };

    mem::drop(btreemap)
}

macro_rules! load_btreemap {
    ($v:ident, $btreemap:ident, $block:block) => {
        let ptr = $v.custom_ptr_val_mut();
        let $btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = Box::from_raw(ptr);
        $block
        mem::forget($btreemap);
    }
}

macro_rules! modify_btreemap {
    ($v:ident, $btreemap:ident, $block:block) => {
        let ptr = $v.custom_ptr_val_mut();
        let mut $btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = Box::from_raw(ptr);
        $block
        let ptr = Box::into_raw($btreemap);
        $v.set_custom(ptr);
    }
}

caml!(btreemap_create, |unit|, <dest>, {
    let mut btreemap: Box<BTreeMap<OCamlString, ocaml::Value>> = Box::new(BTreeMap::new());
    let ptr = Box::into_raw(btreemap);
    dest = ocaml::Value::alloc_custom(ptr, finalize);
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

// TODO Get an ocaml::Str -> &[u8] conversion
// The Rust str API is made for UTF-8, and therefore has some
// restrictions: no arbitrary data, no reversing
fn next_key(key: &[u8]) -> Vec<u8> {
    let mut is_top_key = true;
    let mut r = key.iter().rev().scan(1, |state, &x| {
        let r = x.wrapping_add(*state);
        *state = if r == 0 { 1 } else { 0 };
        if *state == 0 {
            is_top_key = false;
        }
        Some(r)
    }).collect::<Vec<_>>();
    assert!(!is_top_key);
    r.reverse();
    return r;
}

caml!(btreemap_split_off_after, |handle, after_key|, <dest>, {
    // Compute next_key as an OCamlString
    // XXX Will segfault if not really an OCaml string
    let mut after_keys = ocaml::Str::from(after_key.clone());
    let split1 = next_key(after_keys.data());

    // This is a little bit dirty.
    // The OCaml bridge doesn't currently provide a way to
    // construct OCaml strings, so we clone one and update the
    // data.
    // Except there's no way to deep clone a string!
    // So we'll just update it, restore it, hope no one notices.
    let original = after_keys.data().to_vec();
    after_keys.data_mut().copy_from_slice(&split1[..]);

    let map2;
    modify_btreemap!(handle, btreemap, {
        map2 = btreemap.split_off(&OCamlString(after_key));
    });
    let ptr = Box::into_raw(Box::new(map2));
    dest = ocaml::Value::alloc_custom(ptr, finalize);
    after_keys.data_mut().copy_from_slice(&original[..]);
} -> dest);
