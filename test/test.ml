let keys = Hashtbl.create 32

let _ =
    let n = 15000 in
    let m = Btreemap.create () in
    print_endline "created";
    assert (Btreemap.is_empty m);
    print_endline "is empty";
    assert (Btreemap.length m = 0);
    for i = 1 to n do
      let t = string_of_int i in
      let l = Int64.of_int i in
      Hashtbl.replace keys t l;
      Btreemap.add t l m
    done;
    Printf.printf "Length: %d\n" (Btreemap.length m);
    assert (Btreemap.length m = n);
    assert (Btreemap.find_opt "1" m = Some 1L);
    Btreemap.add "1" 555L m;
    assert (Btreemap.find_opt "1" m = Some 555L);
    for i = 0 to n do
        match Btreemap.find_opt (string_of_int i) m with
        | Some x -> assert (i > 0); Printf.printf "some %Ld\n%!" x
        | None -> assert (i = 0); print_endline "none"
    done;
    let m2 = Btreemap.create () in
    Btreemap.iter (fun k v ->
        Printf.printf "%s -> %Ld\n" k v;
        Btreemap.add k v m2;
      ) m;
    Hashtbl.clear keys;
    Btreemap.clear m;
    Btreemap.clear m2;
    assert (Btreemap.is_empty m);
    assert (Btreemap.length m = 0);
    Gc.minor ();
    Gc.full_major ()

(*
let _ =
  let n = 15000 in
  let m = Btreemap.create () in
  for i = 1 to n do
    let t = string_of_int i in
    Btreemap.add t (i, i) m;
  done;
  for i = 1 to n do
    let t = string_of_int i in begin
        match Btreemap.find_opt (string_of_int i) m with
        | Some (x, y) -> Printf.printf "%d some %d %d\n%!" i x y
        | None -> print_endline "none"
      end;
    assert (Btreemap.find_opt t m = Some (i, i));
  done;
  Btreemap.iter (fun k (v1, v2) ->
      Printf.printf "%s -> (%d, %d)\n" k v1 v2) m;
   *)

