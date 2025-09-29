use tau_script::run::run_file;

#[test]
fn test_fibonacci() {
    run_file("./tas/fibonacci.tas".to_string());
}

#[test]
fn test_list() {
    run_file("./tas/list.tas".to_string());
}
