use std::ops;

// Enums
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Belnapian {
    Neither = 0,
    False = 1,
    True = 2,
    Both = 3,
}

#[derive(Clone, Copy, Debug)]
pub enum TernaryTruth {
    False = 1,
    True = 2,
    Unknown = 0b00110100,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruthValuesSet {
    ____ = 0b00000100, // Empty set, if we reach it, then there is an inconsistency somewhere
    N___ = 0b00001100, // Known
    _F__ = 0b00010100, // Known
    NF__ = 0b00011100,
    __T_ = 0b00100100, // Known
    N_T_ = 0b00101100,
    _FT_ = 0b00110100,
    NFT_ = 0b00111100,
    ___B = 0b01000100, // Known
    N__B = 0b01001100,
    _F_B = 0b01010100,
    NF_B = 0b01011100,
    __TB = 0b01100100,
    N_TB = 0b01101100,
    _FTB = 0b01110100,
    NFTB = 0b01111100,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unknown {
    NF__ = 0b00011100,
    N_T_ = 0b00101100,
    _FT_ = 0b00110100,
    NFT_ = 0b00111100,
    N__B = 0b01001100,
    _F_B = 0b01010100,
    NF_B = 0b01011100,
    __TB = 0b01100100,
    N_TB = 0b01101100,
    _FTB = 0b01110100,
    NFTB = 0b01111100,
}

#[derive(Clone, Copy, Debug)]

pub enum EBelnapian {
    Known(Belnapian),
    Unknown(Unknown),
}

// Belnapian Impls
// -----------------------------------------------------------------------------

impl Belnapian {
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (Belnapian::False, _) => Belnapian::False,
            (_, Belnapian::False) => Belnapian::False,
            (Belnapian::Neither, Belnapian::Both) => Belnapian::False,
            (Belnapian::Both, Belnapian::Neither) => Belnapian::False,
            (Belnapian::Neither, _) => Belnapian::Neither,
            (_, Belnapian::Neither) => Belnapian::Neither,
            (Belnapian::Both, _) => Belnapian::Both,
            (_, Belnapian::Both) => Belnapian::Both,
            (Belnapian::True, Belnapian::True) => Belnapian::True,
        }
    }

    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (Belnapian::True, _) => Belnapian::True,
            (_, Belnapian::True) => Belnapian::True,
            (Belnapian::Both, Belnapian::Neither) => Belnapian::True,
            (Belnapian::Neither, Belnapian::Both) => Belnapian::True,
            (Belnapian::Both, _) => Belnapian::Both,
            (_, Belnapian::Both) => Belnapian::Both,
            (Belnapian::Neither, _) => Belnapian::Neither,
            (_, Belnapian::Neither) => Belnapian::Neither,
            (Belnapian::False, Belnapian::False) => Belnapian::False,
        }
    }

    pub fn superposition(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (Belnapian::Both, _) => Belnapian::Both,
            (_, Belnapian::Both) => Belnapian::Both,
            (Belnapian::True, Belnapian::False) => Belnapian::Both,
            (Belnapian::False, Belnapian::True) => Belnapian::Both,
            (Belnapian::True, _) => Belnapian::True,
            (_, Belnapian::True) => Belnapian::True,
            (Belnapian::False, _) => Belnapian::False,
            (_, Belnapian::False) => Belnapian::False,
            (Belnapian::Neither, Belnapian::Neither) => Belnapian::Neither,
        }
    }

    pub fn annihilation(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (Belnapian::Neither, _) => Belnapian::Neither,
            (_, Belnapian::Neither) => Belnapian::Neither,
            (Belnapian::False, Belnapian::True) => Belnapian::Neither,
            (Belnapian::True, Belnapian::False) => Belnapian::Neither,
            (Belnapian::False, _) => Belnapian::False,
            (_, Belnapian::False) => Belnapian::False,
            (Belnapian::True, _) => Belnapian::True,
            (_, Belnapian::True) => Belnapian::True,
            (Belnapian::Both, Belnapian::Both) => Belnapian::Both,
        }
    }
}

impl ops::Not for Belnapian {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Belnapian::Neither => Belnapian::Neither,
            Belnapian::False => Belnapian::True,
            Belnapian::True => Belnapian::False,
            Belnapian::Both => Belnapian::Both,
        }
    }
}

// TernaryTruth Impls
// -----------------------------------------------------------------------------

