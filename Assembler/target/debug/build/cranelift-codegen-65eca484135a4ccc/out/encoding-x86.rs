// x86 recipe predicates.
fn recipe_predicate_op1r_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BinaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1r_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BinaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_rexop1u_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindex(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindexdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindexdisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindex(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindexdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindexdisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1st(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Store { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Store { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ld(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1lddisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1lddisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1adjustsp_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1brfb(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BranchFloat { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}
fn recipe_predicate_rexop1jt_entry(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BranchTableEntry { imm, .. } = *inst {
        return predicates::is_equal(imm, 1) || predicates::is_equal(imm, 2) || predicates::is_equal(imm, 4) || predicates::is_equal(imm, 8);
    }
    unreachable!();
}
fn recipe_predicate_trapff(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCondTrap { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}
fn recipe_predicate_op1icscc_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompareImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1icscc_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompareImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op2f32imm_z(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee32 { imm, .. } = *inst {
        return predicates::is_zero_32_bit_float(imm);
    }
    unreachable!();
}
fn recipe_predicate_mp2f64imm_z(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee64 { imm, .. } = *inst {
        return predicates::is_zero_64_bit_float(imm);
    }
    unreachable!();
}
fn recipe_predicate_mp3furmi_rnd(isap: crate::settings::PredicateView, _: &ir::InstructionData) -> bool {
    isap.test(16)
}
fn recipe_predicate_op2fcscc(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCompare { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}
fn recipe_predicate_mp2r_ib_unsigned_fpr(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::ExtractLane { lane, .. } = *inst {
        return predicates::is_unsigned_int(lane, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_mp3r_ib_unsigned_r(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::InsertLane { lane, .. } = *inst {
        return predicates::is_unsigned_int(lane, 8, 0);
    }
    unreachable!();
}

/// x86 recipe predicate table.
///
/// One entry per recipe, set to Some only when the recipe is guarded by a predicate.
pub static RECIPE_PREDICATES: [RecipePredicate; 287] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1r_ib),
    Some(recipe_predicate_op1r_ib),
    Some(recipe_predicate_op1r_id),
    Some(recipe_predicate_op1r_id),
    None,
    None,
    Some(recipe_predicate_rexop1u_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1adjustsp_ib),
    Some(recipe_predicate_rexop1u_id),
    Some(recipe_predicate_op1adjustsp_ib),
    Some(recipe_predicate_rexop1u_id),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_rexop1jt_entry),
    Some(recipe_predicate_rexop1jt_entry),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_trapff),
    None,
    None,
    Some(recipe_predicate_op1icscc_ib),
    Some(recipe_predicate_op1icscc_ib),
    Some(recipe_predicate_op1icscc_id),
    Some(recipe_predicate_op1icscc_id),
    None,
    None,
    Some(recipe_predicate_op1r_ib),
    Some(recipe_predicate_op1r_ib),
    Some(recipe_predicate_op1r_id),
    Some(recipe_predicate_op1r_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op2f32imm_z),
    Some(recipe_predicate_mp2f64imm_z),
    Some(recipe_predicate_op2f32imm_z),
    Some(recipe_predicate_mp2f64imm_z),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_mp3furmi_rnd),
    Some(recipe_predicate_mp3furmi_rnd),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    None,
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp3r_ib_unsigned_r),
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    None,
    None,
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

// x86 instruction predicates.
fn inst_predicate_0(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        let _ = func;
        return predicates::is_unsigned_int(imm, 32, 0);
    }
    unreachable!();
}
fn inst_predicate_1(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_int(imm);
    }
    unreachable!();
}
fn inst_predicate_2(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { ref args, .. } = *inst {
        let _ = func;
        return predicates::has_length_of(args, 2, func);
    }
    unreachable!();
}
fn inst_predicate_3(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { ref args, .. } = *inst {
        let _ = func;
        return predicates::has_length_of(args, 3, func);
    }
    unreachable!();
}
fn inst_predicate_4(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FuncAddr { func_ref, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_func(func_ref, func);
    }
    unreachable!();
}
fn inst_predicate_5(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryGlobalValue { global_value, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_data(global_value, func);
    }
    unreachable!();
}
fn inst_predicate_6(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Call { func_ref, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_func(func_ref, func);
    }
    unreachable!();
}
fn inst_predicate_7(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B1
}
fn inst_predicate_8(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I16
}
fn inst_predicate_9(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I32
}
fn inst_predicate_10(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I64
}
fn inst_predicate_11(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I8
}
fn inst_predicate_12(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee32 { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_32_bit_float(imm);
    }
    unreachable!();
}
fn inst_predicate_13(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee64 { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_64_bit_float(imm);
    }
    unreachable!();
}
fn inst_predicate_14(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F32
}
fn inst_predicate_15(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F64
}
fn inst_predicate_16(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B8X16
}
fn inst_predicate_17(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B16X8
}
fn inst_predicate_18(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B32X4
}
fn inst_predicate_19(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B64X2
}
fn inst_predicate_20(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I8X16
}
fn inst_predicate_21(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I16X8
}
fn inst_predicate_22(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I32X4
}
fn inst_predicate_23(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I64X2
}
fn inst_predicate_24(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F32X4
}
fn inst_predicate_25(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F64X2
}
fn inst_predicate_26(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryConst { constant_handle, .. } = *inst {
        let _ = func;
        return predicates::is_all_zeroes_128_bit(func.dfg.constants.get(constant_handle));
    }
    unreachable!();
}
fn inst_predicate_27(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryConst { constant_handle, .. } = *inst {
        let _ = func;
        return predicates::is_all_ones_128_bit(func.dfg.constants.get(constant_handle));
    }
    unreachable!();
}
fn inst_predicate_28(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompare { cond, .. } = *inst {
        let _ = func;
        return predicates::is_equal(cond, IntCC::Equal);
    }
    unreachable!();
}

/// x86 instruction predicate table.
///
/// One entry per instruction predicate, so the encoding bytecode can embed indexes into this
/// table.
pub static INST_PREDICATES: [InstPredicate; 29] = [
    inst_predicate_0,
    inst_predicate_1,
    inst_predicate_2,
    inst_predicate_3,
    inst_predicate_4,
    inst_predicate_5,
    inst_predicate_6,
    inst_predicate_7,
    inst_predicate_8,
    inst_predicate_9,
    inst_predicate_10,
    inst_predicate_11,
    inst_predicate_12,
    inst_predicate_13,
    inst_predicate_14,
    inst_predicate_15,
    inst_predicate_16,
    inst_predicate_17,
    inst_predicate_18,
    inst_predicate_19,
    inst_predicate_20,
    inst_predicate_21,
    inst_predicate_22,
    inst_predicate_23,
    inst_predicate_24,
    inst_predicate_25,
    inst_predicate_26,
    inst_predicate_27,
    inst_predicate_28,
];

/// x86 encoding lists.
///
/// This contains the entire encodings bytecode for every single instruction; the encodings
/// interpreter knows where to start from thanks to the initial lookup in the level 1 and level 2
/// table entries below.
pub static ENCLISTS: [u16; 1986] = [
    // 000000: adjust_sp_down.i64 (I64)
    // --> [RexOp1adjustsp#8029] and stop
    0x00eb, 0x8029,
    // end of adjust_sp_down.i64 (I64)
    // 000002: band.i64 (I64)
    // --> [RexOp1rr#8021] and stop
    // 000002: band.b64 (I64)
    // --> [RexOp1rr#8021] and stop
    0x0007, 0x8021,
    // end of band.b64 (I64)
    // end of band.i64 (I64)
    // 000004: band_imm.i64 (I64)
    // --> [RexOp1r_ib#c083]
    0x002e, 0xc083,
    // --> [RexOp1r_id#c081] and stop
    0x0033, 0xc081,
    // end of band_imm.i64 (I64)
    // 000008: bint.i64 (I64)
    // stop unless inst_predicate_7
    // 000008: bint.i32 (I64)
    // stop unless inst_predicate_7
    0x1007,
    // --> [RexOp2urm_noflags#4b6]
    // --> [RexOp2urm_noflags#4b6]
    0x01be, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x01bd, 0x04b6,
    // end of bint.i32 (I64)
    // end of bint.i64 (I64)
    // 00000d: bitcast.i64 (I64)
    // stop unless inst_predicate_15
    0x100f,
    // --> [RexMp2rfumr#857e] and stop
    0x01d5, 0x857e,
    // end of bitcast.i64 (I64)
    // 000010: bnot.i64 (I64)
    // --> [RexOp1ur#a0f7] and stop
    // 000010: bnot.b64 (I64)
    // --> [RexOp1ur#a0f7] and stop
    0x0017, 0xa0f7,
    // end of bnot.b64 (I64)
    // end of bnot.i64 (I64)
    // 000012: bor.i64 (I64)
    // --> [RexOp1rr#8009] and stop
    // 000012: bor.b64 (I64)
    // --> [RexOp1rr#8009] and stop
    0x0007, 0x8009,
    // end of bor.b64 (I64)
    // end of bor.i64 (I64)
    // 000014: bor_imm.i64 (I64)
    // --> [RexOp1r_ib#9083]
    0x002e, 0x9083,
    // --> [RexOp1r_id#9081] and stop
    0x0033, 0x9081,
    // end of bor_imm.i64 (I64)
    // 000018: brnz.i64 (I64)
    // --> [RexOp1tjccb#8075]
    0x016c, 0x8075,
    // --> [RexOp1tjccd#8085] and stop
    0x0171, 0x8085,
    // end of brnz.i64 (I64)
    // 00001c: brz.i64 (I64)
    // --> [RexOp1tjccb#8074]
    0x016c, 0x8074,
    // --> [RexOp1tjccd#8084] and stop
    0x0171, 0x8084,
    // end of brz.i64 (I64)
    // 000020: bxor.i64 (I64)
    // --> [RexOp1rr#8031] and stop
    // 000020: bxor.b64 (I64)
    // --> [RexOp1rr#8031] and stop
    0x0007, 0x8031,
    // end of bxor.b64 (I64)
    // end of bxor.i64 (I64)
    // 000022: bxor_imm.i64 (I64)
    // --> [RexOp1r_ib#e083]
    0x002e, 0xe083,
    // --> [RexOp1r_id#e081] and stop
    0x0033, 0xe081,
    // end of bxor_imm.i64 (I64)
    // 000026: call_indirect.i64 (I64)
    // --> [RexOp1call_r#20ff]
    0x0152, 0x20ff,
    // --> [Op1call_r#20ff] and stop
    // 000028: call_indirect.i32 (I32)
    // --> [Op1call_r#20ff] and stop
    0x0151, 0x20ff,
    // end of call_indirect.i32 (I32)
    // end of call_indirect.i64 (I64)
    // 00002a: clz.i64 (I64)
    // stop unless PredicateView(14)
    0x102b,
    // --> [RexMp2urm#86bd] and stop
    0x004b, 0x86bd,
    // end of clz.i64 (I64)
    // 00002d: copy.i64 (I64)
    // --> [RexOp1umr#8089] and stop
    // 00002d: copy.r64 (I64)
    // --> [RexOp1umr#8089] and stop
    0x0027, 0x8089,
    // end of copy.r64 (I64)
    // end of copy.i64 (I64)
    // 00002f: copy_nop.i64 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i32 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i8 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i16 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f64 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f32 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b8x16 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b16x8 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b32x4 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b64x2 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i8x16 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i16x8 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i32x4 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i64x2 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f32x4 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f64x2 (I64)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i32 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i8 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i16 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i64 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f64 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f32 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b8x16 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b16x8 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b32x4 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.b64x2 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i8x16 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i16x8 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i32x4 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.i64x2 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f32x4 (I32)
    // --> [stacknull#00] and stop
    // 00002f: copy_nop.f64x2 (I32)
    // --> [stacknull#00] and stop
    0x00e7, 0x0000,
    // end of copy_nop.f64x2 (I32)
    // end of copy_nop.f32x4 (I32)
    // end of copy_nop.i64x2 (I32)
    // end of copy_nop.i32x4 (I32)
    // end of copy_nop.i16x8 (I32)
    // end of copy_nop.i8x16 (I32)
    // end of copy_nop.b64x2 (I32)
    // end of copy_nop.b32x4 (I32)
    // end of copy_nop.b16x8 (I32)
    // end of copy_nop.b8x16 (I32)
    // end of copy_nop.f32 (I32)
    // end of copy_nop.f64 (I32)
    // end of copy_nop.i64 (I32)
    // end of copy_nop.i16 (I32)
    // end of copy_nop.i8 (I32)
    // end of copy_nop.i32 (I32)
    // end of copy_nop.f64x2 (I64)
    // end of copy_nop.f32x4 (I64)
    // end of copy_nop.i64x2 (I64)
    // end of copy_nop.i32x4 (I64)
    // end of copy_nop.i16x8 (I64)
    // end of copy_nop.i8x16 (I64)
    // end of copy_nop.b64x2 (I64)
    // end of copy_nop.b32x4 (I64)
    // end of copy_nop.b16x8 (I64)
    // end of copy_nop.b8x16 (I64)
    // end of copy_nop.f32 (I64)
    // end of copy_nop.f64 (I64)
    // end of copy_nop.i16 (I64)
    // end of copy_nop.i8 (I64)
    // end of copy_nop.i32 (I64)
    // end of copy_nop.i64 (I64)
    // 000031: copy_to_ssa.i64 (I64)
    // --> [RexOp1umr_reg_to_ssa#8089] and stop
    // 000031: copy_to_ssa.r64 (I64)
    // --> [RexOp1umr_reg_to_ssa#8089] and stop
    0x00e1, 0x8089,
    // end of copy_to_ssa.r64 (I64)
    // end of copy_to_ssa.i64 (I64)
    // 000033: ctz.i64 (I64)
    // stop unless PredicateView(13)
    0x102a,
    // --> [RexMp2urm#86bc] and stop
    0x004b, 0x86bc,
    // end of ctz.i64 (I64)
    // 000036: fill.i64 (I64)
    // --> [RexOp1fillSib32#808b] and stop
    // 000036: fill.r64 (I64)
    // --> [RexOp1fillSib32#808b] and stop
    0x00c9, 0x808b,
    // end of fill.r64 (I64)
    // end of fill.i64 (I64)
    // 000038: fill_nop.i64 (I64)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.i32 (I64)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.b1 (I64)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.i8 (I64)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.i16 (I64)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.i32 (I32)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.b1 (I32)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.i8 (I32)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.i16 (I32)
    // --> [fillnull#00] and stop
    // 000038: fill_nop.i64 (I32)
    // --> [fillnull#00] and stop
    0x00cf, 0x0000,
    // end of fill_nop.i64 (I32)
    // end of fill_nop.i16 (I32)
    // end of fill_nop.i8 (I32)
    // end of fill_nop.b1 (I32)
    // end of fill_nop.i32 (I32)
    // end of fill_nop.i16 (I64)
    // end of fill_nop.i8 (I64)
    // end of fill_nop.b1 (I64)
    // end of fill_nop.i32 (I64)
    // end of fill_nop.i64 (I64)
    // 00003a: func_addr.i64 (I64)
    // skip 2 unless PredicateView(11)
    0x3028,
    // --> [RexOp1fnaddr8#80b8]
    0x0136, 0x80b8,
    // skip 2 unless PredicateView(9)
    0x3026,
    // --> [RexOp1allones_fnaddr8#80b8]
    0x013a, 0x80b8,
    // skip 2 unless inst_predicate_4
    0x3004,
    // --> [RexOp1pcrel_fnaddr8#808d]
    0x013c, 0x808d,
    // stop unless PredicateView(10)
    0x1027,
    // --> [RexOp1got_fnaddr8#808b] and stop
    0x013f, 0x808b,
    // end of func_addr.i64 (I64)
    // 000046: get_pinned_reg.i64 (I64)
    // --> [get_pinned_reg#00] and stop
    0x0001, 0x0000,
    // end of get_pinned_reg.i64 (I64)
    // 000048: iadd.i64 (I64)
    // --> [RexOp1rr#8001] and stop
    0x0007, 0x8001,
    // end of iadd.i64 (I64)
    // 00004a: iadd_ifcarry.i64 (I64)
    // --> [RexOp1rio#8011] and stop
    0x0013, 0x8011,
    // end of iadd_ifcarry.i64 (I64)
    // 00004c: iadd_ifcin.i64 (I64)
    // --> [RexOp1rin#8011] and stop
    0x000f, 0x8011,
    // end of iadd_ifcin.i64 (I64)
    // 00004e: iadd_ifcout.i64 (I64)
    // --> [RexOp1rout#8001] and stop
    0x000b, 0x8001,
    // end of iadd_ifcout.i64 (I64)
    // 000050: iadd_imm.i64 (I64)
    // --> [RexOp1r_ib#8083]
    0x002e, 0x8083,
    // --> [RexOp1r_id#8081] and stop
    0x0033, 0x8081,
    // end of iadd_imm.i64 (I64)
    // 000054: icmp.i64 (I64)
    // --> [RexOp1icscc#8039] and stop
    0x0193, 0x8039,
    // end of icmp.i64 (I64)
    // 000056: icmp_imm.i64 (I64)
    // --> [RexOp1icscc_ib#f083]
    0x0196, 0xf083,
    // --> [RexOp1icscc_id#f081] and stop
    0x019b, 0xf081,
    // end of icmp_imm.i64 (I64)
    // 00005a: iconst.i64 (I64)
    // skip 4 unless inst_predicate_0
    0x5000,
    // --> [RexOp1pu_id#b8]
    0x0036, 0x00b8,
    // --> [Op1pu_id#b8]
    0x0034, 0x00b8,
    // --> [RexOp1u_id#80c7]
    0x0038, 0x80c7,
    // --> [RexOp1pu_iq#80b8]
    0x003a, 0x80b8,
    // stop unless inst_predicate_1
    // 000063: iconst.i16 (I64)
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#31]
    // --> [RexOp1u_id_z#31]
    0x0042, 0x0031,
    // --> [Op1u_id_z#31] and stop
    // --> [Op1u_id_z#31] and stop
    0x0041, 0x0031,
    // end of iconst.i16 (I64)
    // end of iconst.i64 (I64)
    // 000068: ifcmp.i64 (I64)
    // --> [RexOp1rcmp#8039] and stop
    0x019f, 0x8039,
    // end of ifcmp.i64 (I64)
    // 00006a: ifcmp_imm.i64 (I64)
    // --> [RexOp1rcmp_ib#f083]
    0x01a2, 0xf083,
    // --> [RexOp1rcmp_id#f081] and stop
    0x01a7, 0xf081,
    // end of ifcmp_imm.i64 (I64)
    // 00006e: ifcmp_sp.i64 (I64)
    // --> [RexOp1rcmp_sp#8039] and stop
    0x01ab, 0x8039,
    // end of ifcmp_sp.i64 (I64)
    // 000070: imul.i64 (I64)
    // --> [RexOp2rrx#84af] and stop
    0x001b, 0x84af,
    // end of imul.i64 (I64)
    // 000072: indirect_jump_table_br.i64 (I64)
    // --> [RexOp1indirect_jmp#40ff]
    0x0184, 0x40ff,
    // --> [Op1indirect_jmp#40ff] and stop
    // 000074: indirect_jump_table_br.i32 (I32)
    // --> [Op1indirect_jmp#40ff] and stop
    0x0187, 0x40ff,
    // end of indirect_jump_table_br.i32 (I32)
    // end of indirect_jump_table_br.i64 (I64)
    // 000076: ishl.i64 (I64)
    // --> [RexOp1rc#c0d3] and stop
    0x0047, 0xc0d3,
    // end of ishl.i64 (I64)
    // 000078: ishl_imm.i64 (I64)
    // --> [RexOp1r_ib#c0c1] and stop
    0x002f, 0xc0c1,
    // end of ishl_imm.i64 (I64)
    // 00007a: istore16.i64 (I64)
    // --> [RexMp1st#189]
    // 00007a: istore16.i32 (I64)
    // --> [RexMp1st#189]
    0x008e, 0x0189,
    // --> [Mp1st#189]
    // --> [Mp1st#189]
    0x008c, 0x0189,
    // --> [RexMp1stDisp8#189]
    // --> [RexMp1stDisp8#189]
    0x0096, 0x0189,
    // --> [Mp1stDisp8#189]
    // --> [Mp1stDisp8#189]
    0x0094, 0x0189,
    // --> [RexMp1stDisp32#189]
    // --> [RexMp1stDisp32#189]
    0x009e, 0x0189,
    // --> [Mp1stDisp32#189] and stop
    // --> [Mp1stDisp32#189] and stop
    0x009d, 0x0189,
    // end of istore16.i32 (I64)
    // end of istore16.i64 (I64)
    // 000086: istore16_complex.i64 (I64)
    // stop unless inst_predicate_3
    // 000086: istore16_complex.i32 (I64)
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexMp1stWithIndex#189]
    // --> [RexMp1stWithIndex#189]
    0x006a, 0x0189,
    // --> [Mp1stWithIndex#189]
    // --> [Mp1stWithIndex#189]
    0x0068, 0x0189,
    // --> [RexMp1stWithIndexDisp8#189]
    // --> [RexMp1stWithIndexDisp8#189]
    0x0072, 0x0189,
    // --> [Mp1stWithIndexDisp8#189]
    // --> [Mp1stWithIndexDisp8#189]
    0x0070, 0x0189,
    // --> [RexMp1stWithIndexDisp32#189]
    // --> [RexMp1stWithIndexDisp32#189]
    0x007a, 0x0189,
    // --> [Mp1stWithIndexDisp32#189] and stop
    // --> [Mp1stWithIndexDisp32#189] and stop
    0x0079, 0x0189,
    // end of istore16_complex.i32 (I64)
    // end of istore16_complex.i64 (I64)
    // 000093: istore32.i64 (I64)
    // --> [RexOp1st#89]
    // 000093: store.i32 (I64)
    // --> [RexOp1st#89]
    0x008a, 0x0089,
    // --> [Op1st#89]
    // --> [Op1st#89]
    0x0088, 0x0089,
    // --> [RexOp1stDisp8#89]
    // --> [RexOp1stDisp8#89]
    0x0092, 0x0089,
    // --> [Op1stDisp8#89]
    // --> [Op1stDisp8#89]
    0x0090, 0x0089,
    // --> [RexOp1stDisp32#89]
    // --> [RexOp1stDisp32#89]
    0x009a, 0x0089,
    // --> [Op1stDisp32#89] and stop
    // --> [Op1stDisp32#89] and stop
    0x0099, 0x0089,
    // end of store.i32 (I64)
    // end of istore32.i64 (I64)
    // 00009f: istore8.i64 (I64)
    // --> [RexOp1st#88]
    // 00009f: istore8.i32 (I64)
    // --> [RexOp1st#88]
    0x008a, 0x0088,
    // --> [Op1st_abcd#88]
    // --> [Op1st_abcd#88]
    0x00a0, 0x0088,
    // --> [RexOp1stDisp8#88]
    // --> [RexOp1stDisp8#88]
    0x0092, 0x0088,
    // --> [Op1stDisp8_abcd#88]
    // --> [Op1stDisp8_abcd#88]
    0x00a2, 0x0088,
    // --> [RexOp1stDisp32#88]
    // --> [RexOp1stDisp32#88]
    0x009a, 0x0088,
    // --> [Op1stDisp32_abcd#88] and stop
    // --> [Op1stDisp32_abcd#88] and stop
    0x00a5, 0x0088,
    // end of istore8.i32 (I64)
    // end of istore8.i64 (I64)
    // 0000ab: istore8_complex.i64 (I64)
    // stop unless inst_predicate_3
    // 0000ab: istore8_complex.i32 (I64)
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexOp1stWithIndex_abcd#88]
    // --> [RexOp1stWithIndex_abcd#88]
    0x007e, 0x0088,
    // --> [Op1stWithIndex_abcd#88]
    // --> [Op1stWithIndex_abcd#88]
    0x007c, 0x0088,
    // --> [RexOp1stWithIndexDisp8_abcd#88]
    // --> [RexOp1stWithIndexDisp8_abcd#88]
    0x0082, 0x0088,
    // --> [Op1stWithIndexDisp8_abcd#88]
    // --> [Op1stWithIndexDisp8_abcd#88]
    0x0080, 0x0088,
    // --> [RexOp1stWithIndexDisp32_abcd#88]
    // --> [RexOp1stWithIndexDisp32_abcd#88]
    0x0086, 0x0088,
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    0x0085, 0x0088,
    // end of istore8_complex.i32 (I64)
    // end of istore8_complex.i64 (I64)
    // 0000b8: isub.i64 (I64)
    // --> [RexOp1rr#8029] and stop
    0x0007, 0x8029,
    // end of isub.i64 (I64)
    // 0000ba: isub_ifbin.i64 (I64)
    // --> [RexOp1rin#8019] and stop
    0x000f, 0x8019,
    // end of isub_ifbin.i64 (I64)
    // 0000bc: isub_ifborrow.i64 (I64)
    // --> [RexOp1rio#8019] and stop
    0x0013, 0x8019,
    // end of isub_ifborrow.i64 (I64)
    // 0000be: isub_ifbout.i64 (I64)
    // --> [RexOp1rout#8029] and stop
    0x000b, 0x8029,
    // end of isub_ifbout.i64 (I64)
    // 0000c0: jump_table_base.i64 (I64)
    // --> [RexOp1jt_base#808d] and stop
    0x0181, 0x808d,
    // end of jump_table_base.i64 (I64)
    // 0000c2: jump_table_entry.i64 (I64)
    // --> [RexOp1jt_entry#8063] and stop
    0x017d, 0x8063,
    // end of jump_table_entry.i64 (I64)
    // 0000c4: load.i64 (I64)
    // --> [RexOp1ld#808b]
    0x00b0, 0x808b,
    // --> [RexOp1ldDisp8#808b]
    0x00b8, 0x808b,
    // --> [RexOp1ldDisp32#808b] and stop
    0x00c1, 0x808b,
    // end of load.i64 (I64)
    // 0000ca: load_complex.i64 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp1ldWithIndex#808b]
    0x004e, 0x808b,
    // --> [RexOp1ldWithIndexDisp8#808b]
    0x0056, 0x808b,
    // --> [RexOp1ldWithIndexDisp32#808b] and stop
    0x005f, 0x808b,
    // end of load_complex.i64 (I64)
    // 0000d1: popcnt.i64 (I64)
    // stop unless PredicateView(15)
    0x102c,
    // --> [RexMp2urm#86b8] and stop
    0x004b, 0x86b8,
    // end of popcnt.i64 (I64)
    // 0000d4: regfill.i64 (I64)
    // --> [RexOp1regfill32#808b] and stop
    // 0000d4: regfill.r64 (I64)
    // --> [RexOp1regfill32#808b] and stop
    0x00cd, 0x808b,
    // end of regfill.r64 (I64)
    // end of regfill.i64 (I64)
    // 0000d6: regmove.i64 (I64)
    // --> [RexOp1rmov#8089] and stop
    // 0000d6: regmove.r64 (I64)
    // --> [RexOp1rmov#8089] and stop
    0x002b, 0x8089,
    // end of regmove.r64 (I64)
    // end of regmove.i64 (I64)
    // 0000d8: regspill.i64 (I64)
    // --> [RexOp1regspill32#8089] and stop
    // 0000d8: regspill.r64 (I64)
    // --> [RexOp1regspill32#8089] and stop
    0x00ad, 0x8089,
    // end of regspill.r64 (I64)
    // end of regspill.i64 (I64)
    // 0000da: rotl.i64 (I64)
    // --> [RexOp1rc#80d3] and stop
    0x0047, 0x80d3,
    // end of rotl.i64 (I64)
    // 0000dc: rotl_imm.i64 (I64)
    // --> [RexOp1r_ib#80c1] and stop
    0x002f, 0x80c1,
    // end of rotl_imm.i64 (I64)
    // 0000de: rotr.i64 (I64)
    // --> [RexOp1rc#90d3] and stop
    0x0047, 0x90d3,
    // end of rotr.i64 (I64)
    // 0000e0: rotr_imm.i64 (I64)
    // --> [RexOp1r_ib#90c1] and stop
    0x002f, 0x90c1,
    // end of rotr_imm.i64 (I64)
    // 0000e2: selectif.i64 (I64)
    // --> [RexOp2cmov#8440] and stop
    0x01b7, 0x8440,
    // end of selectif.i64 (I64)
    // 0000e4: set_pinned_reg.i64 (I64)
    // --> [RexOp1set_pinned_reg#8089]
    0x0002, 0x8089,
    // --> [RexOp1set_pinned_reg#8089] and stop
    0x0003, 0x8089,
    // end of set_pinned_reg.i64 (I64)
    // 0000e8: sextend.i64 (I64)
    // skip 2 unless inst_predicate_11
    0x300b,
    // --> [RexOp2urm_noflags#84be]
    0x01be, 0x84be,
    // skip 2 unless inst_predicate_8
    0x3008,
    // --> [RexOp2urm_noflags#84bf]
    0x01be, 0x84bf,
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexOp1urm_noflags#8063] and stop
    0x01c5, 0x8063,
    // end of sextend.i64 (I64)
    // 0000f1: sload16.i64 (I64)
    // --> [RexOp2ld#84bf]
    0x00b4, 0x84bf,
    // --> [RexOp2ldDisp8#84bf]
    0x00bc, 0x84bf,
    // --> [RexOp2ldDisp32#84bf] and stop
    0x00c5, 0x84bf,
    // end of sload16.i64 (I64)
    // 0000f7: sload16_complex.i64 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#84bf]
    0x0052, 0x84bf,
    // --> [RexOp2ldWithIndexDisp8#84bf]
    0x005a, 0x84bf,
    // --> [RexOp2ldWithIndexDisp32#84bf] and stop
    0x0063, 0x84bf,
    // end of sload16_complex.i64 (I64)
    // 0000fe: sload32.i64 (I64)
    // --> [RexOp1ld#8063]
    0x00b0, 0x8063,
    // --> [RexOp1ldDisp8#8063]
    0x00b8, 0x8063,
    // --> [RexOp1ldDisp32#8063] and stop
    0x00c1, 0x8063,
    // end of sload32.i64 (I64)
    // 000104: sload8.i64 (I64)
    // --> [RexOp2ld#84be]
    0x00b4, 0x84be,
    // --> [RexOp2ldDisp8#84be]
    0x00bc, 0x84be,
    // --> [RexOp2ldDisp32#84be] and stop
    0x00c5, 0x84be,
    // end of sload8.i64 (I64)
    // 00010a: sload8_complex.i64 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#84be]
    0x0052, 0x84be,
    // --> [RexOp2ldWithIndexDisp8#84be]
    0x005a, 0x84be,
    // --> [RexOp2ldWithIndexDisp32#84be] and stop
    0x0063, 0x84be,
    // end of sload8_complex.i64 (I64)
    // 000111: spill.i64 (I64)
    // --> [RexOp1spillSib32#8089] and stop
    // 000111: spill.r64 (I64)
    // --> [RexOp1spillSib32#8089] and stop
    0x00a9, 0x8089,
    // end of spill.r64 (I64)
    // end of spill.i64 (I64)
    // 000113: sshr.i64 (I64)
    // --> [RexOp1rc#f0d3] and stop
    0x0047, 0xf0d3,
    // end of sshr.i64 (I64)
    // 000115: sshr_imm.i64 (I64)
    // --> [RexOp1r_ib#f0c1] and stop
    0x002f, 0xf0c1,
    // end of sshr_imm.i64 (I64)
    // 000117: stack_addr.i64 (I64)
    // --> [RexOp1spaddr8_id#808d] and stop
    0x014b, 0x808d,
    // end of stack_addr.i64 (I64)
    // 000119: store.i64 (I64)
    // --> [RexOp1st#8089]
    0x008a, 0x8089,
    // --> [RexOp1stDisp8#8089]
    0x0092, 0x8089,
    // --> [RexOp1stDisp32#8089] and stop
    0x009b, 0x8089,
    // end of store.i64 (I64)
    // 00011f: store_complex.i64 (I64)
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexOp1stWithIndex#8089]
    0x0066, 0x8089,
    // --> [RexOp1stWithIndexDisp8#8089]
    0x006e, 0x8089,
    // --> [RexOp1stWithIndexDisp32#8089] and stop
    0x0077, 0x8089,
    // end of store_complex.i64 (I64)
    // 000126: symbol_value.i64 (I64)
    // skip 2 unless PredicateView(12)
    0x3029,
    // --> [RexOp1gvaddr8#80b8]
    0x0142, 0x80b8,
    // skip 3 unless PredicateView(10)
    0x4027,
    // skip 2 unless inst_predicate_5
    0x3005,
    // --> [RexOp1pcrel_gvaddr8#808d]
    0x0144, 0x808d,
    // stop unless PredicateView(10)
    0x1027,
    // --> [RexOp1got_gvaddr8#808b] and stop
    0x0147, 0x808b,
    // end of symbol_value.i64 (I64)
    // 000130: uextend.i64 (I64)
    // skip 4 unless inst_predicate_11
    0x500b,
    // --> [RexOp2urm_noflags#4b6]
    0x01be, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x01bc, 0x04b6,
    // skip 4 unless inst_predicate_8
    0x5008,
    // --> [RexOp2urm_noflags#4b7]
    0x01be, 0x04b7,
    // --> [Op2urm_noflags#4b7]
    0x01c2, 0x04b7,
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexOp1umr#89]
    // 00013b: copy.i32 (I64)
    // --> [RexOp1umr#89]
    // 00013b: copy.b1 (I64)
    // --> [RexOp1umr#89]
    // 00013b: copy.i8 (I64)
    // --> [RexOp1umr#89]
    // 00013b: copy.i16 (I64)
    // --> [RexOp1umr#89]
    0x0026, 0x0089,
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // 00013d: copy.i32 (I32)
    // --> [Op1umr#89] and stop
    // 00013d: copy.b1 (I32)
    // --> [Op1umr#89] and stop
    // 00013d: copy.r32 (I32)
    // --> [Op1umr#89] and stop
    // 00013d: copy.i8 (I32)
    // --> [Op1umr#89] and stop
    // 00013d: copy.i16 (I32)
    // --> [Op1umr#89] and stop
    0x0025, 0x0089,
    // end of copy.i16 (I32)
    // end of copy.i8 (I32)
    // end of copy.r32 (I32)
    // end of copy.b1 (I32)
    // end of copy.i32 (I32)
    // end of copy.i16 (I64)
    // end of copy.i8 (I64)
    // end of copy.b1 (I64)
    // end of copy.i32 (I64)
    // end of uextend.i64 (I64)
    // 00013f: uload16.i64 (I64)
    // --> [RexOp2ld#84b7]
    0x00b4, 0x84b7,
    // --> [RexOp2ldDisp8#84b7]
    0x00bc, 0x84b7,
    // --> [RexOp2ldDisp32#84b7] and stop
    0x00c5, 0x84b7,
    // end of uload16.i64 (I64)
    // 000145: uload16_complex.i64 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#84b7]
    0x0052, 0x84b7,
    // --> [RexOp2ldWithIndexDisp8#84b7]
    0x005a, 0x84b7,
    // --> [RexOp2ldWithIndexDisp32#84b7] and stop
    0x0063, 0x84b7,
    // end of uload16_complex.i64 (I64)
    // 00014c: uload32.i64 (I64)
    // --> [RexOp1ld#8b]
    // 00014c: load.i32 (I64)
    // --> [RexOp1ld#8b]
    0x00b0, 0x008b,
    // --> [Op1ld#8b]
    // --> [Op1ld#8b]
    0x00ae, 0x008b,
    // --> [RexOp1ldDisp8#8b]
    // --> [RexOp1ldDisp8#8b]
    0x00b8, 0x008b,
    // --> [Op1ldDisp8#8b]
    // --> [Op1ldDisp8#8b]
    0x00b6, 0x008b,
    // --> [RexOp1ldDisp32#8b]
    // --> [RexOp1ldDisp32#8b]
    0x00c0, 0x008b,
    // --> [Op1ldDisp32#8b] and stop
    // --> [Op1ldDisp32#8b] and stop
    0x00bf, 0x008b,
    // end of load.i32 (I64)
    // end of uload32.i64 (I64)
    // 000158: uload8.i64 (I64)
    // --> [RexOp2ld#84b6]
    0x00b4, 0x84b6,
    // --> [RexOp2ldDisp8#84b6]
    0x00bc, 0x84b6,
    // --> [RexOp2ldDisp32#84b6] and stop
    0x00c5, 0x84b6,
    // end of uload8.i64 (I64)
    // 00015e: uload8_complex.i64 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#84b6]
    0x0052, 0x84b6,
    // --> [RexOp2ldWithIndexDisp8#84b6]
    0x005a, 0x84b6,
    // --> [RexOp2ldWithIndexDisp32#84b6] and stop
    0x0063, 0x84b6,
    // end of uload8_complex.i64 (I64)
    // 000165: ushr.i64 (I64)
    // --> [RexOp1rc#d0d3] and stop
    0x0047, 0xd0d3,
    // end of ushr.i64 (I64)
    // 000167: ushr_imm.i64 (I64)
    // --> [RexOp1r_ib#d0c1] and stop
    0x002f, 0xd0c1,
    // end of ushr_imm.i64 (I64)
    // 000169: x86_bsf.i64 (I64)
    // --> [RexOp2bsf_and_bsr#84bc] and stop
    0x01bb, 0x84bc,
    // end of x86_bsf.i64 (I64)
    // 00016b: x86_bsr.i64 (I64)
    // --> [RexOp2bsf_and_bsr#84bd] and stop
    0x01bb, 0x84bd,
    // end of x86_bsr.i64 (I64)
    // 00016d: x86_cvtt2si.i64 (I64)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [RexMp2rfurm#862c]
    0x01e4, 0x862c,
    // stop unless inst_predicate_15
    0x100f,
    // --> [RexMp2rfurm#872c] and stop
    0x01e5, 0x872c,
    // end of x86_cvtt2si.i64 (I64)
    // 000173: x86_pop.i64 (I64)
    // --> [RexOp1popq#58]
    0x00d8, 0x0058,
    // --> [Op1popq#58] and stop
    // 000175: x86_pop.i32 (I32)
    // --> [Op1popq#58] and stop
    0x00d7, 0x0058,
    // end of x86_pop.i32 (I32)
    // end of x86_pop.i64 (I64)
    // 000177: x86_push.i64 (I64)
    // --> [RexOp1pushq#50]
    0x00d4, 0x0050,
    // --> [Op1pushq#50] and stop
    // 000179: x86_push.i32 (I32)
    // --> [Op1pushq#50] and stop
    0x00d3, 0x0050,
    // end of x86_push.i32 (I32)
    // end of x86_push.i64 (I64)
    // 00017b: x86_sdivmodx.i64 (I64)
    // --> [RexOp1div#f0f7] and stop
    0x001f, 0xf0f7,
    // end of x86_sdivmodx.i64 (I64)
    // 00017d: x86_smulx.i64 (I64)
    // --> [RexOp1mulx#d0f7] and stop
    0x0023, 0xd0f7,
    // end of x86_smulx.i64 (I64)
    // 00017f: x86_udivmodx.i64 (I64)
    // --> [RexOp1div#e0f7] and stop
    0x001f, 0xe0f7,
    // end of x86_udivmodx.i64 (I64)
    // 000181: x86_umulx.i64 (I64)
    // --> [RexOp1mulx#c0f7] and stop
    0x0023, 0xc0f7,
    // end of x86_umulx.i64 (I64)
    // 000183: band.i32 (I64)
    // --> [RexOp1rr#21]
    // 000183: band.b32 (I64)
    // --> [RexOp1rr#21]
    // 000183: band.b1 (I64)
    // --> [RexOp1rr#21]
    0x0006, 0x0021,
    // --> [Op1rr#21] and stop
    // --> [Op1rr#21] and stop
    // --> [Op1rr#21] and stop
    // 000185: band.i32 (I32)
    // --> [Op1rr#21] and stop
    // 000185: band.b32 (I32)
    // --> [Op1rr#21] and stop
    // 000185: band.b1 (I32)
    // --> [Op1rr#21] and stop
    0x0005, 0x0021,
    // end of band.b1 (I32)
    // end of band.b32 (I32)
    // end of band.i32 (I32)
    // end of band.b1 (I64)
    // end of band.b32 (I64)
    // end of band.i32 (I64)
    // 000187: band_imm.i32 (I64)
    // --> [RexOp1r_ib#4083]
    0x002e, 0x4083,
    // --> [Op1r_ib#4083]
    0x002c, 0x4083,
    // --> [RexOp1r_id#4081]
    0x0032, 0x4081,
    // --> [Op1r_id#4081] and stop
    0x0031, 0x4081,
    // end of band_imm.i32 (I64)
    // 00018f: bitcast.i32 (I64)
    // stop unless inst_predicate_14
    0x100e,
    // --> [RexMp2rfumr#57e]
    0x01d4, 0x057e,
    // --> [Mp2rfumr#57e] and stop
    0x01d3, 0x057e,
    // end of bitcast.i32 (I64)
    // 000194: bnot.i32 (I64)
    // --> [RexOp1ur#20f7]
    // 000194: bnot.b32 (I64)
    // --> [RexOp1ur#20f7]
    0x0016, 0x20f7,
    // --> [Op1ur#20f7] and stop
    // --> [Op1ur#20f7] and stop
    // 000196: bnot.i32 (I32)
    // --> [Op1ur#20f7] and stop
    // 000196: bnot.b32 (I32)
    // --> [Op1ur#20f7] and stop
    0x0015, 0x20f7,
    // end of bnot.b32 (I32)
    // end of bnot.i32 (I32)
    // end of bnot.b32 (I64)
    // end of bnot.i32 (I64)
    // 000198: bor.i32 (I64)
    // --> [RexOp1rr#09]
    // 000198: bor.b32 (I64)
    // --> [RexOp1rr#09]
    // 000198: bor.b1 (I64)
    // --> [RexOp1rr#09]
    0x0006, 0x0009,
    // --> [Op1rr#09] and stop
    // --> [Op1rr#09] and stop
    // --> [Op1rr#09] and stop
    // 00019a: bor.i32 (I32)
    // --> [Op1rr#09] and stop
    // 00019a: bor.b32 (I32)
    // --> [Op1rr#09] and stop
    // 00019a: bor.b1 (I32)
    // --> [Op1rr#09] and stop
    0x0005, 0x0009,
    // end of bor.b1 (I32)
    // end of bor.b32 (I32)
    // end of bor.i32 (I32)
    // end of bor.b1 (I64)
    // end of bor.b32 (I64)
    // end of bor.i32 (I64)
    // 00019c: bor_imm.i32 (I64)
    // --> [RexOp1r_ib#1083]
    0x002e, 0x1083,
    // --> [Op1r_ib#1083]
    0x002c, 0x1083,
    // --> [RexOp1r_id#1081]
    0x0032, 0x1081,
    // --> [Op1r_id#1081] and stop
    0x0031, 0x1081,
    // end of bor_imm.i32 (I64)
    // 0001a4: brnz.i32 (I64)
    // --> [RexOp1tjccb#75]
    0x016c, 0x0075,
    // --> [Op1tjccb#75]
    0x016a, 0x0075,
    // --> [RexOp1tjccd#85]
    0x0170, 0x0085,
    // --> [Op1tjccd#85] and stop
    0x016f, 0x0085,
    // end of brnz.i32 (I64)
    // 0001ac: brz.i32 (I64)
    // --> [RexOp1tjccb#74]
    0x016c, 0x0074,
    // --> [Op1tjccb#74]
    0x016a, 0x0074,
    // --> [RexOp1tjccd#84]
    0x0170, 0x0084,
    // --> [Op1tjccd#84] and stop
    0x016f, 0x0084,
    // end of brz.i32 (I64)
    // 0001b4: bxor.i32 (I64)
    // --> [RexOp1rr#31]
    // 0001b4: bxor.b32 (I64)
    // --> [RexOp1rr#31]
    // 0001b4: bxor.b1 (I64)
    // --> [RexOp1rr#31]
    0x0006, 0x0031,
    // --> [Op1rr#31] and stop
    // --> [Op1rr#31] and stop
    // --> [Op1rr#31] and stop
    // 0001b6: bxor.i32 (I32)
    // --> [Op1rr#31] and stop
    // 0001b6: bxor.b32 (I32)
    // --> [Op1rr#31] and stop
    // 0001b6: bxor.b1 (I32)
    // --> [Op1rr#31] and stop
    0x0005, 0x0031,
    // end of bxor.b1 (I32)
    // end of bxor.b32 (I32)
    // end of bxor.i32 (I32)
    // end of bxor.b1 (I64)
    // end of bxor.b32 (I64)
    // end of bxor.i32 (I64)
    // 0001b8: bxor_imm.i32 (I64)
    // --> [RexOp1r_ib#6083]
    0x002e, 0x6083,
    // --> [Op1r_ib#6083]
    0x002c, 0x6083,
    // --> [RexOp1r_id#6081]
    0x0032, 0x6081,
    // --> [Op1r_id#6081] and stop
    0x0031, 0x6081,
    // end of bxor_imm.i32 (I64)
    // 0001c0: clz.i32 (I64)
    // stop unless PredicateView(14)
    0x102b,
    // --> [RexMp2urm#6bd]
    0x004a, 0x06bd,
    // --> [Mp2urm#6bd] and stop
    0x0049, 0x06bd,
    // end of clz.i32 (I64)
    // 0001c5: copy_to_ssa.i32 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001c5: copy_to_ssa.b1 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001c5: copy_to_ssa.i8 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001c5: copy_to_ssa.i16 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    0x00e1, 0x0089,
    // end of copy_to_ssa.i16 (I64)
    // end of copy_to_ssa.i8 (I64)
    // end of copy_to_ssa.b1 (I64)
    // end of copy_to_ssa.i32 (I64)
    // 0001c7: ctz.i32 (I64)
    // stop unless PredicateView(13)
    0x102a,
    // --> [RexMp2urm#6bc]
    0x004a, 0x06bc,
    // --> [Mp2urm#6bc] and stop
    0x0049, 0x06bc,
    // end of ctz.i32 (I64)
    // 0001cc: fill.i32 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001cc: fill.b1 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001cc: fill.i8 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001cc: fill.i16 (I64)
    // --> [RexOp1fillSib32#8b]
    0x00c8, 0x008b,
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // 0001ce: fill.i32 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001ce: fill.b1 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001ce: fill.r32 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001ce: fill.i8 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 0001ce: fill.i16 (I32)
    // --> [Op1fillSib32#8b] and stop
    0x00c7, 0x008b,
    // end of fill.i16 (I32)
    // end of fill.i8 (I32)
    // end of fill.r32 (I32)
    // end of fill.b1 (I32)
    // end of fill.i32 (I32)
    // end of fill.i16 (I64)
    // end of fill.i8 (I64)
    // end of fill.b1 (I64)
    // end of fill.i32 (I64)
    // 0001d0: iadd.i32 (I64)
    // --> [RexOp1rr#01]
    0x0006, 0x0001,
    // --> [Op1rr#01] and stop
    // 0001d2: iadd.i32 (I32)
    // --> [Op1rr#01] and stop
    0x0005, 0x0001,
    // end of iadd.i32 (I32)
    // end of iadd.i32 (I64)
    // 0001d4: iadd_ifcarry.i32 (I64)
    // --> [RexOp1rio#11]
    0x0012,
    // 0001d5: iadd_ifcarry.i32 (I32)
    // --> [Op1rio#11] and stop
    0x0011,
    // --> [Op1rio#11] and stop
    0x0011,
    // end of iadd_ifcarry.i32 (I32)
    0x0011,
    // end of iadd_ifcarry.i32 (I64)
    // 0001d8: iadd_ifcin.i32 (I64)
    // --> [RexOp1rin#11]
    0x000e, 0x0011,
    // --> [Op1rin#11] and stop
    // 0001da: iadd_ifcin.i32 (I32)
    // --> [Op1rin#11] and stop
    0x000d, 0x0011,
    // end of iadd_ifcin.i32 (I32)
    // end of iadd_ifcin.i32 (I64)
    // 0001dc: iadd_ifcout.i32 (I64)
    // --> [RexOp1rout#01]
    0x000a, 0x0001,
    // --> [Op1rout#01] and stop
    // 0001de: iadd_ifcout.i32 (I32)
    // --> [Op1rout#01] and stop
    0x0009, 0x0001,
    // end of iadd_ifcout.i32 (I32)
    // end of iadd_ifcout.i32 (I64)
    // 0001e0: iadd_imm.i32 (I64)
    // --> [RexOp1r_ib#83]
    0x002e, 0x0083,
    // --> [Op1r_ib#83]
    0x002c, 0x0083,
    // --> [RexOp1r_id#81]
    0x0032, 0x0081,
    // --> [Op1r_id#81] and stop
    0x0031, 0x0081,
    // end of iadd_imm.i32 (I64)
    // 0001e8: icmp.i32 (I64)
    // --> [RexOp1icscc#39]
    0x0192, 0x0039,
    // --> [Op1icscc#39] and stop
    // 0001ea: icmp.i32 (I32)
    // --> [Op1icscc#39] and stop
    0x0191, 0x0039,
    // end of icmp.i32 (I32)
    // end of icmp.i32 (I64)
    // 0001ec: icmp_imm.i32 (I64)
    // --> [RexOp1icscc_ib#7083]
    0x0196, 0x7083,
    // --> [Op1icscc_ib#7083]
    0x0194, 0x7083,
    // --> [RexOp1icscc_id#7081]
    0x019a, 0x7081,
    // --> [Op1icscc_id#7081] and stop
    0x0199, 0x7081,
    // end of icmp_imm.i32 (I64)
    // 0001f4: iconst.i32 (I64)
    // --> [RexOp1pu_id#b8]
    0x0036, 0x00b8,
    // --> [Op1pu_id#b8]
    0x0034, 0x00b8,
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#31]
    0x0042, 0x0031,
    // --> [Op1u_id_z#31] and stop
    0x0041, 0x0031,
    // end of iconst.i32 (I64)
    // 0001fd: ifcmp.i32 (I64)
    // --> [RexOp1rcmp#39]
    0x019e, 0x0039,
    // --> [Op1rcmp#39] and stop
    // 0001ff: ifcmp.i32 (I32)
    // --> [Op1rcmp#39] and stop
    0x019d, 0x0039,
    // end of ifcmp.i32 (I32)
    // end of ifcmp.i32 (I64)
    // 000201: ifcmp_imm.i32 (I64)
    // --> [RexOp1rcmp_ib#7083]
    0x01a2, 0x7083,
    // --> [Op1rcmp_ib#7083]
    0x01a0, 0x7083,
    // --> [RexOp1rcmp_id#7081]
    0x01a6, 0x7081,
    // --> [Op1rcmp_id#7081] and stop
    0x01a5, 0x7081,
    // end of ifcmp_imm.i32 (I64)
    // 000209: imul.i32 (I64)
    // --> [RexOp2rrx#4af]
    0x001a, 0x04af,
    // --> [Op2rrx#4af] and stop
    // 00020b: imul.i32 (I32)
    // --> [Op2rrx#4af] and stop
    0x0019, 0x04af,
    // end of imul.i32 (I32)
    // end of imul.i32 (I64)
    // 00020d: ireduce.i32 (I64)
    // stop unless inst_predicate_10
    0x100a,
    // --> [null#00] and stop
    0x01c1, 0x0000,
    // end of ireduce.i32 (I64)
    // 000210: ishl.i32 (I64)
    // --> [RexOp1rc#40d3]
    0x0046, 0x40d3,
    // --> [Op1rc#40d3] and stop
    // 000212: ishl.i32 (I32)
    // --> [Op1rc#40d3] and stop
    0x0045, 0x40d3,
    // end of ishl.i32 (I32)
    // end of ishl.i32 (I64)
    // 000214: ishl_imm.i32 (I64)
    // --> [RexOp1r_ib#40c1]
    0x002e, 0x40c1,
    // --> [Op1r_ib#40c1] and stop
    // 000216: ishl_imm.i32 (I32)
    // --> [Op1r_ib#40c1] and stop
    0x002d, 0x40c1,
    // end of ishl_imm.i32 (I32)
    // end of ishl_imm.i32 (I64)
    // 000218: isub.i32 (I64)
    // --> [RexOp1rr#29]
    0x0006, 0x0029,
    // --> [Op1rr#29] and stop
    // 00021a: isub.i32 (I32)
    // --> [Op1rr#29] and stop
    0x0005, 0x0029,
    // end of isub.i32 (I32)
    // end of isub.i32 (I64)
    // 00021c: isub_ifbin.i32 (I64)
    // --> [RexOp1rin#19]
    0x000e, 0x0019,
    // --> [Op1rin#19] and stop
    // 00021e: isub_ifbin.i32 (I32)
    // --> [Op1rin#19] and stop
    0x000d, 0x0019,
    // end of isub_ifbin.i32 (I32)
    // end of isub_ifbin.i32 (I64)
    // 000220: isub_ifborrow.i32 (I64)
    // --> [RexOp1rio#19]
    0x0012, 0x0019,
    // --> [Op1rio#19] and stop
    // 000222: isub_ifborrow.i32 (I32)
    // --> [Op1rio#19] and stop
    0x0011, 0x0019,
    // end of isub_ifborrow.i32 (I32)
    // end of isub_ifborrow.i32 (I64)
    // 000224: isub_ifbout.i32 (I64)
    // --> [RexOp1rout#29]
    0x000a, 0x0029,
    // --> [Op1rout#29] and stop
    // 000226: isub_ifbout.i32 (I32)
    // --> [Op1rout#29] and stop
    0x0009, 0x0029,
    // end of isub_ifbout.i32 (I32)
    // end of isub_ifbout.i32 (I64)
    // 000228: load_complex.i32 (I64)
    // stop unless inst_predicate_2
    // 000228: uload32_complex (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp1ldWithIndex#8b]
    // --> [RexOp1ldWithIndex#8b]
    0x004e, 0x008b,
    // --> [Op1ldWithIndex#8b]
    // --> [Op1ldWithIndex#8b]
    0x004c, 0x008b,
    // --> [RexOp1ldWithIndexDisp8#8b]
    // --> [RexOp1ldWithIndexDisp8#8b]
    0x0056, 0x008b,
    // --> [Op1ldWithIndexDisp8#8b]
    // --> [Op1ldWithIndexDisp8#8b]
    0x0054, 0x008b,
    // --> [RexOp1ldWithIndexDisp32#8b]
    // --> [RexOp1ldWithIndexDisp32#8b]
    0x005e, 0x008b,
    // --> [Op1ldWithIndexDisp32#8b] and stop
    // --> [Op1ldWithIndexDisp32#8b] and stop
    0x005d, 0x008b,
    // end of uload32_complex (I64)
    // end of load_complex.i32 (I64)
    // 000235: popcnt.i32 (I64)
    // stop unless PredicateView(15)
    0x102c,
    // --> [RexMp2urm#6b8]
    0x004a, 0x06b8,
    // --> [Mp2urm#6b8] and stop
    0x0049, 0x06b8,
    // end of popcnt.i32 (I64)
    // 00023a: regfill.i32 (I64)
    // --> [RexOp1regfill32#8b]
    // 00023a: regfill.b1 (I64)
    // --> [RexOp1regfill32#8b]
    // 00023a: regfill.i8 (I64)
    // --> [RexOp1regfill32#8b]
    // 00023a: regfill.i16 (I64)
    // --> [RexOp1regfill32#8b]
    0x00cc, 0x008b,
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // 00023c: regfill.i32 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00023c: regfill.b1 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00023c: regfill.r32 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00023c: regfill.i8 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00023c: regfill.i16 (I32)
    // --> [Op1regfill32#8b] and stop
    0x00cb, 0x008b,
    // end of regfill.i16 (I32)
    // end of regfill.i8 (I32)
    // end of regfill.r32 (I32)
    // end of regfill.b1 (I32)
    // end of regfill.i32 (I32)
    // end of regfill.i16 (I64)
    // end of regfill.i8 (I64)
    // end of regfill.b1 (I64)
    // end of regfill.i32 (I64)
    // 00023e: regmove.i32 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00023e: regmove.b32 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00023e: regmove.i16 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00023e: regmove.b8 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00023e: regmove.b16 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00023e: regmove.r32 (I64)
    // --> [RexOp1rmov#89] and stop
    0x002b, 0x0089,
    // end of regmove.r32 (I64)
    // end of regmove.b16 (I64)
    // end of regmove.b8 (I64)
    // end of regmove.i16 (I64)
    // end of regmove.b32 (I64)
    // end of regmove.i32 (I64)
    // 000240: regspill.i32 (I64)
    // --> [RexOp1regspill32#89]
    // 000240: regspill.b1 (I64)
    // --> [RexOp1regspill32#89]
    // 000240: regspill.i8 (I64)
    // --> [RexOp1regspill32#89]
    // 000240: regspill.i16 (I64)
    // --> [RexOp1regspill32#89]
    0x00ac, 0x0089,
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // 000242: regspill.i32 (I32)
    // --> [Op1regspill32#89] and stop
    // 000242: regspill.b1 (I32)
    // --> [Op1regspill32#89] and stop
    // 000242: regspill.r32 (I32)
    // --> [Op1regspill32#89] and stop
    // 000242: regspill.i8 (I32)
    // --> [Op1regspill32#89] and stop
    // 000242: regspill.i16 (I32)
    // --> [Op1regspill32#89] and stop
    0x00ab, 0x0089,
    // end of regspill.i16 (I32)
    // end of regspill.i8 (I32)
    // end of regspill.r32 (I32)
    // end of regspill.b1 (I32)
    // end of regspill.i32 (I32)
    // end of regspill.i16 (I64)
    // end of regspill.i8 (I64)
    // end of regspill.b1 (I64)
    // end of regspill.i32 (I64)
    // 000244: rotl.i32 (I64)
    // --> [RexOp1rc#d3]
    0x0046, 0x00d3,
    // --> [Op1rc#d3] and stop
    // 000246: rotl.i32 (I32)
    // --> [Op1rc#d3] and stop
    0x0045, 0x00d3,
    // end of rotl.i32 (I32)
    // end of rotl.i32 (I64)
    // 000248: rotl_imm.i32 (I64)
    // --> [RexOp1r_ib#c1]
    0x002e, 0x00c1,
    // --> [Op1r_ib#c1] and stop
    // 00024a: rotl_imm.i32 (I32)
    // --> [Op1r_ib#c1] and stop
    0x002d, 0x00c1,
    // end of rotl_imm.i32 (I32)
    // end of rotl_imm.i32 (I64)
    // 00024c: rotr.i32 (I64)
    // --> [RexOp1rc#10d3]
    0x0046, 0x10d3,
    // --> [Op1rc#10d3] and stop
    // 00024e: rotr.i32 (I32)
    // --> [Op1rc#10d3] and stop
    0x0045, 0x10d3,
    // end of rotr.i32 (I32)
    // end of rotr.i32 (I64)
    // 000250: rotr_imm.i32 (I64)
    // --> [RexOp1r_ib#10c1]
    0x002e, 0x10c1,
    // --> [Op1r_ib#10c1] and stop
    // 000252: rotr_imm.i32 (I32)
    // --> [Op1r_ib#10c1] and stop
    0x002d, 0x10c1,
    // end of rotr_imm.i32 (I32)
    // end of rotr_imm.i32 (I64)
    // 000254: selectif.i32 (I64)
    // --> [RexOp2cmov#440]
    0x01b6, 0x0440,
    // --> [Op2cmov#440] and stop
    // 000256: selectif.i32 (I32)
    // --> [Op2cmov#440] and stop
    0x01b5, 0x0440,
    // end of selectif.i32 (I32)
    // end of selectif.i32 (I64)
    // 000258: sextend.i32 (I64)
    // skip 4 unless inst_predicate_11
    0x500b,
    // --> [RexOp2urm_noflags#4be]
    0x01be, 0x04be,
    // --> [Op2urm_noflags_abcd#4be]
    0x01bc, 0x04be,
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2urm_noflags#4bf]
    0x01be, 0x04bf,
    // --> [Op2urm_noflags#4bf] and stop
    0x01c3, 0x04bf,
    // end of sextend.i32 (I64)
    // 000262: sload16.i32 (I64)
    // --> [RexOp2ld#4bf]
    0x00b4, 0x04bf,
    // --> [Op2ld#4bf]
    0x00b2, 0x04bf,
    // --> [RexOp2ldDisp8#4bf]
    0x00bc, 0x04bf,
    // --> [Op2ldDisp8#4bf]
    0x00ba, 0x04bf,
    // --> [RexOp2ldDisp32#4bf]
    0x00c4, 0x04bf,
    // --> [Op2ldDisp32#4bf] and stop
    0x00c3, 0x04bf,
    // end of sload16.i32 (I64)
    // 00026e: sload16_complex.i32 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#4bf]
    0x0052, 0x04bf,
    // --> [Op2ldWithIndex#4bf]
    0x0050, 0x04bf,
    // --> [RexOp2ldWithIndexDisp8#4bf]
    0x005a, 0x04bf,
    // --> [Op2ldWithIndexDisp8#4bf]
    0x0058, 0x04bf,
    // --> [RexOp2ldWithIndexDisp32#4bf]
    0x0062, 0x04bf,
    // --> [Op2ldWithIndexDisp32#4bf] and stop
    0x0061, 0x04bf,
    // end of sload16_complex.i32 (I64)
    // 00027b: sload8.i32 (I64)
    // --> [RexOp2ld#4be]
    0x00b4, 0x04be,
    // --> [Op2ld#4be]
    0x00b2, 0x04be,
    // --> [RexOp2ldDisp8#4be]
    0x00bc, 0x04be,
    // --> [Op2ldDisp8#4be]
    0x00ba, 0x04be,
    // --> [RexOp2ldDisp32#4be]
    0x00c4, 0x04be,
    // --> [Op2ldDisp32#4be] and stop
    0x00c3, 0x04be,
    // end of sload8.i32 (I64)
    // 000287: sload8_complex.i32 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#4be]
    0x0052, 0x04be,
    // --> [Op2ldWithIndex#4be]
    0x0050, 0x04be,
    // --> [RexOp2ldWithIndexDisp8#4be]
    0x005a, 0x04be,
    // --> [Op2ldWithIndexDisp8#4be]
    0x0058, 0x04be,
    // --> [RexOp2ldWithIndexDisp32#4be]
    0x0062, 0x04be,
    // --> [Op2ldWithIndexDisp32#4be] and stop
    0x0061, 0x04be,
    // end of sload8_complex.i32 (I64)
    // 000294: spill.i32 (I64)
    // --> [RexOp1spillSib32#89]
    // 000294: spill.b1 (I64)
    // --> [RexOp1spillSib32#89]
    // 000294: spill.i8 (I64)
    // --> [RexOp1spillSib32#89]
    // 000294: spill.i16 (I64)
    // --> [RexOp1spillSib32#89]
    0x00a8, 0x0089,
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // 000296: spill.i32 (I32)
    // --> [Op1spillSib32#89] and stop
    // 000296: spill.b1 (I32)
    // --> [Op1spillSib32#89] and stop
    // 000296: spill.r32 (I32)
    // --> [Op1spillSib32#89] and stop
    // 000296: spill.i8 (I32)
    // --> [Op1spillSib32#89] and stop
    // 000296: spill.i16 (I32)
    // --> [Op1spillSib32#89] and stop
    0x00a7, 0x0089,
    // end of spill.i16 (I32)
    // end of spill.i8 (I32)
    // end of spill.r32 (I32)
    // end of spill.b1 (I32)
    // end of spill.i32 (I32)
    // end of spill.i16 (I64)
    // end of spill.i8 (I64)
    // end of spill.b1 (I64)
    // end of spill.i32 (I64)
    // 000298: sshr.i32 (I64)
    // --> [RexOp1rc#70d3]
    0x0046, 0x70d3,
    // --> [Op1rc#70d3] and stop
    // 00029a: sshr.i32 (I32)
    // --> [Op1rc#70d3] and stop
    0x0045, 0x70d3,
    // end of sshr.i32 (I32)
    // end of sshr.i32 (I64)
    // 00029c: sshr_imm.i32 (I64)
    // --> [RexOp1r_ib#70c1]
    0x002e, 0x70c1,
    // --> [Op1r_ib#70c1] and stop
    // 00029e: sshr_imm.i32 (I32)
    // --> [Op1r_ib#70c1] and stop
    0x002d, 0x70c1,
    // end of sshr_imm.i32 (I32)
    // end of sshr_imm.i32 (I64)
    // 0002a0: store_complex.i32 (I64)
    // stop unless inst_predicate_3
    // 0002a0: istore32_complex (I64)
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexOp1stWithIndex#89]
    // --> [RexOp1stWithIndex#89]
    0x0066, 0x0089,
    // --> [Op1stWithIndex#89]
    // --> [Op1stWithIndex#89]
    0x0064, 0x0089,
    // --> [RexOp1stWithIndexDisp8#89]
    // --> [RexOp1stWithIndexDisp8#89]
    0x006e, 0x0089,
    // --> [Op1stWithIndexDisp8#89]
    // --> [Op1stWithIndexDisp8#89]
    0x006c, 0x0089,
    // --> [RexOp1stWithIndexDisp32#89]
    // --> [RexOp1stWithIndexDisp32#89]
    0x0076, 0x0089,
    // --> [Op1stWithIndexDisp32#89] and stop
    // --> [Op1stWithIndexDisp32#89] and stop
    0x0075, 0x0089,
    // end of istore32_complex (I64)
    // end of store_complex.i32 (I64)
    // 0002ad: uextend.i32 (I64)
    // skip 4 unless inst_predicate_11
    0x500b,
    // --> [RexOp2urm_noflags#4b6]
    0x01be, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x01bc, 0x04b6,
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2urm_noflags#4b7]
    0x01be, 0x04b7,
    // --> [Op2urm_noflags#4b7] and stop
    0x01c3, 0x04b7,
    // end of uextend.i32 (I64)
    // 0002b7: uload16.i32 (I64)
    // --> [RexOp2ld#4b7]
    0x00b4, 0x04b7,
    // --> [Op2ld#4b7]
    0x00b2, 0x04b7,
    // --> [RexOp2ldDisp8#4b7]
    0x00bc, 0x04b7,
    // --> [Op2ldDisp8#4b7]
    0x00ba, 0x04b7,
    // --> [RexOp2ldDisp32#4b7]
    0x00c4, 0x04b7,
    // --> [Op2ldDisp32#4b7] and stop
    0x00c3, 0x04b7,
    // end of uload16.i32 (I64)
    // 0002c3: uload16_complex.i32 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#4b7]
    0x0052, 0x04b7,
    // --> [Op2ldWithIndex#4b7]
    0x0050, 0x04b7,
    // --> [RexOp2ldWithIndexDisp8#4b7]
    0x005a, 0x04b7,
    // --> [Op2ldWithIndexDisp8#4b7]
    0x0058, 0x04b7,
    // --> [RexOp2ldWithIndexDisp32#4b7]
    0x0062, 0x04b7,
    // --> [Op2ldWithIndexDisp32#4b7] and stop
    0x0061, 0x04b7,
    // end of uload16_complex.i32 (I64)
    // 0002d0: uload8.i32 (I64)
    // --> [RexOp2ld#4b6]
    0x00b4, 0x04b6,
    // --> [Op2ld#4b6]
    0x00b2, 0x04b6,
    // --> [RexOp2ldDisp8#4b6]
    0x00bc, 0x04b6,
    // --> [Op2ldDisp8#4b6]
    0x00ba, 0x04b6,
    // --> [RexOp2ldDisp32#4b6]
    0x00c4, 0x04b6,
    // --> [Op2ldDisp32#4b6] and stop
    0x00c3, 0x04b6,
    // end of uload8.i32 (I64)
    // 0002dc: uload8_complex.i32 (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2ldWithIndex#4b6]
    0x0052, 0x04b6,
    // --> [Op2ldWithIndex#4b6]
    0x0050, 0x04b6,
    // --> [RexOp2ldWithIndexDisp8#4b6]
    0x005a, 0x04b6,
    // --> [Op2ldWithIndexDisp8#4b6]
    0x0058, 0x04b6,
    // --> [RexOp2ldWithIndexDisp32#4b6]
    0x0062, 0x04b6,
    // --> [Op2ldWithIndexDisp32#4b6] and stop
    0x0061, 0x04b6,
    // end of uload8_complex.i32 (I64)
    // 0002e9: ushr.i32 (I64)
    // --> [RexOp1rc#50d3]
    0x0046, 0x50d3,
    // --> [Op1rc#50d3] and stop
    // 0002eb: ushr.i32 (I32)
    // --> [Op1rc#50d3] and stop
    0x0045, 0x50d3,
    // end of ushr.i32 (I32)
    // end of ushr.i32 (I64)
    // 0002ed: ushr_imm.i32 (I64)
    // --> [RexOp1r_ib#50c1]
    0x002e, 0x50c1,
    // --> [Op1r_ib#50c1] and stop
    // 0002ef: ushr_imm.i32 (I32)
    // --> [Op1r_ib#50c1] and stop
    0x002d, 0x50c1,
    // end of ushr_imm.i32 (I32)
    // end of ushr_imm.i32 (I64)
    // 0002f1: x86_bsf.i32 (I64)
    // --> [RexOp2bsf_and_bsr#4bc]
    0x01ba, 0x04bc,
    // --> [Op2bsf_and_bsr#4bc] and stop
    // 0002f3: x86_bsf.i32 (I32)
    // --> [Op2bsf_and_bsr#4bc] and stop
    0x01b9, 0x04bc,
    // end of x86_bsf.i32 (I32)
    // end of x86_bsf.i32 (I64)
    // 0002f5: x86_bsr.i32 (I64)
    // --> [RexOp2bsf_and_bsr#4bd]
    0x01ba, 0x04bd,
    // --> [Op2bsf_and_bsr#4bd] and stop
    // 0002f7: x86_bsr.i32 (I32)
    // --> [Op2bsf_and_bsr#4bd] and stop
    0x01b9, 0x04bd,
    // end of x86_bsr.i32 (I32)
    // end of x86_bsr.i32 (I64)
    // 0002f9: x86_cvtt2si.i32 (I64)
    // skip 4 unless inst_predicate_14
    0x500e,
    // --> [RexMp2rfurm#62c]
    0x01e4, 0x062c,
    // --> [Mp2rfurm#62c]
    0x01e2, 0x062c,
    // stop unless inst_predicate_15
    0x100f,
    // --> [RexMp2rfurm#72c]
    0x01e4, 0x072c,
    // --> [Mp2rfurm#72c] and stop
    0x01e3, 0x072c,
    // end of x86_cvtt2si.i32 (I64)
    // 000303: x86_sdivmodx.i32 (I64)
    // --> [RexOp1div#70f7]
    0x001e, 0x70f7,
    // --> [Op1div#70f7] and stop
    // 000305: x86_sdivmodx.i32 (I32)
    // --> [Op1div#70f7] and stop
    0x001d, 0x70f7,
    // end of x86_sdivmodx.i32 (I32)
    // end of x86_sdivmodx.i32 (I64)
    // 000307: x86_smulx.i32 (I64)
    // --> [RexOp1mulx#50f7]
    0x0022, 0x50f7,
    // --> [Op1mulx#50f7] and stop
    // 000309: x86_smulx.i32 (I32)
    // --> [Op1mulx#50f7] and stop
    0x0021, 0x50f7,
    // end of x86_smulx.i32 (I32)
    // end of x86_smulx.i32 (I64)
    // 00030b: x86_udivmodx.i32 (I64)
    // --> [RexOp1div#60f7]
    0x001e, 0x60f7,
    // --> [Op1div#60f7] and stop
    // 00030d: x86_udivmodx.i32 (I32)
    // --> [Op1div#60f7] and stop
    0x001d, 0x60f7,
    // end of x86_udivmodx.i32 (I32)
    // end of x86_udivmodx.i32 (I64)
    // 00030f: x86_umulx.i32 (I64)
    // --> [RexOp1mulx#40f7]
    0x0022, 0x40f7,
    // --> [Op1mulx#40f7] and stop
    // 000311: x86_umulx.i32 (I32)
    // --> [Op1mulx#40f7] and stop
    0x0021, 0x40f7,
    // end of x86_umulx.i32 (I32)
    // end of x86_umulx.i32 (I64)
    // 000313: bconst.b32 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 000313: bconst.b1 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 000313: bconst.b8 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 000313: bconst.b16 (I64)
    // --> [RexOp1pu_id_bool#b8]
    0x003e, 0x00b8,
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // 000315: bconst.b32 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 000315: bconst.b1 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 000315: bconst.b8 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 000315: bconst.b16 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    0x003d, 0x00b8,
    // end of bconst.b16 (I32)
    // end of bconst.b8 (I32)
    // end of bconst.b1 (I32)
    // end of bconst.b32 (I32)
    // end of bconst.b16 (I64)
    // end of bconst.b8 (I64)
    // end of bconst.b1 (I64)
    // end of bconst.b32 (I64)
    // 000317: bconst.b64 (I64)
    // --> [RexOp1pu_id_bool#b8] and stop
    0x003f, 0x00b8,
    // end of bconst.b64 (I64)
    // 000319: brnz.b1 (I64)
    // --> [RexOp1t8jccb#75]
    0x0176, 0x0075,
    // --> [Op1t8jccb_abcd#75]
    0x0174, 0x0075,
    // --> [RexOp1t8jccd#85]
    0x017a, 0x0085,
    // --> [Op1t8jccd_abcd#85] and stop
    0x0179, 0x0085,
    // end of brnz.b1 (I64)
    // 000321: brz.b1 (I64)
    // --> [RexOp1t8jccb#74]
    0x0176, 0x0074,
    // --> [Op1t8jccb_abcd#74]
    0x0174, 0x0074,
    // --> [RexOp1t8jccd#84]
    0x017a, 0x0084,
    // --> [Op1t8jccd_abcd#84] and stop
    0x0179, 0x0084,
    // end of brz.b1 (I64)
    // 000329: regmove.b1 (I64)
    // --> [RexOp1rmov#89]
    0x002a, 0x0089,
    // --> [Op1rmov#89] and stop
    // 00032b: regmove.i32 (I32)
    // --> [Op1rmov#89] and stop
    // 00032b: regmove.b32 (I32)
    // --> [Op1rmov#89] and stop
    // 00032b: regmove.b1 (I32)
    // --> [Op1rmov#89] and stop
    // 00032b: regmove.r32 (I32)
    // --> [Op1rmov#89] and stop
    // 00032b: regmove.i16 (I32)
    // --> [Op1rmov#89] and stop
    // 00032b: regmove.b8 (I32)
    // --> [Op1rmov#89] and stop
    // 00032b: regmove.b16 (I32)
    // --> [Op1rmov#89] and stop
    0x0029, 0x0089,
    // end of regmove.b16 (I32)
    // end of regmove.b8 (I32)
    // end of regmove.i16 (I32)
    // end of regmove.r32 (I32)
    // end of regmove.b1 (I32)
    // end of regmove.b32 (I32)
    // end of regmove.i32 (I32)
    // end of regmove.b1 (I64)
    // 00032d: is_null.r64 (I64)
    // --> [RexOp1is_zero#8085] and stop
    0x023b, 0x8085,
    // end of is_null.r64 (I64)
    // 00032f: null.r64 (I64)
    // --> [RexOp1pu_id_ref#b8]
    0x0236, 0x00b8,
    // --> [Op1pu_id_ref#b8] and stop
    // 000331: null.r32 (I32)
    // --> [Op1pu_id_ref#b8] and stop
    0x0235, 0x00b8,
    // end of null.r32 (I32)
    // end of null.r64 (I64)
    // 000333: iconst.i8 (I64)
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#30]
    0x0042, 0x0030,
    // --> [Op1u_id_z#30] and stop
    0x0041, 0x0030,
    // end of iconst.i8 (I64)
    // 000338: ireduce.i8 (I64)
    // skip 2 unless inst_predicate_8
    0x3008,
    // --> [null#00]
    0x01c0, 0x0000,
    // skip 2 unless inst_predicate_9
    // 00033b: ireduce.i16 (I64)
    // skip 2 unless inst_predicate_9
    0x3009,
    // --> [null#00]
    // --> [null#00]
    0x01c0, 0x0000,
    // stop unless inst_predicate_10
    // stop unless inst_predicate_10
    0x100a,
    // --> [null#00] and stop
    // --> [null#00] and stop
    0x01c1, 0x0000,
    // end of ireduce.i16 (I64)
    // end of ireduce.i8 (I64)
    // 000341: regmove.i8 (I64)
    // --> [RexOp1rmov#89]
    0x002a, 0x0089,
    // --> [RexOp1rmov#89]
    0x002a, 0x0089,
    // --> [Op1rmov#89] and stop
    0x0029, 0x0089,
    // end of regmove.i8 (I64)
    // 000347: adjust_sp_down_imm (I64)
    // --> [RexOp1adjustsp_ib#d083]
    0x00f0, 0xd083,
    // --> [RexOp1adjustsp_id#d081] and stop
    0x00f3, 0xd081,
    // end of adjust_sp_down_imm (I64)
    // 00034b: adjust_sp_up_imm (I64)
    // --> [RexOp1adjustsp_ib#8083]
    0x00f0, 0x8083,
    // --> [RexOp1adjustsp_id#8081] and stop
    0x00f3, 0x8081,
    // end of adjust_sp_up_imm (I64)
    // 00034f: brff (I64)
    // --> [RexOp1brfb#70]
    0x0164, 0x0070,
    // --> [Op1brfb#70]
    0x0162, 0x0070,
    // --> [RexOp2brfd#480]
    0x0168, 0x0480,
    // --> [Op2brfd#480] and stop
    0x0167, 0x0480,
    // end of brff (I64)
    // 000357: brif (I64)
    // --> [RexOp1brib#70]
    0x015c, 0x0070,
    // --> [Op1brib#70]
    0x015a, 0x0070,
    // --> [RexOp2brid#480]
    0x0160, 0x0480,
    // --> [Op2brid#480] and stop
    0x015f, 0x0480,
    // end of brif (I64)
    // 00035f: call (I64)
    // skip 2 unless inst_predicate_6
    0x3006,
    // --> [Op1call_id#e8]
    0x014c, 0x00e8,
    // stop unless PredicateView(10)
    0x1027,
    // --> [Op1call_plt_id#e8] and stop
    0x014f, 0x00e8,
    // end of call (I64)
    // 000365: copy_special (I64)
    // --> [RexOp1copysp#8089] and stop
    0x00db, 0x8089,
    // end of copy_special (I64)
    // 000367: debugtrap (I64)
    // --> [debugtrap#00] and stop
    // 000367: debugtrap (I32)
    // --> [debugtrap#00] and stop
    0x018b, 0x0000,
    // end of debugtrap (I32)
    // end of debugtrap (I64)
    // 000369: f32const (I64)
    // stop unless inst_predicate_12
    0x100c,
    // --> [RexOp2f32imm_z#457]
    0x01ca, 0x0457,
    // --> [Op2f32imm_z#457] and stop
    0x01c7, 0x0457,
    // end of f32const (I64)
    // 00036e: f64const (I64)
    // stop unless inst_predicate_13
    0x100d,
    // --> [RexMp2f64imm_z#557]
    0x01cc, 0x0557,
    // --> [Mp2f64imm_z#557] and stop
    0x01c9, 0x0557,
    // end of f64const (I64)
    // 000373: jump (I64)
    // --> [Op1jmpb#eb]
    // 000373: jump (I32)
    // --> [Op1jmpb#eb]
    0x0156, 0x00eb,
    // --> [Op1jmpd#e9] and stop
    // --> [Op1jmpd#e9] and stop
    0x0159, 0x00e9,
    // end of jump (I32)
    // end of jump (I64)
    // 000377: resumable_trap (I64)
    // --> [Op2trap#40b] and stop
    // 000377: trap (I64)
    // --> [Op2trap#40b] and stop
    // 000377: resumable_trap (I32)
    // --> [Op2trap#40b] and stop
    // 000377: trap (I32)
    // --> [Op2trap#40b] and stop
    0x0189, 0x040b,
    // end of trap (I32)
    // end of resumable_trap (I32)
    // end of trap (I64)
    // end of resumable_trap (I64)
    // 000379: return (I64)
    // --> [Op1ret#c3] and stop
    // 000379: return (I32)
    // --> [Op1ret#c3] and stop
    0x0155, 0x00c3,
    // end of return (I32)
    // end of return (I64)
    // 00037b: safepoint (I64)
    // --> [safepoint#00] and stop
    // 00037b: safepoint (I32)
    // --> [safepoint#00] and stop
    0x023d, 0x0000,
    // end of safepoint (I32)
    // end of safepoint (I64)
    // 00037d: sload32_complex (I64)
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp1ldWithIndex#8063]
    0x004e, 0x8063,
    // --> [RexOp1ldWithIndexDisp8#8063]
    0x0056, 0x8063,
    // --> [RexOp1ldWithIndexDisp32#8063] and stop
    0x005f, 0x8063,
    // end of sload32_complex (I64)
    // 000384: trapff (I64)
    // --> [trapff#00] and stop
    // 000384: trapff (I32)
    // --> [trapff#00] and stop
    0x018f, 0x0000,
    // end of trapff (I32)
    // end of trapff (I64)
    // 000386: trapif (I64)
    // --> [trapif#00] and stop
    // 000386: trapif (I32)
    // --> [trapif#00] and stop
    0x018d, 0x0000,
    // end of trapif (I32)
    // end of trapif (I64)
    // 000388: trueff (I64)
    // --> [RexOp2setf#490]
    0x01b2, 0x0490,
    // --> [Op2setf_abcd#490] and stop
    // 00038a: trueff (I32)
    // --> [Op2setf_abcd#490] and stop
    0x01b1, 0x0490,
    // end of trueff (I32)
    // end of trueff (I64)
    // 00038c: trueif (I64)
    // --> [RexOp2seti#490]
    0x01ae, 0x0490,
    // --> [Op2seti_abcd#490] and stop
    // 00038e: trueif (I32)
    // --> [Op2seti_abcd#490] and stop
    0x01ad, 0x0490,
    // end of trueif (I32)
    // end of trueif (I64)
    // 000390: band.f64 (I64)
    // --> [RexOp2fa#454]
    // 000390: band.f32 (I64)
    // --> [RexOp2fa#454]
    0x01f0, 0x0454,
    // --> [Op2fa#454] and stop
    // --> [Op2fa#454] and stop
    // 000392: band.f64 (I32)
    // --> [Op2fa#454] and stop
    // 000392: band.f32 (I32)
    // --> [Op2fa#454] and stop
    0x01ef, 0x0454,
    // end of band.f32 (I32)
    // end of band.f64 (I32)
    // end of band.f32 (I64)
    // end of band.f64 (I64)
    // 000394: band_not.f64 (I64)
    // --> [RexOp2fax#455]
    // 000394: band_not.f32 (I64)
    // --> [RexOp2fax#455]
    0x01f4, 0x0455,
    // --> [Op2fax#455] and stop
    // --> [Op2fax#455] and stop
    // 000396: band_not.f64 (I32)
    // --> [Op2fax#455] and stop
    // 000396: band_not.f32 (I32)
    // --> [Op2fax#455] and stop
    0x01f3, 0x0455,
    // end of band_not.f32 (I32)
    // end of band_not.f64 (I32)
    // end of band_not.f32 (I64)
    // end of band_not.f64 (I64)
    // 000398: bitcast.f64 (I64)
    // stop unless inst_predicate_10
    0x100a,
    // --> [RexMp2frurm#856e] and stop
    0x01d1, 0x856e,
    // end of bitcast.f64 (I64)
    // 00039b: bor.f64 (I64)
    // --> [RexOp2fa#456]
    // 00039b: bor.f32 (I64)
    // --> [RexOp2fa#456]
    0x01f0, 0x0456,
    // --> [Op2fa#456] and stop
    // --> [Op2fa#456] and stop
    // 00039d: bor.f64 (I32)
    // --> [Op2fa#456] and stop
    // 00039d: bor.f32 (I32)
    // --> [Op2fa#456] and stop
    0x01ef, 0x0456,
    // end of bor.f32 (I32)
    // end of bor.f64 (I32)
    // end of bor.f32 (I64)
    // end of bor.f64 (I64)
    // 00039f: bxor.f64 (I64)
    // --> [RexOp2fa#457]
    // 00039f: bxor.f32 (I64)
    // --> [RexOp2fa#457]
    0x01f0, 0x0457,
    // --> [Op2fa#457] and stop
    // --> [Op2fa#457] and stop
    // 0003a1: bxor.f64 (I32)
    // --> [Op2fa#457] and stop
    // 0003a1: bxor.f32 (I32)
    // --> [Op2fa#457] and stop
    0x01ef, 0x0457,
    // end of bxor.f32 (I32)
    // end of bxor.f64 (I32)
    // end of bxor.f32 (I64)
    // end of bxor.f64 (I64)
    // 0003a3: ceil.f64 (I64)
    // stop unless PredicateView(16)
    // 0003a3: floor.f64 (I64)
    // stop unless PredicateView(16)
    // 0003a3: nearest.f64 (I64)
    // stop unless PredicateView(16)
    // 0003a3: trunc.f64 (I64)
    // stop unless PredicateView(16)
    0x102d,
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    0x01e8, 0x0d0b,
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    0x01e7, 0x0d0b,
    // end of trunc.f64 (I64)
    // end of nearest.f64 (I64)
    // end of floor.f64 (I64)
    // end of ceil.f64 (I64)
    // 0003a8: copy.f64 (I64)
    // --> [RexOp2furm#428]
    // 0003a8: copy.f32 (I64)
    // --> [RexOp2furm#428]
    0x01d8, 0x0428,
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b8x16 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b16x8 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b32x4 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b64x2 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i8x16 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i16x8 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i32x4 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i64x2 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.f32x4 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.f64x2 (I64)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.f64 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.f32 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b8x16 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b16x8 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b32x4 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.b64x2 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i8x16 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i16x8 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i32x4 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.i64x2 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.f32x4 (I32)
    // --> [Op2furm#428] and stop
    // 0003aa: copy.f64x2 (I32)
    // --> [Op2furm#428] and stop
    0x01d7, 0x0428,
    // end of copy.f64x2 (I32)
    // end of copy.f32x4 (I32)
    // end of copy.i64x2 (I32)
    // end of copy.i32x4 (I32)
    // end of copy.i16x8 (I32)
    // end of copy.i8x16 (I32)
    // end of copy.b64x2 (I32)
    // end of copy.b32x4 (I32)
    // end of copy.b16x8 (I32)
    // end of copy.b8x16 (I32)
    // end of copy.f32 (I32)
    // end of copy.f64 (I32)
    // end of copy.f64x2 (I64)
    // end of copy.f32x4 (I64)
    // end of copy.i64x2 (I64)
    // end of copy.i32x4 (I64)
    // end of copy.i16x8 (I64)
    // end of copy.i8x16 (I64)
    // end of copy.b64x2 (I64)
    // end of copy.b32x4 (I64)
    // end of copy.b16x8 (I64)
    // end of copy.b8x16 (I64)
    // end of copy.f32 (I64)
    // end of copy.f64 (I64)
    // 0003ac: copy_to_ssa.f64 (I64)
    // --> [RexMp2furm_reg_to_ssa#710] and stop
    0x00e5, 0x0710,
    // end of copy_to_ssa.f64 (I64)
    // 0003ae: fadd.f64 (I64)
    // --> [RexMp2fa#758]
    0x01ec, 0x0758,
    // --> [Mp2fa#758] and stop
    // 0003b0: fadd.f64 (I32)
    // --> [Mp2fa#758] and stop
    0x01eb, 0x0758,
    // end of fadd.f64 (I32)
    // end of fadd.f64 (I64)
    // 0003b2: fcmp.f64 (I64)
    // --> [RexMp2fcscc#52e]
    0x01fc, 0x052e,
    // --> [Mp2fcscc#52e] and stop
    // 0003b4: fcmp.f64 (I32)
    // --> [Mp2fcscc#52e] and stop
    0x01fb, 0x052e,
    // end of fcmp.f64 (I32)
    // end of fcmp.f64 (I64)
    // 0003b6: fcvt_from_sint.f64 (I64)
    // skip 4 unless inst_predicate_9
    0x5009,
    // --> [RexMp2frurm#72a]
    0x01d0, 0x072a,
    // --> [Mp2frurm#72a]
    0x01ce, 0x072a,
    // stop unless inst_predicate_10
    0x100a,
    // --> [RexMp2frurm#872a] and stop
    0x01d1, 0x872a,
    // end of fcvt_from_sint.f64 (I64)
    // 0003be: fdiv.f64 (I64)
    // --> [RexMp2fa#75e]
    0x01ec, 0x075e,
    // --> [Mp2fa#75e] and stop
    // 0003c0: fdiv.f64 (I32)
    // --> [Mp2fa#75e] and stop
    0x01eb, 0x075e,
    // end of fdiv.f64 (I32)
    // end of fdiv.f64 (I64)
    // 0003c2: ffcmp.f64 (I64)
    // --> [RexMp2fcmp#52e]
    0x0204, 0x052e,
    // --> [Mp2fcmp#52e] and stop
    // 0003c4: ffcmp.f64 (I32)
    // --> [Mp2fcmp#52e] and stop
    0x0203, 0x052e,
    // end of ffcmp.f64 (I32)
    // end of ffcmp.f64 (I64)
    // 0003c6: fill.f64 (I64)
    // --> [RexMp2ffillSib32#710]
    0x0126, 0x0710,
    // --> [Mp2ffillSib32#710] and stop
    // 0003c8: fill.f64 (I32)
    // --> [Mp2ffillSib32#710] and stop
    0x0125, 0x0710,
    // end of fill.f64 (I32)
    // end of fill.f64 (I64)
    // 0003ca: fill_nop.f64 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.f32 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b8x16 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b16x8 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b32x4 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b64x2 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i8x16 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i16x8 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i32x4 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i64x2 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.f32x4 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.f64x2 (I64)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.f64 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.f32 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b8x16 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b16x8 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b32x4 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.b64x2 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i8x16 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i16x8 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i32x4 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.i64x2 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.f32x4 (I32)
    // --> [ffillnull#00] and stop
    // 0003ca: fill_nop.f64x2 (I32)
    // --> [ffillnull#00] and stop
    0x00d1, 0x0000,
    // end of fill_nop.f64x2 (I32)
    // end of fill_nop.f32x4 (I32)
    // end of fill_nop.i64x2 (I32)
    // end of fill_nop.i32x4 (I32)
    // end of fill_nop.i16x8 (I32)
    // end of fill_nop.i8x16 (I32)
    // end of fill_nop.b64x2 (I32)
    // end of fill_nop.b32x4 (I32)
    // end of fill_nop.b16x8 (I32)
    // end of fill_nop.b8x16 (I32)
    // end of fill_nop.f32 (I32)
    // end of fill_nop.f64 (I32)
    // end of fill_nop.f64x2 (I64)
    // end of fill_nop.f32x4 (I64)
    // end of fill_nop.i64x2 (I64)
    // end of fill_nop.i32x4 (I64)
    // end of fill_nop.i16x8 (I64)
    // end of fill_nop.i8x16 (I64)
    // end of fill_nop.b64x2 (I64)
    // end of fill_nop.b32x4 (I64)
    // end of fill_nop.b16x8 (I64)
    // end of fill_nop.b8x16 (I64)
    // end of fill_nop.f32 (I64)
    // end of fill_nop.f64 (I64)
    // 0003cc: fmul.f64 (I64)
    // --> [RexMp2fa#759]
    0x01ec, 0x0759,
    // --> [Mp2fa#759] and stop
    // 0003ce: fmul.f64 (I32)
    // --> [Mp2fa#759] and stop
    0x01eb, 0x0759,
    // end of fmul.f64 (I32)
    // end of fmul.f64 (I64)
    // 0003d0: fpromote.f64 (I64)
    // stop unless inst_predicate_14
    0x100e,
    // --> [RexMp2furm#65a]
    0x01e0, 0x065a,
    // --> [Mp2furm#65a] and stop
    0x01df, 0x065a,
    // end of fpromote.f64 (I64)
    // 0003d5: fsub.f64 (I64)
    // --> [RexMp2fa#75c]
    0x01ec, 0x075c,
    // --> [Mp2fa#75c] and stop
    // 0003d7: fsub.f64 (I32)
    // --> [Mp2fa#75c] and stop
    0x01eb, 0x075c,
    // end of fsub.f64 (I32)
    // end of fsub.f64 (I64)
    // 0003d9: load.f64 (I64)
    // --> [RexMp2fld#710]
    0x00f6, 0x0710,
    // --> [Mp2fld#710]
    0x00f4, 0x0710,
    // --> [RexMp2fldDisp8#710]
    0x00fa, 0x0710,
    // --> [Mp2fldDisp8#710]
    0x00f8, 0x0710,
    // --> [RexMp2fldDisp32#710]
    0x00fe, 0x0710,
    // --> [Mp2fldDisp32#710] and stop
    0x00fd, 0x0710,
    // end of load.f64 (I64)
    // 0003e5: load_complex.f64 (I64)
    // --> [RexMp2fldWithIndex#710]
    0x0102, 0x0710,
    // --> [Mp2fldWithIndex#710]
    0x0100, 0x0710,
    // --> [RexMp2fldWithIndexDisp8#710]
    0x0106, 0x0710,
    // --> [Mp2fldWithIndexDisp8#710]
    0x0104, 0x0710,
    // --> [RexMp2fldWithIndexDisp32#710]
    0x010a, 0x0710,
    // --> [Mp2fldWithIndexDisp32#710] and stop
    0x0109, 0x0710,
    // end of load_complex.f64 (I64)
    // 0003f1: raw_bitcast.f64 (I64)
    // skip 2 unless inst_predicate_16
    // 0003f1: raw_bitcast.f32 (I64)
    // skip 2 unless inst_predicate_16
    // 0003f1: raw_bitcast.f64 (I32)
    // skip 2 unless inst_predicate_16
    // 0003f1: raw_bitcast.f32 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_25
    // stop unless inst_predicate_25
    // stop unless inst_predicate_25
    // stop unless inst_predicate_25
    0x1019,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // 00040d: scalar_to_vector.f32x4 (I64)
    // --> [null_fpr#00] and stop
    // 00040d: scalar_to_vector.f64x2 (I64)
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // 00040d: scalar_to_vector.f32x4 (I32)
    // --> [null_fpr#00] and stop
    // 00040d: scalar_to_vector.f64x2 (I32)
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of scalar_to_vector.f64x2 (I32)
    // end of scalar_to_vector.f32x4 (I32)
    // end of raw_bitcast.f32 (I32)
    // end of raw_bitcast.f64 (I32)
    // end of scalar_to_vector.f64x2 (I64)
    // end of scalar_to_vector.f32x4 (I64)
    // end of raw_bitcast.f32 (I64)
    // end of raw_bitcast.f64 (I64)
    // 00040f: regfill.f64 (I64)
    // --> [RexMp2fregfill32#710]
    0x012a, 0x0710,
    // --> [Mp2fregfill32#710] and stop
    // 000411: regfill.f64 (I32)
    // --> [Mp2fregfill32#710] and stop
    0x0129, 0x0710,
    // end of regfill.f64 (I32)
    // end of regfill.f64 (I64)
    // 000413: regmove.f64 (I64)
    // --> [RexOp2frmov#428] and stop
    // 000413: regmove.f32 (I64)
    // --> [RexOp2frmov#428] and stop
    0x01dd, 0x0428,
    // end of regmove.f32 (I64)
    // end of regmove.f64 (I64)
    // 000415: regspill.f64 (I64)
    // --> [RexMp2fregspill32#711]
    0x0132, 0x0711,
    // --> [Mp2fregspill32#711] and stop
    // 000417: regspill.f64 (I32)
    // --> [Mp2fregspill32#711] and stop
    0x0131, 0x0711,
    // end of regspill.f64 (I32)
    // end of regspill.f64 (I64)
    // 000419: spill.f64 (I64)
    // --> [RexMp2fspillSib32#711]
    0x012e, 0x0711,
    // --> [Mp2fspillSib32#711] and stop
    // 00041b: spill.f64 (I32)
    // --> [Mp2fspillSib32#711] and stop
    0x012d, 0x0711,
    // end of spill.f64 (I32)
    // end of spill.f64 (I64)
    // 00041d: sqrt.f64 (I64)
    // --> [RexMp2furm#751]
    0x01e0, 0x0751,
    // --> [Mp2furm#751] and stop
    // 00041f: sqrt.f64 (I32)
    // --> [Mp2furm#751] and stop
    0x01df, 0x0751,
    // end of sqrt.f64 (I32)
    // end of sqrt.f64 (I64)
    // 000421: store.f64 (I64)
    // --> [RexMp2fst#711]
    0x010e, 0x0711,
    // --> [Mp2fst#711]
    0x010c, 0x0711,
    // --> [RexMp2fstDisp8#711]
    0x0112, 0x0711,
    // --> [Mp2fstDisp8#711]
    0x0110, 0x0711,
    // --> [RexMp2fstDisp32#711]
    0x0116, 0x0711,
    // --> [Mp2fstDisp32#711] and stop
    0x0115, 0x0711,
    // end of store.f64 (I64)
    // 00042d: store_complex.f64 (I64)
    // --> [RexMp2fstWithIndex#711]
    0x011a, 0x0711,
    // --> [Mp2fstWithIndex#711]
    0x0118, 0x0711,
    // --> [RexMp2fstWithIndexDisp8#711]
    0x011e, 0x0711,
    // --> [Mp2fstWithIndexDisp8#711]
    0x011c, 0x0711,
    // --> [RexMp2fstWithIndexDisp32#711]
    0x0122, 0x0711,
    // --> [Mp2fstWithIndexDisp32#711] and stop
    0x0121, 0x0711,
    // end of store_complex.f64 (I64)
    // 000439: x86_fmax.f64 (I64)
    // --> [RexMp2fa#75f]
    0x01ec, 0x075f,
    // --> [Mp2fa#75f] and stop
    // 00043b: x86_fmax.f64 (I32)
    // --> [Mp2fa#75f] and stop
    0x01eb, 0x075f,
    // end of x86_fmax.f64 (I32)
    // end of x86_fmax.f64 (I64)
    // 00043d: x86_fmin.f64 (I64)
    // --> [RexMp2fa#75d]
    0x01ec, 0x075d,
    // --> [Mp2fa#75d] and stop
    // 00043f: x86_fmin.f64 (I32)
    // --> [Mp2fa#75d] and stop
    0x01eb, 0x075d,
    // end of x86_fmin.f64 (I32)
    // end of x86_fmin.f64 (I64)
    // 000441: bitcast.f32 (I64)
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.b8x16 (I64)
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.b16x8 (I64)
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.b32x4 (I64)
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.b64x2 (I64)
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.i8x16 (I64)
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.i16x8 (I64)
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.i32x4 (I64)
    // --> [RexMp2frurm#56e]
    // 000442: scalar_to_vector.i64x2 (I64)
    // --> [RexMp2frurm#56e]
    0x01d0, 0x056e,
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    // 000444: scalar_to_vector.b8x16 (I32)
    // --> [Mp2frurm#56e] and stop
    // 000444: scalar_to_vector.b16x8 (I32)
    // --> [Mp2frurm#56e] and stop
    // 000444: scalar_to_vector.b32x4 (I32)
    // --> [Mp2frurm#56e] and stop
    // 000444: scalar_to_vector.i8x16 (I32)
    // --> [Mp2frurm#56e] and stop
    // 000444: scalar_to_vector.i16x8 (I32)
    // --> [Mp2frurm#56e] and stop
    // 000444: scalar_to_vector.i32x4 (I32)
    // --> [Mp2frurm#56e] and stop
    0x01cf, 0x056e,
    // end of scalar_to_vector.i32x4 (I32)
    // end of scalar_to_vector.i16x8 (I32)
    // end of scalar_to_vector.i8x16 (I32)
    // end of scalar_to_vector.b32x4 (I32)
    // end of scalar_to_vector.b16x8 (I32)
    // end of scalar_to_vector.b8x16 (I32)
    // end of scalar_to_vector.i64x2 (I64)
    // end of scalar_to_vector.i32x4 (I64)
    // end of scalar_to_vector.i16x8 (I64)
    // end of scalar_to_vector.i8x16 (I64)
    // end of scalar_to_vector.b64x2 (I64)
    // end of scalar_to_vector.b32x4 (I64)
    // end of scalar_to_vector.b16x8 (I64)
    // end of scalar_to_vector.b8x16 (I64)
    // end of bitcast.f32 (I64)
    // 000446: ceil.f32 (I64)
    // stop unless PredicateView(16)
    // 000446: floor.f32 (I64)
    // stop unless PredicateView(16)
    // 000446: nearest.f32 (I64)
    // stop unless PredicateView(16)
    // 000446: trunc.f32 (I64)
    // stop unless PredicateView(16)
    0x102d,
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    0x01e8, 0x0d0a,
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    0x01e7, 0x0d0a,
    // end of trunc.f32 (I64)
    // end of nearest.f32 (I64)
    // end of floor.f32 (I64)
    // end of ceil.f32 (I64)
    // 00044b: copy_to_ssa.f32 (I64)
    // --> [RexMp2furm_reg_to_ssa#610] and stop
    0x00e5, 0x0610,
    // end of copy_to_ssa.f32 (I64)
    // 00044d: fadd.f32 (I64)
    // --> [RexMp2fa#658]
    0x01ec, 0x0658,
    // --> [Mp2fa#658] and stop
    // 00044f: fadd.f32 (I32)
    // --> [Mp2fa#658] and stop
    0x01eb, 0x0658,
    // end of fadd.f32 (I32)
    // end of fadd.f32 (I64)
    // 000451: fcmp.f32 (I64)
    // --> [RexOp2fcscc#42e]
    0x01f8, 0x042e,
    // --> [Op2fcscc#42e] and stop
    // 000453: fcmp.f32 (I32)
    // --> [Op2fcscc#42e] and stop
    0x01f7, 0x042e,
    // end of fcmp.f32 (I32)
    // end of fcmp.f32 (I64)
    // 000455: fcvt_from_sint.f32 (I64)
    // skip 4 unless inst_predicate_9
    0x5009,
    // --> [RexMp2frurm#62a]
    0x01d0, 0x062a,
    // --> [Mp2frurm#62a]
    0x01ce, 0x062a,
    // stop unless inst_predicate_10
    0x100a,
    // --> [RexMp2frurm#862a] and stop
    0x01d1, 0x862a,
    // end of fcvt_from_sint.f32 (I64)
    // 00045d: fdemote.f32 (I64)
    // stop unless inst_predicate_15
    0x100f,
    // --> [RexMp2furm#75a]
    0x01e0, 0x075a,
    // --> [Mp2furm#75a] and stop
    0x01df, 0x075a,
    // end of fdemote.f32 (I64)
    // 000462: fdiv.f32 (I64)
    // --> [RexMp2fa#65e]
    0x01ec, 0x065e,
    // --> [Mp2fa#65e] and stop
    // 000464: fdiv.f32 (I32)
    // --> [Mp2fa#65e] and stop
    0x01eb, 0x065e,
    // end of fdiv.f32 (I32)
    // end of fdiv.f32 (I64)
    // 000466: ffcmp.f32 (I64)
    // --> [RexOp2fcmp#42e]
    0x0200, 0x042e,
    // --> [Op2fcmp#42e] and stop
    // 000468: ffcmp.f32 (I32)
    // --> [Op2fcmp#42e] and stop
    0x01ff, 0x042e,
    // end of ffcmp.f32 (I32)
    // end of ffcmp.f32 (I64)
    // 00046a: fill.f32 (I64)
    // --> [RexMp2ffillSib32#610]
    0x0126, 0x0610,
    // --> [Mp2ffillSib32#610] and stop
    // 00046c: fill.f32 (I32)
    // --> [Mp2ffillSib32#610] and stop
    0x0125, 0x0610,
    // end of fill.f32 (I32)
    // end of fill.f32 (I64)
    // 00046e: fmul.f32 (I64)
    // --> [RexMp2fa#659]
    0x01ec, 0x0659,
    // --> [Mp2fa#659] and stop
    // 000470: fmul.f32 (I32)
    // --> [Mp2fa#659] and stop
    0x01eb, 0x0659,
    // end of fmul.f32 (I32)
    // end of fmul.f32 (I64)
    // 000472: fsub.f32 (I64)
    // --> [RexMp2fa#65c]
    0x01ec, 0x065c,
    // --> [Mp2fa#65c] and stop
    // 000474: fsub.f32 (I32)
    // --> [Mp2fa#65c] and stop
    0x01eb, 0x065c,
    // end of fsub.f32 (I32)
    // end of fsub.f32 (I64)
    // 000476: load.f32 (I64)
    // --> [RexMp2fld#610]
    0x00f6, 0x0610,
    // --> [Mp2fld#610]
    0x00f4, 0x0610,
    // --> [RexMp2fldDisp8#610]
    0x00fa, 0x0610,
    // --> [Mp2fldDisp8#610]
    0x00f8, 0x0610,
    // --> [RexMp2fldDisp32#610]
    0x00fe, 0x0610,
    // --> [Mp2fldDisp32#610] and stop
    0x00fd, 0x0610,
    // end of load.f32 (I64)
    // 000482: load_complex.f32 (I64)
    // --> [RexMp2fldWithIndex#610]
    0x0102, 0x0610,
    // --> [Mp2fldWithIndex#610]
    0x0100, 0x0610,
    // --> [RexMp2fldWithIndexDisp8#610]
    0x0106, 0x0610,
    // --> [Mp2fldWithIndexDisp8#610]
    0x0104, 0x0610,
    // --> [RexMp2fldWithIndexDisp32#610]
    0x010a, 0x0610,
    // --> [Mp2fldWithIndexDisp32#610] and stop
    0x0109, 0x0610,
    // end of load_complex.f32 (I64)
    // 00048e: regfill.f32 (I64)
    // --> [RexMp2fregfill32#610]
    0x012a, 0x0610,
    // --> [Mp2fregfill32#610] and stop
    // 000490: regfill.f32 (I32)
    // --> [Mp2fregfill32#610] and stop
    0x0129, 0x0610,
    // end of regfill.f32 (I32)
    // end of regfill.f32 (I64)
    // 000492: regspill.f32 (I64)
    // --> [RexMp2fregspill32#611]
    0x0132, 0x0611,
    // --> [Mp2fregspill32#611] and stop
    // 000494: regspill.f32 (I32)
    // --> [Mp2fregspill32#611] and stop
    0x0131, 0x0611,
    // end of regspill.f32 (I32)
    // end of regspill.f32 (I64)
    // 000496: spill.f32 (I64)
    // --> [RexMp2fspillSib32#611]
    0x012e, 0x0611,
    // --> [Mp2fspillSib32#611] and stop
    // 000498: spill.f32 (I32)
    // --> [Mp2fspillSib32#611] and stop
    0x012d, 0x0611,
    // end of spill.f32 (I32)
    // end of spill.f32 (I64)
    // 00049a: sqrt.f32 (I64)
    // --> [RexMp2furm#651]
    0x01e0, 0x0651,
    // --> [Mp2furm#651] and stop
    // 00049c: sqrt.f32 (I32)
    // --> [Mp2furm#651] and stop
    0x01df, 0x0651,
    // end of sqrt.f32 (I32)
    // end of sqrt.f32 (I64)
    // 00049e: store.f32 (I64)
    // --> [RexMp2fst#611]
    0x010e, 0x0611,
    // --> [Mp2fst#611]
    0x010c, 0x0611,
    // --> [RexMp2fstDisp8#611]
    0x0112, 0x0611,
    // --> [Mp2fstDisp8#611]
    0x0110, 0x0611,
    // --> [RexMp2fstDisp32#611]
    0x0116, 0x0611,
    // --> [Mp2fstDisp32#611] and stop
    0x0115, 0x0611,
    // end of store.f32 (I64)
    // 0004aa: store_complex.f32 (I64)
    // --> [RexMp2fstWithIndex#611]
    0x011a, 0x0611,
    // --> [Mp2fstWithIndex#611]
    0x0118, 0x0611,
    // --> [RexMp2fstWithIndexDisp8#611]
    0x011e, 0x0611,
    // --> [Mp2fstWithIndexDisp8#611]
    0x011c, 0x0611,
    // --> [RexMp2fstWithIndexDisp32#611]
    0x0122, 0x0611,
    // --> [Mp2fstWithIndexDisp32#611] and stop
    0x0121, 0x0611,
    // end of store_complex.f32 (I64)
    // 0004b6: x86_fmax.f32 (I64)
    // --> [RexMp2fa#65f]
    0x01ec, 0x065f,
    // --> [Mp2fa#65f] and stop
    // 0004b8: x86_fmax.f32 (I32)
    // --> [Mp2fa#65f] and stop
    0x01eb, 0x065f,
    // end of x86_fmax.f32 (I32)
    // end of x86_fmax.f32 (I64)
    // 0004ba: x86_fmin.f32 (I64)
    // --> [RexMp2fa#65d]
    0x01ec, 0x065d,
    // --> [Mp2fa#65d] and stop
    // 0004bc: x86_fmin.f32 (I32)
    // --> [Mp2fa#65d] and stop
    0x01eb, 0x065d,
    // end of x86_fmin.f32 (I32)
    // end of x86_fmin.f32 (I64)
    // 0004be: band.b8x16 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.b16x8 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.b32x4 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.b64x2 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i8x16 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i16x8 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i32x4 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i64x2 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.f32x4 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.f64x2 (I64)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.b8x16 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.b16x8 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.b32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.b64x2 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i8x16 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i16x8 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.i64x2 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.f32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 0004be: band.f64x2 (I32)
    // --> [Mp2fa#5db] and stop
    0x01eb, 0x05db,
    // end of band.f64x2 (I32)
    // end of band.f32x4 (I32)
    // end of band.i64x2 (I32)
    // end of band.i32x4 (I32)
    // end of band.i16x8 (I32)
    // end of band.i8x16 (I32)
    // end of band.b64x2 (I32)
    // end of band.b32x4 (I32)
    // end of band.b16x8 (I32)
    // end of band.b8x16 (I32)
    // end of band.f64x2 (I64)
    // end of band.f32x4 (I64)
    // end of band.i64x2 (I64)
    // end of band.i32x4 (I64)
    // end of band.i16x8 (I64)
    // end of band.i8x16 (I64)
    // end of band.b64x2 (I64)
    // end of band.b32x4 (I64)
    // end of band.b16x8 (I64)
    // end of band.b8x16 (I64)
    // 0004c0: bor.b8x16 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.b16x8 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.b32x4 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.b64x2 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i8x16 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i16x8 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i32x4 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i64x2 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.f32x4 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.f64x2 (I64)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.b8x16 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.b16x8 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.b32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.b64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i8x16 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i16x8 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.i64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.f32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0004c0: bor.f64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    0x01eb, 0x05eb,
    // end of bor.f64x2 (I32)
    // end of bor.f32x4 (I32)
    // end of bor.i64x2 (I32)
    // end of bor.i32x4 (I32)
    // end of bor.i16x8 (I32)
    // end of bor.i8x16 (I32)
    // end of bor.b64x2 (I32)
    // end of bor.b32x4 (I32)
    // end of bor.b16x8 (I32)
    // end of bor.b8x16 (I32)
    // end of bor.f64x2 (I64)
    // end of bor.f32x4 (I64)
    // end of bor.i64x2 (I64)
    // end of bor.i32x4 (I64)
    // end of bor.i16x8 (I64)
    // end of bor.i8x16 (I64)
    // end of bor.b64x2 (I64)
    // end of bor.b32x4 (I64)
    // end of bor.b16x8 (I64)
    // end of bor.b8x16 (I64)
    // 0004c2: bxor.b8x16 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.b16x8 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.b32x4 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.b64x2 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i8x16 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i16x8 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i32x4 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i64x2 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.f32x4 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.f64x2 (I64)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.b8x16 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.b16x8 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.b32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.b64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i8x16 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i16x8 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.i64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.f32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0004c2: bxor.f64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    0x01eb, 0x05ef,
    // end of bxor.f64x2 (I32)
    // end of bxor.f32x4 (I32)
    // end of bxor.i64x2 (I32)
    // end of bxor.i32x4 (I32)
    // end of bxor.i16x8 (I32)
    // end of bxor.i8x16 (I32)
    // end of bxor.b64x2 (I32)
    // end of bxor.b32x4 (I32)
    // end of bxor.b16x8 (I32)
    // end of bxor.b8x16 (I32)
    // end of bxor.f64x2 (I64)
    // end of bxor.f32x4 (I64)
    // end of bxor.i64x2 (I64)
    // end of bxor.i32x4 (I64)
    // end of bxor.i16x8 (I64)
    // end of bxor.i8x16 (I64)
    // end of bxor.b64x2 (I64)
    // end of bxor.b32x4 (I64)
    // end of bxor.b16x8 (I64)
    // end of bxor.b8x16 (I64)
    // 0004c4: fill.b8x16 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.b16x8 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.b32x4 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.b64x2 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i8x16 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i16x8 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i32x4 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i64x2 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.f32x4 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.f64x2 (I64)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.b8x16 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.b16x8 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.b32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.b64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i8x16 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i16x8 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.i64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.f32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 0004c4: fill.f64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    0x022d, 0x0410,
    // end of fill.f64x2 (I32)
    // end of fill.f32x4 (I32)
    // end of fill.i64x2 (I32)
    // end of fill.i32x4 (I32)
    // end of fill.i16x8 (I32)
    // end of fill.i8x16 (I32)
    // end of fill.b64x2 (I32)
    // end of fill.b32x4 (I32)
    // end of fill.b16x8 (I32)
    // end of fill.b8x16 (I32)
    // end of fill.f64x2 (I64)
    // end of fill.f32x4 (I64)
    // end of fill.i64x2 (I64)
    // end of fill.i32x4 (I64)
    // end of fill.i16x8 (I64)
    // end of fill.i8x16 (I64)
    // end of fill.b64x2 (I64)
    // end of fill.b32x4 (I64)
    // end of fill.b16x8 (I64)
    // end of fill.b8x16 (I64)
    // 0004c6: load.b8x16 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.b16x8 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.b32x4 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.b64x2 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.i8x16 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.i16x8 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.i32x4 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.i64x2 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.f32x4 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.f64x2 (I64)
    // --> [Op2fld#410]
    // 0004c6: load.b8x16 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.b16x8 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.b32x4 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.b64x2 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.i8x16 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.i16x8 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.i32x4 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.i64x2 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.f32x4 (I32)
    // --> [Op2fld#410]
    // 0004c6: load.f64x2 (I32)
    // --> [Op2fld#410]
    0x0222, 0x0410,
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    0x0224, 0x0410,
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    0x0227, 0x0410,
    // end of load.f64x2 (I32)
    // end of load.f32x4 (I32)
    // end of load.i64x2 (I32)
    // end of load.i32x4 (I32)
    // end of load.i16x8 (I32)
    // end of load.i8x16 (I32)
    // end of load.b64x2 (I32)
    // end of load.b32x4 (I32)
    // end of load.b16x8 (I32)
    // end of load.b8x16 (I32)
    // end of load.f64x2 (I64)
    // end of load.f32x4 (I64)
    // end of load.i64x2 (I64)
    // end of load.i32x4 (I64)
    // end of load.i16x8 (I64)
    // end of load.i8x16 (I64)
    // end of load.b64x2 (I64)
    // end of load.b32x4 (I64)
    // end of load.b16x8 (I64)
    // end of load.b8x16 (I64)
    // 0004cc: raw_bitcast.b8x16 (I64)
    // skip 2 unless inst_predicate_17
    // 0004cc: raw_bitcast.b8x16 (I32)
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.b8x16 (I32)
    // end of raw_bitcast.b8x16 (I64)
    // 0004ed: regfill.b8x16 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.b16x8 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.b32x4 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.b64x2 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i8x16 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i16x8 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i32x4 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i64x2 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.f32x4 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.f64x2 (I64)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.b8x16 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.b16x8 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.b32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.b64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i8x16 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i16x8 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.i64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.f32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 0004ed: regfill.f64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    0x022f, 0x0410,
    // end of regfill.f64x2 (I32)
    // end of regfill.f32x4 (I32)
    // end of regfill.i64x2 (I32)
    // end of regfill.i32x4 (I32)
    // end of regfill.i16x8 (I32)
    // end of regfill.i8x16 (I32)
    // end of regfill.b64x2 (I32)
    // end of regfill.b32x4 (I32)
    // end of regfill.b16x8 (I32)
    // end of regfill.b8x16 (I32)
    // end of regfill.f64x2 (I64)
    // end of regfill.f32x4 (I64)
    // end of regfill.i64x2 (I64)
    // end of regfill.i32x4 (I64)
    // end of regfill.i16x8 (I64)
    // end of regfill.i8x16 (I64)
    // end of regfill.b64x2 (I64)
    // end of regfill.b32x4 (I64)
    // end of regfill.b16x8 (I64)
    // end of regfill.b8x16 (I64)
    // 0004ef: regmove.b8x16 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.b16x8 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.b32x4 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.b64x2 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i8x16 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i16x8 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i32x4 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i64x2 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.f32x4 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.f64x2 (I64)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.f64 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.f32 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.b8x16 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.b16x8 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.b32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.b64x2 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i8x16 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i16x8 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.i64x2 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.f32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 0004ef: regmove.f64x2 (I32)
    // --> [Op2frmov#428] and stop
    0x01db, 0x0428,
    // end of regmove.f64x2 (I32)
    // end of regmove.f32x4 (I32)
    // end of regmove.i64x2 (I32)
    // end of regmove.i32x4 (I32)
    // end of regmove.i16x8 (I32)
    // end of regmove.i8x16 (I32)
    // end of regmove.b64x2 (I32)
    // end of regmove.b32x4 (I32)
    // end of regmove.b16x8 (I32)
    // end of regmove.b8x16 (I32)
    // end of regmove.f32 (I32)
    // end of regmove.f64 (I32)
    // end of regmove.f64x2 (I64)
    // end of regmove.f32x4 (I64)
    // end of regmove.i64x2 (I64)
    // end of regmove.i32x4 (I64)
    // end of regmove.i16x8 (I64)
    // end of regmove.i8x16 (I64)
    // end of regmove.b64x2 (I64)
    // end of regmove.b32x4 (I64)
    // end of regmove.b16x8 (I64)
    // end of regmove.b8x16 (I64)
    // 0004f1: regspill.b8x16 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.b16x8 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.b32x4 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.b64x2 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i8x16 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i16x8 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i32x4 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i64x2 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.f32x4 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.f64x2 (I64)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.b8x16 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.b16x8 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.b32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.b64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i8x16 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i16x8 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.i64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.f32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 0004f1: regspill.f64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    0x022b, 0x0411,
    // end of regspill.f64x2 (I32)
    // end of regspill.f32x4 (I32)
    // end of regspill.i64x2 (I32)
    // end of regspill.i32x4 (I32)
    // end of regspill.i16x8 (I32)
    // end of regspill.i8x16 (I32)
    // end of regspill.b64x2 (I32)
    // end of regspill.b32x4 (I32)
    // end of regspill.b16x8 (I32)
    // end of regspill.b8x16 (I32)
    // end of regspill.f64x2 (I64)
    // end of regspill.f32x4 (I64)
    // end of regspill.i64x2 (I64)
    // end of regspill.i32x4 (I64)
    // end of regspill.i16x8 (I64)
    // end of regspill.i8x16 (I64)
    // end of regspill.b64x2 (I64)
    // end of regspill.b32x4 (I64)
    // end of regspill.b16x8 (I64)
    // end of regspill.b8x16 (I64)
    // 0004f3: spill.b8x16 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.b16x8 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.b32x4 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.b64x2 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i8x16 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i16x8 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i32x4 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i64x2 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.f32x4 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.f64x2 (I64)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.b8x16 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.b16x8 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.b32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.b64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i8x16 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i16x8 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.i64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.f32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 0004f3: spill.f64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    0x0229, 0x0411,
    // end of spill.f64x2 (I32)
    // end of spill.f32x4 (I32)
    // end of spill.i64x2 (I32)
    // end of spill.i32x4 (I32)
    // end of spill.i16x8 (I32)
    // end of spill.i8x16 (I32)
    // end of spill.b64x2 (I32)
    // end of spill.b32x4 (I32)
    // end of spill.b16x8 (I32)
    // end of spill.b8x16 (I32)
    // end of spill.f64x2 (I64)
    // end of spill.f32x4 (I64)
    // end of spill.i64x2 (I64)
    // end of spill.i32x4 (I64)
    // end of spill.i16x8 (I64)
    // end of spill.i8x16 (I64)
    // end of spill.b64x2 (I64)
    // end of spill.b32x4 (I64)
    // end of spill.b16x8 (I64)
    // end of spill.b8x16 (I64)
    // 0004f5: store.b8x16 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.b16x8 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.b32x4 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.b64x2 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.i8x16 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.i16x8 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.i32x4 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.i64x2 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.f32x4 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.f64x2 (I64)
    // --> [Op2fst#411]
    // 0004f5: store.b8x16 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.b16x8 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.b32x4 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.b64x2 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.i8x16 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.i16x8 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.i32x4 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.i64x2 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.f32x4 (I32)
    // --> [Op2fst#411]
    // 0004f5: store.f64x2 (I32)
    // --> [Op2fst#411]
    0x021c, 0x0411,
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    0x021e, 0x0411,
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    0x0221, 0x0411,
    // end of store.f64x2 (I32)
    // end of store.f32x4 (I32)
    // end of store.i64x2 (I32)
    // end of store.i32x4 (I32)
    // end of store.i16x8 (I32)
    // end of store.i8x16 (I32)
    // end of store.b64x2 (I32)
    // end of store.b32x4 (I32)
    // end of store.b16x8 (I32)
    // end of store.b8x16 (I32)
    // end of store.f64x2 (I64)
    // end of store.f32x4 (I64)
    // end of store.i64x2 (I64)
    // end of store.i32x4 (I64)
    // end of store.i16x8 (I64)
    // end of store.i8x16 (I64)
    // end of store.b64x2 (I64)
    // end of store.b32x4 (I64)
    // end of store.b16x8 (I64)
    // end of store.b8x16 (I64)
    // 0004fb: vconst.b8x16 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.b16x8 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.b32x4 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.b64x2 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i8x16 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i16x8 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i32x4 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i64x2 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.f32x4 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.f64x2 (I64)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.b8x16 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.b16x8 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.b32x4 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.b64x2 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i8x16 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i16x8 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i32x4 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.i64x2 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.f32x4 (I32)
    // skip 2 unless inst_predicate_26
    // 0004fb: vconst.f64x2 (I32)
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    // --> [Mp2vconst_optimized#5ef]
    0x0218, 0x05ef,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    // --> [Mp2vconst_optimized#574]
    0x0218, 0x0574,
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    0x021b, 0x0410,
    // end of vconst.f64x2 (I32)
    // end of vconst.f32x4 (I32)
    // end of vconst.i64x2 (I32)
    // end of vconst.i32x4 (I32)
    // end of vconst.i16x8 (I32)
    // end of vconst.i8x16 (I32)
    // end of vconst.b64x2 (I32)
    // end of vconst.b32x4 (I32)
    // end of vconst.b16x8 (I32)
    // end of vconst.b8x16 (I32)
    // end of vconst.f64x2 (I64)
    // end of vconst.f32x4 (I64)
    // end of vconst.i64x2 (I64)
    // end of vconst.i32x4 (I64)
    // end of vconst.i16x8 (I64)
    // end of vconst.i8x16 (I64)
    // end of vconst.b64x2 (I64)
    // end of vconst.b32x4 (I64)
    // end of vconst.b16x8 (I64)
    // end of vconst.b8x16 (I64)
    // 000503: x86_pextr.b8x16 (I64)
    // stop unless PredicateView(17)
    // 000503: x86_pextr.i8x16 (I64)
    // stop unless PredicateView(17)
    // 000503: x86_pextr.b8x16 (I32)
    // stop unless PredicateView(17)
    // 000503: x86_pextr.i8x16 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    0x0215, 0x0d14,
    // end of x86_pextr.i8x16 (I32)
    // end of x86_pextr.b8x16 (I32)
    // end of x86_pextr.i8x16 (I64)
    // end of x86_pextr.b8x16 (I64)
    // 000506: x86_pinsr.b8x16 (I64)
    // stop unless PredicateView(17)
    // 000506: x86_pinsr.i8x16 (I64)
    // stop unless PredicateView(17)
    // 000506: x86_pinsr.b8x16 (I32)
    // stop unless PredicateView(17)
    // 000506: x86_pinsr.i8x16 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    0x020d, 0x0d20,
    // end of x86_pinsr.i8x16 (I32)
    // end of x86_pinsr.b8x16 (I32)
    // end of x86_pinsr.i8x16 (I64)
    // end of x86_pinsr.b8x16 (I64)
    // 000509: x86_pshufb.b8x16 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.b16x8 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.b32x4 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.b64x2 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i8x16 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i16x8 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i32x4 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i64x2 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.f32x4 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.f64x2 (I64)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.b8x16 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.b16x8 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.b32x4 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.b64x2 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i8x16 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i16x8 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i32x4 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.i64x2 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.f32x4 (I32)
    // stop unless PredicateView(21)
    // 000509: x86_pshufb.f64x2 (I32)
    // stop unless PredicateView(21)
    0x1032,
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    0x0207, 0x0900,
    // end of x86_pshufb.f64x2 (I32)
    // end of x86_pshufb.f32x4 (I32)
    // end of x86_pshufb.i64x2 (I32)
    // end of x86_pshufb.i32x4 (I32)
    // end of x86_pshufb.i16x8 (I32)
    // end of x86_pshufb.i8x16 (I32)
    // end of x86_pshufb.b64x2 (I32)
    // end of x86_pshufb.b32x4 (I32)
    // end of x86_pshufb.b16x8 (I32)
    // end of x86_pshufb.b8x16 (I32)
    // end of x86_pshufb.f64x2 (I64)
    // end of x86_pshufb.f32x4 (I64)
    // end of x86_pshufb.i64x2 (I64)
    // end of x86_pshufb.i32x4 (I64)
    // end of x86_pshufb.i16x8 (I64)
    // end of x86_pshufb.i8x16 (I64)
    // end of x86_pshufb.b64x2 (I64)
    // end of x86_pshufb.b32x4 (I64)
    // end of x86_pshufb.b16x8 (I64)
    // end of x86_pshufb.b8x16 (I64)
    // 00050c: raw_bitcast.b16x8 (I64)
    // skip 2 unless inst_predicate_16
    // 00050c: raw_bitcast.b16x8 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.b16x8 (I32)
    // end of raw_bitcast.b16x8 (I64)
    // 00052d: x86_pextr.b16x8 (I64)
    // stop unless PredicateView(17)
    // 00052d: x86_pextr.i16x8 (I64)
    // stop unless PredicateView(17)
    // 00052d: x86_pextr.b16x8 (I32)
    // stop unless PredicateView(17)
    // 00052d: x86_pextr.i16x8 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    0x0215, 0x0d15,
    // end of x86_pextr.i16x8 (I32)
    // end of x86_pextr.b16x8 (I32)
    // end of x86_pextr.i16x8 (I64)
    // end of x86_pextr.b16x8 (I64)
    // 000530: x86_pinsr.b16x8 (I64)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    // 000530: x86_pinsr.i16x8 (I64)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    // 000530: x86_pinsr.b16x8 (I32)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    // 000530: x86_pinsr.i16x8 (I32)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    0x020f, 0x05c4,
    // end of x86_pinsr.i16x8 (I32)
    // end of x86_pinsr.b16x8 (I32)
    // end of x86_pinsr.i16x8 (I64)
    // end of x86_pinsr.b16x8 (I64)
    // 000532: raw_bitcast.b32x4 (I64)
    // skip 2 unless inst_predicate_16
    // 000532: raw_bitcast.b32x4 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.b32x4 (I32)
    // end of raw_bitcast.b32x4 (I64)
    // 000553: x86_pextr.b32x4 (I64)
    // stop unless PredicateView(17)
    // 000553: x86_pextr.i32x4 (I64)
    // stop unless PredicateView(17)
    // 000553: x86_pextr.f32x4 (I64)
    // stop unless PredicateView(17)
    // 000553: x86_pextr.b32x4 (I32)
    // stop unless PredicateView(17)
    // 000553: x86_pextr.i32x4 (I32)
    // stop unless PredicateView(17)
    // 000553: x86_pextr.f32x4 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    0x0215, 0x0d16,
    // end of x86_pextr.f32x4 (I32)
    // end of x86_pextr.i32x4 (I32)
    // end of x86_pextr.b32x4 (I32)
    // end of x86_pextr.f32x4 (I64)
    // end of x86_pextr.i32x4 (I64)
    // end of x86_pextr.b32x4 (I64)
    // 000556: x86_pinsr.b32x4 (I64)
    // stop unless PredicateView(17)
    // 000556: x86_pinsr.i32x4 (I64)
    // stop unless PredicateView(17)
    // 000556: x86_pinsr.f32x4 (I64)
    // stop unless PredicateView(17)
    // 000556: x86_pinsr.b32x4 (I32)
    // stop unless PredicateView(17)
    // 000556: x86_pinsr.i32x4 (I32)
    // stop unless PredicateView(17)
    // 000556: x86_pinsr.f32x4 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    0x020d, 0x0d22,
    // end of x86_pinsr.f32x4 (I32)
    // end of x86_pinsr.i32x4 (I32)
    // end of x86_pinsr.b32x4 (I32)
    // end of x86_pinsr.f32x4 (I64)
    // end of x86_pinsr.i32x4 (I64)
    // end of x86_pinsr.b32x4 (I64)
    // 000559: x86_pshufd.b32x4 (I64)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000559: x86_pshufd.i32x4 (I64)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000559: x86_pshufd.f32x4 (I64)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000559: x86_pshufd.b32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000559: x86_pshufd.i32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000559: x86_pshufd.f32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    0x0209, 0x0570,
    // end of x86_pshufd.f32x4 (I32)
    // end of x86_pshufd.i32x4 (I32)
    // end of x86_pshufd.b32x4 (I32)
    // end of x86_pshufd.f32x4 (I64)
    // end of x86_pshufd.i32x4 (I64)
    // end of x86_pshufd.b32x4 (I64)
    // 00055b: raw_bitcast.b64x2 (I64)
    // skip 2 unless inst_predicate_16
    // 00055b: raw_bitcast.b64x2 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.b64x2 (I32)
    // end of raw_bitcast.b64x2 (I64)
    // 00057c: x86_pextr.b64x2 (I64)
    // stop unless PredicateView(17)
    // 00057c: x86_pextr.i64x2 (I64)
    // stop unless PredicateView(17)
    // 00057c: x86_pextr.f64x2 (I64)
    // stop unless PredicateView(17)
    0x102e,
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    0x0217, 0x8d16,
    // end of x86_pextr.f64x2 (I64)
    // end of x86_pextr.i64x2 (I64)
    // end of x86_pextr.b64x2 (I64)
    // 00057f: x86_pinsr.b64x2 (I64)
    // stop unless PredicateView(17)
    // 00057f: x86_pinsr.i64x2 (I64)
    // stop unless PredicateView(17)
    // 00057f: x86_pinsr.f64x2 (I64)
    // stop unless PredicateView(17)
    0x102e,
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    0x0211, 0x8d22,
    // end of x86_pinsr.f64x2 (I64)
    // end of x86_pinsr.i64x2 (I64)
    // end of x86_pinsr.b64x2 (I64)
    // 000582: iadd.i8x16 (I64)
    // --> [Mp2fa#5fc] and stop
    // 000582: iadd.i8x16 (I32)
    // --> [Mp2fa#5fc] and stop
    0x01eb, 0x05fc,
    // end of iadd.i8x16 (I32)
    // end of iadd.i8x16 (I64)
    // 000584: icmp.i8x16 (I64)
    // stop unless inst_predicate_28
    // 000584: icmp.i8x16 (I32)
    // stop unless inst_predicate_28
    0x101c,
    // --> [Mp2icscc_fpr#574] and stop
    // --> [Mp2icscc_fpr#574] and stop
    0x0231, 0x0574,
    // end of icmp.i8x16 (I32)
    // end of icmp.i8x16 (I64)
    // 000587: isub.i8x16 (I64)
    // --> [Mp2fa#5f8] and stop
    // 000587: isub.i8x16 (I32)
    // --> [Mp2fa#5f8] and stop
    0x01eb, 0x05f8,
    // end of isub.i8x16 (I32)
    // end of isub.i8x16 (I64)
    // 000589: raw_bitcast.i8x16 (I64)
    // skip 2 unless inst_predicate_16
    // 000589: raw_bitcast.i8x16 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.i8x16 (I32)
    // end of raw_bitcast.i8x16 (I64)
    // 0005aa: sadd_sat.i8x16 (I64)
    // --> [Mp2fa#5ec] and stop
    // 0005aa: sadd_sat.i8x16 (I32)
    // --> [Mp2fa#5ec] and stop
    0x01eb, 0x05ec,
    // end of sadd_sat.i8x16 (I32)
    // end of sadd_sat.i8x16 (I64)
    // 0005ac: ssub_sat.i8x16 (I64)
    // --> [Mp2fa#5e8] and stop
    // 0005ac: ssub_sat.i8x16 (I32)
    // --> [Mp2fa#5e8] and stop
    0x01eb, 0x05e8,
    // end of ssub_sat.i8x16 (I32)
    // end of ssub_sat.i8x16 (I64)
    // 0005ae: uadd_sat.i8x16 (I64)
    // --> [Mp2fa#5dc] and stop
    // 0005ae: uadd_sat.i8x16 (I32)
    // --> [Mp2fa#5dc] and stop
    0x01eb, 0x05dc,
    // end of uadd_sat.i8x16 (I32)
    // end of uadd_sat.i8x16 (I64)
    // 0005b0: usub_sat.i8x16 (I64)
    // --> [Mp2fa#5d8] and stop
    // 0005b0: usub_sat.i8x16 (I32)
    // --> [Mp2fa#5d8] and stop
    0x01eb, 0x05d8,
    // end of usub_sat.i8x16 (I32)
    // end of usub_sat.i8x16 (I64)
    // 0005b2: iadd.i16x8 (I64)
    // --> [Mp2fa#5fd] and stop
    // 0005b2: iadd.i16x8 (I32)
    // --> [Mp2fa#5fd] and stop
    0x01eb, 0x05fd,
    // end of iadd.i16x8 (I32)
    // end of iadd.i16x8 (I64)
    // 0005b4: icmp.i16x8 (I64)
    // stop unless inst_predicate_28
    // 0005b4: icmp.i16x8 (I32)
    // stop unless inst_predicate_28
    0x101c,
    // --> [Mp2icscc_fpr#575] and stop
    // --> [Mp2icscc_fpr#575] and stop
    0x0231, 0x0575,
    // end of icmp.i16x8 (I32)
    // end of icmp.i16x8 (I64)
    // 0005b7: imul.i16x8 (I64)
    // --> [Mp2fa#5d5] and stop
    // 0005b7: imul.i16x8 (I32)
    // --> [Mp2fa#5d5] and stop
    0x01eb, 0x05d5,
    // end of imul.i16x8 (I32)
    // end of imul.i16x8 (I64)
    // 0005b9: isub.i16x8 (I64)
    // --> [Mp2fa#5f9] and stop
    // 0005b9: isub.i16x8 (I32)
    // --> [Mp2fa#5f9] and stop
    0x01eb, 0x05f9,
    // end of isub.i16x8 (I32)
    // end of isub.i16x8 (I64)
    // 0005bb: raw_bitcast.i16x8 (I64)
    // skip 2 unless inst_predicate_16
    // 0005bb: raw_bitcast.i16x8 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.i16x8 (I32)
    // end of raw_bitcast.i16x8 (I64)
    // 0005dc: sadd_sat.i16x8 (I64)
    // --> [Mp2fa#5ed] and stop
    // 0005dc: sadd_sat.i16x8 (I32)
    // --> [Mp2fa#5ed] and stop
    0x01eb, 0x05ed,
    // end of sadd_sat.i16x8 (I32)
    // end of sadd_sat.i16x8 (I64)
    // 0005de: ssub_sat.i16x8 (I64)
    // --> [Mp2fa#5e9] and stop
    // 0005de: ssub_sat.i16x8 (I32)
    // --> [Mp2fa#5e9] and stop
    0x01eb, 0x05e9,
    // end of ssub_sat.i16x8 (I32)
    // end of ssub_sat.i16x8 (I64)
    // 0005e0: uadd_sat.i16x8 (I64)
    // --> [Mp2fa#5dd] and stop
    // 0005e0: uadd_sat.i16x8 (I32)
    // --> [Mp2fa#5dd] and stop
    0x01eb, 0x05dd,
    // end of uadd_sat.i16x8 (I32)
    // end of uadd_sat.i16x8 (I64)
    // 0005e2: usub_sat.i16x8 (I64)
    // --> [Mp2fa#5d9] and stop
    // 0005e2: usub_sat.i16x8 (I32)
    // --> [Mp2fa#5d9] and stop
    0x01eb, 0x05d9,
    // end of usub_sat.i16x8 (I32)
    // end of usub_sat.i16x8 (I64)
    // 0005e4: iadd.i32x4 (I64)
    // --> [Mp2fa#5fe] and stop
    // 0005e4: iadd.i32x4 (I32)
    // --> [Mp2fa#5fe] and stop
    0x01eb, 0x05fe,
    // end of iadd.i32x4 (I32)
    // end of iadd.i32x4 (I64)
    // 0005e6: icmp.i32x4 (I64)
    // stop unless inst_predicate_28
    // 0005e6: icmp.i32x4 (I32)
    // stop unless inst_predicate_28
    0x101c,
    // --> [Mp2icscc_fpr#576] and stop
    // --> [Mp2icscc_fpr#576] and stop
    0x0231, 0x0576,
    // end of icmp.i32x4 (I32)
    // end of icmp.i32x4 (I64)
    // 0005e9: imul.i32x4 (I64)
    // stop unless PredicateView(17)
    // 0005e9: imul.i32x4 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // --> [Mp3fa#940] and stop
    // --> [Mp3fa#940] and stop
    0x0207, 0x0940,
    // end of imul.i32x4 (I32)
    // end of imul.i32x4 (I64)
    // 0005ec: isub.i32x4 (I64)
    // --> [Mp2fa#5fa] and stop
    // 0005ec: isub.i32x4 (I32)
    // --> [Mp2fa#5fa] and stop
    0x01eb, 0x05fa,
    // end of isub.i32x4 (I32)
    // end of isub.i32x4 (I64)
    // 0005ee: raw_bitcast.i32x4 (I64)
    // skip 2 unless inst_predicate_16
    // 0005ee: raw_bitcast.i32x4 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.i32x4 (I32)
    // end of raw_bitcast.i32x4 (I64)
    // 00060f: iadd.i64x2 (I64)
    // --> [Mp2fa#5d4] and stop
    // 00060f: iadd.i64x2 (I32)
    // --> [Mp2fa#5d4] and stop
    0x01eb, 0x05d4,
    // end of iadd.i64x2 (I32)
    // end of iadd.i64x2 (I64)
    // 000611: icmp.i64x2 (I64)
    // stop unless PredicateView(17)
    // 000611: icmp.i64x2 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // stop unless inst_predicate_28
    // stop unless inst_predicate_28
    0x101c,
    // --> [Mp3icscc_fpr#929] and stop
    // --> [Mp3icscc_fpr#929] and stop
    0x0233, 0x0929,
    // end of icmp.i64x2 (I32)
    // end of icmp.i64x2 (I64)
    // 000615: isub.i64x2 (I64)
    // --> [Mp2fa#5fb] and stop
    // 000615: isub.i64x2 (I32)
    // --> [Mp2fa#5fb] and stop
    0x01eb, 0x05fb,
    // end of isub.i64x2 (I32)
    // end of isub.i64x2 (I64)
    // 000617: raw_bitcast.i64x2 (I64)
    // skip 2 unless inst_predicate_16
    // 000617: raw_bitcast.i64x2 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.i64x2 (I32)
    // end of raw_bitcast.i64x2 (I64)
    // 000638: raw_bitcast.f32x4 (I64)
    // skip 2 unless inst_predicate_16
    // 000638: raw_bitcast.f32x4 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.f32x4 (I32)
    // end of raw_bitcast.f32x4 (I64)
    // 000659: x86_insertps.f32x4 (I64)
    // stop unless PredicateView(17)
    // 000659: x86_insertps.f32x4 (I32)
    // stop unless PredicateView(17)
    0x102e,
    // --> [Mp3fa_ib#d21] and stop
    // --> [Mp3fa_ib#d21] and stop
    0x0213, 0x0d21,
    // end of x86_insertps.f32x4 (I32)
    // end of x86_insertps.f32x4 (I64)
    // 00065c: raw_bitcast.f64x2 (I64)
    // skip 2 unless inst_predicate_16
    // 00065c: raw_bitcast.f64x2 (I32)
    // skip 2 unless inst_predicate_16
    0x3010,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_17
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_18
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_19
    // skip 2 unless inst_predicate_19
    0x3013,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_20
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // skip 2 unless inst_predicate_14
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x020a, 0x0000,
    // stop unless inst_predicate_15
    // stop unless inst_predicate_15
    0x100f,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x020b, 0x0000,
    // end of raw_bitcast.f64x2 (I32)
    // end of raw_bitcast.f64x2 (I64)
    // 00067d: x86_movlhps.f64x2 (I64)
    // --> [Op2fa#416] and stop
    // 00067d: x86_movlhps.f64x2 (I32)
    // --> [Op2fa#416] and stop
    0x01ef, 0x0416,
    // end of x86_movlhps.f64x2 (I32)
    // end of x86_movlhps.f64x2 (I64)
    // 00067f: x86_movsd.f64x2 (I64)
    // --> [Mp2fa#710] and stop
    // 00067f: x86_movsd.f64x2 (I32)
    // --> [Mp2fa#710] and stop
    0x01eb, 0x0710,
    // end of x86_movsd.f64x2 (I32)
    // end of x86_movsd.f64x2 (I64)
    // 000681: adjust_sp_down.i32 (I32)
    // --> [Op1adjustsp#29] and stop
    0x00e9, 0x0029,
    // end of adjust_sp_down.i32 (I32)
    // 000683: band_imm.i32 (I32)
    // --> [Op1r_ib#4083]
    0x002c, 0x4083,
    // --> [Op1r_id#4081] and stop
    0x0031, 0x4081,
    // end of band_imm.i32 (I32)
    // 000687: bint.i32 (I32)
    // stop unless inst_predicate_7
    0x1007,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x01bd, 0x04b6,
    // end of bint.i32 (I32)
    // 00068a: bitcast.i32 (I32)
    // stop unless inst_predicate_14
    0x100e,
    // --> [Mp2rfumr#57e] and stop
    0x01d3, 0x057e,
    // end of bitcast.i32 (I32)
    // 00068d: bor_imm.i32 (I32)
    // --> [Op1r_ib#1083]
    0x002c, 0x1083,
    // --> [Op1r_id#1081] and stop
    0x0031, 0x1081,
    // end of bor_imm.i32 (I32)
    // 000691: brnz.i32 (I32)
    // --> [Op1tjccb#75]
    0x016a, 0x0075,
    // --> [Op1tjccd#85] and stop
    0x016f, 0x0085,
    // end of brnz.i32 (I32)
    // 000695: brz.i32 (I32)
    // --> [Op1tjccb#74]
    0x016a, 0x0074,
    // --> [Op1tjccd#84] and stop
    0x016f, 0x0084,
    // end of brz.i32 (I32)
    // 000699: bxor_imm.i32 (I32)
    // --> [Op1r_ib#6083]
    0x002c, 0x6083,
    // --> [Op1r_id#6081] and stop
    0x0031, 0x6081,
    // end of bxor_imm.i32 (I32)
    // 00069d: clz.i32 (I32)
    // stop unless PredicateView(14)
    0x102b,
    // --> [Mp2urm#6bd] and stop
    0x0049, 0x06bd,
    // end of clz.i32 (I32)
    // 0006a0: copy_to_ssa.i32 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006a0: copy_to_ssa.b1 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006a0: copy_to_ssa.r32 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006a0: copy_to_ssa.i8 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 0006a0: copy_to_ssa.i16 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    0x00df, 0x0089,
    // end of copy_to_ssa.i16 (I32)
    // end of copy_to_ssa.i8 (I32)
    // end of copy_to_ssa.r32 (I32)
    // end of copy_to_ssa.b1 (I32)
    // end of copy_to_ssa.i32 (I32)
    // 0006a2: ctz.i32 (I32)
    // stop unless PredicateView(13)
    0x102a,
    // --> [Mp2urm#6bc] and stop
    0x0049, 0x06bc,
    // end of ctz.i32 (I32)
    // 0006a5: func_addr.i32 (I32)
    // skip 2 unless PredicateView(11)
    0x3028,
    // --> [Op1fnaddr4#b8]
    0x0134, 0x00b8,
    // stop unless PredicateView(9)
    0x1026,
    // --> [Op1allones_fnaddr4#b8] and stop
    0x0139, 0x00b8,
    // end of func_addr.i32 (I32)
    // 0006ab: iadd_imm.i32 (I32)
    // --> [Op1r_ib#83]
    0x002c, 0x0083,
    // --> [Op1r_id#81] and stop
    0x0031, 0x0081,
    // end of iadd_imm.i32 (I32)
    // 0006af: icmp_imm.i32 (I32)
    // --> [Op1icscc_ib#7083]
    0x0194, 0x7083,
    // --> [Op1icscc_id#7081] and stop
    0x0199, 0x7081,
    // end of icmp_imm.i32 (I32)
    // 0006b3: iconst.i32 (I32)
    // --> [Op1pu_id#b8]
    0x0034, 0x00b8,
    // stop unless inst_predicate_1
    // 0006b5: iconst.i16 (I32)
    // stop unless inst_predicate_1
    0x1001,
    // --> [Op1u_id_z#31] and stop
    // --> [Op1u_id_z#31] and stop
    0x0041, 0x0031,
    // end of iconst.i16 (I32)
    // end of iconst.i32 (I32)
    // 0006b8: ifcmp_imm.i32 (I32)
    // --> [Op1rcmp_ib#7083]
    0x01a0, 0x7083,
    // --> [Op1rcmp_id#7081] and stop
    0x01a5, 0x7081,
    // end of ifcmp_imm.i32 (I32)
    // 0006bc: ifcmp_sp.i32 (I32)
    // --> [Op1rcmp_sp#39] and stop
    0x01a9, 0x0039,
    // end of ifcmp_sp.i32 (I32)
    // 0006be: istore16.i32 (I32)
    // --> [Mp1st#189]
    0x008c, 0x0189,
    // --> [Mp1stDisp8#189]
    0x0094, 0x0189,
    // --> [Mp1stDisp32#189] and stop
    0x009d, 0x0189,
    // end of istore16.i32 (I32)
    // 0006c4: istore16_complex.i32 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [Mp1stWithIndex#189]
    0x0068, 0x0189,
    // --> [Mp1stWithIndexDisp8#189]
    0x0070, 0x0189,
    // --> [Mp1stWithIndexDisp32#189] and stop
    0x0079, 0x0189,
    // end of istore16_complex.i32 (I32)
    // 0006cb: istore8.i32 (I32)
    // --> [Op1st_abcd#88]
    0x00a0, 0x0088,
    // --> [Op1stDisp8_abcd#88]
    0x00a2, 0x0088,
    // --> [Op1stDisp32_abcd#88] and stop
    0x00a5, 0x0088,
    // end of istore8.i32 (I32)
    // 0006d1: istore8_complex.i32 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [Op1stWithIndex_abcd#88]
    0x007c, 0x0088,
    // --> [Op1stWithIndexDisp8_abcd#88]
    0x0080, 0x0088,
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    0x0085, 0x0088,
    // end of istore8_complex.i32 (I32)
    // 0006d8: jump_table_base.i32 (I32)
    // --> [Op1jt_base#8d] and stop
    0x0183, 0x008d,
    // end of jump_table_base.i32 (I32)
    // 0006da: jump_table_entry.i32 (I32)
    // --> [Op1jt_entry#8b] and stop
    0x017f, 0x008b,
    // end of jump_table_entry.i32 (I32)
    // 0006dc: load.i32 (I32)
    // --> [Op1ld#8b]
    0x00ae, 0x008b,
    // --> [Op1ldDisp8#8b]
    0x00b6, 0x008b,
    // --> [Op1ldDisp32#8b] and stop
    0x00bf, 0x008b,
    // end of load.i32 (I32)
    // 0006e2: load_complex.i32 (I32)
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op1ldWithIndex#8b]
    0x004c, 0x008b,
    // --> [Op1ldWithIndexDisp8#8b]
    0x0054, 0x008b,
    // --> [Op1ldWithIndexDisp32#8b] and stop
    0x005d, 0x008b,
    // end of load_complex.i32 (I32)
    // 0006e9: popcnt.i32 (I32)
    // stop unless PredicateView(15)
    0x102c,
    // --> [Mp2urm#6b8] and stop
    0x0049, 0x06b8,
    // end of popcnt.i32 (I32)
    // 0006ec: sextend.i32 (I32)
    // skip 2 unless inst_predicate_11
    0x300b,
    // --> [Op2urm_noflags_abcd#4be]
    0x01bc, 0x04be,
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op2urm_noflags#4bf] and stop
    0x01c3, 0x04bf,
    // end of sextend.i32 (I32)
    // 0006f2: sload16.i32 (I32)
    // --> [Op2ld#4bf]
    0x00b2, 0x04bf,
    // --> [Op2ldDisp8#4bf]
    0x00ba, 0x04bf,
    // --> [Op2ldDisp32#4bf] and stop
    0x00c3, 0x04bf,
    // end of sload16.i32 (I32)
    // 0006f8: sload16_complex.i32 (I32)
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2ldWithIndex#4bf]
    0x0050, 0x04bf,
    // --> [Op2ldWithIndexDisp8#4bf]
    0x0058, 0x04bf,
    // --> [Op2ldWithIndexDisp32#4bf] and stop
    0x0061, 0x04bf,
    // end of sload16_complex.i32 (I32)
    // 0006ff: sload8.i32 (I32)
    // --> [Op2ld#4be]
    0x00b2, 0x04be,
    // --> [Op2ldDisp8#4be]
    0x00ba, 0x04be,
    // --> [Op2ldDisp32#4be] and stop
    0x00c3, 0x04be,
    // end of sload8.i32 (I32)
    // 000705: sload8_complex.i32 (I32)
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2ldWithIndex#4be]
    0x0050, 0x04be,
    // --> [Op2ldWithIndexDisp8#4be]
    0x0058, 0x04be,
    // --> [Op2ldWithIndexDisp32#4be] and stop
    0x0061, 0x04be,
    // end of sload8_complex.i32 (I32)
    // 00070c: stack_addr.i32 (I32)
    // --> [Op1spaddr4_id#8d] and stop
    0x0149, 0x008d,
    // end of stack_addr.i32 (I32)
    // 00070e: store.i32 (I32)
    // --> [Op1st#89]
    0x0088, 0x0089,
    // --> [Op1stDisp8#89]
    0x0090, 0x0089,
    // --> [Op1stDisp32#89] and stop
    0x0099, 0x0089,
    // end of store.i32 (I32)
    // 000714: store_complex.i32 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [Op1stWithIndex#89]
    0x0064, 0x0089,
    // --> [Op1stWithIndexDisp8#89]
    0x006c, 0x0089,
    // --> [Op1stWithIndexDisp32#89] and stop
    0x0075, 0x0089,
    // end of store_complex.i32 (I32)
    // 00071b: symbol_value.i32 (I32)
    // stop unless PredicateView(12)
    0x1029,
    // --> [Op1gvaddr4#b8] and stop
    0x0141, 0x00b8,
    // end of symbol_value.i32 (I32)
    // 00071e: uextend.i32 (I32)
    // skip 2 unless inst_predicate_11
    0x300b,
    // --> [Op2urm_noflags_abcd#4b6]
    0x01bc, 0x04b6,
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op2urm_noflags#4b7] and stop
    0x01c3, 0x04b7,
    // end of uextend.i32 (I32)
    // 000724: uload16.i32 (I32)
    // --> [Op2ld#4b7]
    0x00b2, 0x04b7,
    // --> [Op2ldDisp8#4b7]
    0x00ba, 0x04b7,
    // --> [Op2ldDisp32#4b7] and stop
    0x00c3, 0x04b7,
    // end of uload16.i32 (I32)
    // 00072a: uload16_complex.i32 (I32)
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2ldWithIndex#4b7]
    0x0050, 0x04b7,
    // --> [Op2ldWithIndexDisp8#4b7]
    0x0058, 0x04b7,
    // --> [Op2ldWithIndexDisp32#4b7] and stop
    0x0061, 0x04b7,
    // end of uload16_complex.i32 (I32)
    // 000731: uload8.i32 (I32)
    // --> [Op2ld#4b6]
    0x00b2, 0x04b6,
    // --> [Op2ldDisp8#4b6]
    0x00ba, 0x04b6,
    // --> [Op2ldDisp32#4b6] and stop
    0x00c3, 0x04b6,
    // end of uload8.i32 (I32)
    // 000737: uload8_complex.i32 (I32)
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2ldWithIndex#4b6]
    0x0050, 0x04b6,
    // --> [Op2ldWithIndexDisp8#4b6]
    0x0058, 0x04b6,
    // --> [Op2ldWithIndexDisp32#4b6] and stop
    0x0061, 0x04b6,
    // end of uload8_complex.i32 (I32)
    // 00073e: x86_cvtt2si.i32 (I32)
    // skip 2 unless inst_predicate_14
    0x300e,
    // --> [Mp2rfurm#62c]
    0x01e2, 0x062c,
    // stop unless inst_predicate_15
    0x100f,
    // --> [Mp2rfurm#72c] and stop
    0x01e3, 0x072c,
    // end of x86_cvtt2si.i32 (I32)
    // 000744: brnz.b1 (I32)
    // --> [Op1t8jccd_long#85]
    0x0172, 0x0085,
    // --> [Op1t8jccb_abcd#75]
    0x0174, 0x0075,
    // --> [Op1t8jccd_abcd#85] and stop
    0x0179, 0x0085,
    // end of brnz.b1 (I32)
    // 00074a: brz.b1 (I32)
    // --> [Op1t8jccd_long#84]
    0x0172, 0x0084,
    // --> [Op1t8jccb_abcd#74]
    0x0174, 0x0074,
    // --> [Op1t8jccd_abcd#84] and stop
    0x0179, 0x0084,
    // end of brz.b1 (I32)
    // 000750: is_null.r32 (I32)
    // --> [Op1is_zero#85] and stop
    0x0239, 0x0085,
    // end of is_null.r32 (I32)
    // 000752: iconst.i8 (I32)
    // stop unless inst_predicate_1
    0x1001,
    // --> [Op1u_id_z#30] and stop
    0x0041, 0x0030,
    // end of iconst.i8 (I32)
    // 000755: ireduce.i8 (I32)
    // skip 2 unless inst_predicate_8
    0x3008,
    // --> [null#00]
    0x01c0, 0x0000,
    // stop unless inst_predicate_9
    // 000758: ireduce.i16 (I32)
    // stop unless inst_predicate_9
    0x1009,
    // --> [null#00] and stop
    // --> [null#00] and stop
    0x01c1, 0x0000,
    // end of ireduce.i16 (I32)
    // end of ireduce.i8 (I32)
    // 00075b: regmove.i8 (I32)
    // --> [Op1rmov#89]
    0x0028, 0x0089,
    // --> [Op1rmov#89] and stop
    0x0029, 0x0089,
    // end of regmove.i8 (I32)
    // 00075f: ceil.f64 (I32)
    // stop unless PredicateView(16)
    // 00075f: floor.f64 (I32)
    // stop unless PredicateView(16)
    // 00075f: nearest.f64 (I32)
    // stop unless PredicateView(16)
    // 00075f: trunc.f64 (I32)
    // stop unless PredicateView(16)
    0x102d,
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    0x01e7, 0x0d0b,
    // end of trunc.f64 (I32)
    // end of nearest.f64 (I32)
    // end of floor.f64 (I32)
    // end of ceil.f64 (I32)
    // 000762: copy_to_ssa.f64 (I32)
    // --> [Mp2furm_reg_to_ssa#710] and stop
    0x00e3, 0x0710,
    // end of copy_to_ssa.f64 (I32)
    // 000764: fcvt_from_sint.f64 (I32)
    // stop unless inst_predicate_9
    0x1009,
    // --> [Mp2frurm#72a] and stop
    0x01cf, 0x072a,
    // end of fcvt_from_sint.f64 (I32)
    // 000767: fpromote.f64 (I32)
    // stop unless inst_predicate_14
    0x100e,
    // --> [Mp2furm#65a] and stop
    0x01df, 0x065a,
    // end of fpromote.f64 (I32)
    // 00076a: load.f64 (I32)
    // --> [Mp2fld#710]
    0x00f4, 0x0710,
    // --> [Mp2fldDisp8#710]
    0x00f8, 0x0710,
    // --> [Mp2fldDisp32#710] and stop
    0x00fd, 0x0710,
    // end of load.f64 (I32)
    // 000770: load_complex.f64 (I32)
    // --> [Mp2fldWithIndex#710]
    0x0100, 0x0710,
    // --> [Mp2fldWithIndexDisp8#710]
    0x0104, 0x0710,
    // --> [Mp2fldWithIndexDisp32#710] and stop
    0x0109, 0x0710,
    // end of load_complex.f64 (I32)
    // 000776: store.f64 (I32)
    // --> [Mp2fst#711]
    0x010c, 0x0711,
    // --> [Mp2fstDisp8#711]
    0x0110, 0x0711,
    // --> [Mp2fstDisp32#711] and stop
    0x0115, 0x0711,
    // end of store.f64 (I32)
    // 00077c: store_complex.f64 (I32)
    // --> [Mp2fstWithIndex#711]
    0x0118, 0x0711,
    // --> [Mp2fstWithIndexDisp8#711]
    0x011c, 0x0711,
    // --> [Mp2fstWithIndexDisp32#711] and stop
    0x0121, 0x0711,
    // end of store_complex.f64 (I32)
    // 000782: bitcast.f32 (I32)
    // stop unless inst_predicate_9
    0x1009,
    // --> [Mp2frurm#56e] and stop
    0x01cf, 0x056e,
    // end of bitcast.f32 (I32)
    // 000785: ceil.f32 (I32)
    // stop unless PredicateView(16)
    // 000785: floor.f32 (I32)
    // stop unless PredicateView(16)
    // 000785: nearest.f32 (I32)
    // stop unless PredicateView(16)
    // 000785: trunc.f32 (I32)
    // stop unless PredicateView(16)
    0x102d,
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    0x01e7, 0x0d0a,
    // end of trunc.f32 (I32)
    // end of nearest.f32 (I32)
    // end of floor.f32 (I32)
    // end of ceil.f32 (I32)
    // 000788: copy_to_ssa.f32 (I32)
    // --> [Mp2furm_reg_to_ssa#610] and stop
    0x00e3, 0x0610,
    // end of copy_to_ssa.f32 (I32)
    // 00078a: fcvt_from_sint.f32 (I32)
    // stop unless inst_predicate_9
    0x1009,
    // --> [Mp2frurm#62a] and stop
    0x01cf, 0x062a,
    // end of fcvt_from_sint.f32 (I32)
    // 00078d: fdemote.f32 (I32)
    // stop unless inst_predicate_15
    0x100f,
    // --> [Mp2furm#75a] and stop
    0x01df, 0x075a,
    // end of fdemote.f32 (I32)
    // 000790: load.f32 (I32)
    // --> [Mp2fld#610]
    0x00f4, 0x0610,
    // --> [Mp2fldDisp8#610]
    0x00f8, 0x0610,
    // --> [Mp2fldDisp32#610] and stop
    0x00fd, 0x0610,
    // end of load.f32 (I32)
    // 000796: load_complex.f32 (I32)
    // --> [Mp2fldWithIndex#610]
    0x0100, 0x0610,
    // --> [Mp2fldWithIndexDisp8#610]
    0x0104, 0x0610,
    // --> [Mp2fldWithIndexDisp32#610] and stop
    0x0109, 0x0610,
    // end of load_complex.f32 (I32)
    // 00079c: store.f32 (I32)
    // --> [Mp2fst#611]
    0x010c, 0x0611,
    // --> [Mp2fstDisp8#611]
    0x0110, 0x0611,
    // --> [Mp2fstDisp32#611] and stop
    0x0115, 0x0611,
    // end of store.f32 (I32)
    // 0007a2: store_complex.f32 (I32)
    // --> [Mp2fstWithIndex#611]
    0x0118, 0x0611,
    // --> [Mp2fstWithIndexDisp8#611]
    0x011c, 0x0611,
    // --> [Mp2fstWithIndexDisp32#611] and stop
    0x0121, 0x0611,
    // end of store_complex.f32 (I32)
    // 0007a8: adjust_sp_down_imm (I32)
    // --> [Op1adjustsp_ib#5083]
    0x00ec, 0x5083,
    // --> [Op1adjustsp_id#5081] and stop
    0x00ef, 0x5081,
    // end of adjust_sp_down_imm (I32)
    // 0007ac: adjust_sp_up_imm (I32)
    // --> [Op1adjustsp_ib#83]
    0x00ec, 0x0083,
    // --> [Op1adjustsp_id#81] and stop
    0x00ef, 0x0081,
    // end of adjust_sp_up_imm (I32)
    // 0007b0: brff (I32)
    // --> [Op1brfb#70]
    0x0162, 0x0070,
    // --> [Op2brfd#480] and stop
    0x0167, 0x0480,
    // end of brff (I32)
    // 0007b4: brif (I32)
    // --> [Op1brib#70]
    0x015a, 0x0070,
    // --> [Op2brid#480] and stop
    0x015f, 0x0480,
    // end of brif (I32)
    // 0007b8: call (I32)
    // --> [Op1call_id#e8] and stop
    0x014d, 0x00e8,
    // end of call (I32)
    // 0007ba: copy_special (I32)
    // --> [Op1copysp#89] and stop
    0x00dd, 0x0089,
    // end of copy_special (I32)
    // 0007bc: f32const (I32)
    // stop unless inst_predicate_12
    0x100c,
    // --> [Op2f32imm_z#457] and stop
    0x01c7, 0x0457,
    // end of f32const (I32)
    // 0007bf: f64const (I32)
    // stop unless inst_predicate_13
    0x100d,
    // --> [Mp2f64imm_z#557] and stop
    0x01c9, 0x0557,
];

/// x86 level 2 hash tables.
///
/// This hash table, keyed by instruction opcode, contains all the starting offsets for the
/// encodings interpreter, for all the CPU modes. It is jumped to after a lookup on the
/// instruction's controlling type in the level 1 hash table.
pub static LEVEL2: [Level2Entry<u16>; 1614] = [
    // I64
    // 000000: i64, 128 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x000004 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x000014 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x000018 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x000022 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x00001c },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x0000dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x0000da },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x000076 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableBase), offset: 0x0000c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IndirectJumpTableBr), offset: 0x000072 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000078 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x000165 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x0000e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableEntry), offset: 0x0000c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x00002a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x000113 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x000033 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x0000d1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000115 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x0000de },
    Level2Entry { opcode: Some(crate::ir::Opcode::CallIndirect), offset: 0x000026 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FuncAddr), offset: 0x00003a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0000c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0000ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000119 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00011f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x000158 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x00015e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x000104 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x00010a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x00009f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x0000ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x00013f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x000145 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x0000f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x0000f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x00007a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x000086 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x00000d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32), offset: 0x00014c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32), offset: 0x0000fe },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore32), offset: 0x000093 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000008 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StackAddr), offset: 0x000117 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x0000e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SymbolValue), offset: 0x000126 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x000130 },
    Level2Entry { opcode: Some(crate::ir::Opcode::GetPinnedReg), offset: 0x000046 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SetPinnedReg), offset: 0x0000e4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x00005a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x000167 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x00017f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x00017b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x000181 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x00017d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x0000e2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00002d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000111 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000036 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0000d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x000169 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000031 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDown), offset: 0x000000 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x00016d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x00016b },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpSp), offset: 0x00006e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0000d8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0000d4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Push), offset: 0x000177 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pop), offset: 0x000173 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000054 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x000056 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x000068 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x00006a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000048 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0000b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000070 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x000050 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x00004c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x00004e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x00004a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x0000ba },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x0000be },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x0000bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000002 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000012 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000020 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000010 },
    Level2Entry { opcode: None, offset: 0 },
    // 000080: i32, 128 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x000187 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x00019c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x0001a4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x0001b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x0001ac },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x000248 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x000244 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x000210 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x0002e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x000250 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000214 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0002ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x000298 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x00029c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x0001c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x0001c7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x000235 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x00024c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00014c },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000228 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000093 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0002a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x0002d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x0002dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x00027b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x000287 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x00009f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x0000ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x0002b7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x0002c3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x000262 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x00026e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x00007a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x000086 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x00018f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000008 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x00020d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x0002ad },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x000258 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x0001f4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x00030b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x000303 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x00030f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x000307 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x000254 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000294 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00023e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x0002f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x0002f9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x0002f5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000240 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0001e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x0001ec },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x0001fd },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x000201 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0001d0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000218 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000209 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x0001e0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x0001d8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x0001dc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x0001d4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x00021c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x000224 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x000220 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000183 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000198 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000194 },
    Level2Entry { opcode: None, offset: 0 },
    // 000100: b32, 8 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00023e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000313 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000183 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000198 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000194 },
    Level2Entry { opcode: None, offset: 0 },
    // 000108: b64, 8 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000317 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000002 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000012 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000020 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000010 },
    Level2Entry { opcode: None, offset: 0 },
    // 000110: b1, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000294 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x000321 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x000319 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000329 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000240 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000313 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000183 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000198 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001b4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000130: r64, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0000d8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00002d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000111 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000036 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsNull), offset: 0x00032d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0000d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000031 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Null), offset: 0x00032f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0000d4 },
    Level2Entry { opcode: None, offset: 0 },
    // 000140: i8, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000294 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000341 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000333 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000240 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x000338 },
    // 000150: i16, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000294 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00023e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000063 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000240 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x00033b },
    // 000160: b8, 4 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00023e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000313 },
    Level2Entry { opcode: None, offset: 0 },
    // 000164: b16, 4 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00023e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000313 },
    Level2Entry { opcode: None, offset: 0 },
    // 000168: r32, 2 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00023e },
    // 00016a: typeless, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Jump), offset: 0x000373 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brif), offset: 0x000357 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brff), offset: 0x00034f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueff), offset: 0x000388 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopySpecial), offset: 0x000365 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpUpImm), offset: 0x00034b },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDownImm), offset: 0x000347 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Debugtrap), offset: 0x000367 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore32Complex), offset: 0x0002a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32Complex), offset: 0x00037d },
    Level2Entry { opcode: Some(crate::ir::Opcode::ResumableTrap), offset: 0x000377 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Safepoint), offset: 0x00037b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapif), offset: 0x000386 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapff), offset: 0x000384 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Return), offset: 0x000379 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Call), offset: 0x00035f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trap), offset: 0x000377 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F32const), offset: 0x000369 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F64const), offset: 0x00036e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueif), offset: 0x00038c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32Complex), offset: 0x000228 },
    Level2Entry { opcode: None, offset: 0 },
    // 00018a: f64, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003a8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000419 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0003c6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000413 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0003ac },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x00043d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x000439 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000415 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00040f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x0003b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x0003c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x0003ae },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x0003d5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x0003cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x0003be },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0003e5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0003d9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00042d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00041d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000421 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x0003a3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x0003a3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x0003a3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x0003a3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000398 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003f1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fpromote), offset: 0x0003d0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x0003b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000390 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00039b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x00039f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000394 },
    // 0001ca: f32, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003a8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000496 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00046a },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000413 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00044b },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x0004ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x0004b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000492 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00048e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000451 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x000466 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x00044d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000472 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x00046e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000462 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000482 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000476 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0004aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00049a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00049e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x000446 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x000446 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x000446 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x000446 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000441 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003f1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdemote), offset: 0x00045d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x000455 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000390 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00039b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x00039f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000394 },
    // 00020a: b8x16, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0004cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000506 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000503 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00022a: b16x8, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00050c },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000530 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00052d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00024a: b32x4, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000559 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000532 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000553 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00026a: b64x2, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00055b },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x00057f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00057c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00028a: i8x16, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x0005ac },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000589 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000506 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000503 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x0005ae },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x0005b0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000584 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000582 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000587 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x0005aa },
    // 0002aa: i16x8, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00052d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000530 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0005b4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0005b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x0005dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0005b9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x0005e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x0005de },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0005b7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x0005e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 0002ea: i32x4, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0005e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000559 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000553 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0005e6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0005e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0005ec },
    Level2Entry { opcode: None, offset: 0 },
    // 00030a: i64x2, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000617 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x00057f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00057c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000611 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00060f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000615 },
    Level2Entry { opcode: None, offset: 0 },
    // 00032a: f32x4, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000559 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000638 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Insertps), offset: 0x000659 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000553 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000556 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00034a: f64x2, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00065c },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x00057f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movsd), offset: 0x00067f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00057c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movlhps), offset: 0x00067d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // I32
    // 00036a: i32, 128 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x000683 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x00068d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x000691 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x000699 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x000695 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x00024a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x000246 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x000212 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableBase), offset: 0x0006d8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IndirectJumpTableBr), offset: 0x000074 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000216 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x0002eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x000252 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableEntry), offset: 0x0006da },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x00069d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x00029a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x0006a2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x0006e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x00029e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x00024e },
    Level2Entry { opcode: Some(crate::ir::Opcode::CallIndirect), offset: 0x000028 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FuncAddr), offset: 0x0006a5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0006dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0006e2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00070e },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x000714 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x000731 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x000737 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x0006ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x000705 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x0006cb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x0006d1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x000724 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x00072a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x0006f2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x0006f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x0006be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x0006c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x00068a },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0002ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000687 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StackAddr), offset: 0x00070c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x0006ec },
    Level2Entry { opcode: Some(crate::ir::Opcode::SymbolValue), offset: 0x00071b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x00071e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x0006b3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x00030d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x000305 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x000311 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x000309 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x000256 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000296 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00032b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x0002f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDown), offset: 0x000681 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x00073e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x0002f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpSp), offset: 0x0006bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000242 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Push), offset: 0x000179 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pop), offset: 0x000175 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0001ea },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x0006af },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x0001ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x0006b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0001d2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x00021a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x0006ab },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x0001da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x0001de },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x0001d5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x00021e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x000226 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x000222 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000185 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00019a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000196 },
    Level2Entry { opcode: None, offset: 0 },
    // 0003ea: b32, 8 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00032b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000315 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000185 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00019a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001b6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x000196 },
    Level2Entry { opcode: None, offset: 0 },
    // 0003f2: b1, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000296 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x00074a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x000744 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00032b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000242 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000315 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000185 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00019a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001b6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000412: r32, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000242 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000296 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ce },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsNull), offset: 0x000750 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00032b },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006a0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Null), offset: 0x000331 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023c },
    Level2Entry { opcode: None, offset: 0 },
    // 000422: i8, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000296 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00075b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000752 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000242 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x000755 },
    // 000432: i16, 16 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00013d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000296 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00032b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0006a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x0006b5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000242 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00023c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x000758 },
    // 000442: b8, 4 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00032b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000315 },
    Level2Entry { opcode: None, offset: 0 },
    // 000446: b16, 4 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00032b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000315 },
    Level2Entry { opcode: None, offset: 0 },
    // 00044a: i64, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000038 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 00044e: f64, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00041b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0003c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000762 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x00043f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x00043b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000417 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000411 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x0003b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x0003c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x0003b0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x0003d7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x0003ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x0003c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000770 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00076a },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00077c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00041f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000776 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x00075f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x00075f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x00075f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x00075f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003f1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fpromote), offset: 0x000767 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x000764 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000392 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00039d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0003a1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000396 },
    // 00048e: f32, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000498 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00046c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000788 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x0004bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x0004b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000494 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000490 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000453 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x000468 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x00044f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000474 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000470 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000464 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000796 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000790 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0007a2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00049c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00079c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x000785 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x000785 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x000785 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x000785 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000782 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0003f1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdemote), offset: 0x00078d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x00078a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000392 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00039d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0003a1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000396 },
    // 0004ce: typeless, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Jump), offset: 0x000373 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brif), offset: 0x0007b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brff), offset: 0x0007b0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueff), offset: 0x00038a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopySpecial), offset: 0x0007ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpUpImm), offset: 0x0007ac },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDownImm), offset: 0x0007a8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Debugtrap), offset: 0x000367 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trap), offset: 0x000377 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ResumableTrap), offset: 0x000377 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Safepoint), offset: 0x00037b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapif), offset: 0x000386 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapff), offset: 0x000384 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Return), offset: 0x000379 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Call), offset: 0x0007b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F32const), offset: 0x0007bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::F64const), offset: 0x0007bf },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueif), offset: 0x00038e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0004ee: b8x16, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0004cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000444 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000506 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000503 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00050e: b16x8, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00050c },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000444 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000530 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00052d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00052e: b32x4, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000559 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000532 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000444 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000553 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00054e: b64x2, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00055b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00056e: i8x16, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x0005ac },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000589 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000444 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000506 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000503 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x0005ae },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x0005b0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000584 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000582 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000587 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x0005aa },
    // 00058e: i16x8, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x00052d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000530 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0005b4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0005b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x0005dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0005b9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x0005e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x0005de },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0005b7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x0005e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000444 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 0005ce: i32x4, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0005e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000559 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000444 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000553 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0005e6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0005e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0005ec },
    Level2Entry { opcode: None, offset: 0 },
    // 0005ee: i64x2, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000617 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000611 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00060f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000615 },
    Level2Entry { opcode: None, offset: 0 },
    // 00060e: f32x4, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000559 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000638 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Insertps), offset: 0x000659 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000553 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000556 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
    // 00062e: f64x2, 32 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0003aa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x0004f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004c4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x0003ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0004ef },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00002f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00065c },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00040d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x000509 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movsd), offset: 0x00067f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0004f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0004ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movlhps), offset: 0x00067d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0004be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0004c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0004c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0004fb },
    Level2Entry { opcode: None, offset: 0 },
];

