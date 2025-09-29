use tau_script::run::run_file;

#[test]
fn test_fibonacci() {
    run_file("./tus/fibonacci.tus".to_string());
}

#[test]
fn test_list() {
    run_file("./tus/list.tus".to_string());
}
