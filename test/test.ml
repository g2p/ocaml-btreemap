let _ =
    let v = BTreeMap.create 10 in
    assert (BTreeMap.length v = 0);
    Printf.printf "%d\n" (BTreeMap.length v);
    for i = 1 to 100 do
        BTreeMap.push v i
    done;
    assert (BTreeMap.length v = 100);
    Printf.printf "%d\n" (BTreeMap.length v);
    assert BTreeMap.(v.|[0] = Some 1);
    BTreeMap.(v.|[0] <- 555);
    assert BTreeMap.(v.|[0] = Some 555);
    for i = 0 to 100 do
        match BTreeMap.pop v with
        | Some x -> assert (i < 100); Printf.printf "some %d\n" x
        | None -> assert (i = 100); print_endline "none"
    done;
    BTreeMap.clear v;
    assert (BTreeMap.length v = 0)
