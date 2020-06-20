//! # Version 1993 Iso Message Specification
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
    }), // Amount, Reconciliation
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
    }), // Conversion Rate, Reconciliation
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
        data_size: data::Size::FIXED(12),
    }), // Date and Time, Local Transaction
    /*013*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Date, Effective
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
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(12),
    }), // Point of Service Data Code
    /*023*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Card Sequence Number
    /*024*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Function Code
    /*025*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Message Reason Code
    /*026*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(4),
    }), // Card Acceptor Business Code
    /*027*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(1),
    }), // Approval Code Length
    /*028*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Date, Reconciliation
    /*029*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Reconciliation Indicator
    /*030*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(24),
    }), // Amounts, Original
    /*031*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(99),
    }), // Acquirer Reference Data
    /*032*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Acquiring Institution ID Code
    /*033*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Forwarding Institution ID Code
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
        data_type: data::Type::Z,
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
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Action code
    /*040*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Service Code
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
        data_size: data::Size::LL(99),
    }), // Card Acceptor Name / Location
    /*044*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(99),
    }), // Additional Response Data
    /*045*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(76),
    }), // Track 1 Data
    /*046*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(204),
    }), // Additional Data - National
    /*047*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Additional Data - Private
    /*048*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Additional Data - Private
    /*049*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(3),
    }), // Currency Code, Transaction
    /*050*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::FIXED(3),
    }), // Currency Code, Reconciliation
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
        data_type: data::Type::BIN,
        data_size: data::Size::LL(48),
    }), // Security Related Control Information
    /*054*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(120),
    }), // Additional Amount
    /*055*/
    Some(data::Spec {
        data_type: data::Type::BIN,
        data_size: data::Size::LLL(255),
    }), // IC Card System Related Data
    /*056*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LLL(35),
    }), // Original Data Elements
    /*057*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Authorization Life Cycle Code
    /*058*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(11),
    }), // Authorizing Agent Institution Id Code
    /*059*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Transport Data
    /*060*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*061*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
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
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(204),
    }), // Amounts, Original Fees
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
    }), // Country Code, Authorizing Agent Institution
    /*071*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(6),
    }), // Message Number
    /*072*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(999),
    }), // Data Record
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
    }), // Authorizations, Number
    /*082*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Inquiries, Reversal Number
    /*083*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Payments, Number
    /*084*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Payments, Reversal Number
    /*085*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Fee Collections, Number
    /*086*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(16),
    }), // Credits, Amount
    /*087*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(16),
    }), // Credits, Reversal Amount
    /*088*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(16),
    }), // Debits, Amount
    /*089*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(16),
    }), // Debits, Reversal Amount
    /*090*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(42),
    }), // Authorizations, Reversal Number
    /*091*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Country Code, Transaction Destination Institution
    /*092*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(3),
    }), // Country Code, Transaction Originator Institution
    /*093*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Transaction Destination Institution Id Code
    /*094*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Transaction Originator Institution Id Code
    /*095*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(99),
    }), // Card Issuer Reference Data
    /*096*/
    Some(data::Spec {
        data_type: data::Type::BIN,
        data_size: data::Size::LLL(999),
    }), // Key Management Data
    /*097*/
    Some(data::Spec {
        data_type: data::Type::XN,
        data_size: data::Size::FIXED(17),
    }), // Amount, Net Reconciliation
    /*098*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::FIXED(25),
    }), // Payee
    /*099*/
    Some(data::Spec {
        data_type: data::Type::AN,
        data_size: data::Size::LL(11),
    }), // Settlement Institution Id Code
    /*100*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::LL(11),
    }), // Receiving Institution Id Code
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
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(16),
    }), // Credits, Chargeback Amount
    /*106*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(16),
    }), // Debits, Chargeback Amount
    /*107*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Credits, Chargeback Number
    /*108*/
    Some(data::Spec {
        data_type: data::Type::NUM,
        data_size: data::Size::FIXED(10),
    }), // Debits, Chargeback Number
    /*109*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(84),
    }), // Credits, Fee Amounts
    /*110*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LL(84),
    }), // Debits, Fee Amounts
    /*111*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*112*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*113*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*114*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
    /*115*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for ISO use
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
    }), // Reserved for National use
    /*121*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
    /*122*/
    Some(data::Spec {
        data_type: data::Type::ANS,
        data_size: data::Size::LLL(999),
    }), // Reserved for National use
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