impl TernaryTruth {
    pub fn is_unknown(self) -> bool {
        match self {
            TernaryTruth::Unknown => true,
            _ => false,
        }
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (TernaryTruth::False, _) => TernaryTruth::False,
            (_, TernaryTruth::False) => TernaryTruth::False,
            (TernaryTruth::Unknown, _) => TernaryTruth::Unknown,
            (_, TernaryTruth::Unknown) => TernaryTruth::Unknown,
            (TernaryTruth::True, TernaryTruth::True) => TernaryTruth::True,
        }
    }

    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (TernaryTruth::True, _) => TernaryTruth::True,
            (_, TernaryTruth::True) => TernaryTruth::True,
            (TernaryTruth::Unknown, _) => TernaryTruth::Unknown,
            (_, TernaryTruth::Unknown) => TernaryTruth::Unknown,
            (TernaryTruth::False, TernaryTruth::False) => TernaryTruth::False,
        }
    }

    pub fn eq(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (TernaryTruth::False, TernaryTruth::False) => TernaryTruth::True,
            (TernaryTruth::True, TernaryTruth::True) => TernaryTruth::True,
            (TernaryTruth::Unknown, _) => TernaryTruth::Unknown,
            (_, TernaryTruth::Unknown) => TernaryTruth::Unknown,
            _ => TernaryTruth::False,
        }
    }
}

impl ops::Not for TernaryTruth {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            TernaryTruth::False => TernaryTruth::True,
            TernaryTruth::True => TernaryTruth::False,
            TernaryTruth::Unknown => TernaryTruth::Unknown,
        }
    }
}

// Unknown Impls
// -----------------------------------------------------------------------------

impl Unknown {
    pub fn and(self, other: Self) -> EBelnapian {
        match (self, other) {
            (Unknown::NF__, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::_F_B) => EBelnapian::Known(Belnapian::False),
            (Unknown::NF__, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::__TB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::N_T_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::N_T_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N_T_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N_T_, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N_T_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::N_T_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N_T_, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_T_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_T_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_T_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::_FT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::_FT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_FT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::_FT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NFT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NFT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::NFT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NFT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N__B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::N__B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::N__B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::__TB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::_F_B, Unknown::NF__) => EBelnapian::Known(Belnapian::False),
            (Unknown::_F_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::NF_B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::NF_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::__TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::__TB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::__TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::__TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::__TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::__TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::__TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::N_TB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N_TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::N_TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N_TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::_FTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::_FTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_FTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::_FTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NFTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NFTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::NFTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NFTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
        }
    }

    pub fn or(self, other: Self) -> EBelnapian {
        match (self, other) {
            (Unknown::NF__, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::NF__, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NF__, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NF__, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NF__, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF__, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF__, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NF__, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NF__, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF__, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_T_, Unknown::NF__) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::__TB) => EBelnapian::Known(Belnapian::True),
            (Unknown::N_T_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_FT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_FT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_FT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::_FT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_FT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::NFT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NFT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NFT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NFT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N__B, Unknown::NF__) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N__B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::N__B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_F_B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_F_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_F_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_F_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::_F_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_F_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_F_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::NF_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NF_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NF_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NF_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N_T_) => EBelnapian::Known(Belnapian::True),
            (Unknown::__TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::N_TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::N_TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_FTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_FTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_FTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::_FTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::_FTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::NFTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NFTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NFTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::NFTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
        }
    }

