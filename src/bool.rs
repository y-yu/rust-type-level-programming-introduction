pub trait TBool: Default {
    fn as_bool(&self) -> bool;
}

#[derive(Default, Debug)]
pub struct TTrue;

#[derive(Default, Debug)]
pub struct TFalse;

impl TBool for TTrue {
    fn as_bool(&self) -> bool {
        true
    }
}
impl TBool for TFalse {
    fn as_bool(&self) -> bool {
        false
    }
}

trait TNand<RHS: TBool> {
    type Output: TBool;
}

impl<RHS: TBool> TNand<RHS> for TFalse {
    type Output = TTrue;
}

impl TNand<TTrue> for TTrue {
    type Output = TFalse;
}

impl TNand<TFalse> for TTrue {
    type Output = TTrue;
}

#[test]
#[allow(non_snake_case)]
fn 型レベルのNAND演算の結果が正しい() {
    assert!(!<TTrue as TNand<TTrue>>::Output::default().as_bool());
    assert!(<TTrue as TNand<TFalse>>::Output::default().as_bool());
    assert!(<TFalse as TNand<TTrue>>::Output::default().as_bool());
    assert!(<TFalse as TNand<TFalse>>::Output::default().as_bool());
}

pub trait TOr<RHS: TBool> {
    type Output: TBool;
}

impl<RHS: TBool> TOr<RHS> for TFalse {
    type Output = RHS;
}
impl<RHS: TBool> TOr<RHS> for TTrue {
    type Output = TTrue;
}

#[test]
#[allow(non_snake_case)]
fn 型レベルのOR演算の結果が正しい() {
    assert!(<TTrue as TOr<TTrue>>::Output::default().as_bool());
    assert!(<TTrue as TOr<TFalse>>::Output::default().as_bool());
    assert!(<TFalse as TOr<TTrue>>::Output::default().as_bool());
    assert!(!<TFalse as TOr<TFalse>>::Output::default().as_bool());
}

pub trait TAnd<RHS: TBool>: TBool {
    type Output: TBool;
}

impl<RHS: TBool> TAnd<RHS> for TFalse {
    type Output = TFalse;
}
impl<RHS: TBool> TAnd<RHS> for TTrue {
    type Output = RHS;
}

#[test]
#[allow(non_snake_case)]
fn 型レベルのAND演算の結果が正しい() {
    assert!(<TTrue as TAnd<TTrue>>::Output::default().as_bool());
    assert!(!<TTrue as TAnd<TFalse>>::Output::default().as_bool());
    assert!(!<TFalse as TAnd<TTrue>>::Output::default().as_bool());
    assert!(!<TFalse as TAnd<TFalse>>::Output::default().as_bool());
}
