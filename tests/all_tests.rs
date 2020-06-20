use i8583::Packer;
use i8583::Unpacker;

#[test]
#[should_panic]
fn unpack_malformed_message() {
    let mut iso_unpacker = Unpacker::new("080080000000000000000000000000000001".as_bytes());
    let _ = iso_unpacker.unpack(&i8583::v1987::SPEC).unwrap();
}

#[test]
fn unpack_empty_64_bits_message() {
    let mut iso_unpacker = Unpacker::new("08000000000000000000".as_bytes());
    let fields = iso_unpacker.unpack(&i8583::v1987::SPEC).unwrap();

    assert_eq!(fields.len(), 65);
    assert_eq!(fields[0].expect("Field 0"), "0800".as_bytes());
    for field in fields.iter().skip(2) {
        if let Some(_) = field {
            panic!("Didn't expect to find something in empty iso message");
        }
    }
}

#[test]
fn unpack_empty_128_bits_message() {
    let mut iso_unpacker = Unpacker::new("080080000000000000000000000000000000".as_bytes());
    let fields = iso_unpacker.unpack(&i8583::v1987::SPEC).unwrap();

    assert_eq!(fields.len(), 129);
    assert_eq!(fields[0].expect("Field 0"), "0800".as_bytes());
    for field in fields.iter().skip(2) {
        if let Some(_) = field {
            panic!("Didn't expect to find something in empty iso message");
        }
    }
}

#[test]
fn unpack_64_bits() {
    let response = "080022380000008000009A0000070814102414102414102407082020XXXX";
    let mut iso = Unpacker::new(response.as_bytes());
    let fields = iso
        .unpack(&i8583::v1987::SPEC)
        .unwrap_or_else(|err_msg| panic!("{}", err_msg));

    assert_eq!(fields.len(), 65);

    assert_eq!(fields[0].expect("Field 0"), "0800".as_bytes());
    assert_eq!(fields[1].expect("Field 1"), "2238000000800000".as_bytes());
    assert_eq!(fields[3].expect("Field 3"), "9A0000".as_bytes());
    assert_eq!(fields[7].expect("Field 7"), "0708141024".as_bytes());
    assert_eq!(fields[11].expect("Field 11"), "141024".as_bytes());
    assert_eq!(fields[12].expect("Field 12"), "141024".as_bytes());
    assert_eq!(fields[41].expect("Field 41"), "2020XXXX".as_bytes());
}

#[test]
fn unpack_128_bits() {
    let response = "0200F23C46D129E09200000000000000002116543210123456789800000000000001000005111713241713241713240511211153990510020012D0000000006543210335432101234567898D211160102779826800000000000660121BA016621BA33LA1000032RUST LANG                             NG56696E9C7ECABA20D072909F260835BCAC4C8BF0A5BA9F2701809F10200FA501A202F8040000000000000000000F0100000000000000000000000000009F3704EA325A3C9F36020D5A950542802490009A032005119C01009F02060000000002005F2A0205665F340102820258009F1A0205669F03060000000000009F3303E0F8C88407A00000037100019F34034203009F3501229F410400000003015510101511344101F3E96B4DCF42481055A07619B5733880AE2C371A4618A7016DCAD9445C00D101";
    let mut iso = Unpacker::new(response.as_bytes());
    let fields = iso
        .unpack(&i8583::nibss::SPEC)
        .unwrap_or_else(|err_msg| panic!("{}", err_msg));

    assert_eq!(fields.len(), 129);

    assert_eq!(fields[0].expect("Field 0"), "0200".as_bytes());
    assert_eq!(
        fields[1].expect("Field 1"),
        "F23C46D129E092000000000000000021".as_bytes()
    );
    assert_eq!(fields[2].expect("Field 2"), "5432101234567898".as_bytes());
    assert_eq!(fields[3].expect("Field 3"), "000000".as_bytes());
    assert_eq!(fields[4].expect("Field 4"), "000000010000".as_bytes());
    assert_eq!(fields[7].expect("Field 7"), "0511171324".as_bytes());

    assert_eq!(fields[11].expect("Field 11"), "171324".as_bytes());
    assert_eq!(fields[12].expect("Field 12"), "171324".as_bytes());
    assert_eq!(fields[13].expect("Field 13"), "0511".as_bytes());
    assert_eq!(fields[14].expect("Field 14"), "2111".as_bytes());
    assert_eq!(fields[18].expect("Field 18"), "5399".as_bytes());
    assert_eq!(fields[22].expect("Field 22"), "051".as_bytes());
    assert_eq!(fields[23].expect("Field 23"), "002".as_bytes());
    assert_eq!(fields[25].expect("Field 25"), "00".as_bytes());
    assert_eq!(fields[26].expect("Field 26"), "12".as_bytes());
    assert_eq!(fields[28].expect("Field 28"), "D00000000".as_bytes());
    assert_eq!(fields[32].expect("Field 32"), "543210".as_bytes());
    assert_eq!(
        fields[35].expect("Field 35"),
        "5432101234567898D2111601027798268".as_bytes()
    );
    assert_eq!(fields[37].expect("Field 37"), "000000000006".as_bytes());
    assert_eq!(fields[40].expect("Field 40"), "601".as_bytes());
    assert_eq!(fields[41].expect("Field 41"), "21BA0166".as_bytes());
    assert_eq!(fields[42].expect("Field 42"), "21BA33LA1000032".as_bytes());
    assert_eq!(
        fields[43].expect("Field 43"),
        "RUST LANG                             NG".as_bytes()
    );
    assert_eq!(fields[49].expect("Field 49"), "566".as_bytes());
    assert_eq!(fields[52].expect("Field 52"), "96E9C7ECABA20D07".as_bytes());
    assert_eq!(fields[55].expect("Field 55"), "9F260835BCAC4C8BF0A5BA9F2701809F10200FA501A202F8040000000000000000000F0100000000000000000000000000009F3704EA325A3C9F36020D5A950542802490009A032005119C01009F02060000000002005F2A0205665F340102820258009F1A0205669F03060000000000009F3303E0F8C88407A00000037100019F34034203009F3501229F410400000003".as_bytes());

    assert_eq!(
        fields[123].expect("Field 123"),
        "510101511344101".as_bytes()
    );
    assert_eq!(
        fields[128].expect("Field 128"),
        "F3E96B4DCF42481055A07619B5733880AE2C371A4618A7016DCAD9445C00D101".as_bytes()
    );
}

