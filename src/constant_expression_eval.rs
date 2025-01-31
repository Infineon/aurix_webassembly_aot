
use wasmparser::{ ConstExpr,Ieee32, Ieee64};

use crate::{isa_model::Immediate};

macro_rules! _visit_only_const {
     // delegate the macro invocation to sub-invocations of this macro to
    // deal with each instruction on a case-by-case basis.
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
        $(
            _visit_only_const!(visit_one @$proposal $op $({ $($arg: $argty),* })? => $visit);
        )*
    };

    // constant instructions are defined manually, so do nothing.
    (visit_one @mvp I32Const $($rest:tt)*) => {};
    (visit_one @mvp I64Const $($rest:tt)*) => {};
    (visit_one @mvp F32Const $($rest:tt)*) => {};
    (visit_one @mvp F64Const $($rest:tt)*) => {};
    (visit_one @mvp End $($rest:tt)*) => {};

    // a Non-MVP instruction will cause a panic
    (visit_one @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident) => {
        fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
            panic!("Operator {:?} is not a valid part of constant expression", stringify!($op));
        }
    }

}

pub struct ConstantExpressionEval {
    emitted_value: Option<Immediate>
}

impl ConstantExpressionEval {
    pub fn new() -> Self {
        ConstantExpressionEval { emitted_value: None }
    }

    pub fn get_value(&self) -> Immediate {
        self.emitted_value.unwrap()
    }
}

impl <'a> wasmparser::VisitOperator <'a> for ConstantExpressionEval {

    type Output = ();

    wasmparser::for_each_operator!(_visit_only_const);

    fn visit_i32_const(&mut self, value:i32) {
        self.emitted_value = Some(Immediate::Word(value as u32));
    }

    fn visit_i64_const(&mut self,value:i64) {
        self.emitted_value = Some(Immediate::DoubleWord(value as u64));
    }

    fn visit_f32_const(&mut self,value:Ieee32) {
        self.emitted_value = Some(Immediate::Word(value.bits()));
    }

    fn visit_f64_const(&mut self,value:Ieee64) {
        self.emitted_value = Some(Immediate::DoubleWord(value.bits()));
    }

    fn visit_end(&mut self) {
    }
}

impl ConstantExpressionEval {
    pub fn eval_const_expr(expr: ConstExpr) -> Immediate {
        let mut visitor = Self::new();
        let mut op_reader = expr.get_operators_reader();
        while let Ok(..) = op_reader.visit_operator(&mut visitor) {}
        visitor.get_value()
    }
}