/// x86 level 1 hash table for the CPU mode I64.
///
/// This hash table, keyed by instruction controlling type, contains all the level 2
/// hash-tables offsets for the given CPU mode, as well as a legalization identifier indicating
/// which legalization scheme to apply when the instruction doesn't have any valid encoding for
/// this CPU mode.
pub static LEVEL1_I64: [Level1Entry<u16>; 32] = [
    Level1Entry { ty: ir::types::INVALID, log2len: 5, offset: 0x00016a, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::F32X4, log2len: 5, offset: 0x00032a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::B16X8, log2len: 5, offset: 0x00022a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::B64X2, log2len: 5, offset: 0x00026a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::I8X16, log2len: 5, offset: 0x00028a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::B8X16, log2len: 5, offset: 0x00020a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::I16X8, log2len: 6, offset: 0x0002aa, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::I64X2, log2len: 5, offset: 0x00030a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::F64X2, log2len: 5, offset: 0x00034a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::I32X4, log2len: 5, offset: 0x0002ea, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::B1, log2len: 5, offset: 0x000110, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::B8, log2len: 2, offset: 0x000160, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::B16, log2len: 2, offset: 0x000164, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::B32, log2len: 3, offset: 0x000100, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::B64, log2len: 3, offset: 0x000108, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::I8, log2len: 4, offset: 0x000140, legalize: 1 }, // widen
    Level1Entry { ty: ir::types::I16, log2len: 4, offset: 0x000150, legalize: 1 }, // widen
    Level1Entry { ty: ir::types::I32, log2len: 7, offset: 0x000080, legalize: 2 }, // x86_expand
    Level1Entry { ty: ir::types::I64, log2len: 7, offset: 0x000000, legalize: 2 }, // x86_expand
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 3 },
    Level1Entry { ty: ir::types::F32, log2len: 6, offset: 0x0001ca, legalize: 2 }, // x86_expand
    Level1Entry { ty: ir::types::F64, log2len: 6, offset: 0x00018a, legalize: 2 }, // x86_expand
    Level1Entry { ty: ir::types::B32X4, log2len: 5, offset: 0x00024a, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::R32, log2len: 1, offset: 0x000168, legalize: 3 }, // x86_narrow
    Level1Entry { ty: ir::types::R64, log2len: 4, offset: 0x000130, legalize: 3 }, // x86_narrow
];