    pub fn superposition(self, other: Self) -> EBelnapian {
        match (self, other) {
            (Unknown::NF__, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NF__, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::NF__, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NF__, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF__, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::NF__, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF__, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NF__, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF__, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NF__, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_T_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N_T_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::N_T_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N_T_, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_T_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::N_T_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_T_, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::N_T_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_T_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::N_T_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::_FT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NFT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::NFT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NFT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NFT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NFT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N__B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::N__B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N__B, Unknown::N__B) => EBelnapian::Unknown(Unknown::N__B),
            (Unknown::N__B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::N__B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::N__B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::N__B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::NF__) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_F_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_F_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_F_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::_F_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_F_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_F_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NF_B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NF_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::NF_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NF_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NF_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::N_TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::N_TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::N_TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::N_TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::N_TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::_FTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::_FTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NFTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NFTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NFTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::NFTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
            (Unknown::NFTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
        }
    }

    pub fn annihilation(self, other: Self) -> EBelnapian {
        match (self, other) {
            (Unknown::NF__, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N_T_) => EBelnapian::Known(Belnapian::Neither),
            (Unknown::NF__, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::__TB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF__, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::N_T_, Unknown::NF__) => EBelnapian::Known(Belnapian::Neither),
            (Unknown::N_T_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::__TB) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_T_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_FT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::_FT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_FT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NFT_, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::NFT_, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::N__B) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFT_, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N__B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::N__B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N__B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N__B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N__B, Unknown::N__B) => EBelnapian::Unknown(Unknown::N__B),
            (Unknown::N__B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::N__B, Unknown::__TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N__B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N__B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::_F_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_F_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_F_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_F_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::_F_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
            (Unknown::_F_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::_F_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_F_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NF_B, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::NF_B, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NF_B, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NF_B, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
            (Unknown::NF_B, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NF_B, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::__TB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::__TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::__TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::__TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::__TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
            (Unknown::__TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::__TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::__TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::N_TB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::N_TB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N_TB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::N_TB, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::__TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
            (Unknown::N_TB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::N_TB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::_FTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::_FTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::_FTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::_FTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
            (Unknown::NFTB, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
            (Unknown::NFTB, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFTB, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
            (Unknown::NFTB, Unknown::N__B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::_F_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::__TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NFTB),
            (Unknown::NFTB, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
        }
    }

    pub fn eq(self, other: Self) -> EBelnapian {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (Unknown::NFTB, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::NFT_, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::NF_B, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::N_TB, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FTB, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::NF__, Unknown::__TB) => EBelnapian::Known(Belnapian::False),
            (Unknown::NF__, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::N_T_, Unknown::_F_B) => EBelnapian::Known(Belnapian::False),
            (Unknown::N_T_, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::N__B, Unknown::_FT_) => EBelnapian::Known(Belnapian::False),
            (Unknown::N__B, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_FT_, Unknown::N__B) => EBelnapian::Known(Belnapian::False),
            (Unknown::_FT_, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::_F_B, Unknown::N_T_) => EBelnapian::Known(Belnapian::False),
            (Unknown::_F_B, _) => EBelnapian::Unknown(Unknown::_FT_),
            (Unknown::__TB, Unknown::NF__) => EBelnapian::Known(Belnapian::False),
            (Unknown::__TB, _) => EBelnapian::Unknown(Unknown::_FT_),
        }
    }
}

impl ops::Not for Unknown {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Unknown::NFTB => Unknown::NFTB,
            Unknown::NFT_ => Unknown::NFT_,
            Unknown::NF_B => Unknown::N_TB,
            Unknown::N_TB => Unknown::NF_B,
            Unknown::_FTB => Unknown::_FTB,
            Unknown::NF__ => Unknown::N_T_,
            Unknown::N_T_ => Unknown::NF__,
            Unknown::N__B => Unknown::N__B,
            Unknown::_FT_ => Unknown::_FT_,
            Unknown::_F_B => Unknown::__TB,
            Unknown::__TB => Unknown::_F_B,
        }
    }
}

// EBelnapian Free Functions
// -----------------------------------------------------------------------------

fn and_ebelnapian_unknown(a: Belnapian, b: Unknown) -> EBelnapian {
    match (a, b) {
        (Belnapian::Neither, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::N_T_) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::_F_B) => EBelnapian::Known(Belnapian::False),
        (Belnapian::Neither, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::__TB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::NF__) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::N_T_) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::_FT_) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::NFT_) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::N__B) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::_F_B) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::NF_B) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::__TB) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::N_TB) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::_FTB) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::NFTB) => EBelnapian::Known(Belnapian::False),
        (Belnapian::True, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::True, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
        (Belnapian::True, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
        (Belnapian::True, Unknown::N__B) => EBelnapian::Unknown(Unknown::N__B),
        (Belnapian::True, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::True, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
        (Belnapian::True, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::True, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
        (Belnapian::True, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
        (Belnapian::True, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
        (Belnapian::Both, Unknown::NF__) => EBelnapian::Known(Belnapian::False),
        (Belnapian::Both, Unknown::N_T_) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::NFT_) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::N__B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::NF_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::__TB) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::N_TB) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::NFTB) => EBelnapian::Unknown(Unknown::_F_B),
    }
}

fn or_ebelnapian_unknown(a: Belnapian, b: Unknown) -> EBelnapian {
    match (a, b) {
        (Belnapian::Neither, Unknown::NF__) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::_FT_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::NFT_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::_F_B) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::NF_B) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::__TB) => EBelnapian::Known(Belnapian::True),
        (Belnapian::Neither, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::_FTB) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::NFTB) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::False, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::False, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
        (Belnapian::False, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
        (Belnapian::False, Unknown::N__B) => EBelnapian::Unknown(Unknown::N__B),
        (Belnapian::False, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::False, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
        (Belnapian::False, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::False, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
        (Belnapian::False, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
        (Belnapian::False, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
        (Belnapian::True, Unknown::NF__) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::N_T_) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::_FT_) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::NFT_) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::N__B) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::_F_B) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::NF_B) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::__TB) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::N_TB) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::_FTB) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::NFTB) => EBelnapian::Known(Belnapian::True),
        (Belnapian::Both, Unknown::NF__) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::N_T_) => EBelnapian::Known(Belnapian::True),
        (Belnapian::Both, Unknown::_FT_) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::NFT_) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::N__B) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::_F_B) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::NF_B) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::N_TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::_FTB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::NFTB) => EBelnapian::Unknown(Unknown::__TB),
    }
}

