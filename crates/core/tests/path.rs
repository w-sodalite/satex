use satex_core::util::{canonicalize, remove_end_sep, remove_start_sep};

#[test]
fn test_canonicalize() {
    assert_eq!("/api/v1/resource", canonicalize("/api/v1/resource"));
    assert_eq!("/api/resource", canonicalize("/api/v1/../resource"));
    assert_eq!("/api/v1/resource", canonicalize("/api/v1/./resource"));
    assert_eq!("/api/resource", canonicalize("/api/v1/./../resource"));
    assert_eq!("/", canonicalize("/.."));
    assert_eq!("/", canonicalize("../.."));
    assert_eq!("/", canonicalize("/."));
    assert_eq!("/", canonicalize("./."));
}

#[test]
fn test_remove_start_sep() {
    assert_eq!("api/vi/resource", remove_start_sep("api/vi/resource"));
    assert_eq!("api/vi/resource", remove_start_sep("/api/vi/resource"));
    assert_eq!("api/vi/resource", remove_start_sep("//api/vi/resource"));
    assert_eq!("api/vi/resource", remove_start_sep("///api/vi/resource"));
    assert_eq!("", remove_start_sep("///////////////////"));
    assert_eq!("a", remove_start_sep("///////////////////a"));
}

#[test]
fn test_remove_end_sep() {
    assert_eq!("/api/vi/resource", remove_end_sep("/api/vi/resource"));
    assert_eq!("/api/vi/resource", remove_end_sep("/api/vi/resource/"));
    assert_eq!("/api/vi/resource", remove_end_sep("/api/vi/resource//"));
    assert_eq!("/api/vi/resource", remove_end_sep("/api/vi/resource///"));
    assert_eq!("", remove_end_sep("///////////////////"));
    assert_eq!("a", remove_end_sep("a///////////////////"));
}