/// x86 level 1 hash table for the CPU mode I32.
///
/// This hash table, keyed by instruction controlling type, contains all the level 2
/// hash-tables offsets for the given CPU mode, as well as a legalization identifier indicating
/// which legalization scheme to apply when the instruction doesn't have any valid encoding for
/// this CPU mode.
pub static LEVEL1_I32: [Level1Entry<u16>; 32] = [
    Level1Entry { ty: ir::types::INVALID, log2len: 5, offset: 0x0004ce, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::F32X4, log2len: 5, offset: 0x00060e, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::B16X8, log2len: 5, offset: 0x00050e, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::B64X2, log2len: 5, offset: 0x00054e, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::I8X16, log2len: 5, offset: 0x00056e, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::I16X8, log2len: 6, offset: 0x00058e, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::I64X2, log2len: 5, offset: 0x0005ee, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::F64X2, log2len: 5, offset: 0x00062e, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::I32X4, log2len: 5, offset: 0x0005ce, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::B1, log2len: 5, offset: 0x0003f2, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::B8, log2len: 2, offset: 0x000442, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::B16, log2len: 2, offset: 0x000446, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::B32, log2len: 3, offset: 0x0003ea, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::B8X16, log2len: 5, offset: 0x0004ee, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::I8, log2len: 4, offset: 0x000422, legalize: 1 }, // widen
    Level1Entry { ty: ir::types::I16, log2len: 4, offset: 0x000432, legalize: 1 }, // widen
    Level1Entry { ty: ir::types::I32, log2len: 7, offset: 0x00036a, legalize: 2 }, // x86_expand
    Level1Entry { ty: ir::types::I64, log2len: 2, offset: 0x00044a, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
    Level1Entry { ty: ir::types::F32, log2len: 6, offset: 0x00048e, legalize: 2 }, // x86_expand
    Level1Entry { ty: ir::types::F64, log2len: 6, offset: 0x00044e, legalize: 2 }, // x86_expand
    Level1Entry { ty: ir::types::B32X4, log2len: 5, offset: 0x00052e, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::R32, log2len: 4, offset: 0x000412, legalize: 4 }, // narrow_flags
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 4 },
];

