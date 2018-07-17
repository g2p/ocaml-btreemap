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
      Btreemap.add m t i
    done;
    Printf.printf "Length: %d\n" (Btreemap.length m);
    assert (Btreemap.length m = n);
    assert (Btreemap.find_opt m "1" = Some 1);
    Btreemap.add m "1" 555;
    assert (Btreemap.find_opt m "1" = Some 555);
    for i = 0 to n do
        match Btreemap.find_opt m (string_of_int i) with
        | Some x -> assert (i > 0); Printf.printf "some %d\n%!" x
        | None -> assert (i = 0); print_endline "none"
    done;
    Btreemap.clear m;
    assert (Btreemap.is_empty m);
    assert (Btreemap.length m = 0);
    Gc.minor ();
    Gc.full_major ()
