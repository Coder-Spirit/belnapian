//! # Belnapian
//! Belnapian is a library that provides basic types and operations for
//! [multiple-valued logics](https://en.wikipedia.org/wiki/Many-valued_logic).
//!
//! ## 3-Valued Logic
//!
//! For [3-valued logics](https://en.wikipedia.org/wiki/Three-valued_logic), we
//! provide the `TernaryTruth` type:
//! ```ignore
//! pub enum TernaryTruth {
//!     False,
//!     True,
//!     Unknown,
//! }
//! ```
//!
//! Belnapian also provides the basic operations `and`, `or`, `not` and `xor`,
//! but it does not provide "implication" connectives, as they differ between
//! different 3-valued logic systems such as Kleene logic, RM3 logic, or
//! [Łukasiewicz logic](https://en.wikipedia.org/wiki/%C5%81ukasiewicz_logic).
//!
//! My personal recommendation on using these "truth values" is to treat them not as
//! the actual truth values attributed to a proposition, but to treat them as our
//! subjective knowledge of them (hence the term "Unknown", which implies the need
//! of a sentient being not-knowing something).
//!
//! ## 4-Valued Belnap Logic
//!
//! Belnapian, unsurprisingly, also provides basic support for
//! [Belnap's 4-valued logic](https://en.wikipedia.org/wiki/Four-valued_logic#Belnap).
//!
//! As in the case of support for 3-valued logics, this library only provides basic
//! building blocks, and not any kind of fully fledged inference system.
//!
//! ```ignore
//! pub enum Belnapian {
//!     // The `Neither` truth value is useful to identify propositions to
//!     // which we cannot assign any classical truth value. This often
//!     // happens when the proposition is not well-formed or when it is
//!     // self-contradictory.
//!     Neither,
//!
//!     False,
//!     True,
//!
//!     // We can understand `Both` as a superposition of `True` and
//!     // `False`. A natural case where it makes sense to assign this
//!     // truth value is when we have a proposition that, given our
//!     // current set of axioms, could be either `True` or `False`
//!     // (remember Gödel's incompleteness theorems).
//!     //
//!     // In other words, in case that a proposition (or its negation) is
//!     // independent of our axioms and could be added as a new axiom
//!     // without causing any inconsistency, then we can assign the
//!     // `Both` truth value to it.
//!     Both,
//! }
//! ```
//!
//! In constrast to the case of the 3-valued logics with an `Unknown` value, the
//! `Both` and `Neither` truth values aren't necessarily tied to our subjective
//! knowledge of the truth value for a given proposition.
//!
//! Assuming that we operate with a well-known set of axioms, we could use them to
//! talk about the "real" underlying truth value for a given proposition.
//!
//! ## 15-Valued Extended Belnap Logic (with Unknown values)
//!
//! The most important feature of the Belnapian library is its support for a
//! 15-valued logic combining Belnap's 4-valued logic with subjective unknown
//! values.
//!
//! ```ignore
//! pub enum EBelnapian {
//!     Known(Belnapian),
//!     Unknown(Unknown),
//! }
//!
//! // The enum variants' names are ugly, but once we know what they
//! // represent, it becomes much easier to use & understand them.
//! pub enum Unknown {
//!     NF__, // Could be Neither or False
//!     N_T_,
//!     _FT_, // Could be False or True
//!     NFT_,
//!     N__B,
//!     _F_B,
//!     NF_B,
//!     __TB, // Could be True or Both
//!     N_TB,
//!     _FTB,
//!     NFTB,
//! }
//! ```
//!
//! Once we have more than 2 "objective" truth values, our unknowns can represent
//! more than one set of possible values (in 3-valued logic, the `Unknown` value
//! represents the set `{False, True}`).
//!
//! Our "unknown values" represent the sets present in the power set of
//! `{Neither, False, True, Both}`, except for the null set `ø` and the singletons
//! `{Neither}`, `{False}`, `{True}`, and `{Both}` (that is, `2⁴-5 = 16-5 = 11`
//! values).
//!
//! The amazing aspect of these "unknown values" is that we can still apply classic
//! logic operations to them and obtain useful results. This library relies on
//! pre-computed tables to save you a ton of time when dealing with uncertainty in
//! logic calculations.

