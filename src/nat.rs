use crate::bool::*;

pub trait TNat: Default {
    type IsZero: TBool;
    fn as_int(&self) -> i32;
}

#[derive(Default, Debug)]
pub struct TZero;

#[derive(Default, Debug)]
pub struct TSucc<N: TNat>(N);

impl TNat for TZero {
    type IsZero = TTrue;

    fn as_int(&self) -> i32 {
        0
    }
}

impl<N: TNat> TNat for TSucc<N> {
    type IsZero = TFalse;

    fn as_int(&self) -> i32 {
        N::default().as_int() + 1
    }
}

pub type TOne = TSucc<TZero>;
pub type TTwo = TSucc<TOne>;
pub type TThree = TSucc<TTwo>;
pub type TFour = TSucc<TThree>;
pub type TFive = TSucc<TFour>;
pub type TSix = TSucc<TFive>;
pub type TSeven = TSucc<TSix>;
pub type TEight = TSucc<TSeven>;

pub trait TAdd<RHS: TNat>: TNat {
    type Output: TNat;
}

impl<RHS: TNat> TAdd<RHS> for TZero {
    type Output = RHS;
}

impl<RHS: TNat, N: TAdd<RHS>> TAdd<RHS> for TSucc<N> {
    type Output = TSucc<<N as TAdd<RHS>>::Output>;
}

#[test]
fn 型レベルの加算の結果が正しい() {
    // 0 + 0
    assert_eq!(0, <TZero as TAdd<TZero>>::Output::default().as_int());
    // 1 + 0
    assert_eq!(1, <TZero as TAdd<TOne>>::Output::default().as_int());
    // 1 + 1
    assert_eq!(2, <TOne as TAdd<TOne>>::Output::default().as_int());
    // 5 + 5
    type TTwo = <TOne as TAdd<TOne>>::Output;
    type TFive = <<TTwo as TAdd<TTwo>>::Output as TAdd<TOne>>::Output;
    assert_eq!(10, <TFive as TAdd<TFive>>::Output::default().as_int());
}

pub trait TSub<RHS: TNat>: TNat {
    type Output: TNat;
    type IsZero: TBool;
}

impl<LHS: TNat> TSub<TZero> for LHS {
    type Output = LHS;
    type IsZero = <LHS as TNat>::IsZero;
}
impl<N: TNat> TSub<TSucc<N>> for TZero {
    type Output = TZero;
    type IsZero = TTrue;
}
impl<N: TNat, M: TSub<N>> TSub<TSucc<N>> for TSucc<M> {
    type Output = <M as TSub<N>>::Output;
    type IsZero = <<M as TSub<N>>::Output as TNat>::IsZero;
}

#[test]
fn 型レベルの減算の結果が正しい() {
    // 0 - 0
    assert_eq!(0, <TZero as TSub<TZero>>::Output::default().as_int());
    // 0 - 1
    assert_eq!(0, <TZero as TSub<TOne>>::Output::default().as_int());
    // 1 - 1
    assert_eq!(0, <TOne as TSub<TOne>>::Output::default().as_int());
    // 6 - 5
    assert_eq!(1, <TSix as TSub<TFive>>::Output::default().as_int());
}

pub trait TEqual<RHS: TNat> {
    type Output: TBool;
}

impl<N: TNat, M: TNat, Out1: TBool, Out2: TBool> TEqual<N> for M
where
    N: TSub<M, IsZero = Out1>,
    M: TSub<N, IsZero = Out2>,
    Out1: TAnd<Out2>,
{
    type Output = <Out1 as TAnd<Out2>>::Output;
}

#[test]
fn 型レベルの等価の結果が正しい() {
    assert!(<TZero as TEqual<TZero>>::Output::default().as_bool());
    assert!(!<TOne as TEqual<TZero>>::Output::default().as_bool());
    assert!(<TEight as TEqual<<TFive as TAdd<TThree>>::Output>>::Output::default().as_bool());
}
