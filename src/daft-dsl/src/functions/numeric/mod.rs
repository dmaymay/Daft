mod abs;
mod ceil;
mod clip;
mod floor;
mod round;
mod sign;

use abs::AbsEvaluator;
use ceil::CeilEvaluator;
use clip::ClipEvaluator;
use floor::FloorEvaluator;
use round::RoundEvaluator;
use sign::SignEvaluator;

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use crate::Expr;

use super::FunctionEvaluator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NumericExpr {
    Abs,
    Ceil,
    Floor,
    Sign,
    Round(i32),
    Clip(Option<f64>, Option<f64>),
}

impl PartialEq for NumericExpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumericExpr::Abs, NumericExpr::Abs) => true,
            (NumericExpr::Ceil, NumericExpr::Ceil) => true,
            (NumericExpr::Floor, NumericExpr::Floor) => true,
            (NumericExpr::Sign, NumericExpr::Sign) => true,
            (NumericExpr::Round(a), NumericExpr::Round(b)) => a == b,
            (NumericExpr::Clip(a_lower, a_upper), NumericExpr::Clip(b_lower, b_upper)) => {
                a_lower == b_lower && a_upper == b_upper
            }
            _ => false,
        }
    }
}

impl Eq for NumericExpr {}

impl Hash for NumericExpr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            NumericExpr::Round(val) => val.hash(state),
            NumericExpr::Clip(lower, upper) => {
                lower.is_some().hash(state);
                upper.is_some().hash(state);
            }
            _ => (),
        }
    }
}

impl NumericExpr {
    #[inline]
    pub fn get_evaluator(&self) -> &dyn FunctionEvaluator {
        use NumericExpr::*;
        match self {
            Abs => &AbsEvaluator {},
            Ceil => &CeilEvaluator {},
            Floor => &FloorEvaluator {},
            Sign => &SignEvaluator {},
            Round(_) => &RoundEvaluator {},
            Clip(_, _) => &ClipEvaluator {},
        }
    }
}

pub fn abs(input: &Expr) -> Expr {
    Expr::Function {
        func: super::FunctionExpr::Numeric(NumericExpr::Abs),
        inputs: vec![input.clone()],
    }
}

pub fn ceil(input: &Expr) -> Expr {
    Expr::Function {
        func: super::FunctionExpr::Numeric(NumericExpr::Ceil),
        inputs: vec![input.clone()],
    }
}

pub fn floor(input: &Expr) -> Expr {
    Expr::Function {
        func: super::FunctionExpr::Numeric(NumericExpr::Floor),
        inputs: vec![input.clone()],
    }
}

pub fn sign(input: &Expr) -> Expr {
    Expr::Function {
        func: super::FunctionExpr::Numeric(NumericExpr::Sign),
        inputs: vec![input.clone()],
    }
}

pub fn round(input: &Expr, decimal: i32) -> Expr {
    Expr::Function {
        func: super::FunctionExpr::Numeric(NumericExpr::Round(decimal)),
        inputs: vec![input.clone()],
    }
}

pub fn clip(input: &Expr, lower: Option<f64>, upper: Option<f64>) -> Expr {
    Expr::Function {
        func: super::FunctionExpr::Numeric(NumericExpr::Clip(lower, upper)),
        inputs: vec![input.clone()],
    }
}
