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
      Hashtbl.replace keys t i;
      Btreemap.add t i m
    done;
    Printf.printf "Length: %d\n" (Btreemap.length m);
    assert (Btreemap.length m = n);
    assert (Btreemap.find_opt "1" m = Some 1);
    Btreemap.add "1" 555 m;
    assert (Btreemap.find_opt "1" m = Some 555);
    for i = 0 to n do
        match Btreemap.find_opt (string_of_int i) m with
        | Some x -> assert (i > 0); Printf.printf "some %d\n%!" x
        | None -> assert (i = 0); print_endline "none"
    done;
    Btreemap.clear m;
    assert (Btreemap.is_empty m);
    assert (Btreemap.length m = 0);
    Gc.minor ();
    Gc.full_major ()