fn superposition_ebelnapian_unknown(a: Belnapian, b: Unknown) -> EBelnapian {
    match (a, b) {
        (Belnapian::Neither, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Neither, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Neither, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
        (Belnapian::Neither, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
        (Belnapian::Neither, Unknown::N__B) => EBelnapian::Unknown(Unknown::N__B),
        (Belnapian::Neither, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Neither, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
        (Belnapian::Neither, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Neither, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
        (Belnapian::Neither, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
        (Belnapian::Neither, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
        (Belnapian::False, Unknown::NF__) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::N_T_) => EBelnapian::Unknown(Unknown::_FT_),
        (Belnapian::False, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
        (Belnapian::False, Unknown::NFT_) => EBelnapian::Unknown(Unknown::_FT_),
        (Belnapian::False, Unknown::N__B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::False, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::False, Unknown::NF_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::False, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::False, Unknown::N_TB) => EBelnapian::Unknown(Unknown::_FTB),
        (Belnapian::False, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
        (Belnapian::False, Unknown::NFTB) => EBelnapian::Unknown(Unknown::_FTB),
        (Belnapian::True, Unknown::NF__) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::N_T_) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::_FT_) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::NFT_) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::N__B) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::True, Unknown::_F_B) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::True, Unknown::NF_B) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::True, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::True, Unknown::N_TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::True, Unknown::_FTB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::True, Unknown::NFTB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::NF__) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::N_T_) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::_FT_) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::NFT_) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::N__B) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::_F_B) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::NF_B) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::__TB) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::N_TB) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::_FTB) => EBelnapian::Known(Belnapian::Both),
        (Belnapian::Both, Unknown::NFTB) => EBelnapian::Known(Belnapian::Both),
    }
}

fn annihilation_ebelnapian_unknown(a: Belnapian, b: Unknown) -> EBelnapian {
    match (a, b) {
        (Belnapian::Neither, Unknown::NF__) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::N_T_) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::_FT_) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::NFT_) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::N__B) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::_F_B) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::NF_B) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::__TB) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::N_TB) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::_FTB) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::Neither, Unknown::NFTB) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::False, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::N_T_) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::False, Unknown::_FT_) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::N__B) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::_F_B) => EBelnapian::Known(Belnapian::False),
        (Belnapian::False, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::__TB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::N_TB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::_FTB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::False, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::True, Unknown::NF__) => EBelnapian::Known(Belnapian::Neither),
        (Belnapian::True, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::_FT_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::NFT_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::N__B) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::_F_B) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::NF_B) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::__TB) => EBelnapian::Known(Belnapian::True),
        (Belnapian::True, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::_FTB) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::True, Unknown::NFTB) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Both, Unknown::NF__) => EBelnapian::Unknown(Unknown::NF__),
        (Belnapian::Both, Unknown::N_T_) => EBelnapian::Unknown(Unknown::N_T_),
        (Belnapian::Both, Unknown::_FT_) => EBelnapian::Unknown(Unknown::_FT_),
        (Belnapian::Both, Unknown::NFT_) => EBelnapian::Unknown(Unknown::NFT_),
        (Belnapian::Both, Unknown::N__B) => EBelnapian::Unknown(Unknown::N__B),
        (Belnapian::Both, Unknown::_F_B) => EBelnapian::Unknown(Unknown::_F_B),
        (Belnapian::Both, Unknown::NF_B) => EBelnapian::Unknown(Unknown::NF_B),
        (Belnapian::Both, Unknown::__TB) => EBelnapian::Unknown(Unknown::__TB),
        (Belnapian::Both, Unknown::N_TB) => EBelnapian::Unknown(Unknown::N_TB),
        (Belnapian::Both, Unknown::_FTB) => EBelnapian::Unknown(Unknown::_FTB),
        (Belnapian::Both, Unknown::NFTB) => EBelnapian::Unknown(Unknown::NFTB),
    }
}

fn eq_ebelnapian_unknown(a: Belnapian, b: Unknown) -> EBelnapian {
    match (a, b) {
        // DO NOT REORDER THESE MATCHES
        (Belnapian::Neither, u) => match u {
            Unknown::_FTB => EBelnapian::Known(Belnapian::False),
            Unknown::_FT_ => EBelnapian::Known(Belnapian::False),
            Unknown::_F_B => EBelnapian::Known(Belnapian::False),
            Unknown::__TB => EBelnapian::Known(Belnapian::False),
            _ => EBelnapian::Unknown(Unknown::_FT_),
        },
        (Belnapian::False, u) => match u {
            Unknown::N_TB => EBelnapian::Known(Belnapian::False),
            Unknown::N_T_ => EBelnapian::Known(Belnapian::False),
            Unknown::N__B => EBelnapian::Known(Belnapian::False),
            Unknown::__TB => EBelnapian::Known(Belnapian::False),
            _ => EBelnapian::Unknown(Unknown::_FT_),
        },
        (Belnapian::True, u) => match u {
            Unknown::NF_B => EBelnapian::Known(Belnapian::False),
            Unknown::NF__ => EBelnapian::Known(Belnapian::False),
            Unknown::N__B => EBelnapian::Known(Belnapian::False),
            Unknown::_F_B => EBelnapian::Known(Belnapian::False),
            _ => EBelnapian::Unknown(Unknown::_FT_),
        },
        (Belnapian::Both, u) => match u {
            Unknown::NFT_ => EBelnapian::Known(Belnapian::False),
            Unknown::NF__ => EBelnapian::Known(Belnapian::False),
            Unknown::N_T_ => EBelnapian::Known(Belnapian::False),
            Unknown::_FT_ => EBelnapian::Known(Belnapian::False),
            _ => EBelnapian::Unknown(Unknown::_FT_),
        },
    }
}

