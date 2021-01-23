use super::*;
//use super::flags::Bitfield;

#[test]
fn open_device_and_read_data() {
    let mut comedi = Comedi::open(0).unwrap();
    comedi.data_read(0, 0, 0, ARef::Ground).unwrap();

    let datalen = 10;
    let mut data = vec![0; datalen];
    comedi
        .data_read_n(0, 0, 0, ARef::Ground, &mut data)
        .unwrap();
    data.iter().for_each(|el| assert_ne!(*el, 0));
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
    let mut dev = Comedi::open(0).unwrap();
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
    let subdevice = 0;
    let bufsz = 10000;
    let amount = 100;
    let chanlist = vec![
        (0, 0, ARef::Ground),
        (1, 0, ARef::Ground),
        (2, 0, ARef::Ground),
    ];
    let mut total = 0;

    let mut comedi = Comedi::open(0).unwrap();
    let mut cmd = comedi
        .get_cmd_generic_timed(subdevice, chanlist.len().try_into().unwrap(), 1000)
        .unwrap();
    cmd.set_chanlist(&chanlist);

    for ((chan1, rng1, aref1), (chan2, rng2, aref2)) in
        cmd.chanlist().unwrap().iter().zip(&chanlist)
    {
        assert_eq!(chan1, chan2);
        assert_eq!(rng1, rng2);
        assert_eq!(aref1, aref2);
    }

    cmd.set_stop(StopTrigger::Count, amount);

    loop {
        match comedi.command_test(&mut cmd).unwrap() {
            CommandTestResult::Ok => {
                println!("Test succeeded!");
                print_cmd(&cmd);
                break;
            }
            oth => {
                println!("Test failed with: {:?}", oth);
                print_cmd(&cmd);
            }
        };
    }

    comedi.set_read_subdevice(subdevice).unwrap();
    assert_eq!(comedi.get_read_subdevice().unwrap(), subdevice);

    let subdev_flags = comedi.get_subdevice_flags(subdevice).unwrap();

    comedi.command(&cmd).unwrap();
    if subdev_flags.is_set(SDF::LSAMPL) {
        let mut buf = vec![0; bufsz];
        loop {
            let read = comedi.read_sampl::<LSampl>(&mut buf).unwrap();
            if read == 0 {
                break;
            }
            total += read;
        }
    } else {
        let mut buf = vec![0; bufsz];
        loop {
            let read = comedi.read_sampl::<Sampl>(&mut buf).unwrap();
            if read == 0 {
                break;
            }
            total += read;
        }
    };
    assert_eq!(total, chanlist.len() * amount as usize);
}

fn print_cmd(cmd: &Cmd) {
    println!(
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
        chanlist: {:?},\n}}",
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
