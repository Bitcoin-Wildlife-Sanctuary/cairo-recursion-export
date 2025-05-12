// largely the same as https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/constraint_framework/expr/evaluator.rs

use cairo_recursion_gvn::{ValueNumber, ValueNumberContent, GVN_SYSTEM};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Mul;
use stwo_prover::constraint_framework::expr::assignment::ExprVarAssignment;
use stwo_prover::constraint_framework::{
    EvalAtRow, Relation, RelationEntry, INTERACTION_TRACE_IDX,
};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::lookups::utils::Fraction;

pub struct ValueNumberLogupAtRow {
    pub interaction: usize,
    pub claimed_sum: ValueNumber,
    pub fracs: Vec<Fraction<ValueNumber, ValueNumber>>,
    pub is_finalized: bool,
    pub is_first: ValueNumber,
    pub cumsum_shift: ValueNumber,
}

impl ValueNumberLogupAtRow {
    pub fn new(interaction: usize) -> Self {
        let claimed_sum_name = "claimed_sum".to_string();
        let column_size_name = "column_size".to_string();

        Self {
            interaction,
            claimed_sum: ValueNumberContent::ParamExt(claimed_sum_name.clone()).get_id(),
            fracs: vec![],
            is_finalized: true,
            is_first: ValueNumber::zero(),
            cumsum_shift: ValueNumberContent::ParamExt(claimed_sum_name).get_id()
                * ValueNumberContent::ParamBase(column_size_name)
                    .get_id()
                    .inverse(),
        }
    }
}

/// Returns the expression
/// `value[0] * <relation>_alpha0 + value[1] * <relation>_alpha1 + ... - <relation>_z.`
fn combine_value_number<R: Relation<ValueNumber, ValueNumber>>(
    relation: &R,
    values: &[ValueNumber],
) -> ValueNumber {
    const Z_SUFFIX: &str = "_z";
    const ALPHA_SUFFIX: &str = "_alpha";

    let z = ValueNumberContent::ParamExt(relation.get_name().to_owned() + Z_SUFFIX).get_id();
    assert!(
        relation.get_size() >= values.len(),
        "Not enough alpha powers to combine values"
    );
    let alpha_powers = (0..relation.get_size()).map(|i| {
        ValueNumberContent::ParamExt(relation.get_name().to_owned() + ALPHA_SUFFIX + &i.to_string())
            .get_id()
    });
    values
        .iter()
        .zip(alpha_powers)
        .fold(ValueNumber::zero(), |acc, (value, power)| {
            acc + power * value.clone()
        })
        - z
}

/// An Evaluator that saves all constraint expressions.
pub struct ValueNumberEvaluator {
    pub cur_var_index: usize,
    pub constraints: Vec<ValueNumber>,
    pub logup: ValueNumberLogupAtRow,
}

impl ValueNumberEvaluator {
    pub fn new() -> Self {
        Self {
            cur_var_index: 0,
            constraints: vec![],
            logup: ValueNumberLogupAtRow::new(INTERACTION_TRACE_IDX),
        }
    }
}

impl EvalAtRow for ValueNumberEvaluator {
    type F = ValueNumber;
    type EF = ValueNumber;

    fn next_interaction_mask<const N: usize>(
        &mut self,
        interaction: usize,
        offsets: [isize; N],
    ) -> [Self::F; N] {
        let res = std::array::from_fn(|i| {
            let col = (interaction, self.cur_var_index, offsets[i]);
            ValueNumberContent::Col(col).get_id()
        });
        self.cur_var_index += 1;
        res
    }

    fn add_constraint<G>(&mut self, constraint: G)
    where
        Self::EF: Mul<G, Output = Self::EF> + From<G>,
    {
        self.constraints.push(constraint.into())
    }

    fn combine_ef(values: [Self::F; SECURE_EXTENSION_DEGREE]) -> Self::EF {
        let i = ValueNumber::from(SecureField::from_u32_unchecked(0, 1, 0, 0));
        let j = ValueNumber::from(SecureField::from_u32_unchecked(0, 0, 1, 0));
        let ij = ValueNumber::from(SecureField::from_u32_unchecked(0, 0, 0, 1));
        values[0] + values[1] * i + values[2] * j + values[3] * ij
    }