// EBelnapian Impls
// -----------------------------------------------------------------------------

impl EBelnapian {
    pub fn is_unknown(self) -> bool {
        match self {
            EBelnapian::Unknown(_) => true,
            _ => false,
        }
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (EBelnapian::Known(a), EBelnapian::Known(b)) => EBelnapian::Known(a.and(b)),
            (EBelnapian::Unknown(a), EBelnapian::Unknown(b)) => a.and(b),
            (EBelnapian::Known(a), EBelnapian::Unknown(b)) => and_ebelnapian_unknown(a, b),
            (EBelnapian::Unknown(a), EBelnapian::Known(b)) => and_ebelnapian_unknown(b, a),
        }
    }

    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (EBelnapian::Known(a), EBelnapian::Known(b)) => EBelnapian::Known(a.or(b)),
            (EBelnapian::Unknown(a), EBelnapian::Unknown(b)) => a.or(b),
            (EBelnapian::Known(a), EBelnapian::Unknown(b)) => or_ebelnapian_unknown(a, b),
            (EBelnapian::Unknown(a), EBelnapian::Known(b)) => or_ebelnapian_unknown(b, a),
        }
    }

    pub fn superposition(self, other: Self) -> Self {
        match (self, other) {
            (EBelnapian::Known(a), EBelnapian::Known(b)) => EBelnapian::Known(a.superposition(b)),
            (EBelnapian::Unknown(a), EBelnapian::Unknown(b)) => a.superposition(b),
            (EBelnapian::Known(a), EBelnapian::Unknown(b)) => {
                superposition_ebelnapian_unknown(a, b)
            }
            (EBelnapian::Unknown(a), EBelnapian::Known(b)) => {
                superposition_ebelnapian_unknown(b, a)
            }
        }
    }

    pub fn annihilation(self, other: Self) -> Self {
        match (self, other) {
            (EBelnapian::Known(a), EBelnapian::Known(b)) => EBelnapian::Known(a.annihilation(b)),
            (EBelnapian::Unknown(a), EBelnapian::Unknown(b)) => a.annihilation(b),
            (EBelnapian::Known(a), EBelnapian::Unknown(b)) => annihilation_ebelnapian_unknown(a, b),
            (EBelnapian::Unknown(a), EBelnapian::Known(b)) => annihilation_ebelnapian_unknown(b, a),
        }
    }

    pub fn eq(self, other: Self) -> Self {
        match (self, other) {
            (EBelnapian::Known(a), EBelnapian::Known(b)) => EBelnapian::Known((a == b).into()),
            (EBelnapian::Unknown(a), EBelnapian::Unknown(b)) => a.eq(b),
            (EBelnapian::Known(a), EBelnapian::Unknown(b)) => eq_ebelnapian_unknown(a, b),
            (EBelnapian::Unknown(a), EBelnapian::Known(b)) => eq_ebelnapian_unknown(b, a),
        }
    }
}

impl ops::Not for EBelnapian {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            EBelnapian::Known(value) => EBelnapian::Known(!value),
            EBelnapian::Unknown(value) => EBelnapian::Unknown(!value),
        }
    }
}

// Conversions
// -----------------------------------------------------------------------------

impl From<bool> for Belnapian {
    fn from(value: bool) -> Self {
        match value {
            false => Belnapian::False,
            true => Belnapian::True,
        }
    }
}

impl TryFrom<Belnapian> for bool {
    type Error = ();

