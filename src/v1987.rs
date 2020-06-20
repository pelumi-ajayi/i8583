//! # Version 1987 Iso Message Specification
//!

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
    }), // Bitmap
    /*002*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(19),
    }), // Primary Account Number
    /*003*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Processing Code
    /*004*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // Amount, Transaction
    /*005*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // Amount, Settlement
    /*006*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // Amount, Cardholder Billing
    /*007*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Date and Time, Transmission
    /*008*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(8),
    }), // Amount, Cardholder Billing Fee
    /*009*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(8),
    }), // Conversion Rate, Settlement
    /*010*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(8),
    }), // Conversion Rate, Cardholder Billing
    /*011*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Systems Trace Audit Number
    /*012*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Time, Local Transaction
    /*013*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, Local Transaction
    /*014*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, Expiration
    /*015*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, Settlement
    /*016*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, Conversion
    /*017*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, Capture
    /*018*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Merchant Type
    /*019*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Country Code, Acquiring Institution
    /*020*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Country Code, Primary Account Number
    /*021*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Country Code, Forwarding Institution
    /*022*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Point of Service Entry Mode
    /*023*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Application PAN number
    /*024*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Network International Identifier
    /*025*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(2),
    }), // Point of Service Condition Code
    /*026*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(2),
    }), // Point of Service PIN Capture Code
    /*027*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(1),
    }), // Authorization Identification Response Length
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
    }), // Acquiring Institution. ID Code
    /*033*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Forwarding Institution. ID Code
    /*034*/
    Some(data::Spec {
        data_type: data::Type::NS,
        data_size: data::Size::LL(28),
    }), // Primary Account Number, Extended
    /*035*/
    Some(data::Spec {
        data_type: data::Type::Z,
        data_size: data::Size::LL(37),
    }), // Track 2 Data
    /*036*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::LLL(104),
    }), // Track 3 Data
    /*037*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(12),
    }), // Retrieval Reference Number
    /*038*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Approval Code
    /*039*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(2),
    }), // Response code
    /*040*/
    Some(data::Spec {
        data_type: data::Type::ANS,
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
    }), // Card Acceptor ID Code
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
    }), // Track 1 Data
    /*046*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Additional Data - ISO
    /*047*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Additional Data - National
    /*048*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Additional Data - Private
    /*049*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(3),
    }), // Currency Code, Transaction
    /*050*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(3),
    }), // Currency Code, Settlement
    /*051*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(3),
    }), // Currency Code, Cardholder Billing
    /*052*/
    Some(data::Spec {
        data_type: data::Type::BIN,
        data_size: data::Size::FIXED(8),
    }), // PIN Data
    /*053*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(16),
    }), // Security Related Control Information
    /*054*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(120),
    }), // Additional Amount
    /*055*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*056*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*057*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*058*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*059*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*060*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*061*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*062*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*063*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLLL(9999),
    }), // Reserved for Private use
    /*064*/
    Some(data::Spec {
        data_type: data::Type::BIN,
        data_size: data::Size::FIXED(8),
    }), // Message Authentication Code
    /*065*/
    Some(data::Spec {
        data_type: data::Type::BIN,
        data_size: data::Size::FIXED(8),
    }), // Reserved for ISO use
    /*066*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(1),
    }), // Code, Settlement
    /*067*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(2),
    }), // Extended Payment Code
    /*068*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Country Code, Receiving Institution
    /*069*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Country Code, Settlement Institution
    /*070*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Network Management Information Code
    /*071*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Message Number
    /*072*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Message Number Last
    /*073*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Date, Action
    /*074*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Credits, Number
    /*075*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Credits, Reversal Number
    /*076*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Debits, Number
    /*077*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Debits, Reversal Number
    /*078*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Transfer, Number
    /*079*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Transfer, Reversal Number
    /*080*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Inquiries, Number
    /*081*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Authorization, Number
    /*082*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // Credits, Processing Fee
    /*083*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // Credits, Transaction Fee
    /*084*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // Debits, Processing Fee
    /*085*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(12),
    }), // Debits, Transaction Fee
    /*086*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(15),
    }), // Credits, Amount
    /*087*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(15),
    }), // Credits, Reversal Amount
    /*088*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(15),
    }), // Debits, Amount
    /*089*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(15),
    }), // Debits, Reversal Amount
    /*090*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(42),
    }), // Original Data Element
    /*091*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(1),
    }), // File Update Code
    /*092*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(2),
    }), // File Security Code
    /*093*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(5),
    }), // Response Indicator
    /*094*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(42),
    }), // Service Indicator
    /*095*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(42),
    }), // Replacement amount
    /*096*/
    Some(data::Spec {
        data_type: data::Type::BIN,
        data_size: data::Size::FIXED(8),
    }), // Message Security Code
    /*097*/
    Some(data::Spec {
        data_type: data::Type::XN,
        data_size: data::Size::FIXED(17),
    }), // Amount, Net Settlement
    /*098*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(25),
    }), // Payee
    /*099*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Settlement Institution ID Code
    /*100*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Receiving Institution ID Code
    /*101*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(17),
    }), // File Name
    /*102*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(28),
    }), // Account ID 1
    /*103*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(28),
    }), // Account ID 2
    /*104*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(100),
    }), // Transaction Description
    /*105*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*106*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*107*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*108*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*109*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*110*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*111*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*112*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*113*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*114*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*115*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*116*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*117*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*118*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*119*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*120*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*121*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*122*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*123*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*124*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*125*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*126*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*127*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for Private use
    /*128*/
    Some(data::Spec {
        data_type: data::Type::BIN,
        data_size: data::Size::FIXED(8),
    }), // Message Authentication Code
];