use std::ops;

// Enums
// -----------------------------------------------------------------------------

/// See [Wikipedia :: Four-valued Logic :: Belnap](https://en.wikipedia.org/wiki/Four-valued_logic#Belnap)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Belnapian {
    /// The `Neither` truth value is useful to identify propositions to which we
    /// cannot assign any classical truth value. This often happens when the
    /// proposition is not well-formed or when it is self-contradictory.
    Neither,
    False,
    True,

    /// We can understand `Both` as a superposition of `True` and `False`. A natural
    /// case where it makes sense to assign this truth value is when we have a
    /// proposition that, given our current set of axioms, could be either `True` or
    /// `False` (remember
    /// [Gödel's incompleteness theorems](https://en.wikipedia.org/wiki/G%C3%B6del%27s_incompleteness_theorems)).
    ///
    /// In other words, in case that a proposition (or its negation) is independent
    /// of our axioms and could be added as a new axiom without causing any
    /// inconsistency, then we can assign the `Both` truth value to it.
    Both,
}

#[derive(Clone, Copy, Debug)]
pub enum TernaryTruth {
    False,
    True,
    Unknown,
}

/// The [`TruthValuesPowerSet`] enum represents power sets of the set of 4 truth
/// values in Belnap's 4-valued logic. It's a superset of the [`Unknown`] enum.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruthValuesPowerSet {
    ____, // Empty set, if we reach it, then there is an inconsistency somewhere
    N___, // Known
    _F__, // Known
    NF__,
    __T_, // Known
    N_T_,
    _FT_,
    NFT_,
    ___B, // Known
    N__B,
    _F_B,
    NF_B,
    __TB,
    N_TB,
    _FTB,
    NFTB,
}

/// The [`Unknown`] enum represents the 11 possible ways in which we can have
/// ignorance about the truth value of a proposition. It's a subset of the
/// [`TruthValuesPowerSet`] enum.
///
/// From the set of 4 truth values, we can compute its power set, which has 16
/// elements, and then remove the empty set and every set with only one element,
/// leaving us with 11.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unknown {
    NF__,
    N_T_,
    _FT_,
    NFT_,
    N__B,
    _F_B,
    NF_B,
    __TB,
    N_TB,
    _FTB,
    NFTB,
}

/// The [`EBelnapian`] enum represents a "union" of the [`Belnapian`] and
/// [`Unknown`] enums.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum EBelnapian {
    Known(Belnapian),
    Unknown(Unknown),
}

// Traits
// -----------------------------------------------------------------------------

pub trait TruthValue: Copy {}

pub trait AndOp: TruthValue {
    fn and(self, other: Self) -> Self;
}

pub trait OrOp: TruthValue {
    fn or(self, other: Self) -> Self;
}

pub trait XorOp: TruthValue {
    fn xor(self, other: Self) -> Self;
}

pub trait LogicOperand: ops::Not + AndOp + OrOp {}

pub trait TruthValuesSet: Copy {
    fn could_be_neither(self) -> bool;
    fn could_be_false(self) -> bool;
    fn could_be_true(self) -> bool;
    fn could_be_both(self) -> bool;
    fn is_unknown(self) -> bool;
    fn is_empty(self) -> bool;
}

// Belnapian Impls
// -----------------------------------------------------------------------------

