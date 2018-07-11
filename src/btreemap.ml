let () = Callback.register "compare" compare
let () = Printf.printf "Callback registered\n"

type ('key, 'a) t

external create: unit -> ('key, 'a) t = "btreemap_create"
external length: ('key, 'a) t -> int = "btreemap_length"
external is_empty: ('key, 'a) t -> bool = "btreemap_is_empty"
external clear: ('key, 'a) t -> unit = "btreemap_clear"
external find_opt: ('key, 'a) t -> 'key -> 'a option = "btreemap_find_opt"
external add: ('key, 'a) t -> 'key -> 'a -> unit = "btreemap_add"
external iter: ('key, 'a) t -> ('key -> 'a -> unit) -> unit = "btreemap_iter"

