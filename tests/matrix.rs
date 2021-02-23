extern crate neuronet;

use neuronet::{Matrix};

#[test]
fn matrix_test_t(){
    let mut m = Matrix::new(2,3);
    m.set(0,0,1);
    m.set(0,1,2);
    m.set(0,2,3);
    let mt = m.t();
    assert_eq!(mt.get(0,0),1);
    assert_eq!(mt.get(1,0),2);
    assert_eq!(mt.get(2,0),3);
}

#[test]
fn matrix_test_mul(){
    let mut m1 = Matrix::new(2,2);
    m1.set(0,0,1);
    m1.set(0,1,2);
    m1.set(1,0,3);
    m1.set(1,1,4);
    let mut m2 = Matrix::new(2,2);
    m2.set(0,0,5);
    m2.set(0,1,6);
    m2.set(1,0,7);
    m2.set(1,1,8);
    let m3 = Matrix::mul(&m1,&m2);
    println!("{}", m1);
    println!("{}", m2);
    println!("{}", m3);
    assert_eq!(m3.get(0,0),19);
    assert_eq!(m3.get(0,1),22);
    assert_eq!(m3.get(1,0),43);
    assert_eq!(m3.get(1,1),50);
}
