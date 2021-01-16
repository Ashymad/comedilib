use super::*;

#[test]
fn open_device_and_read_data() {
    let dev = Comedi::open(0).unwrap();
    let sample = dev.data_read(0, 0, 0, AREF::GROUND).unwrap();
}
