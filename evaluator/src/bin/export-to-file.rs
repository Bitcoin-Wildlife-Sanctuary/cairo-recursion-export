use cairo_recursion_evaluator::mock_parameters::*;
use cairo_recursion_evaluator::{ValueNumberAssignment, ValueNumberEvaluator};
use cairo_recursion_gvn::{ValueNumber, ValueNumberContent, GVN_SYSTEM};
use miniz_oxide::deflate::compress_to_vec;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use stwo_prover::constraint_framework::expr::ExprEvaluator;
use stwo_prover::constraint_framework::FrameworkEval;

#[derive(Serialize, Deserialize)]
pub struct ExportedData {
    pub map: Vec<(ValueNumberContent, ValueNumber)>,
    pub assignment: ValueNumberAssignment,
    pub constraints: Vec<ValueNumber>,
}

fn export<T: FrameworkEval>(name: &str, f: fn() -> T) {
    let mut file = BufWriter::new(
        File::create(format!("{}/../data/{}", env!("CARGO_MANIFEST_DIR"), name)).unwrap(),
    );
    let eval = f();

    GVN_SYSTEM.lock().unwrap().lock();
    let e = eval.evaluate(ValueNumberEvaluator::new());

    let expr_eval = eval.evaluate(ExprEvaluator::new());
    let assignment = ValueNumberAssignment::from(&expr_eval.random_assignment());

    println!(
        "{} has {} entries, {} columns, {} constraints",
        name,
        GVN_SYSTEM.lock().unwrap().map.len(),
        assignment.col_map.len(),
        e.constraints.len(),
    );

    let data = ExportedData {
        map: GVN_SYSTEM
            .lock()
            .unwrap()
            .map
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>(),
        assignment,
        constraints: e.constraints,
    };
    file.write(&compress_to_vec(&bincode::serialize(&data).unwrap(), 6))
        .unwrap();
    file.flush().unwrap();
    GVN_SYSTEM.lock().unwrap().unlock();
}

fn main() {
    export("add_opcode", mock_add_opcode);
    export("add_opcode_small", mock_add_opcode_small);
    export("add_ap_opcode", mock_add_ap_opcode);
    export("assert_eq_opcode", mock_assert_eq_opcode);
    export("assert_eq_opcode_imm", mock_assert_eq_opcode_imm);
    export(
        "assert_eq_opcode_double_deref",
        mock_assert_eq_opcode_double_deref,
    );
    export("blake_compress_opcode", mock_blake_compress_opcode);
    export("call_opcode", mock_call_opcode);
    export("call_opcode_op_1_base_fp", mock_call_opcode_op_1_base_fp);
    export("call_opcode_rel", mock_call_opcode_rel);
    export("generic_opcode", mock_generic_opcode);
    export("jnz_opcode", mock_jnz_opcode);
    export("jnz_opcode_taken", mock_jnz_opcode_taken);
    export("jump_opcode", mock_jump_opcode);
    export("jump_opcode_double_deref", mock_jump_opcode_double_deref);
    export("jump_opcode_rel", mock_jump_opcode_rel);
    export("jump_opcode_rel_imm", mock_jump_opcode_rel_imm);
    export("mul_opcode", mock_mul_opcode);
    export("mul_opcode_small", mock_mul_opcode_small);
    export("qm_31_add_mul_opcode", mock_qm_31_add_mul_opcode);
    export("ret_opcode", mock_ret_opcode);
    export("verify_instruction", mock_verify_instruction);
    export("blake_round", mock_blake_round);
    export("blake_g", mock_blake_g);
    export("blake_round_sigma", mock_blake_round_sigma);
    export("triple_xor_32", mock_triple_xor_32);
    export("verify_bitwise_xor_12", mock_verify_bitwise_xor_12);
    export("memory_address_to_id", mock_memory_address_to_id);
    export("memory_id_to_big_big", mock_memory_id_to_big_big);
    export("memory_id_to_big_small", mock_memory_id_to_big_small);
    export("range_check_6", mock_range_check_6);
    export("range_check_8", mock_range_check_8);
    export("range_check_11", mock_range_check_11);
    export("range_check_12", mock_range_check_12);
    export("range_check_18", mock_range_check_18);
    export("range_check_19", mock_range_check_19);
    export("range_check_3_6", mock_range_check_3_6);
    export("range_check_4_3", mock_range_check_4_3);
    export("range_check_4_4", mock_range_check_4_4);
    export("range_check_5_4", mock_range_check_5_4);
    export("range_check_9_9", mock_range_check_9_9);
    export("range_check_7_2_5", mock_range_check_7_2_5);
    export("range_check_3_6_6_3", mock_range_check_3_6_6_3);
    export("range_check_4_4_4_4", mock_range_check_4_4_4_4);
    export("range_check_3_3_3_3_3", mock_range_check_3_3_3_3_3);
    export("verify_bitwise_xor_4", mock_verify_bitwise_xor_4);
    export("verify_bitwise_xor_7", mock_verify_bitwise_xor_7);
    export("verify_bitwise_xor_8", mock_verify_bitwise_xor_8);
    export("verify_bitwise_xor_9", mock_verify_bitwise_xor_9);
}