/// x86 recipe names, using the same recipe index spaces as the one specified by the
/// corresponding binemit file.
static RECIPE_NAMES: [&str; 287] = [
    "get_pinned_reg",
    "RexOp1set_pinned_reg",
    "Op1rr",
    "RexOp1rr",
    "Op1rout",
    "RexOp1rout",
    "Op1rin",
    "RexOp1rin",
    "Op1rio",
    "RexOp1rio",
    "Op1ur",
    "RexOp1ur",
    "Op2rrx",
    "RexOp2rrx",
    "Op1div",
    "RexOp1div",
    "Op1mulx",
    "RexOp1mulx",
    "Op1umr",
    "RexOp1umr",
    "Op1rmov",
    "RexOp1rmov",
    "Op1r_ib",
    "RexOp1r_ib",
    "Op1r_id",
    "RexOp1r_id",
    "Op1pu_id",
    "RexOp1pu_id",
    "RexOp1u_id",
    "RexOp1pu_iq",
    "Op1pu_id_bool",
    "RexOp1pu_id_bool",
    "Op1u_id_z",
    "RexOp1u_id_z",
    "Op1rc",
    "RexOp1rc",
    "Mp2urm",
    "RexMp2urm",
    "Op1ldWithIndex",
    "RexOp1ldWithIndex",
    "Op2ldWithIndex",
    "RexOp2ldWithIndex",
    "Op1ldWithIndexDisp8",
    "RexOp1ldWithIndexDisp8",
    "Op2ldWithIndexDisp8",
    "RexOp2ldWithIndexDisp8",
    "Op1ldWithIndexDisp32",
    "RexOp1ldWithIndexDisp32",
    "Op2ldWithIndexDisp32",
    "RexOp2ldWithIndexDisp32",
    "Op1stWithIndex",
    "RexOp1stWithIndex",
    "Mp1stWithIndex",
    "RexMp1stWithIndex",
    "Op1stWithIndexDisp8",
    "RexOp1stWithIndexDisp8",
    "Mp1stWithIndexDisp8",
    "RexMp1stWithIndexDisp8",
    "Op1stWithIndexDisp32",
    "RexOp1stWithIndexDisp32",
    "Mp1stWithIndexDisp32",
    "RexMp1stWithIndexDisp32",
    "Op1stWithIndex_abcd",
    "RexOp1stWithIndex_abcd",
    "Op1stWithIndexDisp8_abcd",
    "RexOp1stWithIndexDisp8_abcd",
    "Op1stWithIndexDisp32_abcd",
    "RexOp1stWithIndexDisp32_abcd",
    "Op1st",
    "RexOp1st",
    "Mp1st",
    "RexMp1st",
    "Op1stDisp8",
    "RexOp1stDisp8",
    "Mp1stDisp8",
    "RexMp1stDisp8",
    "Op1stDisp32",
    "RexOp1stDisp32",
    "Mp1stDisp32",
    "RexMp1stDisp32",
    "Op1st_abcd",
    "Op1stDisp8_abcd",
    "Op1stDisp32_abcd",
    "Op1spillSib32",
    "RexOp1spillSib32",
    "Op1regspill32",
    "RexOp1regspill32",
    "Op1ld",
    "RexOp1ld",
    "Op2ld",
    "RexOp2ld",
    "Op1ldDisp8",
    "RexOp1ldDisp8",
    "Op2ldDisp8",
    "RexOp2ldDisp8",
    "Op1ldDisp32",
    "RexOp1ldDisp32",
    "Op2ldDisp32",
    "RexOp2ldDisp32",
    "Op1fillSib32",
    "RexOp1fillSib32",
    "Op1regfill32",
    "RexOp1regfill32",
    "fillnull",
    "ffillnull",
    "Op1pushq",
    "RexOp1pushq",
    "Op1popq",
    "RexOp1popq",
    "RexOp1copysp",
    "Op1copysp",
    "Op1umr_reg_to_ssa",
    "RexOp1umr_reg_to_ssa",
    "Mp2furm_reg_to_ssa",
    "RexMp2furm_reg_to_ssa",
    "stacknull",
    "Op1adjustsp",
    "RexOp1adjustsp",
    "Op1adjustsp_ib",
    "Op1adjustsp_id",
    "RexOp1adjustsp_ib",
    "RexOp1adjustsp_id",
    "Mp2fld",
    "RexMp2fld",
    "Mp2fldDisp8",
    "RexMp2fldDisp8",
    "Mp2fldDisp32",
    "RexMp2fldDisp32",
    "Mp2fldWithIndex",
    "RexMp2fldWithIndex",
    "Mp2fldWithIndexDisp8",
    "RexMp2fldWithIndexDisp8",
    "Mp2fldWithIndexDisp32",
    "RexMp2fldWithIndexDisp32",
    "Mp2fst",
    "RexMp2fst",
    "Mp2fstDisp8",
    "RexMp2fstDisp8",
    "Mp2fstDisp32",
    "RexMp2fstDisp32",
    "Mp2fstWithIndex",
    "RexMp2fstWithIndex",
    "Mp2fstWithIndexDisp8",
    "RexMp2fstWithIndexDisp8",
    "Mp2fstWithIndexDisp32",
    "RexMp2fstWithIndexDisp32",
    "Mp2ffillSib32",
    "RexMp2ffillSib32",
    "Mp2fregfill32",
    "RexMp2fregfill32",
    "Mp2fspillSib32",
    "RexMp2fspillSib32",
    "Mp2fregspill32",
    "RexMp2fregspill32",
    "Op1fnaddr4",
    "RexOp1fnaddr8",
    "Op1allones_fnaddr4",
    "RexOp1allones_fnaddr8",
    "RexOp1pcrel_fnaddr8",
    "RexOp1got_fnaddr8",
    "Op1gvaddr4",
    "RexOp1gvaddr8",
    "RexOp1pcrel_gvaddr8",
    "RexOp1got_gvaddr8",
    "Op1spaddr4_id",
    "RexOp1spaddr8_id",
    "Op1call_id",
    "Op1call_plt_id",
    "Op1call_r",
    "RexOp1call_r",
    "Op1ret",
    "Op1jmpb",
    "Op1jmpd",
    "Op1brib",
    "RexOp1brib",
    "Op2brid",
    "RexOp2brid",
    "Op1brfb",
    "RexOp1brfb",
    "Op2brfd",
    "RexOp2brfd",
    "Op1tjccb",
    "RexOp1tjccb",
    "Op1tjccd",
    "RexOp1tjccd",
    "Op1t8jccd_long",
    "Op1t8jccb_abcd",
    "RexOp1t8jccb",
    "Op1t8jccd_abcd",
    "RexOp1t8jccd",
    "RexOp1jt_entry",
    "Op1jt_entry",
    "RexOp1jt_base",
    "Op1jt_base",
    "RexOp1indirect_jmp",
    "Op1indirect_jmp",
    "Op2trap",
    "debugtrap",
    "trapif",
    "trapff",
    "Op1icscc",
    "RexOp1icscc",
    "Op1icscc_ib",
    "RexOp1icscc_ib",
    "Op1icscc_id",
    "RexOp1icscc_id",
    "Op1rcmp",
    "RexOp1rcmp",
    "Op1rcmp_ib",
    "RexOp1rcmp_ib",
    "Op1rcmp_id",
    "RexOp1rcmp_id",
    "Op1rcmp_sp",
    "RexOp1rcmp_sp",
    "Op2seti_abcd",
    "RexOp2seti",
    "Op2setf_abcd",
    "RexOp2setf",
    "Op2cmov",
    "RexOp2cmov",
    "Op2bsf_and_bsr",
    "RexOp2bsf_and_bsr",
    "Op2urm_noflags_abcd",
    "RexOp2urm_noflags",
    "null",
    "Op2urm_noflags",
    "RexOp1urm_noflags",
    "Op2f32imm_z",
    "Mp2f64imm_z",
    "RexOp2f32imm_z",
    "RexMp2f64imm_z",
    "Mp2frurm",
    "RexMp2frurm",
    "Mp2rfumr",
    "RexMp2rfumr",
    "Op2furm",
    "RexOp2furm",
    "Op2frmov",
    "RexOp2frmov",
    "Mp2furm",
    "RexMp2furm",
    "Mp2rfurm",
    "RexMp2rfurm",
    "Mp3furmi_rnd",
    "RexMp3furmi_rnd",
    "Mp2fa",
    "RexMp2fa",
    "Op2fa",
    "RexOp2fa",
    "Op2fax",
    "RexOp2fax",
    "Op2fcscc",
    "RexOp2fcscc",
    "Mp2fcscc",
    "RexMp2fcscc",
    "Op2fcmp",
    "RexOp2fcmp",
    "Mp2fcmp",
    "RexMp2fcmp",
    "Mp3fa",
    "Mp2r_ib_unsigned_fpr",
    "null_fpr",
    "Mp3r_ib_unsigned_r",
    "Mp2r_ib_unsigned_r",
    "RexMp3r_ib_unsigned_r",
    "Mp3fa_ib",
    "Mp3r_ib_unsigned_gpr",
    "RexMp3r_ib_unsigned_gpr",
    "Mp2vconst_optimized",
    "Op2vconst",
    "Op2fst",
    "Op2fstDisp8",
    "Op2fstDisp32",
    "Op2fld",
    "Op2fldDisp8",
    "Op2fldDisp32",
    "Op2fspillSib32",
    "Op2fregspill32",
    "Op2ffillSib32",
    "Op2fregfill32",
    "Mp2icscc_fpr",
    "Mp3icscc_fpr",
    "Op1pu_id_ref",
    "RexOp1pu_id_ref",
    "Op1is_zero",
    "RexOp1is_zero",
    "safepoint",
];

