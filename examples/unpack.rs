use i8583::v1987;
use i8583::Unpacker;

fn main() {
    let response = "08002238000000800000000000070814102414102414102407082020XXXX";
    let mut iso = Unpacker::new(response.as_bytes());
    let fields = iso
        .unpack(&v1987::SPEC)
        .unwrap_or_else(|err_msg| panic!("{}", err_msg));

    for (i, field) in fields.iter().enumerate() {
        if let Some(val) = field {
            println!("[{:03}] {}", i, std::str::from_utf8(val).unwrap());
        }
    }
}
