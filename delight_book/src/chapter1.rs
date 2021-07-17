// pub fn aot_v1_mop<'a>
#![feature(asm)]

// TABLE 1–1. EXPRESSIONS OF C AND COMPUTER ALGEBR
pub fn ground_get_hex(index:i32) -> usize {
    let a = vec![0,1,2,3,4,5,6,7,8,9,10].iter();
    # [cfg(target_arch = "riscv64")]
    unsafe{

    }
    // # [cfg(target_arch = "x86_64")]
    //     unsafe{
    //     asm!(mov,)
    // }
    todo!()
}

pub fn ground_get_kth()->usize{
    todo!()
}

pub fn ground_arith()->usize{
    todo!()
}

#[test]
fn test_ground() {

}
