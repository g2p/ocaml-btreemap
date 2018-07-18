#[macro_use]
extern crate ocaml;
use ocaml::ToValue;

use std::collections::BTreeMap;
use std::{mem, ptr};

extern "C" fn finalize(value: ocaml::core::Value) {
    let handle = ocaml::Value(value);
    let ptr = handle.custom_ptr_val_mut::<BTreeMap<Vec<u8>, ocaml::Value>>();
    unsafe {
        ptr::drop_in_place(ptr)
    }
}

// Converts an OCaml string (which may contain arbitrary, non-utf8 data)
// to a Vec<u8>.
// Nb: might be optimised to use slices instead, if we can figure out
// ownership.
fn str_val_to_vec(value : ocaml::Value) -> Vec<u8> {
    let vals = ocaml::Str::from(value);
    return vals.data().to_vec();
}

fn vec_to_str_val(vec : &Vec<u8>) -> ocaml::Value {
    ocaml::Value::from(
        ocaml::Str::from(vec.as_slice()))
}

macro_rules! btreemap {
    ($v:ident, $btreemap:ident, $block:block) => {
        let ptr = $v.custom_ptr_val_mut();
        let mut $btreemap: Box<BTreeMap<Vec<u8>, ocaml::Value>> = Box::from_raw(ptr);
        $block
        mem::forget($btreemap);
    }
}

caml!(btreemap_create, |n|, <dest>, {
    let mut btreemap: Box<BTreeMap<Vec<u8>, ocaml::Value>> = Box::new(BTreeMap::new());
    let ptr = Box::into_raw(btreemap);
    dest = ocaml::Value::alloc_custom(ptr, finalize);
} -> dest);

caml!(btreemap_length, |handle|, <dest>, {
    btreemap!(handle, btreemap, {
        dest = ocaml::Value::usize(btreemap.len());
    });
} -> dest);

caml!(btreemap_is_empty, |handle|, <dest>, {
    btreemap!(handle, btreemap, {
        dest = ocaml::Value::bool(btreemap.is_empty());
    });
} -> dest);

caml!(btreemap_clear, |handle|, {
    btreemap!(handle, btreemap, {
        btreemap.clear()
    });
});

caml!(btreemap_find_opt, |handle, index|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some(val) = btreemap.get(&str_val_to_vec(index)) {
            dest = ocaml::Value::some(val.clone());
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_add, |handle, index, x|, {
    btreemap!(handle, btreemap, {
        btreemap.insert(str_val_to_vec(index.clone()), x);
    });
});

caml!(btreemap_iter, |handle, callback|, {
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.iter() {
            callback.call2(vec_to_str_val(k), v.clone())
                .expect("Callback failure");
        }
    });
});

caml!(btreemap_exists, |handle, callback|, <dest>, {
    btreemap!(handle, btreemap, {
        let found = btreemap.iter().any(
            |(ref k, v)| {
                callback.call2(vec_to_str_val(k), v.clone())
                    .expect("Callback failure").usize_val() != 0
            });
        dest = ocaml::Value::bool(found);
    });
} -> dest);

caml!(btreemap_remove, |handle, index|, {
    btreemap!(handle, btreemap, {
        btreemap.remove(&str_val_to_vec(index));
    });
});

caml!(btreemap_min_binding, |handle|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.iter().next() {
            let tuple : ocaml::Tuple = tuple!(
                vec_to_str_val(k), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_max_binding, |handle|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.iter().next_back() {
            let tuple : ocaml::Tuple = tuple!(
                vec_to_str_val(k), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_mem, |handle, index|, <dest>, {
    btreemap!(handle, btreemap, {
        dest = ocaml::Value::bool(
            btreemap.contains_key(&str_val_to_vec(index)));
    });
} -> dest);

caml!(btreemap_fold, |handle, callback, acc|, <dest>, {
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.iter() {
            acc = callback.call3(vec_to_str_val(k), v.clone(), acc)
                .expect("Callback failure");
        }
    });
    dest = acc;
} -> dest);

caml!(btreemap_find_first_opt, |handle, start_inclusive|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.range(str_val_to_vec(start_inclusive)..).next() {
            let tuple : ocaml::Tuple = tuple!(vec_to_str_val(k), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_find_last_opt, |handle, end_exclusive|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, ref v)) = btreemap.range(..str_val_to_vec(end_exclusive)).next_back() {
            let tuple : ocaml::Tuple = tuple!(vec_to_str_val(k), v.clone());
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_iter_range,
      |handle, start_inclusive, end_exclusive, callback|,
{
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.range(
            str_val_to_vec(start_inclusive)..str_val_to_vec(end_exclusive))
        {
            callback.call2(vec_to_str_val(k), v.clone())
                .expect("Callback failure");
        }
    });
});

caml!(btreemap_iter_inclusive_range,
      |handle, start_inclusive, end_inclusive, callback|,
{
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.range(
            str_val_to_vec(start_inclusive)..=str_val_to_vec(end_inclusive))
        {
            callback.call2(vec_to_str_val(k), v.clone())
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
    let mut after_keys = ocaml::Str::from(after_key.clone());
    let split1 = next_key(after_keys.data());

    let mut map2;
    btreemap!(handle, btreemap, {
        map2 = btreemap.split_off(&split1);
    });
    let ptr = Box::into_raw(Box::new(map2));
    dest = ocaml::Value::alloc_custom(ptr, finalize);
} -> dest);