    fn add_to_relation<R: Relation<Self::F, Self::EF>>(
        &mut self,
        entry: RelationEntry<'_, Self::F, Self::EF, R>,
    ) {
        let intermediate = combine_value_number(entry.relation, entry.values);
        let frac = Fraction::new(entry.multiplicity.clone(), intermediate);
        self.write_logup_frac(frac);
    }

    fn write_logup_frac(&mut self, fraction: Fraction<Self::EF, Self::EF>) {
        if self.logup.fracs.is_empty() {
            self.logup.is_finalized = false;
        }
        self.logup.fracs.push(fraction.clone());
    }

    /// Finalize the logup by adding the constraints for the fractions, batched by
    /// the given `batching`.
    /// `batching` should contain the batch into which every logup entry should be inserted.
    fn finalize_logup_batched(&mut self, batching: &Vec<usize>) {
        assert!(!self.logup.is_finalized, "LogupAtRow was already finalized");
        assert_eq!(
            batching.len(),
            self.logup.fracs.len(),
            "Batching must be of the same length as the number of entries"
        );

        let last_batch = *batching.iter().max().unwrap();

        let mut fracs_by_batch =
            std::collections::HashMap::<usize, Vec<Fraction<Self::EF, Self::EF>>>::new();

        for (batch, frac) in batching.iter().zip(self.logup.fracs.iter()) {
            fracs_by_batch
                .entry(*batch)
                .or_insert_with(Vec::new)
                .push(frac.clone());
        }

        let keys_set: std::collections::HashSet<_> = fracs_by_batch.keys().cloned().collect();
        let all_batches_set: std::collections::HashSet<_> = (0..last_batch + 1).collect();

        assert_eq!(
            keys_set, all_batches_set,
            "Batching must contain all consecutive batches"
        );

        let mut prev_col_cumsum = <Self::EF as num_traits::Zero>::zero();

        // All batches except the last are cumulatively summed in new interaction columns.
        for batch_id in 0..last_batch {
            let cur_frac: Fraction<_, _> = fracs_by_batch[&batch_id].iter().cloned().sum();
            let [cur_cumsum] = self.next_extension_interaction_mask(self.logup.interaction, [0]);
            let diff = cur_cumsum.clone() - prev_col_cumsum.clone();
            prev_col_cumsum = cur_cumsum;
            self.add_constraint(diff * cur_frac.denominator - cur_frac.numerator);
        }

        let frac: Fraction<_, _> = fracs_by_batch[&last_batch].clone().into_iter().sum();
        let [prev_row_cumsum, cur_cumsum] =
            self.next_extension_interaction_mask(self.logup.interaction, [-1, 0]);

        let diff = cur_cumsum - prev_row_cumsum - prev_col_cumsum.clone();
        // Instead of checking diff = num / denom, check diff = num / denom - cumsum_shift.
        // This makes (num / denom - cumsum_shift) have sum zero, which makes the constraint
        // uniform - apply on all rows.
        let fixed_diff = diff + self.logup.cumsum_shift.clone();

        self.add_constraint(fixed_diff * frac.denominator - frac.numerator);

        self.logup.is_finalized = true;
    }

    /// Finalizes the row's logup in the default way. Currently, this means no batching.
    fn finalize_logup(&mut self) {
        let batches = (0..self.logup.fracs.len()).collect();
        self.finalize_logup_batched(&batches)
    }

