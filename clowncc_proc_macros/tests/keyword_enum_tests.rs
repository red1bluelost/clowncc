use clowncc_proc_macros::KeywordEnum;

#[test]
fn single_keyword() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, KeywordEnum)]
    enum E {
        #[keyword = "A"]
        A,
    }

    assert_eq!("A".parse(), Ok(E::A));
    assert_eq!("B".parse::<E>(), Err(()));
}

#[test]
fn multiple_keywords() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, KeywordEnum)]
    enum E {
        #[keyword = "a"]
        A,
        #[keyword = "bb"]
        Bb,
        #[keyword = "ccc"]
        Ccc,
        #[keyword = "d"]
        D,
        #[keyword = "e1"]
        E1,
    }

    assert_eq!("a".parse(), Ok(E::A));
    assert_eq!("bb".parse(), Ok(E::Bb));
    assert_eq!("ccc".parse(), Ok(E::Ccc));
    assert_eq!("d".parse(), Ok(E::D));
    assert_eq!("e1".parse(), Ok(E::E1));

    assert_eq!("B".parse::<E>(), Err(()));
    assert_eq!("hello".parse::<E>(), Err(()));
    assert_eq!("gary".parse::<E>(), Err(()));
    assert_eq!("nope".parse::<E>(), Err(()));
    assert_eq!("wowie asd".parse::<E>(), Err(()));
    assert_eq!("hi".parse::<E>(), Err(()));
}
