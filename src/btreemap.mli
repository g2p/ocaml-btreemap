type 'a t

val create: unit -> 'a t
val length: 'a t -> int
val is_empty: 'a t -> bool
val clear: 'a t -> unit
val find_opt: 'a t -> string -> 'a option
val mem: 'a t -> string -> bool
val add: 'a t -> string -> 'a -> unit
val remove: 'a t -> string -> unit
val iter: 'a t -> (string -> 'a -> unit) -> unit
val iter_range: 'a t -> string -> string -> (string -> 'a -> unit) -> unit
val iter_inclusive_range: 'a t -> string -> string -> (string -> 'a -> unit) -> unit
val fold: 'a t -> (string -> 'a -> 'b -> 'b) -> 'b -> 'b
val exists: 'a t -> (string -> 'a -> bool) -> bool
val min_binding: 'a t -> (string * 'a) option
val max_binding: 'a t -> (string * 'a) option
val find_first_opt: 'a t -> string -> (string * 'a) option
val find_last_opt: 'a t -> string -> (string * 'a) option
val split_off_after: 'a t -> string -> 'a t

