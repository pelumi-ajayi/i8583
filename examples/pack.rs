use i8583::nibss;
use i8583::Packer;

fn main() {
    let config = &nibss::SPEC;

    let mut iso_packer = Packer::new(config);

    if let Err(msg) = set_elements(&mut iso_packer) {
        eprintln!("{}", msg);
        return;
    }

    let iso = match iso_packer.pack(Vec::with_capacity(1024)) {
        Ok(iso) => {
            iso
        }
        Err(msg) => {
            eprintln!("{}", msg);
            return;
        }
    };

    println!("{}", std::str::from_utf8(&iso).unwrap());
}

fn set_elements(iso_packer: &mut Packer) -> Result<(), String> {
    let pan = String::from("5432101234567898");

    iso_packer.set_field(0, "0200".as_bytes().into())?;
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
