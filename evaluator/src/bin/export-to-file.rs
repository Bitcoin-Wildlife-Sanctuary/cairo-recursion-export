use cairo_air::components::generic_opcode::{Claim, Eval};
use cairo_air::relations;
use cairo_recursion_evaluator::{ValueNumberAssignment, ValueNumberEvaluator};
use cairo_recursion_gvn::GVN_SYSTEM;
use miniz_oxide::deflate::compress_to_vec;
use std::fs::File;
use std::io::{BufWriter, Write};
use stwo_prover::constraint_framework::expr::ExprEvaluator;
use stwo_prover::constraint_framework::FrameworkEval;

fn main() {
    let mut file_eval =
        BufWriter::new(File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/eval.bin")).unwrap());
    let mut file_testdata =
        BufWriter::new(File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/testdata.bin")).unwrap());

    let eval = Eval {
        claim: Claim {
            log_size: 4, /* does not change the layout */
        },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    };

    GVN_SYSTEM.lock().unwrap().lock();
    let _ = eval.evaluate(ValueNumberEvaluator::new());
    println!(
        "number of entries: {}",
        GVN_SYSTEM.lock().unwrap().map.len()
    );

    let expr_eval = eval.evaluate(ExprEvaluator::new());
    let assignment = ValueNumberAssignment::from(&expr_eval.random_assignment());

    file_eval
        .write(&compress_to_vec(
            &bincode::serialize(&GVN_SYSTEM.lock().unwrap().map.iter().collect::<Vec<_>>())
                .unwrap(),
            6,
        ))
        .unwrap();

    file_testdata
        .write(&compress_to_vec(
            &bincode::serialize(&assignment).unwrap(),
            6,
        ))
        .unwrap();

    file_eval.flush().unwrap();
    file_testdata.flush().unwrap();

    GVN_SYSTEM.lock().unwrap().unlock();
}
