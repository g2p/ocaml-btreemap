#[macro_use]
extern crate ocaml;
use ocaml::ToValue;

use std::collections::BTreeMap;
use std::{mem, ptr};

extern "C" fn finalize(value: ocaml::core::Value) {
    let handle = ocaml::Value(value);
    let ptr = handle.custom_ptr_val_mut::<BTreeMap<Vec<u8>, u64>>();
    unsafe {
        ptr::drop_in_place(ptr)
    }
}

// Converts an OCaml string (which may contain arbitrary, non-utf8 data)
// to a Vec<u8>.
// Nb: might be optimised to use slices instead, if we can figure out
// ownership.
fn str_val_to_vec(value: ocaml::Value) -> Vec<u8> {
    let vals = ocaml::Str::from(value);
    return vals.data().to_vec();
}

// Caller must declare with caml_local! or similar
fn vec_to_str_val(vec: &Vec<u8>) -> ocaml::Value {
    ocaml::Value::from(
        ocaml::Str::from(vec.as_slice()))
}

fn val_to_u64(value: ocaml::Value) -> u64 {
    value.int64_val() as u64
}

// Caller must declare with caml_local! or similar
fn u64_to_val(unsigned: u64) -> ocaml::Value {
    ocaml::Value::int64(unsigned as i64)
}

macro_rules! btreemap {
    ($v:ident, $btreemap:ident, $block:block) => {
        let ptr = $v.custom_ptr_val_mut();
        let mut $btreemap: Box<BTreeMap<Vec<u8>, u64>> = Box::from_raw(ptr);
        $block
        mem::forget($btreemap);
    }
}

caml!(btreemap_create, |n|, <dest>, {
    let mut btreemap: Box<BTreeMap<Vec<u8>, u64>> = Box::new(BTreeMap::new());
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

caml!(btreemap_find_opt, |index, handle|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some(v) = btreemap.get(&str_val_to_vec(index)) {
            caml_local!(v1);
            v1 = u64_to_val(*v);
            dest = ocaml::Value::some(v1);
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_add, |index, x, handle|, {
    btreemap!(handle, btreemap, {
        btreemap.insert(str_val_to_vec(index), val_to_u64(x));
    });
});

caml!(btreemap_update, |index, x, handle|, {
    btreemap!(handle, btreemap, {
        assert!(btreemap.insert(str_val_to_vec(index), val_to_u64(x)).is_some());
    });
});

caml!(btreemap_xadd, |index, x, handle|, {
    btreemap!(handle, btreemap, {
        assert!(btreemap.insert(str_val_to_vec(index), val_to_u64(x)).is_none());
    });
});

caml!(btreemap_iter, |callback, handle|, {
    caml_local!(k1, v1);
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.iter() {
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            callback.call2_exn(k1, v1).expect("Callback failure");
        }
    });
});

caml!(btreemap_exists, |callback, handle|, <dest>, {
    btreemap!(handle, btreemap, {
        let found = btreemap.iter().any(
            |(ref k, v)| {
                caml_local!(k1, v1);
                k1 = vec_to_str_val(k);
                v1 = u64_to_val(*v);
                callback.call2(k1, v1)
                    .expect("Callback failure").usize_val() != 0
            });
        dest = ocaml::Value::bool(found);
    });
} -> dest);

caml!(btreemap_remove, |index, handle|, {
    btreemap!(handle, btreemap, {
        btreemap.remove(&str_val_to_vec(index));
    });
});

caml!(btreemap_min_binding, |handle|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, v)) = btreemap.iter().next() {
            caml_local!(k1, v1, tuple);
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            let tuple : ocaml::Tuple = tuple!(k1, v1);
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_max_binding, |handle|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, v)) = btreemap.iter().next_back() {
            caml_local!(k1, v1, tuple);
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            let tuple : ocaml::Tuple = tuple!(k1, v1);
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_mem, |index, handle|, <dest>, {
    btreemap!(handle, btreemap, {
        dest = ocaml::Value::bool(
            btreemap.contains_key(&str_val_to_vec(index)));
    });
} -> dest);

caml!(btreemap_fold, |callback, handle, acc|, <dest>, {
    caml_local!(k1, v1, acc1);
    acc1 = acc;
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.iter() {
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            acc1 = callback.call3(k1, v1, acc1).expect("Callback failure");
        }
    });
    dest = acc1;
} -> dest);

caml!(btreemap_find_first_opt, |start_inclusive, handle|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, v)) = btreemap.range(str_val_to_vec(start_inclusive)..).next() {
            caml_local!(k1, v1, tuple);
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            let tuple : ocaml::Tuple = tuple!(k1, v1);
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_find_last_opt, |end_exclusive, handle|, <dest>, {
    btreemap!(handle, btreemap, {
        if let Some((ref k, v)) = btreemap.range(..str_val_to_vec(end_exclusive)).next_back() {
            caml_local!(k1, v1, tuple);
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            let tuple : ocaml::Tuple = tuple!(k1, v1);
            dest = ocaml::Value::some(ocaml::Value::from(tuple));
        } else {
            dest = ocaml::Value::none();
        }
    });
} -> dest);

caml!(btreemap_iter_range,
      |start_inclusive, end_exclusive, callback, handle|,
{
    caml_local!(k1, v1);
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.range(
            str_val_to_vec(start_inclusive)..str_val_to_vec(end_exclusive))
        {
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            callback.call2(k1, v1).expect("Callback failure");
        }
    });
});

caml!(btreemap_iter_inclusive_range,
      |start_inclusive, end_inclusive, callback, handle|,
{
    caml_local!(k1, v1);
    btreemap!(handle, btreemap, {
        for (k, v) in btreemap.range(
            str_val_to_vec(start_inclusive)..=str_val_to_vec(end_inclusive))
        {
            k1 = vec_to_str_val(k);
            v1 = u64_to_val(*v);
            callback.call2(k1, v1).expect("Callback failure");
        }
    });
});

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

caml!(btreemap_split_off_after, |after_key, handle|, <dest>, {
    let mut after_keys = ocaml::Str::from(after_key);
    let split1 = next_key(after_keys.data());

    let mut map2;
    btreemap!(handle, btreemap, {
        map2 = btreemap.split_off(&split1);
    });
    let ptr = Box::into_raw(Box::new(map2));
    dest = ocaml::Value::alloc_custom(ptr, finalize);
} -> dest);
