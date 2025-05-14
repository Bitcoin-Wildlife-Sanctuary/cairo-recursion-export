use cairo_air::components::*;
use cairo_air::relations;

pub fn mock_add_opcode() -> add_opcode::Eval {
    use add_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_add_opcode_small() -> add_opcode_small::Eval {
    use add_opcode_small::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_add_ap_opcode() -> add_ap_opcode::Eval {
    use add_ap_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_assert_eq_opcode() -> assert_eq_opcode::Eval {
    use assert_eq_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_assert_eq_opcode_imm() -> assert_eq_opcode_imm::Eval {
    use assert_eq_opcode_imm::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_assert_eq_opcode_double_deref() -> assert_eq_opcode_double_deref::Eval {
    use assert_eq_opcode_double_deref::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_blake_compress_opcode() -> blake_compress_opcode::Eval {
    use blake_compress_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
        verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
        blake_round_lookup_elements: relations::BlakeRound::dummy(),
        triple_xor_32_lookup_elements: relations::TripleXor32::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_call_opcode() -> call_opcode::Eval {
    use call_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_call_opcode_op_1_base_fp() -> call_opcode_op_1_base_fp::Eval {
    use call_opcode_op_1_base_fp::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_call_opcode_rel() -> call_opcode_rel::Eval {
    use call_opcode_rel::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_generic_opcode() -> generic_opcode::Eval {
    use generic_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_jnz_opcode() -> jnz_opcode::Eval {
    use jnz_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_jnz_opcode_taken() -> jnz_opcode_taken::Eval {
    use jnz_opcode_taken::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_jump_opcode() -> jump_opcode::Eval {
    use jump_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_jump_opcode_double_deref() -> jump_opcode_double_deref::Eval {
    use jump_opcode_double_deref::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_jump_opcode_rel() -> jump_opcode_rel::Eval {
    use jump_opcode_rel::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_jump_opcode_rel_imm() -> jump_opcode_rel_imm::Eval {
    use jump_opcode_rel_imm::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_mul_opcode() -> mul_opcode::Eval {
    use mul_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_mul_opcode_small() -> mul_opcode_small::Eval {
    use mul_opcode_small::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        range_check_11_lookup_elements: relations::RangeCheck_11::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_qm_31_add_mul_opcode() -> qm_31_add_mul_opcode::Eval {
    use qm_31_add_mul_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_ret_opcode() -> ret_opcode::Eval {
    use ret_opcode::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        opcodes_lookup_elements: relations::Opcodes::dummy(),
    }
}

pub fn mock_verify_instruction() -> verify_instruction::Eval {
    use verify_instruction::*;
    Eval {
        claim: Claim { log_size: 4 },
        range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
        range_check_4_3_lookup_elements: relations::RangeCheck_4_3::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    }
}

pub fn mock_blake_round() -> blake_round::Eval {
    use blake_round::*;
    Eval {
        claim: Claim { log_size: 4 },
        blake_round_sigma_lookup_elements: relations::BlakeRoundSigma::dummy(),
        range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
        memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
        memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        blake_g_lookup_elements: relations::BlakeG::dummy(),
        blake_round_lookup_elements: relations::BlakeRound::dummy(),
    }
}

pub fn mock_blake_g() -> blake_g::Eval {
    use blake_g::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
        verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12::dummy(),
        verify_bitwise_xor_4_lookup_elements: relations::VerifyBitwiseXor_4::dummy(),
        verify_bitwise_xor_7_lookup_elements: relations::VerifyBitwiseXor_7::dummy(),
        verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9::dummy(),
        blake_g_lookup_elements: relations::BlakeG::dummy(),
    }
}

pub fn mock_blake_round_sigma() -> blake_round_sigma::Eval {
    use blake_round_sigma::*;
    Eval {
        claim: Claim {},
        blake_round_sigma_lookup_elements: relations::BlakeRoundSigma::dummy(),
    }
}

pub fn mock_triple_xor_32() -> triple_xor_32::Eval {
    use triple_xor_32::*;
    Eval {
        claim: Claim { log_size: 4 },
        verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
        triple_xor_32_lookup_elements: relations::TripleXor32::dummy(),
    }
}

pub fn mock_verify_bitwise_xor_12() -> verify_bitwise_xor_12::Eval {
    use verify_bitwise_xor_12::*;
    Eval {
        claim: Claim {},
        verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12::dummy(),
    }
}

pub fn mock_memory_address_to_id() -> memory_address_to_id::Eval {
    use memory_address_to_id::*;
    Eval {
        log_size: 4,
        lookup_elements: relations::MemoryAddressToId::dummy(),
    }
}

pub fn mock_memory_id_to_big_big() -> memory_id_to_big::BigEval {
    use memory_id_to_big::*;
    BigEval {
        log_n_rows: 4,
        lookup_elements: relations::MemoryIdToBig::dummy(),
        range9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
    }
}

pub fn mock_memory_id_to_big_small() -> memory_id_to_big::SmallEval {
    use memory_id_to_big::*;
    SmallEval {
        log_n_rows: 4,
        lookup_elements: relations::MemoryIdToBig::dummy(),
        range_check_9_9_relation: relations::RangeCheck_9_9::dummy(),
    }
}

pub fn mock_range_check_6() -> range_check_6::Eval {
    use range_check_6::*;
    Eval {
        lookup_elements: relations::RangeCheck_6::dummy(),
    }
}

pub fn mock_range_check_8() -> range_check_8::Eval {
    use range_check_8::*;
    Eval {
        lookup_elements: relations::RangeCheck_8::dummy(),
    }
}

pub fn mock_range_check_11() -> range_check_11::Eval {
    use range_check_11::*;
    Eval {
        lookup_elements: relations::RangeCheck_11::dummy(),
    }
}

pub fn mock_range_check_12() -> range_check_12::Eval {
    use range_check_12::*;
    Eval {
        lookup_elements: relations::RangeCheck_12::dummy(),
    }
}

pub fn mock_range_check_18() -> range_check_18::Eval {
    use range_check_18::*;
    Eval {
        lookup_elements: relations::RangeCheck_18::dummy(),
    }
}

pub fn mock_range_check_19() -> range_check_19::Eval {
    use range_check_19::*;
    Eval {
        lookup_elements: relations::RangeCheck_19::dummy(),
    }
}

pub fn mock_range_check_3_6() -> range_check_3_6::Eval {
    use range_check_3_6::*;
    Eval {
        lookup_elements: relations::RangeCheck_3_6::dummy(),
    }
}

pub fn mock_range_check_4_3() -> range_check_4_3::Eval {
    use range_check_4_3::*;
    Eval {
        lookup_elements: relations::RangeCheck_4_3::dummy(),
    }
}

pub fn mock_range_check_4_4() -> range_check_4_4::Eval {
    use range_check_4_4::*;
    Eval {
        lookup_elements: relations::RangeCheck_4_4::dummy(),
    }
}

pub fn mock_range_check_5_4() -> range_check_5_4::Eval {
    use range_check_5_4::*;
    Eval {
        lookup_elements: relations::RangeCheck_5_4::dummy(),
    }
}

pub fn mock_range_check_9_9() -> range_check_9_9::Eval {
    use range_check_9_9::*;
    Eval {
        lookup_elements: relations::RangeCheck_9_9::dummy(),
    }
}

pub fn mock_range_check_7_2_5() -> range_check_7_2_5::Eval {
    use range_check_7_2_5::*;
    Eval {
        lookup_elements: relations::RangeCheck_7_2_5::dummy(),
    }
}

pub fn mock_range_check_3_6_6_3() -> range_check_3_6_6_3::Eval {
    use range_check_3_6_6_3::*;
    Eval {
        lookup_elements: relations::RangeCheck_3_6_6_3::dummy(),
    }
}

pub fn mock_range_check_4_4_4_4() -> range_check_4_4_4_4::Eval {
    use range_check_4_4_4_4::*;
    Eval {
        lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
    }
}

pub fn mock_range_check_3_3_3_3_3() -> range_check_3_3_3_3_3::Eval {
    use range_check_3_3_3_3_3::*;
    Eval {
        lookup_elements: relations::RangeCheck_3_3_3_3_3::dummy(),
    }
}

pub fn mock_verify_bitwise_xor_4() -> verify_bitwise_xor_4::Eval {
    use verify_bitwise_xor_4::*;
    Eval {
        claim: Claim {},
        verify_bitwise_xor_4_lookup_elements: relations::VerifyBitwiseXor_4::dummy(),
    }
}

pub fn mock_verify_bitwise_xor_7() -> verify_bitwise_xor_7::Eval {
    use verify_bitwise_xor_7::*;
    Eval {
        claim: Claim {},
        verify_bitwise_xor_7_lookup_elements: relations::VerifyBitwiseXor_7::dummy(),
    }
}

pub fn mock_verify_bitwise_xor_8() -> verify_bitwise_xor_8::Eval {
    use verify_bitwise_xor_8::*;
    Eval {
        claim: Claim {},
        verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
    }
}

pub fn mock_verify_bitwise_xor_9() -> verify_bitwise_xor_9::Eval {
    use verify_bitwise_xor_9::*;
    Eval {
        claim: Claim {},
        verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9::dummy(),
    }
}
