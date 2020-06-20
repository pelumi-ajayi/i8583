#![feature(test)]
extern crate test;

use test::Bencher;

use i8583::Packer;
use i8583::Unpacker;


#[bench]
fn unpack_benchmark(b: &mut Bencher) {
    let iso = "0800E03E001040C0001C040000000000000015454000000000003000000000001000000010105050202D00000000051234512345678123456789012345009TEST DATA016LONGER TEST DATA044The quick brown fox jumped over the lazy dog301";
    b.iter(|| {
        let mut iso_unpacker = Unpacker::new(iso.as_bytes());
        let _fields = match iso_unpacker.unpack(&i8583::v1987::SPEC) {
            Ok(val) => val,
            Err(msg) => {
                eprintln!("Unpack Error: {}", msg);
                return;
            }
        };
    });
}

#[bench]
fn pack_unpack_benchmark(b: &mut Bencher) -> Result<(), std::borrow::Cow<'static, str>> {
    let spec = &i8583::v1987::SPEC;

    let mut iso_packer = Packer::new(spec);
    set_elements_bench(&mut iso_packer)?;

    b.iter(|| {
        let iso = iso_packer.pack(Vec::with_capacity(1024)).unwrap();
        unpack_message(&iso, spec);
    });

    Ok(())
}

#[bench]
fn set_pack_unpack_benchmark(b: &mut Bencher) {
    let spec = &i8583::v1987::SPEC;

    b.iter(|| {
        let mut iso_packer = Packer::new(spec);
        set_elements_bench(&mut iso_packer).unwrap();
        let iso = iso_packer.pack(Vec::with_capacity(1024)).unwrap();
        unpack_message(&iso, spec);
    });
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

fn set_elements_bench(iso_packer: &mut Packer) -> Result<(), String> {
    iso_packer.set_field(0, "0800".as_bytes().into())?;
    iso_packer.set_field(2, "454000000000003".as_bytes().into())?;
    iso_packer.set_field(3, "000000".as_bytes().into())?;
    iso_packer.set_field(28, "D00000000".as_bytes().into())?;
    iso_packer.set_field(11, "000001".as_bytes().into())?;
    iso_packer.set_field(34, "12345".as_bytes().into())?;
    iso_packer.set_field(41, "12345678".as_bytes().into())?;
    iso_packer.set_field(42, "123456789012345".as_bytes().into())?;
    iso_packer.set_field(12, "000000".as_bytes().into())?;
    iso_packer.set_field(13, "0101".as_bytes().into())?;
    iso_packer.set_field(14, "0505".as_bytes().into())?;
    iso_packer.set_field(15, "0202".as_bytes().into())?;
    iso_packer.set_field(60, "TEST DATA".as_bytes().into())?;
    iso_packer.set_field(61, "LONGER TEST DATA".as_bytes().into())?;
    iso_packer.set_field(
        62,
        "The quick brown fox jumped over the lazy dog"
            .as_bytes()
            .into(),
    )?;
    iso_packer.set_field(70, "301".as_bytes().into())?;

    Ok(())
}