#[test]
fn set_pack_and_unpack() -> Result<(), std::borrow::Cow<'static, str>> {
    let spec = &i8583::nibss::SPEC;

    let mut iso_packer = Packer::new(spec);
    set_elements(&mut iso_packer)?;

    let iso = iso_packer.pack(Vec::with_capacity(1024))?;
    unpack_message(&iso, spec);

    Ok(())
}

fn unpack_message(response: &Vec<u8>, spec: &[Option<i8583::data::Spec>; 129]) {
    let mut iso_unpacker = Unpacker::new(response);
    let _fields = match iso_unpacker.unpack(spec) {
        Ok(val) => val,
        Err(msg) => {
            eprintln!("Unpack Error: {}", msg);
            return;
        }
    };
}

fn set_elements(iso_packer: &mut Packer) -> Result<(), String> {
    let pan = String::from("5432101234567898");

    iso_packer.set_field(0, "0200".as_bytes().into())?;
    // Try std::borrow::Cow::Borrowed(pan.as_bytes()) and see what error you get
    iso_packer.set_field(2, pan.into_bytes().into())?;
    iso_packer.set_field(3, "000000".as_bytes().into())?;
    iso_packer.set_field(4, "000000010000".as_bytes().into())?;
    iso_packer.set_field(7, "0511171324".as_bytes().into())?;
    iso_packer.set_field(11, "171324".as_bytes().into())?;
    iso_packer.set_field(12, "171324".as_bytes().into())?;
    iso_packer.set_field(13, "0511".as_bytes().into())?;
    iso_packer.set_field(14, "2111".as_bytes().into())?;
    iso_packer.set_field(18, "5399".as_bytes().into())?;
    iso_packer.set_field(22, "051".as_bytes().into())?;
    iso_packer.set_field(23, "002".as_bytes().into())?;
    iso_packer.set_field(25, "00".as_bytes().into())?;
    iso_packer.set_field(26, "12".as_bytes().into())?;
    iso_packer.set_field(28, "D00000000".as_bytes().into())?;
    iso_packer.set_field(32, "543210".as_bytes().into())?;
    iso_packer.set_field(35, "5432101234567898D2111601027798268".as_bytes().into())?;
    iso_packer.set_field(37, "000000000006".as_bytes().into())?;
    iso_packer.set_field(40, "601".as_bytes().into())?;
    iso_packer.set_field(41, "21BA0166".as_bytes().into())?;
    iso_packer.set_field(42, "21BA33LA1000032".as_bytes().into())?;
    iso_packer.set_field(
        43,
        "RUST LANG                             NG".as_bytes().into(),
    )?;
    iso_packer.set_field(49, "566".as_bytes().into())?;
    iso_packer.set_field(52, "96E9C7ECABA20D07".as_bytes().into())?;
    iso_packer.set_field(55, "9F260835BCAC4C8BF0A5BA9F2701809F10200FA501A202F8040000000000000000000F0100000000000000000000000000009F3704EA325A3C9F36020D5A950542802490009A032005119C01009F02060000000002005F2A0205665F340102820258009F1A0205669F03060000000000009F3303E0F8C88407A00000037100019F34034203009F3501229F410400000003".as_bytes().into())?;
    iso_packer.set_field(123, "015510101511344".as_bytes().into())?;
    iso_packer.set_field(
        128,
        "101F3E96B4DCF42481055A07619B5733880AE2C371A4618A7016DCAD9445C00D"
            .as_bytes()
            .into(),
    )?;

    Ok(())
}
