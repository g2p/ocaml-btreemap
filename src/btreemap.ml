type ('key, 'a) t

external create: unit -> ('key, 'a) t = "btreemap_create"
external length: ('key, 'a) t -> int = "btreemap_length"
external is_empty: ('key, 'a) t -> bool = "btreemap_is_empty"
external clear: ('key, 'a) t -> unit = "btreemap_clear"
external find_opt: ('key, 'a) t -> 'key -> 'a option = "btreemap_find_opt"
external mem: ('key, 'a) t -> 'key -> bool = "btreemap_mem"
external add: ('key, 'a) t -> 'key -> 'a -> unit = "btreemap_add"
external remove: ('key, 'a) t -> 'key -> unit = "btreemap_remove"
external iter: ('key, 'a) t -> ('key -> 'a -> unit) -> unit = "btreemap_iter"
external iter_range: ('key, 'a) t -> 'key -> 'key -> ('key -> 'a -> unit) -> unit = "btreemap_iter_range"
external iter_inclusive_range: ('key, 'a) t -> 'key -> 'key -> ('key -> 'a -> unit) -> unit = "btreemap_iter_inclusive_range"
external fold: ('key, 'a) t -> ('key -> 'a -> 'b -> 'b) -> 'b -> 'b = "btreemap_fold"
external min_binding: ('key, 'a) t -> ('key * 'a) option = "btreemap_min_binding"
external max_binding: ('key, 'a) t -> ('key * 'a) option = "btreemap_max_binding"
external find_first_opt: ('key, 'a) t -> 'key -> ('key * 'a) option = "btreemap_find_first_opt"

