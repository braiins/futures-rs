extern crate futures;

use futures::future::*;

#[test]
fn smoke() {
    let v = vec![
        finished(1).boxed(),
        failed(2).boxed(),
        finished(3).boxed(),
    ];

    let (i, idx, v) = select_all(v).wait().ok().unwrap();
    assert_eq!(i, 1);
    assert_eq!(idx, 0);

    let (i, idx, v) = select_all(v).wait().err().unwrap();
    assert_eq!(i, 2);
    assert_eq!(idx, 0);

    let (i, idx, v) = select_all(v).wait().ok().unwrap();
    assert_eq!(i, 3);
    assert_eq!(idx, 0);

    assert!(v.len() == 0);
}
