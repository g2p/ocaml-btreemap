type 'a t

external create: unit -> 'a t = "btreemap_create"
external length: 'a t -> int = "btreemap_length"
external is_empty: 'a t -> bool = "btreemap_is_empty"
external clear: 'a t -> unit = "btreemap_clear"
external find_opt: string -> 'a t -> 'a option = "btreemap_find_opt"
external mem: string -> 'a t -> bool = "btreemap_mem"
external add: string -> 'a -> 'a t -> unit = "btreemap_add"
external update: string -> 'a -> 'a t -> unit = "btreemap_update"
external xadd: string -> 'a -> 'a t -> unit = "btreemap_xadd"
external remove: string -> 'a t -> unit = "btreemap_remove"
external iter: (string -> 'a -> unit) -> 'a t -> unit = "btreemap_iter"
external iter_range: string -> string -> (string -> 'a -> unit) -> 'a t -> unit = "btreemap_iter_range"
external iter_inclusive_range: string -> string -> (string -> 'a -> unit) -> 'a t -> unit = "btreemap_iter_inclusive_range"
external fold: (string -> 'a -> 'b -> 'b) -> 'a t -> 'b -> 'b = "btreemap_fold"
external exists: (string -> 'a -> bool) -> 'a t -> bool = "btreemap_exists"
external min_binding: 'a t -> (string * 'a) option = "btreemap_min_binding"
external max_binding: 'a t -> (string * 'a) option = "btreemap_max_binding"
external find_first_opt: string -> 'a t -> (string * 'a) option = "btreemap_find_first_opt"
external find_last_opt: string -> 'a t -> (string * 'a) option = "btreemap_find_last_opt"
external split_off_after: string -> 'a t -> 'a t = "btreemap_split_off_after"

