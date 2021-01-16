use super::*;

#[test]
fn open_device_and_read_data() {
    let dev = Comedi::open(0).unwrap();
    let _sample = dev.data_read(0, 0, 0, AREF::GROUND).unwrap();
}

#[test]
fn range_lifetime_test() {
    //let range = { // shouldn't compile
    let dev = Comedi::open(0).unwrap();
    dev.get_range(0, 0, 0).unwrap();
    //};
}

#[test]
fn convert_to_phys() {
    let dev = Comedi::open(0).unwrap();
    set_global_oor_behavior(OORBehavior::NAN);
    let range = dev.get_range(0, 0, 0).unwrap();
    let maxdata = dev.get_maxdata(0, 0).unwrap();
    let sample = dev.data_read(0, 0, 0, AREF::GROUND).unwrap();
    to_phys(sample, range, maxdata).expect("Out of range value");
}