/// x86 recipe constraints list, using the same recipe index spaces as the one
/// specified by the corresponding binemit file. These constraints are used by register
/// allocation to select the right location to use for input and output values.
static RECIPE_CONSTRAINTS: [RecipeConstraints; 287] = [
    // Constraints for recipe get_pinned_reg:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(15),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1set_pinned_reg:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rout:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rout:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rin:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rin:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rio:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rio:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1ur:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1ur:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2rrx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2rrx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1div:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(2),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1div:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(2),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1mulx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1mulx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1rmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1rmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1r_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1r_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1r_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1r_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1pu_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1u_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_iq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1pu_id_bool:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id_bool:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1u_id_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1u_id_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(1),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(1),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2urm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2urm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndex_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndex_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1st_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1spillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1spillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1regspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1regspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1fillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1fillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1regfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1regfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe fillnull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe ffillnull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1pushq:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pushq:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1popq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1popq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1copysp:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1copysp:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1umr_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1umr_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe stacknull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp_ib:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp_ib:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1fnaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1allones_fnaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1allones_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pcrel_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1got_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1gvaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pcrel_gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1got_gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1spaddr4_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1spaddr8_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_plt_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1call_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1ret:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1jmpb:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jmpd:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1brib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1brib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2brid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2brid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1brfb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1brfb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2brfd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2brfd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1tjccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1tjccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1tjccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1tjccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccd_long:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccb_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1t8jccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccd_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1t8jccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1jt_entry:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jt_entry:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1jt_base:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jt_base:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1indirect_jmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1indirect_jmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2trap:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe debugtrap:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe trapif:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe trapff:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1icscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1icscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1icscc_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1icscc_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1icscc_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1icscc_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rcmp_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rcmp_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rcmp_sp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp_sp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2seti_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2seti:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2setf_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2setf:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2cmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(2),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2cmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2bsf_and_bsr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2bsf_and_bsr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2urm_noflags_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe null:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2f32imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2f64imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2f32imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2f64imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2rfumr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2rfumr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2frmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2frmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2rfurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2rfurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3furmi_rnd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3furmi_rnd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2r_ib_unsigned_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe null_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3fa_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3r_ib_unsigned_gpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3r_ib_unsigned_gpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2vconst_optimized:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2vconst:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1pu_id_ref:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id_ref:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1is_zero:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1is_zero:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe safepoint:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
];

