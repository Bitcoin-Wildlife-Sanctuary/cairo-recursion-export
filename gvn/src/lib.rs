use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::sync::{LazyLock, Mutex};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::FieldExpOps;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct ValueNumber(pub u32);

#[derive(Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum ValueNumberContent {
    Col((usize, usize, isize)),
    ParamBase(String),
    ParamExt(String),
    ConstBase(BaseField),
    ConstExt(SecureField),
    Add(ValueNumber, ValueNumber),
    Mul(ValueNumber, ValueNumber),
    Neg(ValueNumber),
    Inv(ValueNumber),
}

#[derive(Default, Serialize, Deserialize)]
pub struct GVNSystem {
    pub next_id: u32,
    pub map: HashMap<ValueNumberContent, ValueNumber>,
}

pub static GVN_SYSTEM: LazyLock<Mutex<GVNSystem>> = LazyLock::new(Default::default);

impl ValueNumberContent {
    pub fn get_id(&self) -> ValueNumber {
        let mut cs = GVN_SYSTEM.lock().unwrap();
        let r = cs.map.get(self);
        if let Some(r) = r {
            *r
        } else {
            let id = ValueNumber(cs.next_id);
            cs.next_id += 1;
            cs.map.insert(self.clone(), id);
            id
        }
    }

    pub fn exists(&self) -> bool {
        let cs = GVN_SYSTEM.lock().unwrap();
        cs.map.contains_key(self)
    }
}

impl From<BaseField> for ValueNumber {
    fn from(base_field: BaseField) -> Self {
        ValueNumberContent::ConstBase(base_field).get_id()
    }
}

impl From<SecureField> for ValueNumber {
    fn from(secure_field: SecureField) -> Self {
        if secure_field.0 .1.is_zero() && secure_field.1.is_zero() {
            ValueNumber::from(secure_field.0 .0)
        } else {
            ValueNumberContent::ConstExt(secure_field).get_id()
        }
    }
}

impl Add for ValueNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0 < rhs.0 {
            ValueNumberContent::Add(self, rhs).get_id()
        } else {
            ValueNumberContent::Add(rhs, self).get_id()
        }
    }
}

impl Sub for ValueNumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let c1 = ValueNumberContent::Neg(rhs).get_id();
        ValueNumberContent::Add(self, c1).get_id()
    }
}

impl Mul for ValueNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.0 < rhs.0 {
            ValueNumberContent::Mul(self, rhs).get_id()
        } else {
            ValueNumberContent::Mul(rhs, self).get_id()
        }
    }
}

impl AddAssign for ValueNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for ValueNumber {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for ValueNumber {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Neg for ValueNumber {
    type Output = Self;
    fn neg(self) -> Self::Output {
        ValueNumberContent::Neg(self).get_id()
    }
}

impl Zero for ValueNumber {
    fn zero() -> Self {
        ValueNumberContent::ConstBase(BaseField::zero()).get_id()
    }

    fn is_zero(&self) -> bool {
        panic!("Can't check if an expression is zero.");
    }
}

impl One for ValueNumber {
    fn one() -> Self {
        ValueNumberContent::ConstBase(BaseField::one()).get_id()
    }
}

impl FieldExpOps for ValueNumber {
    fn inverse(&self) -> Self {
        ValueNumberContent::Inv(*self).get_id()
    }
}

impl Add<BaseField> for ValueNumber {
    type Output = Self;
    fn add(self, rhs: BaseField) -> Self::Output {
        self + ValueNumber::from(rhs)
    }
}

impl AddAssign<BaseField> for ValueNumber {
    fn add_assign(&mut self, rhs: BaseField) {
        *self = *self + rhs;
    }
}

impl Mul<BaseField> for ValueNumber {
    type Output = Self;
    fn mul(self, rhs: BaseField) -> Self::Output {
        self * ValueNumber::from(rhs)
    }
}

impl Mul<SecureField> for ValueNumber {
    type Output = Self;
    fn mul(self, rhs: SecureField) -> Self::Output {
        self * ValueNumber::from(rhs)
    }
}

impl Add<SecureField> for ValueNumber {
    type Output = Self;
    fn add(self, rhs: SecureField) -> Self::Output {
        self + ValueNumber::from(rhs)
    }
}

impl Sub<SecureField> for ValueNumber {
    type Output = Self;
    fn sub(self, rhs: SecureField) -> Self::Output {
        self + ValueNumber::from(-rhs)
    }
}

impl GVNSystem {
    pub fn export(&self) -> HashMap<ValueNumber, ValueNumberContent> {
        self.map.iter().map(|(k, v)| (*v, k.clone())).collect()
    }
}