impl TruthValue for Belnapian {}

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

    /// The XOR operation cannot be generalized to Belnap's 4-valued logic
    /// without introducing new assumptions, because the propositions
    /// - `(A OR B) AND ¬(A AND B)`
    /// - `(a AND ¬b) OR (¬a AND b)`
    ///
    /// which are equivalent in classical logic, have different truth tables in
    /// Belnap's 4-valued logic.
    ///
    /// For this implementation, we chose the proposition
    /// - `(a AND ¬b) OR (¬a AND b)`
    ///
    /// as it's closer to the natural language interpretation of XOR.
    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            (Belnapian::Neither, Belnapian::Both) => Belnapian::False,
            (Belnapian::Both, Belnapian::Neither) => Belnapian::False,
            (Belnapian::Neither, _) => Belnapian::Neither,
            (_, Belnapian::Neither) => Belnapian::Neither,
            (Belnapian::Both, _) => Belnapian::Both,
            (_, Belnapian::Both) => Belnapian::Both,
            (a, b) if a == b => Belnapian::False,
            _ => Belnapian::True,
        }
    }

    pub fn not(self) -> Self {
        match self {
            Belnapian::Neither => Belnapian::Neither,
            Belnapian::False => Belnapian::True,
            Belnapian::True => Belnapian::False,
            Belnapian::Both => Belnapian::Both,
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

impl AndOp for Belnapian {
    fn and(self, other: Self) -> Self {
        self.and(other)
    }
}

impl OrOp for Belnapian {
    fn or(self, other: Self) -> Self {
        self.or(other)
    }
}

impl XorOp for Belnapian {
    /// The XOR operation cannot be generalized to Belnap's 4-valued logic
    /// without introducing new assumptions, because the propositions
    /// - `(A OR B) AND ¬(A AND B)`
    /// - `(a AND ¬b) OR (¬a AND b)`
    ///
    /// which are equivalent in classical logic, have different truth tables in
    /// Belnap's 4-valued logic.
    ///
    /// For this implementation, we chose the proposition
    /// - `(a AND ¬b) OR (¬a AND b)`
    ///
    /// as it's closer to the natural language interpretation of XOR.
    fn xor(self, other: Self) -> Self {
        self.xor(other)
    }
}

impl ops::Not for Belnapian {
    type Output = Self;

    fn not(self) -> Self {
        self.not()
    }
}

impl LogicOperand for Belnapian {}

// TernaryTruth Impls
// -----------------------------------------------------------------------------

impl TruthValue for TernaryTruth {}

impl TernaryTruth {
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

    pub fn xor(self, other: Self) -> Self {
        match (self, other) {
            // DO NOT REORDER THESE MATCHES
            (TernaryTruth::Unknown, _) => TernaryTruth::Unknown,
            (_, TernaryTruth::Unknown) => TernaryTruth::Unknown,
            (TernaryTruth::True, TernaryTruth::True) => TernaryTruth::False,
            (TernaryTruth::False, TernaryTruth::False) => TernaryTruth::False,
            _ => TernaryTruth::True,
        }
    }

    pub fn not(self) -> Self {
        match self {
            TernaryTruth::False => TernaryTruth::True,
            TernaryTruth::True => TernaryTruth::False,
            TernaryTruth::Unknown => TernaryTruth::Unknown,
        }
    }

    pub fn is_unknown(self) -> bool {
        match self {
            TernaryTruth::Unknown => true,
            _ => false,
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

impl AndOp for TernaryTruth {
    fn and(self, other: Self) -> Self {
        self.and(other)
    }
}

impl OrOp for TernaryTruth {
    fn or(self, other: Self) -> Self {
        self.or(other)
    }
}

impl XorOp for TernaryTruth {
    fn xor(self, other: Self) -> Self {
        self.xor(other)
    }
}

impl ops::Not for TernaryTruth {
    type Output = Self;

    fn not(self) -> Self {
        self.not()
    }
}

impl LogicOperand for TernaryTruth {}

// TruthValuesPowerSet Impls
// -----------------------------------------------------------------------------

impl TruthValuesPowerSet {
    pub fn could_be_neither(self) -> bool {
        match self {
            TruthValuesPowerSet::NFTB
            | TruthValuesPowerSet::N_TB
            | TruthValuesPowerSet::NF_B
            | TruthValuesPowerSet::N__B
            | TruthValuesPowerSet::NFT_
            | TruthValuesPowerSet::N_T_
            | TruthValuesPowerSet::NF__
            | TruthValuesPowerSet::N___ => true,
            _ => false,
        }
    }

    pub fn could_be_false(self) -> bool {
        match self {
            TruthValuesPowerSet::NFTB
            | TruthValuesPowerSet::_FTB
            | TruthValuesPowerSet::NF_B
            | TruthValuesPowerSet::_F_B
            | TruthValuesPowerSet::NFT_
            | TruthValuesPowerSet::_FT_
            | TruthValuesPowerSet::NF__
            | TruthValuesPowerSet::_F__ => true,
            _ => false,
        }
    }

    pub fn could_be_true(self) -> bool {
        match self {
            TruthValuesPowerSet::NFTB
            | TruthValuesPowerSet::_FTB
            | TruthValuesPowerSet::N_TB
            | TruthValuesPowerSet::__TB
            | TruthValuesPowerSet::NFT_
            | TruthValuesPowerSet::_FT_
            | TruthValuesPowerSet::N_T_
            | TruthValuesPowerSet::__T_ => true,
            _ => false,
        }
    }

    pub fn could_be_both(self) -> bool {
        match self {
            TruthValuesPowerSet::NFTB
            | TruthValuesPowerSet::_FTB
            | TruthValuesPowerSet::N_TB
            | TruthValuesPowerSet::__TB
            | TruthValuesPowerSet::NF_B
            | TruthValuesPowerSet::N__B
            | TruthValuesPowerSet::_F_B
            | TruthValuesPowerSet::___B => true,
            _ => false,
        }
    }

    pub fn is_unknown(self) -> bool {
        match self {
            TruthValuesPowerSet::____
            | TruthValuesPowerSet::N___
            | TruthValuesPowerSet::_F__
            | TruthValuesPowerSet::__T_
            | TruthValuesPowerSet::___B => false,
            _ => true,
        }
    }

    pub fn is_empty(self) -> bool {
        return self == TruthValuesPowerSet::____;
    }
}

impl TruthValuesSet for TruthValuesPowerSet {
    fn could_be_neither(self) -> bool {
        self.could_be_neither()
    }

    fn could_be_false(self) -> bool {
        self.could_be_false()
    }

    fn could_be_true(self) -> bool {
        self.could_be_true()
    }

    fn could_be_both(self) -> bool {
        self.could_be_both()
    }

    fn is_unknown(self) -> bool {
        self.is_unknown()
    }

    fn is_empty(self) -> bool {
        self.is_empty()
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

    pub fn not(self) -> Self {
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

    pub fn could_be_neither(self) -> bool {
        match self {
            Unknown::NFTB
            | Unknown::N_TB
            | Unknown::NF_B
            | Unknown::N__B
            | Unknown::NFT_
            | Unknown::N_T_
            | Unknown::NF__ => true,
            _ => false,
        }
    }

    pub fn could_be_false(self) -> bool {
        match self {
            Unknown::NFTB
            | Unknown::_FTB
            | Unknown::NF_B
            | Unknown::_F_B
            | Unknown::NFT_
            | Unknown::_FT_
            | Unknown::NF__ => true,
            _ => false,
        }
    }

    pub fn could_be_true(self) -> bool {
        match self {
            Unknown::NFTB
            | Unknown::_FTB
            | Unknown::N_TB
            | Unknown::__TB
            | Unknown::NFT_
            | Unknown::_FT_
            | Unknown::N_T_ => true,
            _ => false,
        }
    }

    pub fn could_be_both(self) -> bool {
        match self {
            Unknown::NFTB
            | Unknown::_FTB
            | Unknown::N_TB
            | Unknown::__TB
            | Unknown::NF_B
            | Unknown::N__B
            | Unknown::_F_B => true,
            _ => false,
        }
    }

    pub fn is_unknown(self) -> bool {
        true
    }

    pub fn is_empty(self) -> bool {
        false
    }
}

impl ops::Not for Unknown {
    type Output = Self;

    fn not(self) -> Self {
        self.not()
    }
}

impl TruthValuesSet for Unknown {
    fn could_be_neither(self) -> bool {
        self.could_be_neither()
    }

    fn could_be_false(self) -> bool {
        self.could_be_false()
    }

    fn could_be_true(self) -> bool {
        self.could_be_true()
    }

    fn could_be_both(self) -> bool {
        self.could_be_both()
    }

    fn is_unknown(self) -> bool {
        true
    }

    fn is_empty(self) -> bool {
        false
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

impl TruthValue for EBelnapian {}

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

    pub fn not(self) -> Self {
        match self {
            EBelnapian::Known(value) => EBelnapian::Known(!value),
            EBelnapian::Unknown(value) => EBelnapian::Unknown(!value),
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

impl AndOp for EBelnapian {
    fn and(self, other: Self) -> Self {
        self.and(other)
    }
}

impl OrOp for EBelnapian {
    fn or(self, other: Self) -> Self {
        self.or(other)
    }
}

impl ops::Not for EBelnapian {
    type Output = Self;

    fn not(self) -> Self {
        self.not()
    }
}

impl LogicOperand for EBelnapian {}

// Conversions
// -----------------------------------------------------------------------------
// Bool:

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

// -----------------------------------------------------------------------------
// TernaryTruth:

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

impl From<TernaryTruth> for EBelnapian {
    fn from(value: TernaryTruth) -> Self {
        match value {
            TernaryTruth::False => EBelnapian::Known(Belnapian::False),
            TernaryTruth::True => EBelnapian::Known(Belnapian::True),
            TernaryTruth::Unknown => EBelnapian::Unknown(Unknown::_FT_),
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

// -----------------------------------------------------------------------------
// Belnapian:

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

// -----------------------------------------------------------------------------
// Unknown:

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

impl From<Unknown> for TruthValuesPowerSet {
    fn from(value: Unknown) -> Self {
        match value {
            Unknown::NF__ => TruthValuesPowerSet::NF__,
            Unknown::N_T_ => TruthValuesPowerSet::N_T_,
            Unknown::_FT_ => TruthValuesPowerSet::_FT_,
            Unknown::NFT_ => TruthValuesPowerSet::NFT_,
            Unknown::N__B => TruthValuesPowerSet::N__B,
            Unknown::_F_B => TruthValuesPowerSet::_F_B,
            Unknown::NF_B => TruthValuesPowerSet::NF_B,
            Unknown::__TB => TruthValuesPowerSet::__TB,
            Unknown::N_TB => TruthValuesPowerSet::N_TB,
            Unknown::_FTB => TruthValuesPowerSet::_FTB,
            Unknown::NFTB => TruthValuesPowerSet::NFTB,
        }
    }
}

impl TryFrom<TruthValuesPowerSet> for Unknown {
    type Error = ();

    fn try_from(value: TruthValuesPowerSet) -> Result<Unknown, Self::Error> {
        match value {
            TruthValuesPowerSet::NF__ => Ok(Unknown::NF__),
            TruthValuesPowerSet::N_T_ => Ok(Unknown::N_T_),
            TruthValuesPowerSet::_FT_ => Ok(Unknown::_FT_),
            TruthValuesPowerSet::NFT_ => Ok(Unknown::NFT_),
            TruthValuesPowerSet::N__B => Ok(Unknown::N__B),
            TruthValuesPowerSet::_F_B => Ok(Unknown::_F_B),
            TruthValuesPowerSet::NF_B => Ok(Unknown::NF_B),
            TruthValuesPowerSet::__TB => Ok(Unknown::__TB),
            TruthValuesPowerSet::N_TB => Ok(Unknown::N_TB),
            TruthValuesPowerSet::_FTB => Ok(Unknown::_FTB),
            TruthValuesPowerSet::NFTB => Ok(Unknown::NFTB),
            _ => Err(()), // TODO: Improve?
        }
    }
}
