let _ =
    Callback.register "compare" compare;
    Printf.printf "Callback registered again\n";
    Printf.eprintf "Callback registered again\n";
    let m = Btreemap.create () in
    assert (Btreemap.is_empty m);
    assert (Btreemap.length m = 0);
    Printf.printf "%d\n" (Btreemap.length m);
    for i = 1 to 100 do
        Btreemap.add m i i
    done;
    assert (Btreemap.length m = 100);
    Printf.printf "%d\n" (Btreemap.length m);
    assert (Btreemap.find_opt m 1 = Some 1);
    Btreemap.add m 1 555;
    assert (Btreemap.find_opt m 1 = Some 555);
    for i = 0 to 100 do
        match Btreemap.find_opt m i with
        | Some x -> assert (i > 0); Printf.printf "some %d\n" x
        | None -> assert (i = 0); print_endline "none"
    done;
    Btreemap.clear m;
    assert (Btreemap.is_empty m);
    assert (Btreemap.length m = 0);
