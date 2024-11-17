#[cfg(test)]
mod belnapian_tests {
    use belnapian::*;

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

    #[test]
    fn test_packing() {
        assert_eq!(1, std::mem::size_of::<Belnapian>());
    }
}

#[cfg(test)]
mod ternary_truth_tests {
    use belnapian::*;

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

    #[test]
    fn test_packing() {
        assert_eq!(1, std::mem::size_of::<TernaryTruth>());
    }
}

#[cfg(test)]
mod truth_values_set_test {
    use belnapian::*;

    #[test]
    fn test_packing() {
        assert_eq!(1, std::mem::size_of::<TruthValuesPowerSet>());
    }
}

#[cfg(test)]
mod unknown_test {
    use belnapian::*;

    #[test]
    fn test_packing() {
        assert_eq!(1, std::mem::size_of::<Unknown>());
    }
}

#[cfg(test)]
mod ebelnapian_tests {
    use belnapian::*;

    #[test]
    fn test_packing() {
        assert_eq!(1, std::mem::size_of::<EBelnapian>());
    }
}
