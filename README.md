# i8583
Types, associated functions and specifications for packing and unpacking ISO8583 financial transaction messages.

For most practical cases, IS08583 messages are created to be sent over the wire as an application level protocol on top of TCP, for this reason all interfaces are in bytes ```[u8]``` for easy use with sockets.

# Dependencies
None

# Examples
There are two examples included, ```pack``` and ```unpack``` , run the commands below in the crate root directory to execute.
```
$ cargo run --example pack
```
```
$ cargo run --example unpack
```

# Usage
Unpack ISO8583 Message.
```
use i8583::v1987;
use i8583::Unpacker;

fn main() {
    let response = "08002238000000800000000000070814102414102414102407082020XXXX";
    let mut iso = Unpacker::new(response.as_bytes());
    let fields = iso.unpack(&v1987::SPEC).unwrap();

    for (i, field) in fields.iter().enumerate() {
        if let Some(val) = field {
            println!("[{:03}] {}", i, std::str::from_utf8(val).unwrap());
        }
    }
}

```
Pack ISO8583 Message.
```
use i8583::v1987;
use i8583::Packer;

fn main() {

    let mut iso_packer = Packer::new(&v1987::SPEC);

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
    let pan = String::from("454000000000003");

    iso_packer.set_field(0, "0800".as_bytes().into())?;
    iso_packer.set_field(2, pan.into_bytes().into())?;
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
```

# Benchmarks
Use below command to run benchmarks.

```
$ rustup run nightly cargo bench
```

Output on macOS Catalina, 2 GHz Dual-Core Intel Core i5, 8 GB 1867 MHz LPDDR3.
```
test pack_unpack_benchmark     ... bench:       1,129 ns/iter (+/- 155)
test set_pack_unpack_benchmark ... bench:       3,037 ns/iter (+/- 245)
test unpack_benchmark          ... bench:         285 ns/iter (+/- 30)
```
