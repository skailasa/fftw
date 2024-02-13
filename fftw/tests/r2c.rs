use fftw::plan::*;
use fftw::types::*;
use num_traits::Zero;
use std::time::*;

/// Check successive forward and backward transform equals to the identity
#[test]
fn c2r2c_identity() {
    let n = 100000;
    let size_real = 152;
    let ntargets = 32768;
    let n = size_real*ntargets;
    let s = Instant::now();
    let mut a = vec![1.0; n];
    let mut b = vec![c64::zero(); n/2 + 1];
    println!("alloc time {:?} {:?}", s.elapsed(), a[0]);

    let r2c: R2CPlan64 = R2CPlan::new(&[n], &mut a, &mut b, Flag::MEASURE).unwrap();
    
    // // println!("before {:?} {:?}", a, a.len());

    let s = Instant::now();
    r2c.r2c(&mut a, &mut b).unwrap();
    println!("fft {:?}", s.elapsed());
    
    // println!("after {:?} {:?}", b, b.len());
    // for v in a.iter() {
    //     let ans = [n as f64, 0.0];
    //     let dif = ((v[0] - ans[0]).powf(2.) + (v[1]-ans[1]).powf(2.0)).powf(0.5);
    //     if dif > 1e-7 {
    //         panic!("Large difference: v={:?}, dif={}", v, dif);
    //     }
    // }


    assert!(false)
}