    /// Finalizes the row's logup, batched in pairs.
    /// TODO(alont) Remove this once a better batching mechanism is implemented.
    fn finalize_logup_in_pairs(&mut self) {
        let batches = (0..self.logup.fracs.len()).map(|n| n / 2).collect();
        self.finalize_logup_batched(&batches)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ValueNumberAssignment {
    pub col_map: HashMap<(usize, usize, isize), BaseField>,
    pub param_base_map: HashMap<String, BaseField>,
    pub param_ext_map: HashMap<String, SecureField>,
}

impl From<&ExprVarAssignment> for ValueNumberAssignment {
    fn from(assign: &ExprVarAssignment) -> Self {
        let col_map = assign.0.iter().map(|(k, v)| (k.clone(), *v)).collect();

        let param_base_map = assign.1.iter().map(|(k, v)| (k.clone(), *v)).collect();

        let param_ext_map = assign.2.iter().map(|(k, v)| (k.clone(), *v)).collect();

        Self {
            col_map,
            param_base_map,
            param_ext_map,
        }
    }
}

pub struct ValueNumberConstraint;

impl ValueNumberConstraint {
    pub fn compute(
        evaluator: &ValueNumberEvaluator,
        assign: &ValueNumberAssignment,
    ) -> Vec<SecureField> {
        let system = GVN_SYSTEM.lock().unwrap();
        let data = system.export();
        let mut values = Vec::with_capacity(system.next_id as usize);

        for i in 0..system.next_id {
            let c = &data[&ValueNumber(i)];
            match c {
                ValueNumberContent::Col(column_expr) => {
                    values.push(SecureField::from(assign.col_map[column_expr]));
                }
                ValueNumberContent::ParamBase(str) => {
                    values.push(SecureField::from(assign.param_base_map[str]));
                }
                ValueNumberContent::ParamExt(str) => {
                    values.push(SecureField::from(assign.param_ext_map[str]));
                }
                ValueNumberContent::ConstBase(v) => {
                    values.push(SecureField::from(*v));
                }
                ValueNumberContent::ConstExt(v) => {
                    values.push(*v);
                }
                ValueNumberContent::Add(v1, v2) => {
                    let d1 = values[v1.0 as usize];
                    let d2 = values[v2.0 as usize];
                    values.push(d1 + d2);
                }
                ValueNumberContent::Mul(v1, v2) => {
                    let d1 = values[v1.0 as usize];
                    let d2 = values[v2.0 as usize];
                    values.push(d1 * d2);
                }
                ValueNumberContent::Neg(v) => {
                    values.push(-values[v.0 as usize]);
                }
                ValueNumberContent::Inv(v) => {
                    values.push(values[v.0 as usize].inverse());
                }
            }
        }

        let mut constraints_values = vec![];
        for v in evaluator.constraints.iter() {
            constraints_values.push(values[v.0 as usize]);
        }
        constraints_values
    }
}

#[cfg(test)]
mod test {
    use crate::{ValueNumberAssignment, ValueNumberConstraint, ValueNumberEvaluator};
    use cairo_air::components::generic_opcode::{Claim, Eval};
    use cairo_air::relations;
    use cairo_recursion_gvn::GVN_SYSTEM;
    use num_traits::Zero;
    use rand::prelude::SmallRng;
    use rand::{Rng, SeedableRng};
    use std::ops::Deref;
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::constraint_framework::FrameworkEval;
    use stwo_prover::core::fields::qm31::QM31;

    #[test]
    fn test_generic_opcode_constraints() {
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        GVN_SYSTEM.lock().unwrap().lock();

        let evaluator = eval.evaluate(ValueNumberEvaluator::new());

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let constraints =
            ValueNumberConstraint::compute(&evaluator, &ValueNumberAssignment::from(&assignment));
        let expected_constraints = expr_eval
            .constraints
            .iter()
            .map(|c| c.assign(&assignment))
            .collect::<Vec<_>>();
        assert_eq!(constraints, expected_constraints);

        let mut sum = QM31::zero();

        let mut rng = SmallRng::seed_from_u64(0);
        for v in expected_constraints {
            sum += v * rng.gen::<QM31>();
        }
        assert_eq!(
            sum,
            QM31::from_u32_unchecked(676299912, 1544525990, 1973677465, 1109885774)
        );

        GVN_SYSTEM.lock().unwrap().unlock();
    }

    #[test]
    fn test_log_size_equivalance() {
        let eval_4 = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };
        let eval_10 = Eval {
            claim: Claim { log_size: 10 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        GVN_SYSTEM.lock().unwrap().lock();
        let _ = eval_4.evaluate(ValueNumberEvaluator::new());
        let aa =
            bincode::serialize(&GVN_SYSTEM.lock().unwrap().map.iter().collect::<Vec<_>>()).unwrap();
        let a = GVN_SYSTEM.lock().unwrap().map.clone();
        GVN_SYSTEM.lock().unwrap().unlock();

        GVN_SYSTEM.lock().unwrap().lock();
        let _ = eval_10.evaluate(ValueNumberEvaluator::new());
        let bb =
            bincode::serialize(&GVN_SYSTEM.lock().unwrap().map.iter().collect::<Vec<_>>()).unwrap();
        let b = GVN_SYSTEM.lock().unwrap().map.clone();
        GVN_SYSTEM.lock().unwrap().unlock();

        assert_eq!(a, b);
    }
}
