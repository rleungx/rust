// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use infer::InferCtxt;
use infer::canonical::{Canonical, Canonicalize};
use traits::EvaluationResult;
use traits::query::CanonicalPredicateGoal;
use ty::{ParamEnv, ParamEnvAnd, Predicate, TyCtxt};

impl<'cx, 'gcx, 'tcx> InferCtxt<'cx, 'gcx, 'tcx> {
    /// Evaluates whether the predicate can be satisfied (by any means)
    /// in the given `ParamEnv`.
    pub fn predicate_may_hold(
        &self,
        param_env: ParamEnv<'tcx>,
        predicate: Predicate<'tcx>,
    ) -> bool {
        self.evaluate_obligation(param_env, predicate).may_apply()
    }

    /// Evaluates whether the predicate can be satisfied in the given
    /// `ParamEnv`, and returns `false` if not certain. However, this is
    /// not entirely accurate if inference variables are involved.
    pub fn predicate_must_hold(
        &self,
        param_env: ParamEnv<'tcx>,
        predicate: Predicate<'tcx>,
    ) -> bool {
        self.evaluate_obligation(param_env, predicate) == EvaluationResult::EvaluatedToOk
    }

    fn evaluate_obligation(
        &self,
        param_env: ParamEnv<'tcx>,
        predicate: Predicate<'tcx>,
    ) -> EvaluationResult {
        let (c_pred, _) = self.canonicalize_query(&param_env.and(predicate));

        self.tcx.global_tcx().evaluate_obligation(c_pred)
    }
}

impl<'gcx: 'tcx, 'tcx> Canonicalize<'gcx, 'tcx> for ParamEnvAnd<'tcx, Predicate<'tcx>> {
    type Canonicalized = CanonicalPredicateGoal<'gcx>;

    fn intern(
        _gcx: TyCtxt<'_, 'gcx, 'gcx>,
        value: Canonical<'gcx, Self::Lifted>,
    ) -> Self::Canonicalized {
        value
    }
}
