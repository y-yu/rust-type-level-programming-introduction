use crate::bool::*;
use crate::nat::*;

pub trait HList {}

#[derive(Default, Debug)]
pub struct HNil;
#[derive(Debug)]
pub struct HCons<H, L: HList>(H, L);

impl HList for HNil {}
impl<H, L: HList> HList for HCons<H, L> {}

#[test]
fn いろいろな型を入れることができる() {
    let _: HCons<TThree, HCons<TTwo, HCons<TOne, HCons<TZero, HNil>>>> = HCons(
        TThree::default(),
        HCons(
            TTwo::default(),
            HCons(TOne::default(), HCons(TZero::default(), HNil::default())),
        ),
    );
}

pub trait TContains<N: TNat> {
    type Output: TBool;
}

impl<N: TNat> TContains<N> for HNil {
    type Output = TFalse;
}

impl<E: TNat, H: TNat, L: HList, Out1: TBool, Out2: TBool> TContains<E> for HCons<H, L>
where
    E: TEqual<H, Output = Out1>,
    L: TContains<E, Output = Out2>,
    Out1: TOr<Out2>,
{
    type Output = <Out1 as TOr<Out2>>::Output;
}

#[test]
#[allow(non_snake_case)]
fn TContainsのテスト() {
    type OneTwoThree = HCons<TOne, HCons<TTwo, HCons<TThree, HNil>>>;

    assert!(<OneTwoThree as TContains<TTwo>>::Output::default().as_bool());
    assert!(!<OneTwoThree as TContains<TZero>>::Output::default().as_bool());
}
