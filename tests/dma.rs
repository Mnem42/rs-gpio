use rs_gpio::dma::test;

#[test]
fn dma_test_1(){
    panic!("result: {}",unsafe { test() });
}