/// x86 recipe sizing descriptors, using the same recipe index spaces as the one
/// specified by the corresponding binemit file. These are used to compute the final size of an
/// instruction, as well as to compute the range of branches.
static RECIPE_SIZING: [RecipeSizing; 287] = [
    // Code size information for recipe get_pinned_reg:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1set_pinned_reg:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rr:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rr:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rout:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rout:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rin:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rin:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rio:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rio:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ur:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ur:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2rrx:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2rrx:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1div:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1div:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1mulx:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1mulx:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1umr:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1umr:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rmov:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rmov:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1r_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1r_ib:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1r_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1r_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1u_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_iq:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id_bool:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id_bool:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1u_id_z:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1u_id_z:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rc:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rc:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2urm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2urm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndex:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndexDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndexDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndex:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndex_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndex_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp8_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp8_abcd:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp32_abcd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp32_abcd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1st:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1st:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1st:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1st:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp8:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp32:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1st_abcd:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp8_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp32_abcd:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1spillSib32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1spillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1regspill32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1regspill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ld:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ld:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldDisp8:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldDisp32:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1fillSib32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1fillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1regfill32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1regfill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe fillnull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe ffillnull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pushq:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pushq:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1popq:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1popq:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1copysp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1copysp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1umr_reg_to_ssa:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1umr_reg_to_ssa:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe stacknull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp_ib:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fld:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fld:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndex:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fst:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fst:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndex:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2ffillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2ffillSib32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fregfill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fregfill32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fspillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fspillSib32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fregspill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fregspill32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1fnaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1fnaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1allones_fnaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1allones_fnaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pcrel_fnaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1got_fnaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1gvaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1gvaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pcrel_gvaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1got_gvaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1spaddr4_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1spaddr8_id:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_plt_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_r:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1call_r:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ret:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1jmpb:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe Op1jmpd:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 32 }),
    },
    // Code size information for recipe Op1brib:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe RexOp1brib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 3, bits: 8 }),
    },
    // Code size information for recipe Op2brid:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 6, bits: 32 }),
    },
    // Code size information for recipe RexOp2brid:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 7, bits: 32 }),
    },
    // Code size information for recipe Op1brfb:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe RexOp1brfb:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 3, bits: 8 }),
    },
    // Code size information for recipe Op2brfd:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 6, bits: 32 }),
    },
    // Code size information for recipe RexOp2brfd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 7, bits: 32 }),
    },
    // Code size information for recipe Op1tjccb:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 4, bits: 8 }),
    },
    // Code size information for recipe RexOp1tjccb:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 8 }),
    },
    // Code size information for recipe Op1tjccd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 8, bits: 32 }),
    },
    // Code size information for recipe RexOp1tjccd:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 9, bits: 32 }),
    },
    // Code size information for recipe Op1t8jccd_long:
    RecipeSizing {
        base_size: 12,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 12, bits: 32 }),
    },
    // Code size information for recipe Op1t8jccb_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 4, bits: 8 }),
    },
    // Code size information for recipe RexOp1t8jccb:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 8 }),
    },
    // Code size information for recipe Op1t8jccd_abcd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 8, bits: 32 }),
    },
    // Code size information for recipe RexOp1t8jccd:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 9, bits: 32 }),
    },
    // Code size information for recipe RexOp1jt_entry:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1jt_entry:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1jt_base:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1jt_base:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1indirect_jmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1indirect_jmp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2trap:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe debugtrap:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe trapif:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe trapff:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1icscc:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1icscc:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1icscc_ib:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1icscc_ib:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1icscc_id:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1icscc_id:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rcmp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rcmp_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp_ib:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rcmp_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rcmp_sp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp_sp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2seti_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2seti:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2setf_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2setf:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2cmov:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2cmov:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2bsf_and_bsr:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2bsf_and_bsr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2urm_noflags_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2urm_noflags:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe null:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2urm_noflags:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1urm_noflags:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2f32imm_z:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2f64imm_z:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2f32imm_z:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2f64imm_z:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2frurm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2frurm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2rfumr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2rfumr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2furm:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2furm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2frmov:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2frmov:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2furm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2furm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2rfurm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2rfurm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3furmi_rnd:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3furmi_rnd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fa:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fax:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fax:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fcscc:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fcscc:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fcscc:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fcscc:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fcmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3fa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2r_ib_unsigned_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe null_fpr:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3r_ib_unsigned_r:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2r_ib_unsigned_r:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3r_ib_unsigned_r:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3fa_ib:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3r_ib_unsigned_gpr:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3r_ib_unsigned_gpr:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2vconst_optimized:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2vconst:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fst:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_in_reg_1,
        branch_range: None,
    },
    // Code size information for recipe Op2fld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_in_reg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2fspillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fregspill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ffillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fregfill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2icscc_fpr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3icscc_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id_ref:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id_ref:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1is_zero:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1is_zero:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe safepoint:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
];

pub static INFO: isa::EncInfo = isa::EncInfo {
    constraints: &RECIPE_CONSTRAINTS,
    sizing: &RECIPE_SIZING,
    names: &RECIPE_NAMES,
};
