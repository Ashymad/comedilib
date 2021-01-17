use super::*;

#[test]
fn open_device_and_read_data() {
    let dev = Comedi::open(0).unwrap();
    let _sample = dev.data_read(0, 0, 0, ARef::Ground).unwrap();
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
    set_global_oor_behavior(OORBehavior::NaN);
    let range = dev.get_range(0, 0, 0).unwrap();
    let maxdata = dev.get_maxdata(0, 0).unwrap();
    let sample = dev.data_read(0, 0, 0, ARef::Ground).unwrap();
    let _unitstr = match range.unit() {
        Unit::MiliAmper => "mA",
        Unit::Volt => "V",
        Unit::None => "",
    };
    to_phys(sample, &range, maxdata).expect(&format!(
        "Value out of range [{}, {}]",
        range.min(),
        range.max()
    ));
}

#[test]
fn cr_pack_test() {
    let chan1 = 3;
    let packed = cr_pack(chan1, 13, ARef::Diff);
    let (chan2, rng, aref) = cr_unpack(packed);
    assert_eq!(chan1, chan2);
    assert_eq!(13, rng);
    assert_eq!(ARef::Diff, aref);
}

#[test]
fn cmd_test() {
    let comedi = Comedi::open(0).unwrap();
    let mut cmd = comedi.get_cmd_generic_timed(0, 16, 1000).unwrap();
    print_cmd(&cmd);
    let chanlist = vec![(0, 0, ARef::Ground), (1, 1, ARef::Diff), (2, 2, ARef::Common)];
    cmd.set_chanlist(&chanlist);
    print_cmd(&cmd);
    for ((chan1, rng1, aref1), (chan2, rng2, aref2)) in cmd.chanlist().unwrap().iter().zip(chanlist) {
        assert_eq!(*chan1, chan2);
        assert_eq!(*rng1, rng2);
        assert_eq!(*aref1, aref2);
    }
}

fn print_cmd(cmd: &Cmd) {
    print!(
        "Cmd {{\n  \
        subdev: {},\n  \
        start_src: {:?},\n  \
        start_arg: {},\n  \
        scan_begin_src: {:?},\n  \
        scan_begin_arg: {},\n  \
        convert_src: {:?},\n  \
        convert_arg: {},\n  \
        scan_end_src: {:?},\n  \
        scan_end_arg: {},\n  \
        stop_src: {:?},\n  \
        stop_arg: {},\n  \
        chanlist: {:?},\n}}\n",
        cmd.subdev(),
        cmd.start_src(),
        cmd.start_arg(),
        cmd.scan_begin_src(),
        cmd.scan_begin_arg(),
        cmd.convert_src(),
        cmd.convert_arg(),
        cmd.scan_end_src(),
        cmd.scan_end_arg(),
        cmd.stop_src(),
        cmd.stop_arg(),
        cmd.chanlist()
    );
}
