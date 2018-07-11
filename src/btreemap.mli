type ('key, 'a) t

val create: unit -> ('key, 'a) t
val length: ('key, 'a) t -> int
val is_empty: ('key, 'a) t -> bool
val clear: ('key, 'a) t -> unit
val find_opt: ('key, 'a) t -> 'key -> 'a option
val mem: ('key, 'a) t -> 'key -> bool
val add: ('key, 'a) t -> 'key -> 'a -> unit
val remove: ('key, 'a) t -> 'key -> unit
val iter: ('key, 'a) t -> ('key -> 'a -> unit) -> unit
val fold: ('key, 'a) t -> ('key -> 'a -> 'b -> 'b) -> 'b -> 'b
val max_binding: ('key, 'a) t -> ('key * 'a) option
val find_first_opt: ('key, 'a) t -> 'key -> ('key * 'a) option

