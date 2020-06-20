//! # NIBSS Iso Message Specification
//!
//! Nigeria Interbank Settlement System iso message specification


use super::data;

#[allow(dead_code)]
pub const SPEC: [Option<data::Spec>; 129] = [
    /*000*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // MTI
    /*001*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::BMP,
    }), // bitmap
    /*002*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(19),
    }), // pan
    /*003*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(6),
    }), // ps
    /*004*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // amount
    /*005*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // amount,settlement
    /*006*/ None,
    /*007*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // transmission date and time
    /*008*/ None,
    /*009*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(8),
    }), // Conversion rate
    /*010*/ None,
    /*011*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // stan
    /*012*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Time, local
    /*013*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, local
    /*014*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // expiration
    /*015*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, settlement
    /*016*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, conversion
    /*017*/ None,
    /*018*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Merchant type
    /*019*/ None,
    /*020*/ None,
    /*021*/ None,
    /*022*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // POS Entry Mode
    /*023*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Card sequence number
    /*024*/ None,
    /*025*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(2),
    }), // POS Condition Code
    /*026*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(2),
    }), // POS PIN Capture Code
    /*027*/ None,
    /*028*/
    Some(data::Spec {
        data_type: data::Type::XN,
        data_size: data::Size::FIXED(9),
    }), // Amount, Transaction Fee
    /*029*/
    Some(data::Spec {
        data_type: data::Type::XN,
        data_size: data::Size::FIXED(9),
    }), // Amount, Settlement Fee
    /*030*/
    Some(data::Spec {
        data_type: data::Type::XN,
        data_size: data::Size::FIXED(9),
    }), // Amount, Transaction Processing Fee
    /*031*/
    Some(data::Spec {
        data_type: data::Type::XN,
        data_size: data::Size::FIXED(9),
    }), // Amount, Settlement Processing Fee
    /*032*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::LL(11),
    }), // Acquiring Inst. ID Code
    /*033*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Forwarding Inst. ID Code
    /*034*/ None,
    /*035*/
    Some(data::Spec {
        data_type: data::Type::Z,
        data_size: data::Size::LL(37),
    }), // Track 2
    /*036*/ None,
    /*037*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(12),
    }), // RRN
    /*038*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Auth code
    /*039*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(2),
    }), // Response code
    /*040*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Service Restriction Code
    /*041*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(8),
    }), // Card Acceptor Terminal ID
    /*042*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(15),
    }), // Card Acceptor ID
    /*043*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(40),
    }), // Card Acceptor Name / Location
    /*044*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(25),
    }), // Additional Response Data
    /*045*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(76),
    }), // Track 1
    /*046*/ None,
    /*047*/ None,
    /*048*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Additional Data
    /*049*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Currency Code, Transaction
    /*050*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Currency Code, Settlement
    /*051*/ None,
    /*052*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(16),
    }), // PIN Data
    /*053*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(96),
    }),
    /*054*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::LLL(120),
    }), // Additional Amount
    /*055*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(510),
    }), // ICC System Related Data
    /*056*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LLL(4),
    }), // Message Reason Code
    /*057*/ None,
    /*058*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LLL(11),
    }), // Authorizing Agent Code
    /*059*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(255),
    }), // Echo Data
    /*060*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Payment Information
    /*061*/ None,
    /*062*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Private Field 1
    /*063*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLLL(9999),
    }), // Private Field 2
    /*064*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(64),
    }), // Private Field 1,
    /*065*/ None,
    /*066*/ None,
    /*067*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(2),
    }), // Extended Payment Code
    /*068*/ None,
    /*069*/ None,
    /*070*/ None,
    /*071*/ None,
    /*072*/ None,
    /*073*/ None,
    /*074*/ None,
    /*075*/ None,
    /*076*/ None,
    /*077*/ None,
    /*078*/ None,
    /*079*/ None,
    /*080*/ None,
    /*081*/ None,
    /*082*/ None,
    /*083*/ None,
    /*084*/ None,
    /*085*/ None,
    /*086*/ None,
    /*087*/ None,
    /*088*/ None,
    /*089*/ None,
    /*090*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(42),
    }), // Original Data Element
    /*091*/ None,
    /*092*/ None,
    /*093*/ None,
    /*094*/ None,
    /*095*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(42),
    }), // Replacement amount
    /*096*/ None,
    /*097*/ None,
    /*098*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(25),
    }), // Payee
    /*099*/ None,
    /*100*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }),
    /*101*/ None,
    /*102*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(28),
    }),
    /*103*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(28),
    }),
    /*104*/ None,
    /*105*/ None,
    /*106*/ None,
    /*107*/ None,
    /*108*/ None,
    /*109*/ None,
    /*110*/ None,
    /*111*/ None,
    /*112*/ None,
    /*113*/ None,
    /*114*/ None,
    /*115*/ None,
    /*116*/ None,
    /*117*/ None,
    /*118*/ None,
    /*119*/ None,
    /*120*/ None,
    /*121*/ None,
    /*122*/ None,
    /*123*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::LLL(15),
    }),
    /*124*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLLL(9999),
    }),
    /*125*/ None,
    /*126*/ None,
    /*127*/ None,
    /*128*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(64),
    }),
];