    fn try_from(value: Belnapian) -> Result<bool, Self::Error> {
        match value {
            Belnapian::False => Ok(false),
            Belnapian::True => Ok(true),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl From<bool> for TernaryTruth {
    fn from(value: bool) -> Self {
        match value {
            false => TernaryTruth::False,
            true => TernaryTruth::True,
        }
    }
}

impl TryFrom<TernaryTruth> for bool {
    type Error = ();

    fn try_from(value: TernaryTruth) -> Result<bool, Self::Error> {
        match value {
            TernaryTruth::False => Ok(false),
            TernaryTruth::True => Ok(true),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl TryFrom<Belnapian> for TernaryTruth {
    type Error = ();

    fn try_from(value: Belnapian) -> Result<TernaryTruth, Self::Error> {
        match value {
            Belnapian::False => Ok(TernaryTruth::False),
            Belnapian::True => Ok(TernaryTruth::True),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl TryFrom<TernaryTruth> for Belnapian {
    type Error = ();

    fn try_from(value: TernaryTruth) -> Result<Belnapian, Self::Error> {
        match value {
            TernaryTruth::False => Ok(Belnapian::False),
            TernaryTruth::True => Ok(Belnapian::True),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl From<bool> for EBelnapian {
    fn from(value: bool) -> Self {
        match value {
            false => EBelnapian::Known(Belnapian::False),
            true => EBelnapian::Known(Belnapian::True),
        }
    }
}

impl TryFrom<EBelnapian> for bool {
    type Error = ();

    fn try_from(value: EBelnapian) -> Result<bool, Self::Error> {
        match value {
            EBelnapian::Known(Belnapian::False) => Ok(false),
            EBelnapian::Known(Belnapian::True) => Ok(true),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl From<TernaryTruth> for EBelnapian {
    fn from(value: TernaryTruth) -> Self {
        match value {
            TernaryTruth::False => EBelnapian::Known(Belnapian::False),
            TernaryTruth::True => EBelnapian::Known(Belnapian::True),
            TernaryTruth::Unknown => EBelnapian::Unknown(Unknown::_FT_),
        }
    }
}

impl From<Belnapian> for EBelnapian {
    fn from(value: Belnapian) -> Self {
        EBelnapian::Known(value)
    }
}

impl TryFrom<EBelnapian> for Belnapian {
    type Error = ();

    fn try_from(value: EBelnapian) -> Result<Belnapian, Self::Error> {
        match value {
            EBelnapian::Known(value) => Ok(value),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl TryFrom<EBelnapian> for TernaryTruth {
    type Error = ();

    fn try_from(value: EBelnapian) -> Result<TernaryTruth, Self::Error> {
        match value {
            EBelnapian::Known(Belnapian::False) => Ok(TernaryTruth::False),
            EBelnapian::Known(Belnapian::True) => Ok(TernaryTruth::True),
            EBelnapian::Unknown(Unknown::_FT_) => Ok(TernaryTruth::Unknown),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl From<Unknown> for EBelnapian {
    fn from(value: Unknown) -> Self {
        EBelnapian::Unknown(value)
    }
}

impl TryFrom<EBelnapian> for Unknown {
    type Error = ();

    fn try_from(value: EBelnapian) -> Result<Unknown, Self::Error> {
        match value {
            EBelnapian::Unknown(value) => Ok(value),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl TryFrom<Unknown> for TernaryTruth {
    type Error = ();

    fn try_from(value: Unknown) -> Result<TernaryTruth, Self::Error> {
        match value {
            Unknown::_FT_ => Ok(TernaryTruth::Unknown),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl TryFrom<TernaryTruth> for Unknown {
    type Error = ();

    fn try_from(value: TernaryTruth) -> Result<Unknown, Self::Error> {
        match value {
            TernaryTruth::Unknown => Ok(Unknown::_FT_),
            _ => Err(()), // TODO: Improve?
        }
    }
}

impl From<Unknown> for TruthValuesSet {
    fn from(value: Unknown) -> Self {
        match value {
            Unknown::NF__ => TruthValuesSet::NF__,
            Unknown::N_T_ => TruthValuesSet::N_T_,
            Unknown::_FT_ => TruthValuesSet::_FT_,
            Unknown::NFT_ => TruthValuesSet::NFT_,
            Unknown::N__B => TruthValuesSet::N__B,
            Unknown::_F_B => TruthValuesSet::_F_B,
            Unknown::NF_B => TruthValuesSet::NF_B,
            Unknown::__TB => TruthValuesSet::__TB,
            Unknown::N_TB => TruthValuesSet::N_TB,
            Unknown::_FTB => TruthValuesSet::_FTB,
            Unknown::NFTB => TruthValuesSet::NFTB,
        }
    }
}

impl TryFrom<TruthValuesSet> for Unknown {
    type Error = ();

    fn try_from(value: TruthValuesSet) -> Result<Unknown, Self::Error> {
        match value {
            TruthValuesSet::NF__ => Ok(Unknown::NF__),
            TruthValuesSet::N_T_ => Ok(Unknown::N_T_),
            TruthValuesSet::_FT_ => Ok(Unknown::_FT_),
            TruthValuesSet::NFT_ => Ok(Unknown::NFT_),
            TruthValuesSet::N__B => Ok(Unknown::N__B),
            TruthValuesSet::_F_B => Ok(Unknown::_F_B),
            TruthValuesSet::NF_B => Ok(Unknown::NF_B),
            TruthValuesSet::__TB => Ok(Unknown::__TB),
            TruthValuesSet::N_TB => Ok(Unknown::N_TB),
            TruthValuesSet::_FTB => Ok(Unknown::_FTB),
            TruthValuesSet::NFTB => Ok(Unknown::NFTB),
            _ => Err(()), // TODO: Improve?
        }
    }
}

// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod belnapian_tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(
            Belnapian::Neither,
            Belnapian::Neither.and(Belnapian::Neither)
        );
        assert_eq!(Belnapian::False, Belnapian::Neither.and(Belnapian::False));
        assert_eq!(Belnapian::Neither, Belnapian::Neither.and(Belnapian::True));
        assert_eq!(Belnapian::False, Belnapian::Neither.and(Belnapian::Both));

        assert_eq!(Belnapian::False, Belnapian::False.and(Belnapian::Neither));
        assert_eq!(Belnapian::False, Belnapian::False.and(Belnapian::False));
        assert_eq!(Belnapian::False, Belnapian::False.and(Belnapian::True));
        assert_eq!(Belnapian::False, Belnapian::False.and(Belnapian::Both));

        assert_eq!(Belnapian::Neither, Belnapian::True.and(Belnapian::Neither));
        assert_eq!(Belnapian::False, Belnapian::True.and(Belnapian::False));
        assert_eq!(Belnapian::True, Belnapian::True.and(Belnapian::True));
        assert_eq!(Belnapian::Both, Belnapian::True.and(Belnapian::Both));

        assert_eq!(Belnapian::False, Belnapian::Both.and(Belnapian::Neither));
        assert_eq!(Belnapian::False, Belnapian::Both.and(Belnapian::False));
        assert_eq!(Belnapian::Both, Belnapian::Both.and(Belnapian::True));
        assert_eq!(Belnapian::Both, Belnapian::Both.and(Belnapian::Both));
    }

    #[test]
    fn test_or() {
        assert_eq!(
            Belnapian::Neither,
            Belnapian::Neither.or(Belnapian::Neither)
        );
        assert_eq!(Belnapian::Neither, Belnapian::Neither.or(Belnapian::False));
        assert_eq!(Belnapian::True, Belnapian::Neither.or(Belnapian::True));
        assert_eq!(Belnapian::True, Belnapian::Neither.or(Belnapian::Both));

        assert_eq!(Belnapian::Neither, Belnapian::False.or(Belnapian::Neither));
        assert_eq!(Belnapian::False, Belnapian::False.or(Belnapian::False));
        assert_eq!(Belnapian::True, Belnapian::False.or(Belnapian::True));
        assert_eq!(Belnapian::Both, Belnapian::False.or(Belnapian::Both));

        assert_eq!(Belnapian::True, Belnapian::True.or(Belnapian::Neither));
        assert_eq!(Belnapian::True, Belnapian::True.or(Belnapian::False));
        assert_eq!(Belnapian::True, Belnapian::True.or(Belnapian::True));
        assert_eq!(Belnapian::True, Belnapian::True.or(Belnapian::Both));

        assert_eq!(Belnapian::True, Belnapian::Both.or(Belnapian::Neither));
        assert_eq!(Belnapian::Both, Belnapian::Both.or(Belnapian::False));
        assert_eq!(Belnapian::True, Belnapian::Both.or(Belnapian::True));
        assert_eq!(Belnapian::Both, Belnapian::Both.or(Belnapian::Both));
    }

    #[test]
    fn test_not() {
        assert_eq!(Belnapian::Neither, !Belnapian::Neither);
        assert_eq!(Belnapian::True, !Belnapian::False);
        assert_eq!(Belnapian::False, !Belnapian::True);
        assert_eq!(Belnapian::Both, !Belnapian::Both);
    }

    #[test]
    fn test_superposition() {
        assert_eq!(
            Belnapian::Neither,
            Belnapian::Neither.superposition(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::False,
            Belnapian::Neither.superposition(Belnapian::False)
        );
        assert_eq!(
            Belnapian::True,
            Belnapian::Neither.superposition(Belnapian::True)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::Neither.superposition(Belnapian::Both)
        );

        assert_eq!(
            Belnapian::False,
            Belnapian::False.superposition(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::False,
            Belnapian::False.superposition(Belnapian::False)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::False.superposition(Belnapian::True)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::False.superposition(Belnapian::Both)
        );

        assert_eq!(
            Belnapian::True,
            Belnapian::True.superposition(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::True.superposition(Belnapian::False)
        );
        assert_eq!(
            Belnapian::True,
            Belnapian::True.superposition(Belnapian::True)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::True.superposition(Belnapian::Both)
        );

        assert_eq!(
            Belnapian::Both,
            Belnapian::Both.superposition(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::Both.superposition(Belnapian::False)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::Both.superposition(Belnapian::True)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::Both.superposition(Belnapian::Both)
        );
    }

    #[test]
    fn test_annihilation() {
        assert_eq!(
            Belnapian::Neither,
            Belnapian::Neither.annihilation(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::Neither,
            Belnapian::Neither.annihilation(Belnapian::False)
        );
        assert_eq!(
            Belnapian::Neither,
            Belnapian::Neither.annihilation(Belnapian::True)
        );
        assert_eq!(
            Belnapian::Neither,
            Belnapian::Neither.annihilation(Belnapian::Both)
        );

        assert_eq!(
            Belnapian::Neither,
            Belnapian::False.annihilation(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::False,
            Belnapian::False.annihilation(Belnapian::False)
        );
        assert_eq!(
            Belnapian::Neither,
            Belnapian::False.annihilation(Belnapian::True)
        );
        assert_eq!(
            Belnapian::False,
            Belnapian::False.annihilation(Belnapian::Both)
        );

        assert_eq!(
            Belnapian::Neither,
            Belnapian::True.annihilation(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::Neither,
            Belnapian::True.annihilation(Belnapian::False)
        );
        assert_eq!(
            Belnapian::True,
            Belnapian::True.annihilation(Belnapian::True)
        );
        assert_eq!(
            Belnapian::True,
            Belnapian::True.annihilation(Belnapian::Both)
        );

        assert_eq!(
            Belnapian::Neither,
            Belnapian::Both.annihilation(Belnapian::Neither)
        );
        assert_eq!(
            Belnapian::False,
            Belnapian::Both.annihilation(Belnapian::False)
        );
        assert_eq!(
            Belnapian::True,
            Belnapian::Both.annihilation(Belnapian::True)
        );
        assert_eq!(
            Belnapian::Both,
            Belnapian::Both.annihilation(Belnapian::Both)
        );
    }

    #[test]
    fn test_bool_conversions() {
        assert_eq!(Belnapian::False, Belnapian::from(false));
        assert_eq!(Belnapian::True, Belnapian::from(true));

        assert_eq!(Belnapian::False, false.into());
        assert_eq!(Belnapian::True, true.into());

        assert_eq!(Ok(false), Belnapian::False.try_into());
        assert_eq!(Ok(true), Belnapian::True.try_into());
    }
}

#[cfg(test)]
mod ternary_truth_tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(Ok(false), TernaryTruth::False.and(TernaryTruth::False).try_into());
        assert_eq!(Ok(false), TernaryTruth::False.and(TernaryTruth::True).try_into());
        assert_eq!(Ok(false), TernaryTruth::False.and(TernaryTruth::Unknown).try_into());

        assert_eq!(Ok(false), TernaryTruth::True.and(TernaryTruth::False).try_into());
        assert_eq!(Ok(true), TernaryTruth::True.and(TernaryTruth::True).try_into());
        assert!(TernaryTruth::True.and(TernaryTruth::Unknown).is_unknown());

        assert_eq!(Ok(false), TernaryTruth::Unknown.and(TernaryTruth::False).try_into());
        assert!(TernaryTruth::Unknown.and(TernaryTruth::True).is_unknown());
        assert!(TernaryTruth::Unknown.and(TernaryTruth::Unknown).is_unknown());
    }

    #[test]
    fn test_or() {
        assert_eq!(Ok(false), TernaryTruth::False.or(TernaryTruth::False).try_into());
        assert_eq!(Ok(true), TernaryTruth::False.or(TernaryTruth::True).try_into());
        assert!(TernaryTruth::False.or(TernaryTruth::Unknown).is_unknown());

        assert_eq!(Ok(true), TernaryTruth::True.or(TernaryTruth::False).try_into());
        assert_eq!(Ok(true), TernaryTruth::True.or(TernaryTruth::True).try_into());
        assert_eq!(Ok(true), TernaryTruth::True.or(TernaryTruth::Unknown).try_into());

        assert!(TernaryTruth::Unknown.or(TernaryTruth::False).is_unknown());
        assert_eq!(Ok(true), TernaryTruth::Unknown.or(TernaryTruth::True).try_into());
        assert!(TernaryTruth::Unknown.or(TernaryTruth::Unknown).is_unknown());
    }

    #[test]
    fn test_eq() {
        assert_eq!(Ok(true), TernaryTruth::False.eq(TernaryTruth::False).try_into());
        assert_eq!(Ok(false), TernaryTruth::False.eq(TernaryTruth::True).try_into());
        assert!(TernaryTruth::False.eq(TernaryTruth::Unknown).is_unknown());

        assert_eq!(Ok(false), TernaryTruth::True.eq(TernaryTruth::False).try_into());
        assert_eq!(Ok(true), TernaryTruth::True.eq(TernaryTruth::True).try_into());
        assert!(TernaryTruth::True.eq(TernaryTruth::Unknown).is_unknown());

        assert!(TernaryTruth::Unknown.eq(TernaryTruth::False).is_unknown());
        assert!(TernaryTruth::Unknown.eq(TernaryTruth::True).is_unknown());
        assert!(TernaryTruth::Unknown.eq(TernaryTruth::Unknown).is_unknown());
    }
}

#[cfg(test)]
mod truth_values_set_test {
    // use super::*;
}

#[cfg(test)]
mod unknown_test {
    // use super::*;
}

#[cfg(test)]
mod ebelnapian_tests {
    // use super::*;
}
