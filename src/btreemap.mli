type ('key, 'a) t

val create: unit -> ('key, 'a) t
val length: ('key, 'a) t -> int
val is_empty: ('key, 'a) t -> bool
val clear: ('key, 'a) t -> unit
val find_opt: ('key, 'a) t -> 'key -> 'a option
val add: ('key, 'a) t -> 'key -> 'a -> unit
val iter: ('key, 'a) t -> ('key -> 'a -> unit) -> unit

