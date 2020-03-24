/// An instruction format
///
/// Every opcode has a corresponding instruction format
/// which is represented by both the `InstructionFormat`
/// and the `InstructionData` enums.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InstructionFormat {
    /// Unary(imms=(), vals=1)
    Unary,
    /// UnaryImm(imms=(imm: imm64), vals=0)
    UnaryImm,
    /// UnaryIeee32(imms=(imm: ieee32), vals=0)
    UnaryIeee32,
    /// UnaryIeee64(imms=(imm: ieee64), vals=0)
    UnaryIeee64,
    /// UnaryBool(imms=(imm: boolean), vals=0)
    UnaryBool,
    /// UnaryConst(imms=(constant_handle: poolConstant), vals=0)
    UnaryConst,
    /// UnaryGlobalValue(imms=(global_value: global_value), vals=0)
    UnaryGlobalValue,
    /// Binary(imms=(), vals=2)
    Binary,
    /// BinaryImm(imms=(imm: imm64), vals=1)
    BinaryImm,
    /// Ternary(imms=(), vals=3)
    Ternary,
    /// MultiAry(imms=(), vals=0)
    MultiAry,
    /// NullAry(imms=(), vals=0)
    NullAry,
    /// InsertLane(imms=(lane: uimm8), vals=2)
    InsertLane,
    /// ExtractLane(imms=(lane: uimm8), vals=1)
    ExtractLane,
    /// Shuffle(imms=(mask: uimm128), vals=2)
    Shuffle,
    /// IntCompare(imms=(cond: intcc), vals=2)
    IntCompare,
    /// IntCompareImm(imms=(cond: intcc, imm: imm64), vals=1)
    IntCompareImm,
    /// IntCond(imms=(cond: intcc), vals=1)
    IntCond,
    /// FloatCompare(imms=(cond: floatcc), vals=2)
    FloatCompare,
    /// FloatCond(imms=(cond: floatcc), vals=1)
    FloatCond,
    /// IntSelect(imms=(cond: intcc), vals=3)
    IntSelect,
    /// Jump(imms=(destination: ebb), vals=0)
    Jump,
    /// Branch(imms=(destination: ebb), vals=1)
    Branch,
    /// BranchInt(imms=(cond: intcc, destination: ebb), vals=1)
    BranchInt,
    /// BranchFloat(imms=(cond: floatcc, destination: ebb), vals=1)
    BranchFloat,
    /// BranchIcmp(imms=(cond: intcc, destination: ebb), vals=2)
    BranchIcmp,
    /// BranchTable(imms=(destination: ebb, table: jump_table), vals=1)
    BranchTable,
    /// BranchTableEntry(imms=(imm: uimm8, table: jump_table), vals=2)
    BranchTableEntry,
    /// BranchTableBase(imms=(table: jump_table), vals=0)
    BranchTableBase,
    /// IndirectJump(imms=(table: jump_table), vals=1)
    IndirectJump,
    /// Call(imms=(func_ref: func_ref), vals=0)
    Call,
    /// CallIndirect(imms=(sig_ref: sig_ref), vals=1)
    CallIndirect,
    /// FuncAddr(imms=(func_ref: func_ref), vals=0)
    FuncAddr,
    /// Load(imms=(flags: memflags, offset: offset32), vals=1)
    Load,
    /// LoadComplex(imms=(flags: memflags, offset: offset32), vals=0)
    LoadComplex,
    /// Store(imms=(flags: memflags, offset: offset32), vals=2)
    Store,
    /// StoreComplex(imms=(flags: memflags, offset: offset32), vals=1)
    StoreComplex,
    /// StackLoad(imms=(stack_slot: stack_slot, offset: offset32), vals=0)
    StackLoad,
    /// StackStore(imms=(stack_slot: stack_slot, offset: offset32), vals=1)
    StackStore,
    /// HeapAddr(imms=(heap: heap, imm: uimm32), vals=1)
    HeapAddr,
    /// TableAddr(imms=(table: table, offset: offset32), vals=1)
    TableAddr,
    /// RegMove(imms=(src: regunit, dst: regunit), vals=1)
    RegMove,
    /// CopySpecial(imms=(src: regunit, dst: regunit), vals=0)
    CopySpecial,
    /// CopyToSsa(imms=(src: regunit), vals=0)
    CopyToSsa,
    /// RegSpill(imms=(src: regunit, dst: stack_slot), vals=1)
    RegSpill,
    /// RegFill(imms=(src: stack_slot, dst: regunit), vals=1)
    RegFill,
    /// Trap(imms=(code: trapcode), vals=0)
    Trap,
    /// CondTrap(imms=(code: trapcode), vals=1)
    CondTrap,
    /// IntCondTrap(imms=(cond: intcc, code: trapcode), vals=1)
    IntCondTrap,
    /// FloatCondTrap(imms=(cond: floatcc, code: trapcode), vals=1)
    FloatCondTrap,
}

impl<'a> From<&'a InstructionData> for InstructionFormat {
    fn from(inst: &'a InstructionData) -> Self {
        match *inst {
            InstructionData::Binary { .. } => {
                InstructionFormat::Binary
            }
            InstructionData::BinaryImm { .. } => {
                InstructionFormat::BinaryImm
            }
            InstructionData::Branch { .. } => {
                InstructionFormat::Branch
            }
            InstructionData::BranchFloat { .. } => {
                InstructionFormat::BranchFloat
            }
            InstructionData::BranchIcmp { .. } => {
                InstructionFormat::BranchIcmp
            }
            InstructionData::BranchInt { .. } => {
                InstructionFormat::BranchInt
            }
            InstructionData::BranchTable { .. } => {
                InstructionFormat::BranchTable
            }
            InstructionData::BranchTableBase { .. } => {
                InstructionFormat::BranchTableBase
            }
            InstructionData::BranchTableEntry { .. } => {
                InstructionFormat::BranchTableEntry
            }
            InstructionData::Call { .. } => {
                InstructionFormat::Call
            }
            InstructionData::CallIndirect { .. } => {
                InstructionFormat::CallIndirect
            }
            InstructionData::CondTrap { .. } => {
                InstructionFormat::CondTrap
            }
            InstructionData::CopySpecial { .. } => {
                InstructionFormat::CopySpecial
            }
            InstructionData::CopyToSsa { .. } => {
                InstructionFormat::CopyToSsa
            }
            InstructionData::ExtractLane { .. } => {
                InstructionFormat::ExtractLane
            }
            InstructionData::FloatCompare { .. } => {
                InstructionFormat::FloatCompare
            }
            InstructionData::FloatCond { .. } => {
                InstructionFormat::FloatCond
            }
            InstructionData::FloatCondTrap { .. } => {
                InstructionFormat::FloatCondTrap
            }
            InstructionData::FuncAddr { .. } => {
                InstructionFormat::FuncAddr
            }
            InstructionData::HeapAddr { .. } => {
                InstructionFormat::HeapAddr
            }
            InstructionData::IndirectJump { .. } => {
                InstructionFormat::IndirectJump
            }
            InstructionData::InsertLane { .. } => {
                InstructionFormat::InsertLane
            }
            InstructionData::IntCompare { .. } => {
                InstructionFormat::IntCompare
            }
            InstructionData::IntCompareImm { .. } => {
                InstructionFormat::IntCompareImm
            }
            InstructionData::IntCond { .. } => {
                InstructionFormat::IntCond
            }
            InstructionData::IntCondTrap { .. } => {
                InstructionFormat::IntCondTrap
            }
            InstructionData::IntSelect { .. } => {
                InstructionFormat::IntSelect
            }
            InstructionData::Jump { .. } => {
                InstructionFormat::Jump
            }
            InstructionData::Load { .. } => {
                InstructionFormat::Load
            }
            InstructionData::LoadComplex { .. } => {
                InstructionFormat::LoadComplex
            }
            InstructionData::MultiAry { .. } => {
                InstructionFormat::MultiAry
            }
            InstructionData::NullAry { .. } => {
                InstructionFormat::NullAry
            }
            InstructionData::RegFill { .. } => {
                InstructionFormat::RegFill
            }
            InstructionData::RegMove { .. } => {
                InstructionFormat::RegMove
            }
            InstructionData::RegSpill { .. } => {
                InstructionFormat::RegSpill
            }
            InstructionData::Shuffle { .. } => {
                InstructionFormat::Shuffle
            }
            InstructionData::StackLoad { .. } => {
                InstructionFormat::StackLoad
            }
            InstructionData::StackStore { .. } => {
                InstructionFormat::StackStore
            }
            InstructionData::Store { .. } => {
                InstructionFormat::Store
            }
            InstructionData::StoreComplex { .. } => {
                InstructionFormat::StoreComplex
            }
            InstructionData::TableAddr { .. } => {
                InstructionFormat::TableAddr
            }
            InstructionData::Ternary { .. } => {
                InstructionFormat::Ternary
            }
            InstructionData::Trap { .. } => {
                InstructionFormat::Trap
            }
            InstructionData::Unary { .. } => {
                InstructionFormat::Unary
            }
            InstructionData::UnaryBool { .. } => {
                InstructionFormat::UnaryBool
            }
            InstructionData::UnaryConst { .. } => {
                InstructionFormat::UnaryConst
            }
            InstructionData::UnaryGlobalValue { .. } => {
                InstructionFormat::UnaryGlobalValue
            }
            InstructionData::UnaryIeee32 { .. } => {
                InstructionFormat::UnaryIeee32
            }
            InstructionData::UnaryIeee64 { .. } => {
                InstructionFormat::UnaryIeee64
            }
            InstructionData::UnaryImm { .. } => {
                InstructionFormat::UnaryImm
            }
        }
    }
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum InstructionData {
    Unary {
        opcode: Opcode,
        arg: Value,
    },
    UnaryImm {
        opcode: Opcode,
        imm: ir::immediates::Imm64,
    },
    UnaryIeee32 {
        opcode: Opcode,
        imm: ir::immediates::Ieee32,
    },
    UnaryIeee64 {
        opcode: Opcode,
        imm: ir::immediates::Ieee64,
    },
    UnaryBool {
        opcode: Opcode,
        imm: bool,
    },
    UnaryConst {
        opcode: Opcode,
        constant_handle: ir::Constant,
    },
    UnaryGlobalValue {
        opcode: Opcode,
        global_value: ir::GlobalValue,
    },
    Binary {
        opcode: Opcode,
        args: [Value; 2],
    },
    BinaryImm {
        opcode: Opcode,
        arg: Value,
        imm: ir::immediates::Imm64,
    },
    Ternary {
        opcode: Opcode,
        args: [Value; 3],
    },
    MultiAry {
        opcode: Opcode,
        args: ValueList,
    },
    NullAry {
        opcode: Opcode,
    },
    InsertLane {
        opcode: Opcode,
        args: [Value; 2],
        lane: ir::immediates::Uimm8,
    },
    ExtractLane {
        opcode: Opcode,
        arg: Value,
        lane: ir::immediates::Uimm8,
    },
    Shuffle {
        opcode: Opcode,
        args: [Value; 2],
        mask: ir::Immediate,
    },
    IntCompare {
        opcode: Opcode,
        args: [Value; 2],
        cond: ir::condcodes::IntCC,
    },
    IntCompareImm {
        opcode: Opcode,
        arg: Value,
        cond: ir::condcodes::IntCC,
        imm: ir::immediates::Imm64,
    },
    IntCond {
        opcode: Opcode,
        arg: Value,
        cond: ir::condcodes::IntCC,
    },
    FloatCompare {
        opcode: Opcode,
        args: [Value; 2],
        cond: ir::condcodes::FloatCC,
    },
    FloatCond {
        opcode: Opcode,
        arg: Value,
        cond: ir::condcodes::FloatCC,
    },
    IntSelect {
        opcode: Opcode,
        args: [Value; 3],
        cond: ir::condcodes::IntCC,
    },
    Jump {
        opcode: Opcode,
        args: ValueList,
        destination: ir::Ebb,
    },
    Branch {
        opcode: Opcode,
        args: ValueList,
        destination: ir::Ebb,
    },
    BranchInt {
        opcode: Opcode,
        args: ValueList,
        cond: ir::condcodes::IntCC,
        destination: ir::Ebb,
    },
    BranchFloat {
        opcode: Opcode,
        args: ValueList,
        cond: ir::condcodes::FloatCC,
        destination: ir::Ebb,
    },
    BranchIcmp {
        opcode: Opcode,
        args: ValueList,
        cond: ir::condcodes::IntCC,
        destination: ir::Ebb,
    },
    BranchTable {
        opcode: Opcode,
        arg: Value,
        destination: ir::Ebb,
        table: ir::JumpTable,
    },
    BranchTableEntry {
        opcode: Opcode,
        args: [Value; 2],
        imm: ir::immediates::Uimm8,
        table: ir::JumpTable,
    },
    BranchTableBase {
        opcode: Opcode,
        table: ir::JumpTable,
    },
    IndirectJump {
        opcode: Opcode,
        arg: Value,
        table: ir::JumpTable,
    },
    Call {
        opcode: Opcode,
        args: ValueList,
        func_ref: ir::FuncRef,
    },
    CallIndirect {
        opcode: Opcode,
        args: ValueList,
        sig_ref: ir::SigRef,
    },
    FuncAddr {
        opcode: Opcode,
        func_ref: ir::FuncRef,
    },
    Load {
        opcode: Opcode,
        arg: Value,
        flags: ir::MemFlags,
        offset: ir::immediates::Offset32,
    },
    LoadComplex {
        opcode: Opcode,
        args: ValueList,
        flags: ir::MemFlags,
        offset: ir::immediates::Offset32,
    },
    Store {
        opcode: Opcode,
        args: [Value; 2],
        flags: ir::MemFlags,
        offset: ir::immediates::Offset32,
    },
    StoreComplex {
        opcode: Opcode,
        args: ValueList,
        flags: ir::MemFlags,
        offset: ir::immediates::Offset32,
    },
    StackLoad {
        opcode: Opcode,
        stack_slot: ir::StackSlot,
        offset: ir::immediates::Offset32,
    },
    StackStore {
        opcode: Opcode,
        arg: Value,
        stack_slot: ir::StackSlot,
        offset: ir::immediates::Offset32,
    },
    HeapAddr {
        opcode: Opcode,
        arg: Value,
        heap: ir::Heap,
        imm: ir::immediates::Uimm32,
    },
    TableAddr {
        opcode: Opcode,
        arg: Value,
        table: ir::Table,
        offset: ir::immediates::Offset32,
    },
    RegMove {
        opcode: Opcode,
        arg: Value,
        src: isa::RegUnit,
        dst: isa::RegUnit,
    },
    CopySpecial {
        opcode: Opcode,
        src: isa::RegUnit,
        dst: isa::RegUnit,
    },
    CopyToSsa {
        opcode: Opcode,
        src: isa::RegUnit,
    },
    RegSpill {
        opcode: Opcode,
        arg: Value,
        src: isa::RegUnit,
        dst: ir::StackSlot,
    },
    RegFill {
        opcode: Opcode,
        arg: Value,
        src: ir::StackSlot,
        dst: isa::RegUnit,
    },
    Trap {
        opcode: Opcode,
        code: ir::TrapCode,
    },
    CondTrap {
        opcode: Opcode,
        arg: Value,
        code: ir::TrapCode,
    },
    IntCondTrap {
        opcode: Opcode,
        arg: Value,
        cond: ir::condcodes::IntCC,
        code: ir::TrapCode,
    },
    FloatCondTrap {
        opcode: Opcode,
        arg: Value,
        cond: ir::condcodes::FloatCC,
        code: ir::TrapCode,
    },
}

impl InstructionData {
    /// Get the opcode of this instruction.
    pub fn opcode(&self) -> Opcode {
        match *self {
            InstructionData::Binary { opcode, .. } |
            InstructionData::BinaryImm { opcode, .. } |
            InstructionData::Branch { opcode, .. } |
            InstructionData::BranchFloat { opcode, .. } |
            InstructionData::BranchIcmp { opcode, .. } |
            InstructionData::BranchInt { opcode, .. } |
            InstructionData::BranchTable { opcode, .. } |
            InstructionData::BranchTableBase { opcode, .. } |
            InstructionData::BranchTableEntry { opcode, .. } |
            InstructionData::Call { opcode, .. } |
            InstructionData::CallIndirect { opcode, .. } |
            InstructionData::CondTrap { opcode, .. } |
            InstructionData::CopySpecial { opcode, .. } |
            InstructionData::CopyToSsa { opcode, .. } |
            InstructionData::ExtractLane { opcode, .. } |
            InstructionData::FloatCompare { opcode, .. } |
            InstructionData::FloatCond { opcode, .. } |
            InstructionData::FloatCondTrap { opcode, .. } |
            InstructionData::FuncAddr { opcode, .. } |
            InstructionData::HeapAddr { opcode, .. } |
            InstructionData::IndirectJump { opcode, .. } |
            InstructionData::InsertLane { opcode, .. } |
            InstructionData::IntCompare { opcode, .. } |
            InstructionData::IntCompareImm { opcode, .. } |
            InstructionData::IntCond { opcode, .. } |
            InstructionData::IntCondTrap { opcode, .. } |
            InstructionData::IntSelect { opcode, .. } |
            InstructionData::Jump { opcode, .. } |
            InstructionData::Load { opcode, .. } |
            InstructionData::LoadComplex { opcode, .. } |
            InstructionData::MultiAry { opcode, .. } |
            InstructionData::NullAry { opcode, .. } |
            InstructionData::RegFill { opcode, .. } |
            InstructionData::RegMove { opcode, .. } |
            InstructionData::RegSpill { opcode, .. } |
            InstructionData::Shuffle { opcode, .. } |
            InstructionData::StackLoad { opcode, .. } |
            InstructionData::StackStore { opcode, .. } |
            InstructionData::Store { opcode, .. } |
            InstructionData::StoreComplex { opcode, .. } |
            InstructionData::TableAddr { opcode, .. } |
            InstructionData::Ternary { opcode, .. } |
            InstructionData::Trap { opcode, .. } |
            InstructionData::Unary { opcode, .. } |
            InstructionData::UnaryBool { opcode, .. } |
            InstructionData::UnaryConst { opcode, .. } |
            InstructionData::UnaryGlobalValue { opcode, .. } |
            InstructionData::UnaryIeee32 { opcode, .. } |
            InstructionData::UnaryIeee64 { opcode, .. } |
            InstructionData::UnaryImm { opcode, .. } => {
                opcode
            }
        }
    }

    /// Get the controlling type variable operand.
    pub fn typevar_operand(&self, pool: &ir::ValueListPool) -> Option<Value> {
        match *self {
            InstructionData::BranchTableBase { .. } |
            InstructionData::CopySpecial { .. } |
            InstructionData::CopyToSsa { .. } |
            InstructionData::FuncAddr { .. } |
            InstructionData::NullAry { .. } |
            InstructionData::StackLoad { .. } |
            InstructionData::Trap { .. } |
            InstructionData::UnaryBool { .. } |
            InstructionData::UnaryConst { .. } |
            InstructionData::UnaryGlobalValue { .. } |
            InstructionData::UnaryIeee32 { .. } |
            InstructionData::UnaryIeee64 { .. } |
            InstructionData::UnaryImm { .. } => {
                None
            }
            InstructionData::BinaryImm { arg, .. } |
            InstructionData::BranchTable { arg, .. } |
            InstructionData::CondTrap { arg, .. } |
            InstructionData::ExtractLane { arg, .. } |
            InstructionData::FloatCond { arg, .. } |
            InstructionData::FloatCondTrap { arg, .. } |
            InstructionData::HeapAddr { arg, .. } |
            InstructionData::IndirectJump { arg, .. } |
            InstructionData::IntCompareImm { arg, .. } |
            InstructionData::IntCond { arg, .. } |
            InstructionData::IntCondTrap { arg, .. } |
            InstructionData::Load { arg, .. } |
            InstructionData::RegFill { arg, .. } |
            InstructionData::RegMove { arg, .. } |
            InstructionData::RegSpill { arg, .. } |
            InstructionData::StackStore { arg, .. } |
            InstructionData::TableAddr { arg, .. } |
            InstructionData::Unary { arg, .. } => {
                Some(arg)
            }
            InstructionData::Binary { args: ref args_arity2, .. } |
            InstructionData::BranchTableEntry { args: ref args_arity2, .. } |
            InstructionData::FloatCompare { args: ref args_arity2, .. } |
            InstructionData::InsertLane { args: ref args_arity2, .. } |
            InstructionData::IntCompare { args: ref args_arity2, .. } |
            InstructionData::Shuffle { args: ref args_arity2, .. } |
            InstructionData::Store { args: ref args_arity2, .. } => {
                Some(args_arity2[0])
            }
            InstructionData::IntSelect { args: ref args_arity3, .. } => {
                Some(args_arity3[0])
            }
            InstructionData::Ternary { args: ref args_arity3, .. } => {
                Some(args_arity3[1])
            }
            InstructionData::Branch { ref args, .. } |
            InstructionData::BranchFloat { ref args, .. } |
            InstructionData::BranchIcmp { ref args, .. } |
            InstructionData::BranchInt { ref args, .. } |
            InstructionData::Call { ref args, .. } |
            InstructionData::CallIndirect { ref args, .. } |
            InstructionData::Jump { ref args, .. } |
            InstructionData::LoadComplex { ref args, .. } |
            InstructionData::MultiAry { ref args, .. } |
            InstructionData::StoreComplex { ref args, .. } => {
                args.get(0, pool)
            }
        }
    }

    /// Get the value arguments to this instruction.
    pub fn arguments<'a>(&'a self, pool: &'a ir::ValueListPool) -> &[Value] {
        match *self {
            InstructionData::BranchTableBase { .. } |
            InstructionData::CopySpecial { .. } |
            InstructionData::CopyToSsa { .. } |
            InstructionData::FuncAddr { .. } |
            InstructionData::NullAry { .. } |
            InstructionData::StackLoad { .. } |
            InstructionData::Trap { .. } |
            InstructionData::UnaryBool { .. } |
            InstructionData::UnaryConst { .. } |
            InstructionData::UnaryGlobalValue { .. } |
            InstructionData::UnaryIeee32 { .. } |
            InstructionData::UnaryIeee64 { .. } |
            InstructionData::UnaryImm { .. } => {
                &[]
            }
            InstructionData::Binary { args: ref args_arity2, .. } |
            InstructionData::BranchTableEntry { args: ref args_arity2, .. } |
            InstructionData::FloatCompare { args: ref args_arity2, .. } |
            InstructionData::InsertLane { args: ref args_arity2, .. } |
            InstructionData::IntCompare { args: ref args_arity2, .. } |
            InstructionData::Shuffle { args: ref args_arity2, .. } |
            InstructionData::Store { args: ref args_arity2, .. } => {
                args_arity2
            }
            InstructionData::IntSelect { args: ref args_arity3, .. } |
            InstructionData::Ternary { args: ref args_arity3, .. } => {
                args_arity3
            }
            InstructionData::BinaryImm { ref arg, .. } |
            InstructionData::BranchTable { ref arg, .. } |
            InstructionData::CondTrap { ref arg, .. } |
            InstructionData::ExtractLane { ref arg, .. } |
            InstructionData::FloatCond { ref arg, .. } |
            InstructionData::FloatCondTrap { ref arg, .. } |
            InstructionData::HeapAddr { ref arg, .. } |
            InstructionData::IndirectJump { ref arg, .. } |
            InstructionData::IntCompareImm { ref arg, .. } |
            InstructionData::IntCond { ref arg, .. } |
            InstructionData::IntCondTrap { ref arg, .. } |
            InstructionData::Load { ref arg, .. } |
            InstructionData::RegFill { ref arg, .. } |
            InstructionData::RegMove { ref arg, .. } |
            InstructionData::RegSpill { ref arg, .. } |
            InstructionData::StackStore { ref arg, .. } |
            InstructionData::TableAddr { ref arg, .. } |
            InstructionData::Unary { ref arg, .. } => {
                core::slice::from_ref(arg)
            }
            InstructionData::Branch { ref args, .. } |
            InstructionData::BranchFloat { ref args, .. } |
            InstructionData::BranchIcmp { ref args, .. } |
            InstructionData::BranchInt { ref args, .. } |
            InstructionData::Call { ref args, .. } |
            InstructionData::CallIndirect { ref args, .. } |
            InstructionData::Jump { ref args, .. } |
            InstructionData::LoadComplex { ref args, .. } |
            InstructionData::MultiAry { ref args, .. } |
            InstructionData::StoreComplex { ref args, .. } => {
                args.as_slice(pool)
            }
        }
    }

    /// Get mutable references to the value arguments to this
    /// instruction.
    pub fn arguments_mut<'a>(&'a mut self, pool: &'a mut ir::ValueListPool) -> &mut [Value] {
        match *self {
            InstructionData::BranchTableBase { .. } |
            InstructionData::CopySpecial { .. } |
            InstructionData::CopyToSsa { .. } |
            InstructionData::FuncAddr { .. } |
            InstructionData::NullAry { .. } |
            InstructionData::StackLoad { .. } |
            InstructionData::Trap { .. } |
            InstructionData::UnaryBool { .. } |
            InstructionData::UnaryConst { .. } |
            InstructionData::UnaryGlobalValue { .. } |
            InstructionData::UnaryIeee32 { .. } |
            InstructionData::UnaryIeee64 { .. } |
            InstructionData::UnaryImm { .. } => {
                &mut []
            }
            InstructionData::Binary { args: ref mut args_arity2, .. } |
            InstructionData::BranchTableEntry { args: ref mut args_arity2, .. } |
            InstructionData::FloatCompare { args: ref mut args_arity2, .. } |
            InstructionData::InsertLane { args: ref mut args_arity2, .. } |
            InstructionData::IntCompare { args: ref mut args_arity2, .. } |
            InstructionData::Shuffle { args: ref mut args_arity2, .. } |
            InstructionData::Store { args: ref mut args_arity2, .. } => {
                args_arity2
            }
            InstructionData::IntSelect { args: ref mut args_arity3, .. } |
            InstructionData::Ternary { args: ref mut args_arity3, .. } => {
                args_arity3
            }
            InstructionData::BinaryImm { ref mut arg, .. } |
            InstructionData::BranchTable { ref mut arg, .. } |
            InstructionData::CondTrap { ref mut arg, .. } |
            InstructionData::ExtractLane { ref mut arg, .. } |
            InstructionData::FloatCond { ref mut arg, .. } |
            InstructionData::FloatCondTrap { ref mut arg, .. } |
            InstructionData::HeapAddr { ref mut arg, .. } |
            InstructionData::IndirectJump { ref mut arg, .. } |
            InstructionData::IntCompareImm { ref mut arg, .. } |
            InstructionData::IntCond { ref mut arg, .. } |
            InstructionData::IntCondTrap { ref mut arg, .. } |
            InstructionData::Load { ref mut arg, .. } |
            InstructionData::RegFill { ref mut arg, .. } |
            InstructionData::RegMove { ref mut arg, .. } |
            InstructionData::RegSpill { ref mut arg, .. } |
            InstructionData::StackStore { ref mut arg, .. } |
            InstructionData::TableAddr { ref mut arg, .. } |
            InstructionData::Unary { ref mut arg, .. } => {
                core::slice::from_mut(arg)
            }
            InstructionData::Branch { ref mut args, .. } |
            InstructionData::BranchFloat { ref mut args, .. } |
            InstructionData::BranchIcmp { ref mut args, .. } |
            InstructionData::BranchInt { ref mut args, .. } |
            InstructionData::Call { ref mut args, .. } |
            InstructionData::CallIndirect { ref mut args, .. } |
            InstructionData::Jump { ref mut args, .. } |
            InstructionData::LoadComplex { ref mut args, .. } |
            InstructionData::MultiAry { ref mut args, .. } |
            InstructionData::StoreComplex { ref mut args, .. } => {
                args.as_mut_slice(pool)
            }
        }
    }

    /// Take out the value list with all the value arguments and return
    /// it.
    ///
    /// This leaves the value list in the instruction empty. Use
    /// `put_value_list` to put the value list back.
    pub fn take_value_list(&mut self) -> Option<ir::ValueList> {
        match *self {
            InstructionData::Branch { ref mut args, .. } |
            InstructionData::BranchFloat { ref mut args, .. } |
            InstructionData::BranchIcmp { ref mut args, .. } |
            InstructionData::BranchInt { ref mut args, .. } |
            InstructionData::Call { ref mut args, .. } |
            InstructionData::CallIndirect { ref mut args, .. } |
            InstructionData::Jump { ref mut args, .. } |
            InstructionData::LoadComplex { ref mut args, .. } |
            InstructionData::MultiAry { ref mut args, .. } |
            InstructionData::StoreComplex { ref mut args, .. } => {
                Some(args.take())
            }
            _ => {
                None
            }
        }
    }

    /// Put back a value list.
    ///
    /// After removing a value list with `take_value_list()`, use this
    /// method to put it back. It is required that this instruction has
    /// a format that accepts a value list, and that the existing value
    /// list is empty. This avoids leaking list pool memory.
    pub fn put_value_list(&mut self, vlist: ir::ValueList) {
        let args = match *self {
            InstructionData::MultiAry { ref mut args, .. } => args,
            InstructionData::Jump { ref mut args, .. } => args,
            InstructionData::Branch { ref mut args, .. } => args,
            InstructionData::BranchInt { ref mut args, .. } => args,
            InstructionData::BranchFloat { ref mut args, .. } => args,
            InstructionData::BranchIcmp { ref mut args, .. } => args,
            InstructionData::Call { ref mut args, .. } => args,
            InstructionData::CallIndirect { ref mut args, .. } => args,
            InstructionData::LoadComplex { ref mut args, .. } => args,
            InstructionData::StoreComplex { ref mut args, .. } => args,
            _ => panic!("No value list: {:?}", self),
        };
        debug_assert!(args.is_empty(), "Value list already in use");
        *args = vlist;
    }

    /// Compare two `InstructionData` for equality.
    ///
    /// This operation requires a reference to a `ValueListPool` to
    /// determine if the contents of any `ValueLists` are equal.
    pub fn eq(&self, other: &Self, pool: &ir::ValueListPool) -> bool {
        if ::core::mem::discriminant(self) != ::core::mem::discriminant(other) {
            return false;
        }
        match (self, other) {
            (&InstructionData::Unary { opcode: ref opcode1, arg: ref arg1 }, &InstructionData::Unary { opcode: ref opcode2, arg: ref arg2 }) => {
                opcode1 == opcode2
                && arg1 == arg2
            }
            (&InstructionData::UnaryImm { opcode: ref opcode1, imm: ref imm1 }, &InstructionData::UnaryImm { opcode: ref opcode2, imm: ref imm2 }) => {
                opcode1 == opcode2
                && imm1 == imm2
            }
            (&InstructionData::UnaryIeee32 { opcode: ref opcode1, imm: ref imm1 }, &InstructionData::UnaryIeee32 { opcode: ref opcode2, imm: ref imm2 }) => {
                opcode1 == opcode2
                && imm1 == imm2
            }
            (&InstructionData::UnaryIeee64 { opcode: ref opcode1, imm: ref imm1 }, &InstructionData::UnaryIeee64 { opcode: ref opcode2, imm: ref imm2 }) => {
                opcode1 == opcode2
                && imm1 == imm2
            }
            (&InstructionData::UnaryBool { opcode: ref opcode1, imm: ref imm1 }, &InstructionData::UnaryBool { opcode: ref opcode2, imm: ref imm2 }) => {
                opcode1 == opcode2
                && imm1 == imm2
            }
            (&InstructionData::UnaryConst { opcode: ref opcode1, constant_handle: ref constant_handle1 }, &InstructionData::UnaryConst { opcode: ref opcode2, constant_handle: ref constant_handle2 }) => {
                opcode1 == opcode2
                && constant_handle1 == constant_handle2
            }
            (&InstructionData::UnaryGlobalValue { opcode: ref opcode1, global_value: ref global_value1 }, &InstructionData::UnaryGlobalValue { opcode: ref opcode2, global_value: ref global_value2 }) => {
                opcode1 == opcode2
                && global_value1 == global_value2
            }
            (&InstructionData::Binary { opcode: ref opcode1, args: ref args1 }, &InstructionData::Binary { opcode: ref opcode2, args: ref args2 }) => {
                opcode1 == opcode2
                && args1 == args2
            }
            (&InstructionData::BinaryImm { opcode: ref opcode1, arg: ref arg1, imm: ref imm1 }, &InstructionData::BinaryImm { opcode: ref opcode2, arg: ref arg2, imm: ref imm2 }) => {
                opcode1 == opcode2
                && imm1 == imm2
                && arg1 == arg2
            }
            (&InstructionData::Ternary { opcode: ref opcode1, args: ref args1 }, &InstructionData::Ternary { opcode: ref opcode2, args: ref args2 }) => {
                opcode1 == opcode2
                && args1 == args2
            }
            (&InstructionData::MultiAry { opcode: ref opcode1, args: ref args1 }, &InstructionData::MultiAry { opcode: ref opcode2, args: ref args2 }) => {
                opcode1 == opcode2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::NullAry { opcode: ref opcode1 }, &InstructionData::NullAry { opcode: ref opcode2 }) => {
                opcode1 == opcode2
            }
            (&InstructionData::InsertLane { opcode: ref opcode1, args: ref args1, lane: ref lane1 }, &InstructionData::InsertLane { opcode: ref opcode2, args: ref args2, lane: ref lane2 }) => {
                opcode1 == opcode2
                && lane1 == lane2
                && args1 == args2
            }
            (&InstructionData::ExtractLane { opcode: ref opcode1, arg: ref arg1, lane: ref lane1 }, &InstructionData::ExtractLane { opcode: ref opcode2, arg: ref arg2, lane: ref lane2 }) => {
                opcode1 == opcode2
                && lane1 == lane2
                && arg1 == arg2
            }
            (&InstructionData::Shuffle { opcode: ref opcode1, args: ref args1, mask: ref mask1 }, &InstructionData::Shuffle { opcode: ref opcode2, args: ref args2, mask: ref mask2 }) => {
                opcode1 == opcode2
                && mask1 == mask2
                && args1 == args2
            }
            (&InstructionData::IntCompare { opcode: ref opcode1, args: ref args1, cond: ref cond1 }, &InstructionData::IntCompare { opcode: ref opcode2, args: ref args2, cond: ref cond2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && args1 == args2
            }
            (&InstructionData::IntCompareImm { opcode: ref opcode1, arg: ref arg1, cond: ref cond1, imm: ref imm1 }, &InstructionData::IntCompareImm { opcode: ref opcode2, arg: ref arg2, cond: ref cond2, imm: ref imm2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && imm1 == imm2
                && arg1 == arg2
            }
            (&InstructionData::IntCond { opcode: ref opcode1, arg: ref arg1, cond: ref cond1 }, &InstructionData::IntCond { opcode: ref opcode2, arg: ref arg2, cond: ref cond2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && arg1 == arg2
            }
            (&InstructionData::FloatCompare { opcode: ref opcode1, args: ref args1, cond: ref cond1 }, &InstructionData::FloatCompare { opcode: ref opcode2, args: ref args2, cond: ref cond2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && args1 == args2
            }
            (&InstructionData::FloatCond { opcode: ref opcode1, arg: ref arg1, cond: ref cond1 }, &InstructionData::FloatCond { opcode: ref opcode2, arg: ref arg2, cond: ref cond2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && arg1 == arg2
            }
            (&InstructionData::IntSelect { opcode: ref opcode1, args: ref args1, cond: ref cond1 }, &InstructionData::IntSelect { opcode: ref opcode2, args: ref args2, cond: ref cond2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && args1 == args2
            }
            (&InstructionData::Jump { opcode: ref opcode1, args: ref args1, destination: ref destination1 }, &InstructionData::Jump { opcode: ref opcode2, args: ref args2, destination: ref destination2 }) => {
                opcode1 == opcode2
                && destination1 == destination2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::Branch { opcode: ref opcode1, args: ref args1, destination: ref destination1 }, &InstructionData::Branch { opcode: ref opcode2, args: ref args2, destination: ref destination2 }) => {
                opcode1 == opcode2
                && destination1 == destination2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::BranchInt { opcode: ref opcode1, args: ref args1, cond: ref cond1, destination: ref destination1 }, &InstructionData::BranchInt { opcode: ref opcode2, args: ref args2, cond: ref cond2, destination: ref destination2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && destination1 == destination2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::BranchFloat { opcode: ref opcode1, args: ref args1, cond: ref cond1, destination: ref destination1 }, &InstructionData::BranchFloat { opcode: ref opcode2, args: ref args2, cond: ref cond2, destination: ref destination2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && destination1 == destination2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::BranchIcmp { opcode: ref opcode1, args: ref args1, cond: ref cond1, destination: ref destination1 }, &InstructionData::BranchIcmp { opcode: ref opcode2, args: ref args2, cond: ref cond2, destination: ref destination2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && destination1 == destination2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::BranchTable { opcode: ref opcode1, arg: ref arg1, destination: ref destination1, table: ref table1 }, &InstructionData::BranchTable { opcode: ref opcode2, arg: ref arg2, destination: ref destination2, table: ref table2 }) => {
                opcode1 == opcode2
                && destination1 == destination2
                && table1 == table2
                && arg1 == arg2
            }
            (&InstructionData::BranchTableEntry { opcode: ref opcode1, args: ref args1, imm: ref imm1, table: ref table1 }, &InstructionData::BranchTableEntry { opcode: ref opcode2, args: ref args2, imm: ref imm2, table: ref table2 }) => {
                opcode1 == opcode2
                && imm1 == imm2
                && table1 == table2
                && args1 == args2
            }
            (&InstructionData::BranchTableBase { opcode: ref opcode1, table: ref table1 }, &InstructionData::BranchTableBase { opcode: ref opcode2, table: ref table2 }) => {
                opcode1 == opcode2
                && table1 == table2
            }
            (&InstructionData::IndirectJump { opcode: ref opcode1, arg: ref arg1, table: ref table1 }, &InstructionData::IndirectJump { opcode: ref opcode2, arg: ref arg2, table: ref table2 }) => {
                opcode1 == opcode2
                && table1 == table2
                && arg1 == arg2
            }
            (&InstructionData::Call { opcode: ref opcode1, args: ref args1, func_ref: ref func_ref1 }, &InstructionData::Call { opcode: ref opcode2, args: ref args2, func_ref: ref func_ref2 }) => {
                opcode1 == opcode2
                && func_ref1 == func_ref2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::CallIndirect { opcode: ref opcode1, args: ref args1, sig_ref: ref sig_ref1 }, &InstructionData::CallIndirect { opcode: ref opcode2, args: ref args2, sig_ref: ref sig_ref2 }) => {
                opcode1 == opcode2
                && sig_ref1 == sig_ref2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::FuncAddr { opcode: ref opcode1, func_ref: ref func_ref1 }, &InstructionData::FuncAddr { opcode: ref opcode2, func_ref: ref func_ref2 }) => {
                opcode1 == opcode2
                && func_ref1 == func_ref2
            }
            (&InstructionData::Load { opcode: ref opcode1, arg: ref arg1, flags: ref flags1, offset: ref offset1 }, &InstructionData::Load { opcode: ref opcode2, arg: ref arg2, flags: ref flags2, offset: ref offset2 }) => {
                opcode1 == opcode2
                && flags1 == flags2
                && offset1 == offset2
                && arg1 == arg2
            }
            (&InstructionData::LoadComplex { opcode: ref opcode1, args: ref args1, flags: ref flags1, offset: ref offset1 }, &InstructionData::LoadComplex { opcode: ref opcode2, args: ref args2, flags: ref flags2, offset: ref offset2 }) => {
                opcode1 == opcode2
                && flags1 == flags2
                && offset1 == offset2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::Store { opcode: ref opcode1, args: ref args1, flags: ref flags1, offset: ref offset1 }, &InstructionData::Store { opcode: ref opcode2, args: ref args2, flags: ref flags2, offset: ref offset2 }) => {
                opcode1 == opcode2
                && flags1 == flags2
                && offset1 == offset2
                && args1 == args2
            }
            (&InstructionData::StoreComplex { opcode: ref opcode1, args: ref args1, flags: ref flags1, offset: ref offset1 }, &InstructionData::StoreComplex { opcode: ref opcode2, args: ref args2, flags: ref flags2, offset: ref offset2 }) => {
                opcode1 == opcode2
                && flags1 == flags2
                && offset1 == offset2
                && args1.as_slice(pool) == args2.as_slice(pool)
            }
            (&InstructionData::StackLoad { opcode: ref opcode1, stack_slot: ref stack_slot1, offset: ref offset1 }, &InstructionData::StackLoad { opcode: ref opcode2, stack_slot: ref stack_slot2, offset: ref offset2 }) => {
                opcode1 == opcode2
                && stack_slot1 == stack_slot2
                && offset1 == offset2
            }
            (&InstructionData::StackStore { opcode: ref opcode1, arg: ref arg1, stack_slot: ref stack_slot1, offset: ref offset1 }, &InstructionData::StackStore { opcode: ref opcode2, arg: ref arg2, stack_slot: ref stack_slot2, offset: ref offset2 }) => {
                opcode1 == opcode2
                && stack_slot1 == stack_slot2
                && offset1 == offset2
                && arg1 == arg2
            }
            (&InstructionData::HeapAddr { opcode: ref opcode1, arg: ref arg1, heap: ref heap1, imm: ref imm1 }, &InstructionData::HeapAddr { opcode: ref opcode2, arg: ref arg2, heap: ref heap2, imm: ref imm2 }) => {
                opcode1 == opcode2
                && heap1 == heap2
                && imm1 == imm2
                && arg1 == arg2
            }
            (&InstructionData::TableAddr { opcode: ref opcode1, arg: ref arg1, table: ref table1, offset: ref offset1 }, &InstructionData::TableAddr { opcode: ref opcode2, arg: ref arg2, table: ref table2, offset: ref offset2 }) => {
                opcode1 == opcode2
                && table1 == table2
                && offset1 == offset2
                && arg1 == arg2
            }
            (&InstructionData::RegMove { opcode: ref opcode1, arg: ref arg1, src: ref src1, dst: ref dst1 }, &InstructionData::RegMove { opcode: ref opcode2, arg: ref arg2, src: ref src2, dst: ref dst2 }) => {
                opcode1 == opcode2
                && src1 == src2
                && dst1 == dst2
                && arg1 == arg2
            }
            (&InstructionData::CopySpecial { opcode: ref opcode1, src: ref src1, dst: ref dst1 }, &InstructionData::CopySpecial { opcode: ref opcode2, src: ref src2, dst: ref dst2 }) => {
                opcode1 == opcode2
                && src1 == src2
                && dst1 == dst2
            }
            (&InstructionData::CopyToSsa { opcode: ref opcode1, src: ref src1 }, &InstructionData::CopyToSsa { opcode: ref opcode2, src: ref src2 }) => {
                opcode1 == opcode2
                && src1 == src2
            }
            (&InstructionData::RegSpill { opcode: ref opcode1, arg: ref arg1, src: ref src1, dst: ref dst1 }, &InstructionData::RegSpill { opcode: ref opcode2, arg: ref arg2, src: ref src2, dst: ref dst2 }) => {
                opcode1 == opcode2
                && src1 == src2
                && dst1 == dst2
                && arg1 == arg2
            }
            (&InstructionData::RegFill { opcode: ref opcode1, arg: ref arg1, src: ref src1, dst: ref dst1 }, &InstructionData::RegFill { opcode: ref opcode2, arg: ref arg2, src: ref src2, dst: ref dst2 }) => {
                opcode1 == opcode2
                && src1 == src2
                && dst1 == dst2
                && arg1 == arg2
            }
            (&InstructionData::Trap { opcode: ref opcode1, code: ref code1 }, &InstructionData::Trap { opcode: ref opcode2, code: ref code2 }) => {
                opcode1 == opcode2
                && code1 == code2
            }
            (&InstructionData::CondTrap { opcode: ref opcode1, arg: ref arg1, code: ref code1 }, &InstructionData::CondTrap { opcode: ref opcode2, arg: ref arg2, code: ref code2 }) => {
                opcode1 == opcode2
                && code1 == code2
                && arg1 == arg2
            }
            (&InstructionData::IntCondTrap { opcode: ref opcode1, arg: ref arg1, cond: ref cond1, code: ref code1 }, &InstructionData::IntCondTrap { opcode: ref opcode2, arg: ref arg2, cond: ref cond2, code: ref code2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && code1 == code2
                && arg1 == arg2
            }
            (&InstructionData::FloatCondTrap { opcode: ref opcode1, arg: ref arg1, cond: ref cond1, code: ref code1 }, &InstructionData::FloatCondTrap { opcode: ref opcode2, arg: ref arg2, cond: ref cond2, code: ref code2 }) => {
                opcode1 == opcode2
                && cond1 == cond2
                && code1 == code2
                && arg1 == arg2
            }
            _ => unreachable!()
        }
    }

    /// Hash an `InstructionData`.
    ///
    /// This operation requires a reference to a `ValueListPool` to
    /// hash the contents of any `ValueLists`.
    pub fn hash<H: ::core::hash::Hasher>(&self, state: &mut H, pool: &ir::ValueListPool) {
        match *self {
            InstructionData::Unary{opcode, ref arg} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::UnaryImm{opcode, imm} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::UnaryIeee32{opcode, imm} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::UnaryIeee64{opcode, imm} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::UnaryBool{opcode, imm} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::UnaryConst{opcode, constant_handle} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&constant_handle, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::UnaryGlobalValue{opcode, global_value} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&global_value, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::Binary{opcode, ref args} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::BinaryImm{opcode, ref arg, imm} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::Ternary{opcode, ref args} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::MultiAry{opcode, ref args} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::NullAry{opcode} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::InsertLane{opcode, ref args, lane} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&lane, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::ExtractLane{opcode, ref arg, lane} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&lane, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::Shuffle{opcode, ref args, mask} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&mask, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::IntCompare{opcode, ref args, cond} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::IntCompareImm{opcode, ref arg, cond, imm} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::IntCond{opcode, ref arg, cond} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::FloatCompare{opcode, ref args, cond} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::FloatCond{opcode, ref arg, cond} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::IntSelect{opcode, ref args, cond} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::Jump{opcode, ref args, destination} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&destination, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::Branch{opcode, ref args, destination} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&destination, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::BranchInt{opcode, ref args, cond, destination} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(&destination, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::BranchFloat{opcode, ref args, cond, destination} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(&destination, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::BranchIcmp{opcode, ref args, cond, destination} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(&destination, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::BranchTable{opcode, ref arg, destination, table} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&destination, state);
                ::core::hash::Hash::hash(&table, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::BranchTableEntry{opcode, ref args, imm, table} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(&table, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::BranchTableBase{opcode, table} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&table, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::IndirectJump{opcode, ref arg, table} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&table, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::Call{opcode, ref args, func_ref} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&func_ref, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::CallIndirect{opcode, ref args, sig_ref} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&sig_ref, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::FuncAddr{opcode, func_ref} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&func_ref, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::Load{opcode, ref arg, flags, offset} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&flags, state);
                ::core::hash::Hash::hash(&offset, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::LoadComplex{opcode, ref args, flags, offset} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&flags, state);
                ::core::hash::Hash::hash(&offset, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::Store{opcode, ref args, flags, offset} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&flags, state);
                ::core::hash::Hash::hash(&offset, state);
                ::core::hash::Hash::hash(args, state);
            }
            InstructionData::StoreComplex{opcode, ref args, flags, offset} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&flags, state);
                ::core::hash::Hash::hash(&offset, state);
                ::core::hash::Hash::hash(args.as_slice(pool), state);
            }
            InstructionData::StackLoad{opcode, stack_slot, offset} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&stack_slot, state);
                ::core::hash::Hash::hash(&offset, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::StackStore{opcode, ref arg, stack_slot, offset} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&stack_slot, state);
                ::core::hash::Hash::hash(&offset, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::HeapAddr{opcode, ref arg, heap, imm} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&heap, state);
                ::core::hash::Hash::hash(&imm, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::TableAddr{opcode, ref arg, table, offset} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&table, state);
                ::core::hash::Hash::hash(&offset, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::RegMove{opcode, ref arg, src, dst} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&src, state);
                ::core::hash::Hash::hash(&dst, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::CopySpecial{opcode, src, dst} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&src, state);
                ::core::hash::Hash::hash(&dst, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::CopyToSsa{opcode, src} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&src, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::RegSpill{opcode, ref arg, src, dst} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&src, state);
                ::core::hash::Hash::hash(&dst, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::RegFill{opcode, ref arg, src, dst} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&src, state);
                ::core::hash::Hash::hash(&dst, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::Trap{opcode, code} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&code, state);
                ::core::hash::Hash::hash(&(), state);
            }
            InstructionData::CondTrap{opcode, ref arg, code} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&code, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::IntCondTrap{opcode, ref arg, cond, code} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(&code, state);
                ::core::hash::Hash::hash(arg, state);
            }
            InstructionData::FloatCondTrap{opcode, ref arg, cond, code} => {
                ::core::hash::Hash::hash( &::core::mem::discriminant(self), state);
                ::core::hash::Hash::hash(&opcode, state);
                ::core::hash::Hash::hash(&cond, state);
                ::core::hash::Hash::hash(&code, state);
                ::core::hash::Hash::hash(arg, state);
            }
        }
    }
}

/// An instruction opcode.
///
/// All instructions from all supported ISAs are present.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Opcode {
    /// `jump EBB, args`. (Jump)
    Jump = 1,
    /// `fallthrough EBB, args`. (Jump)
    Fallthrough,
    /// `brz c, EBB, args`. (Branch)
    /// Type inferred from `c`.
    Brz,
    /// `brnz c, EBB, args`. (Branch)
    /// Type inferred from `c`.
    Brnz,
    /// `br_icmp Cond, x, y, EBB, args`. (BranchIcmp)
    /// Type inferred from `x`.
    BrIcmp,
    /// `brif Cond, f, EBB, args`. (BranchInt)
    Brif,
    /// `brff Cond, f, EBB, args`. (BranchFloat)
    Brff,
    /// `br_table x, EBB, JT`. (BranchTable)
    /// Type inferred from `x`.
    BrTable,
    /// `entry = jump_table_entry x, addr, Size, JT`. (BranchTableEntry)
    /// Type inferred from `x`.
    JumpTableEntry,
    /// `addr = jump_table_base JT`. (BranchTableBase)
    JumpTableBase,
    /// `indirect_jump_table_br addr, JT`. (IndirectJump)
    /// Type inferred from `addr`.
    IndirectJumpTableBr,
    /// `debugtrap`. (NullAry)
    Debugtrap,
    /// `trap code`. (Trap)
    Trap,
    /// `trapz c, code`. (CondTrap)
    /// Type inferred from `c`.
    Trapz,
    /// `resumable_trap code`. (Trap)
    ResumableTrap,
    /// `trapnz c, code`. (CondTrap)
    /// Type inferred from `c`.
    Trapnz,
    /// `trapif Cond, f, code`. (IntCondTrap)
    Trapif,
    /// `trapff Cond, f, code`. (FloatCondTrap)
    Trapff,
    /// `return rvals`. (MultiAry)
    Return,
    /// `fallthrough_return rvals`. (MultiAry)
    FallthroughReturn,
    /// `rvals = call FN, args`. (Call)
    Call,
    /// `rvals = call_indirect SIG, callee, args`. (CallIndirect)
    /// Type inferred from `callee`.
    CallIndirect,
    /// `addr = func_addr FN`. (FuncAddr)
    FuncAddr,
    /// `a = load MemFlags, p, Offset`. (Load)
    Load,
    /// `a = load_complex MemFlags, args, Offset`. (LoadComplex)
    LoadComplex,
    /// `store MemFlags, x, p, Offset`. (Store)
    /// Type inferred from `x`.
    Store,
    /// `store_complex MemFlags, x, args, Offset`. (StoreComplex)
    /// Type inferred from `x`.
    StoreComplex,
    /// `a = uload8 MemFlags, p, Offset`. (Load)
    Uload8,
    /// `a = uload8_complex MemFlags, args, Offset`. (LoadComplex)
    Uload8Complex,
    /// `a = sload8 MemFlags, p, Offset`. (Load)
    Sload8,
    /// `a = sload8_complex MemFlags, args, Offset`. (LoadComplex)
    Sload8Complex,
    /// `istore8 MemFlags, x, p, Offset`. (Store)
    /// Type inferred from `x`.
    Istore8,
    /// `istore8_complex MemFlags, x, args, Offset`. (StoreComplex)
    /// Type inferred from `x`.
    Istore8Complex,
    /// `a = uload16 MemFlags, p, Offset`. (Load)
    Uload16,
    /// `a = uload16_complex MemFlags, args, Offset`. (LoadComplex)
    Uload16Complex,
    /// `a = sload16 MemFlags, p, Offset`. (Load)
    Sload16,
    /// `a = sload16_complex MemFlags, args, Offset`. (LoadComplex)
    Sload16Complex,
    /// `istore16 MemFlags, x, p, Offset`. (Store)
    /// Type inferred from `x`.
    Istore16,
    /// `istore16_complex MemFlags, x, args, Offset`. (StoreComplex)
    /// Type inferred from `x`.
    Istore16Complex,
    /// `a = uload32 MemFlags, p, Offset`. (Load)
    /// Type inferred from `p`.
    Uload32,
    /// `a = uload32_complex MemFlags, args, Offset`. (LoadComplex)
    Uload32Complex,
    /// `a = sload32 MemFlags, p, Offset`. (Load)
    /// Type inferred from `p`.
    Sload32,
    /// `a = sload32_complex MemFlags, args, Offset`. (LoadComplex)
    Sload32Complex,
    /// `istore32 MemFlags, x, p, Offset`. (Store)
    /// Type inferred from `x`.
    Istore32,
    /// `istore32_complex MemFlags, x, args, Offset`. (StoreComplex)
    Istore32Complex,
    /// `a = stack_load SS, Offset`. (StackLoad)
    StackLoad,
    /// `stack_store x, SS, Offset`. (StackStore)
    /// Type inferred from `x`.
    StackStore,
    /// `addr = stack_addr SS, Offset`. (StackLoad)
    StackAddr,
    /// `a = global_value GV`. (UnaryGlobalValue)
    GlobalValue,
    /// `a = symbol_value GV`. (UnaryGlobalValue)
    SymbolValue,
    /// `addr = heap_addr H, p, Size`. (HeapAddr)
    HeapAddr,
    /// `addr = get_pinned_reg`. (NullAry)
    GetPinnedReg,
    /// `set_pinned_reg addr`. (Unary)
    /// Type inferred from `addr`.
    SetPinnedReg,
    /// `addr = table_addr T, p, Offset`. (TableAddr)
    TableAddr,
    /// `a = iconst N`. (UnaryImm)
    Iconst,
    /// `a = f32const N`. (UnaryIeee32)
    F32const,
    /// `a = f64const N`. (UnaryIeee64)
    F64const,
    /// `a = bconst N`. (UnaryBool)
    Bconst,
    /// `a = vconst N`. (UnaryConst)
    Vconst,
    /// `a = shuffle a, b, mask`. (Shuffle)
    /// Type inferred from `a`.
    Shuffle,
    /// `a = null`. (NullAry)
    Null,
    /// `nop`. (NullAry)
    Nop,
    /// `a = select c, x, y`. (Ternary)
    /// Type inferred from `x`.
    Select,
    /// `a = selectif cc, flags, x, y`. (IntSelect)
    Selectif,
    /// `a = copy x`. (Unary)
    /// Type inferred from `x`.
    Copy,
    /// `a = spill x`. (Unary)
    /// Type inferred from `x`.
    Spill,
    /// `a = fill x`. (Unary)
    /// Type inferred from `x`.
    Fill,
    /// `a = fill_nop x`. (Unary)
    /// Type inferred from `x`.
    FillNop,
    /// `regmove x, src, dst`. (RegMove)
    /// Type inferred from `x`.
    Regmove,
    /// `copy_special src, dst`. (CopySpecial)
    CopySpecial,
    /// `a = copy_to_ssa src`. (CopyToSsa)
    CopyToSsa,
    /// `a = copy_nop x`. (Unary)
    /// Type inferred from `x`.
    CopyNop,
    /// `adjust_sp_down delta`. (Unary)
    /// Type inferred from `delta`.
    AdjustSpDown,
    /// `adjust_sp_up_imm Offset`. (UnaryImm)
    AdjustSpUpImm,
    /// `adjust_sp_down_imm Offset`. (UnaryImm)
    AdjustSpDownImm,
    /// `f = ifcmp_sp addr`. (Unary)
    /// Type inferred from `addr`.
    IfcmpSp,
    /// `regspill x, src, SS`. (RegSpill)
    /// Type inferred from `x`.
    Regspill,
    /// `regfill x, SS, dst`. (RegFill)
    /// Type inferred from `x`.
    Regfill,
    /// `safepoint args`. (MultiAry)
    Safepoint,
    /// `lo, hi = vsplit x`. (Unary)
    /// Type inferred from `x`.
    Vsplit,
    /// `a = vconcat x, y`. (Binary)
    /// Type inferred from `x`.
    Vconcat,
    /// `a = vselect c, x, y`. (Ternary)
    /// Type inferred from `x`.
    Vselect,
    /// `a = splat x`. (Unary)
    Splat,
    /// `a = insertlane x, Idx, y`. (InsertLane)
    /// Type inferred from `x`.
    Insertlane,
    /// `a = extractlane x, Idx`. (ExtractLane)
    /// Type inferred from `x`.
    Extractlane,
    /// `a = icmp Cond, x, y`. (IntCompare)
    /// Type inferred from `x`.
    Icmp,
    /// `a = icmp_imm Cond, x, Y`. (IntCompareImm)
    /// Type inferred from `x`.
    IcmpImm,
    /// `f = ifcmp x, y`. (Binary)
    /// Type inferred from `x`.
    Ifcmp,
    /// `f = ifcmp_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    IfcmpImm,
    /// `a = iadd x, y`. (Binary)
    /// Type inferred from `x`.
    Iadd,
    /// `a = uadd_sat x, y`. (Binary)
    /// Type inferred from `x`.
    UaddSat,
    /// `a = sadd_sat x, y`. (Binary)
    /// Type inferred from `x`.
    SaddSat,
    /// `a = isub x, y`. (Binary)
    /// Type inferred from `x`.
    Isub,
    /// `a = usub_sat x, y`. (Binary)
    /// Type inferred from `x`.
    UsubSat,
    /// `a = ssub_sat x, y`. (Binary)
    /// Type inferred from `x`.
    SsubSat,
    /// `a = ineg x`. (Unary)
    /// Type inferred from `x`.
    Ineg,
    /// `a = imul x, y`. (Binary)
    /// Type inferred from `x`.
    Imul,
    /// `a = umulhi x, y`. (Binary)
    /// Type inferred from `x`.
    Umulhi,
    /// `a = smulhi x, y`. (Binary)
    /// Type inferred from `x`.
    Smulhi,
    /// `a = udiv x, y`. (Binary)
    /// Type inferred from `x`.
    Udiv,
    /// `a = sdiv x, y`. (Binary)
    /// Type inferred from `x`.
    Sdiv,
    /// `a = urem x, y`. (Binary)
    /// Type inferred from `x`.
    Urem,
    /// `a = srem x, y`. (Binary)
    /// Type inferred from `x`.
    Srem,
    /// `a = iadd_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    IaddImm,
    /// `a = imul_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    ImulImm,
    /// `a = udiv_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    UdivImm,
    /// `a = sdiv_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    SdivImm,
    /// `a = urem_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    UremImm,
    /// `a = srem_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    SremImm,
    /// `a = irsub_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    IrsubImm,
    /// `a = iadd_cin x, y, c_in`. (Ternary)
    /// Type inferred from `y`.
    IaddCin,
    /// `a = iadd_ifcin x, y, c_in`. (Ternary)
    /// Type inferred from `y`.
    IaddIfcin,
    /// `a, c_out = iadd_cout x, y`. (Binary)
    /// Type inferred from `x`.
    IaddCout,
    /// `a, c_out = iadd_ifcout x, y`. (Binary)
    /// Type inferred from `x`.
    IaddIfcout,
    /// `a, c_out = iadd_carry x, y, c_in`. (Ternary)
    /// Type inferred from `y`.
    IaddCarry,
    /// `a, c_out = iadd_ifcarry x, y, c_in`. (Ternary)
    /// Type inferred from `y`.
    IaddIfcarry,
    /// `a = isub_bin x, y, b_in`. (Ternary)
    /// Type inferred from `y`.
    IsubBin,
    /// `a = isub_ifbin x, y, b_in`. (Ternary)
    /// Type inferred from `y`.
    IsubIfbin,
    /// `a, b_out = isub_bout x, y`. (Binary)
    /// Type inferred from `x`.
    IsubBout,
    /// `a, b_out = isub_ifbout x, y`. (Binary)
    /// Type inferred from `x`.
    IsubIfbout,
    /// `a, b_out = isub_borrow x, y, b_in`. (Ternary)
    /// Type inferred from `y`.
    IsubBorrow,
    /// `a, b_out = isub_ifborrow x, y, b_in`. (Ternary)
    /// Type inferred from `y`.
    IsubIfborrow,
    /// `a = band x, y`. (Binary)
    /// Type inferred from `x`.
    Band,
    /// `a = bor x, y`. (Binary)
    /// Type inferred from `x`.
    Bor,
    /// `a = bxor x, y`. (Binary)
    /// Type inferred from `x`.
    Bxor,
    /// `a = bnot x`. (Unary)
    /// Type inferred from `x`.
    Bnot,
    /// `a = band_not x, y`. (Binary)
    /// Type inferred from `x`.
    BandNot,
    /// `a = bor_not x, y`. (Binary)
    /// Type inferred from `x`.
    BorNot,
    /// `a = bxor_not x, y`. (Binary)
    /// Type inferred from `x`.
    BxorNot,
    /// `a = band_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    BandImm,
    /// `a = bor_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    BorImm,
    /// `a = bxor_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    BxorImm,
    /// `a = rotl x, y`. (Binary)
    /// Type inferred from `x`.
    Rotl,
    /// `a = rotr x, y`. (Binary)
    /// Type inferred from `x`.
    Rotr,
    /// `a = rotl_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    RotlImm,
    /// `a = rotr_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    RotrImm,
    /// `a = ishl x, y`. (Binary)
    /// Type inferred from `x`.
    Ishl,
    /// `a = ushr x, y`. (Binary)
    /// Type inferred from `x`.
    Ushr,
    /// `a = sshr x, y`. (Binary)
    /// Type inferred from `x`.
    Sshr,
    /// `a = ishl_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    IshlImm,
    /// `a = ushr_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    UshrImm,
    /// `a = sshr_imm x, Y`. (BinaryImm)
    /// Type inferred from `x`.
    SshrImm,
    /// `a = bitrev x`. (Unary)
    /// Type inferred from `x`.
    Bitrev,
    /// `a = clz x`. (Unary)
    /// Type inferred from `x`.
    Clz,
    /// `a = cls x`. (Unary)
    /// Type inferred from `x`.
    Cls,
    /// `a = ctz x`. (Unary)
    /// Type inferred from `x`.
    Ctz,
    /// `a = popcnt x`. (Unary)
    /// Type inferred from `x`.
    Popcnt,
    /// `a = fcmp Cond, x, y`. (FloatCompare)
    /// Type inferred from `x`.
    Fcmp,
    /// `f = ffcmp x, y`. (Binary)
    /// Type inferred from `x`.
    Ffcmp,
    /// `a = fadd x, y`. (Binary)
    /// Type inferred from `x`.
    Fadd,
    /// `a = fsub x, y`. (Binary)
    /// Type inferred from `x`.
    Fsub,
    /// `a = fmul x, y`. (Binary)
    /// Type inferred from `x`.
    Fmul,
    /// `a = fdiv x, y`. (Binary)
    /// Type inferred from `x`.
    Fdiv,
    /// `a = sqrt x`. (Unary)
    /// Type inferred from `x`.
    Sqrt,
    /// `a = fma x, y, z`. (Ternary)
    /// Type inferred from `y`.
    Fma,
    /// `a = fneg x`. (Unary)
    /// Type inferred from `x`.
    Fneg,
    /// `a = fabs x`. (Unary)
    /// Type inferred from `x`.
    Fabs,
    /// `a = fcopysign x, y`. (Binary)
    /// Type inferred from `x`.
    Fcopysign,
    /// `a = fmin x, y`. (Binary)
    /// Type inferred from `x`.
    Fmin,
    /// `a = fmax x, y`. (Binary)
    /// Type inferred from `x`.
    Fmax,
    /// `a = ceil x`. (Unary)
    /// Type inferred from `x`.
    Ceil,
    /// `a = floor x`. (Unary)
    /// Type inferred from `x`.
    Floor,
    /// `a = trunc x`. (Unary)
    /// Type inferred from `x`.
    Trunc,
    /// `a = nearest x`. (Unary)
    /// Type inferred from `x`.
    Nearest,
    /// `a = is_null x`. (Unary)
    /// Type inferred from `x`.
    IsNull,
    /// `a = trueif Cond, f`. (IntCond)
    Trueif,
    /// `a = trueff Cond, f`. (FloatCond)
    Trueff,
    /// `a = bitcast x`. (Unary)
    Bitcast,
    /// `a = raw_bitcast x`. (Unary)
    RawBitcast,
    /// `a = scalar_to_vector s`. (Unary)
    ScalarToVector,
    /// `a = breduce x`. (Unary)
    Breduce,
    /// `a = bextend x`. (Unary)
    Bextend,
    /// `a = bint x`. (Unary)
    Bint,
    /// `a = bmask x`. (Unary)
    Bmask,
    /// `a = ireduce x`. (Unary)
    Ireduce,
    /// `a = uextend x`. (Unary)
    Uextend,
    /// `a = sextend x`. (Unary)
    Sextend,
    /// `a = fpromote x`. (Unary)
    Fpromote,
    /// `a = fdemote x`. (Unary)
    Fdemote,
    /// `a = fcvt_to_uint x`. (Unary)
    FcvtToUint,
    /// `a = fcvt_to_uint_sat x`. (Unary)
    FcvtToUintSat,
    /// `a = fcvt_to_sint x`. (Unary)
    FcvtToSint,
    /// `a = fcvt_to_sint_sat x`. (Unary)
    FcvtToSintSat,
    /// `a = fcvt_from_uint x`. (Unary)
    FcvtFromUint,
    /// `a = fcvt_from_sint x`. (Unary)
    FcvtFromSint,
    /// `lo, hi = isplit x`. (Unary)
    /// Type inferred from `x`.
    Isplit,
    /// `a = iconcat lo, hi`. (Binary)
    /// Type inferred from `lo`.
    Iconcat,
    /// `q, r = x86_udivmodx nlo, nhi, d`. (Ternary)
    /// Type inferred from `nhi`.
    X86Udivmodx,
    /// `q, r = x86_sdivmodx nlo, nhi, d`. (Ternary)
    /// Type inferred from `nhi`.
    X86Sdivmodx,
    /// `resLo, resHi = x86_umulx argL, argR`. (Binary)
    /// Type inferred from `argL`.
    X86Umulx,
    /// `resLo, resHi = x86_smulx argL, argR`. (Binary)
    /// Type inferred from `argL`.
    X86Smulx,
    /// `a = x86_cvtt2si x`. (Unary)
    X86Cvtt2si,
    /// `a = x86_fmin x, y`. (Binary)
    /// Type inferred from `x`.
    X86Fmin,
    /// `a = x86_fmax x, y`. (Binary)
    /// Type inferred from `x`.
    X86Fmax,
    /// `x86_push x`. (Unary)
    /// Type inferred from `x`.
    X86Push,
    /// `x = x86_pop`. (NullAry)
    X86Pop,
    /// `y, rflags = x86_bsr x`. (Unary)
    /// Type inferred from `x`.
    X86Bsr,
    /// `y, rflags = x86_bsf x`. (Unary)
    /// Type inferred from `x`.
    X86Bsf,
    /// `a = x86_pshufd a, i`. (ExtractLane)
    /// Type inferred from `a`.
    X86Pshufd,
    /// `a = x86_pshufb a, b`. (Binary)
    /// Type inferred from `a`.
    X86Pshufb,
    /// `a = x86_pextr x, Idx`. (ExtractLane)
    /// Type inferred from `x`.
    X86Pextr,
    /// `a = x86_pinsr x, Idx, y`. (InsertLane)
    /// Type inferred from `x`.
    X86Pinsr,
    /// `a = x86_insertps x, Idx, y`. (InsertLane)
    /// Type inferred from `x`.
    X86Insertps,
    /// `a = x86_movsd x, y`. (Binary)
    /// Type inferred from `x`.
    X86Movsd,
    /// `a = x86_movlhps x, y`. (Binary)
    /// Type inferred from `x`.
    X86Movlhps,
}

impl Opcode {
    /// True for instructions that terminate the EBB
    pub fn is_terminator(self) -> bool {
        match self {
            Opcode::BrTable |
            Opcode::Fallthrough |
            Opcode::FallthroughReturn |
            Opcode::IndirectJumpTableBr |
            Opcode::Jump |
            Opcode::Return |
            Opcode::Trap => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// True for all branch or jump instructions.
    pub fn is_branch(self) -> bool {
        match self {
            Opcode::BrIcmp |
            Opcode::BrTable |
            Opcode::Brff |
            Opcode::Brif |
            Opcode::Brnz |
            Opcode::Brz |
            Opcode::Fallthrough |
            Opcode::IndirectJumpTableBr |
            Opcode::Jump => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// True for all indirect branch or jump instructions.
    pub fn is_indirect_branch(self) -> bool {
        match self {
            Opcode::IndirectJumpTableBr => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Is this a call instruction?
    pub fn is_call(self) -> bool {
        match self {
            Opcode::Call |
            Opcode::CallIndirect => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Is this a return instruction?
    pub fn is_return(self) -> bool {
        match self {
            Opcode::FallthroughReturn |
            Opcode::Return => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Is this a ghost instruction?
    pub fn is_ghost(self) -> bool {
        match self {
            Opcode::Iconcat |
            Opcode::Isplit |
            Opcode::Vconcat |
            Opcode::Vsplit => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Can this instruction read from memory?
    pub fn can_load(self) -> bool {
        match self {
            Opcode::Debugtrap |
            Opcode::Fill |
            Opcode::FillNop |
            Opcode::JumpTableEntry |
            Opcode::Load |
            Opcode::LoadComplex |
            Opcode::Sload16 |
            Opcode::Sload16Complex |
            Opcode::Sload32 |
            Opcode::Sload32Complex |
            Opcode::Sload8 |
            Opcode::Sload8Complex |
            Opcode::StackLoad |
            Opcode::Uload16 |
            Opcode::Uload16Complex |
            Opcode::Uload32 |
            Opcode::Uload32Complex |
            Opcode::Uload8 |
            Opcode::Uload8Complex |
            Opcode::X86Pop => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Can this instruction write to memory?
    pub fn can_store(self) -> bool {
        match self {
            Opcode::Debugtrap |
            Opcode::Istore16 |
            Opcode::Istore16Complex |
            Opcode::Istore32 |
            Opcode::Istore32Complex |
            Opcode::Istore8 |
            Opcode::Istore8Complex |
            Opcode::Spill |
            Opcode::StackStore |
            Opcode::Store |
            Opcode::StoreComplex |
            Opcode::X86Push => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Can this instruction cause a trap?
    pub fn can_trap(self) -> bool {
        match self {
            Opcode::FcvtToSint |
            Opcode::FcvtToUint |
            Opcode::ResumableTrap |
            Opcode::Sdiv |
            Opcode::Srem |
            Opcode::Trap |
            Opcode::Trapff |
            Opcode::Trapif |
            Opcode::Trapnz |
            Opcode::Trapz |
            Opcode::Udiv |
            Opcode::Urem |
            Opcode::X86Sdivmodx |
            Opcode::X86Udivmodx => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Does this instruction have other side effects besides can_* flags?
    pub fn other_side_effects(self) -> bool {
        match self {
            Opcode::AdjustSpDown |
            Opcode::AdjustSpDownImm |
            Opcode::AdjustSpUpImm |
            Opcode::CopySpecial |
            Opcode::CopyToSsa |
            Opcode::Debugtrap |
            Opcode::GetPinnedReg |
            Opcode::Regfill |
            Opcode::Regmove |
            Opcode::Regspill |
            Opcode::Safepoint |
            Opcode::SetPinnedReg |
            Opcode::X86Pop |
            Opcode::X86Push => {
                true
            }
            _ => {
                false
            }
        }
    }

    /// Does this instruction write to CPU flags?
    pub fn writes_cpu_flags(self) -> bool {
        match self {
            Opcode::Ffcmp |
            Opcode::IaddIfcarry |
            Opcode::IaddIfcout |
            Opcode::Ifcmp |
            Opcode::IfcmpImm |
            Opcode::IfcmpSp |
            Opcode::IsubIfborrow |
            Opcode::IsubIfbout |
            Opcode::X86Bsf |
            Opcode::X86Bsr => {
                true
            }
            _ => {
                false
            }
        }
    }

}

const OPCODE_FORMAT: [InstructionFormat; 205] = [
    InstructionFormat::Jump, // jump
    InstructionFormat::Jump, // fallthrough
    InstructionFormat::Branch, // brz
    InstructionFormat::Branch, // brnz
    InstructionFormat::BranchIcmp, // br_icmp
    InstructionFormat::BranchInt, // brif
    InstructionFormat::BranchFloat, // brff
    InstructionFormat::BranchTable, // br_table
    InstructionFormat::BranchTableEntry, // jump_table_entry
    InstructionFormat::BranchTableBase, // jump_table_base
    InstructionFormat::IndirectJump, // indirect_jump_table_br
    InstructionFormat::NullAry, // debugtrap
    InstructionFormat::Trap, // trap
    InstructionFormat::CondTrap, // trapz
    InstructionFormat::Trap, // resumable_trap
    InstructionFormat::CondTrap, // trapnz
    InstructionFormat::IntCondTrap, // trapif
    InstructionFormat::FloatCondTrap, // trapff
    InstructionFormat::MultiAry, // return
    InstructionFormat::MultiAry, // fallthrough_return
    InstructionFormat::Call, // call
    InstructionFormat::CallIndirect, // call_indirect
    InstructionFormat::FuncAddr, // func_addr
    InstructionFormat::Load, // load
    InstructionFormat::LoadComplex, // load_complex
    InstructionFormat::Store, // store
    InstructionFormat::StoreComplex, // store_complex
    InstructionFormat::Load, // uload8
    InstructionFormat::LoadComplex, // uload8_complex
    InstructionFormat::Load, // sload8
    InstructionFormat::LoadComplex, // sload8_complex
    InstructionFormat::Store, // istore8
    InstructionFormat::StoreComplex, // istore8_complex
    InstructionFormat::Load, // uload16
    InstructionFormat::LoadComplex, // uload16_complex
    InstructionFormat::Load, // sload16
    InstructionFormat::LoadComplex, // sload16_complex
    InstructionFormat::Store, // istore16
    InstructionFormat::StoreComplex, // istore16_complex
    InstructionFormat::Load, // uload32
    InstructionFormat::LoadComplex, // uload32_complex
    InstructionFormat::Load, // sload32
    InstructionFormat::LoadComplex, // sload32_complex
    InstructionFormat::Store, // istore32
    InstructionFormat::StoreComplex, // istore32_complex
    InstructionFormat::StackLoad, // stack_load
    InstructionFormat::StackStore, // stack_store
    InstructionFormat::StackLoad, // stack_addr
    InstructionFormat::UnaryGlobalValue, // global_value
    InstructionFormat::UnaryGlobalValue, // symbol_value
    InstructionFormat::HeapAddr, // heap_addr
    InstructionFormat::NullAry, // get_pinned_reg
    InstructionFormat::Unary, // set_pinned_reg
    InstructionFormat::TableAddr, // table_addr
    InstructionFormat::UnaryImm, // iconst
    InstructionFormat::UnaryIeee32, // f32const
    InstructionFormat::UnaryIeee64, // f64const
    InstructionFormat::UnaryBool, // bconst
    InstructionFormat::UnaryConst, // vconst
    InstructionFormat::Shuffle, // shuffle
    InstructionFormat::NullAry, // null
    InstructionFormat::NullAry, // nop
    InstructionFormat::Ternary, // select
    InstructionFormat::IntSelect, // selectif
    InstructionFormat::Unary, // copy
    InstructionFormat::Unary, // spill
    InstructionFormat::Unary, // fill
    InstructionFormat::Unary, // fill_nop
    InstructionFormat::RegMove, // regmove
    InstructionFormat::CopySpecial, // copy_special
    InstructionFormat::CopyToSsa, // copy_to_ssa
    InstructionFormat::Unary, // copy_nop
    InstructionFormat::Unary, // adjust_sp_down
    InstructionFormat::UnaryImm, // adjust_sp_up_imm
    InstructionFormat::UnaryImm, // adjust_sp_down_imm
    InstructionFormat::Unary, // ifcmp_sp
    InstructionFormat::RegSpill, // regspill
    InstructionFormat::RegFill, // regfill
    InstructionFormat::MultiAry, // safepoint
    InstructionFormat::Unary, // vsplit
    InstructionFormat::Binary, // vconcat
    InstructionFormat::Ternary, // vselect
    InstructionFormat::Unary, // splat
    InstructionFormat::InsertLane, // insertlane
    InstructionFormat::ExtractLane, // extractlane
    InstructionFormat::IntCompare, // icmp
    InstructionFormat::IntCompareImm, // icmp_imm
    InstructionFormat::Binary, // ifcmp
    InstructionFormat::BinaryImm, // ifcmp_imm
    InstructionFormat::Binary, // iadd
    InstructionFormat::Binary, // uadd_sat
    InstructionFormat::Binary, // sadd_sat
    InstructionFormat::Binary, // isub
    InstructionFormat::Binary, // usub_sat
    InstructionFormat::Binary, // ssub_sat
    InstructionFormat::Unary, // ineg
    InstructionFormat::Binary, // imul
    InstructionFormat::Binary, // umulhi
    InstructionFormat::Binary, // smulhi
    InstructionFormat::Binary, // udiv
    InstructionFormat::Binary, // sdiv
    InstructionFormat::Binary, // urem
    InstructionFormat::Binary, // srem
    InstructionFormat::BinaryImm, // iadd_imm
    InstructionFormat::BinaryImm, // imul_imm
    InstructionFormat::BinaryImm, // udiv_imm
    InstructionFormat::BinaryImm, // sdiv_imm
    InstructionFormat::BinaryImm, // urem_imm
    InstructionFormat::BinaryImm, // srem_imm
    InstructionFormat::BinaryImm, // irsub_imm
    InstructionFormat::Ternary, // iadd_cin
    InstructionFormat::Ternary, // iadd_ifcin
    InstructionFormat::Binary, // iadd_cout
    InstructionFormat::Binary, // iadd_ifcout
    InstructionFormat::Ternary, // iadd_carry
    InstructionFormat::Ternary, // iadd_ifcarry
    InstructionFormat::Ternary, // isub_bin
    InstructionFormat::Ternary, // isub_ifbin
    InstructionFormat::Binary, // isub_bout
    InstructionFormat::Binary, // isub_ifbout
    InstructionFormat::Ternary, // isub_borrow
    InstructionFormat::Ternary, // isub_ifborrow
    InstructionFormat::Binary, // band
    InstructionFormat::Binary, // bor
    InstructionFormat::Binary, // bxor
    InstructionFormat::Unary, // bnot
    InstructionFormat::Binary, // band_not
    InstructionFormat::Binary, // bor_not
    InstructionFormat::Binary, // bxor_not
    InstructionFormat::BinaryImm, // band_imm
    InstructionFormat::BinaryImm, // bor_imm
    InstructionFormat::BinaryImm, // bxor_imm
    InstructionFormat::Binary, // rotl
    InstructionFormat::Binary, // rotr
    InstructionFormat::BinaryImm, // rotl_imm
    InstructionFormat::BinaryImm, // rotr_imm
    InstructionFormat::Binary, // ishl
    InstructionFormat::Binary, // ushr
    InstructionFormat::Binary, // sshr
    InstructionFormat::BinaryImm, // ishl_imm
    InstructionFormat::BinaryImm, // ushr_imm
    InstructionFormat::BinaryImm, // sshr_imm
    InstructionFormat::Unary, // bitrev
    InstructionFormat::Unary, // clz
    InstructionFormat::Unary, // cls
    InstructionFormat::Unary, // ctz
    InstructionFormat::Unary, // popcnt
    InstructionFormat::FloatCompare, // fcmp
    InstructionFormat::Binary, // ffcmp
    InstructionFormat::Binary, // fadd
    InstructionFormat::Binary, // fsub
    InstructionFormat::Binary, // fmul
    InstructionFormat::Binary, // fdiv
    InstructionFormat::Unary, // sqrt
    InstructionFormat::Ternary, // fma
    InstructionFormat::Unary, // fneg
    InstructionFormat::Unary, // fabs
    InstructionFormat::Binary, // fcopysign
    InstructionFormat::Binary, // fmin
    InstructionFormat::Binary, // fmax
    InstructionFormat::Unary, // ceil
    InstructionFormat::Unary, // floor
    InstructionFormat::Unary, // trunc
    InstructionFormat::Unary, // nearest
    InstructionFormat::Unary, // is_null
    InstructionFormat::IntCond, // trueif
    InstructionFormat::FloatCond, // trueff
    InstructionFormat::Unary, // bitcast
    InstructionFormat::Unary, // raw_bitcast
    InstructionFormat::Unary, // scalar_to_vector
    InstructionFormat::Unary, // breduce
    InstructionFormat::Unary, // bextend
    InstructionFormat::Unary, // bint
    InstructionFormat::Unary, // bmask
    InstructionFormat::Unary, // ireduce
    InstructionFormat::Unary, // uextend
    InstructionFormat::Unary, // sextend
    InstructionFormat::Unary, // fpromote
    InstructionFormat::Unary, // fdemote
    InstructionFormat::Unary, // fcvt_to_uint
    InstructionFormat::Unary, // fcvt_to_uint_sat
    InstructionFormat::Unary, // fcvt_to_sint
    InstructionFormat::Unary, // fcvt_to_sint_sat
    InstructionFormat::Unary, // fcvt_from_uint
    InstructionFormat::Unary, // fcvt_from_sint
    InstructionFormat::Unary, // isplit
    InstructionFormat::Binary, // iconcat
    InstructionFormat::Ternary, // x86_udivmodx
    InstructionFormat::Ternary, // x86_sdivmodx
    InstructionFormat::Binary, // x86_umulx
    InstructionFormat::Binary, // x86_smulx
    InstructionFormat::Unary, // x86_cvtt2si
    InstructionFormat::Binary, // x86_fmin
    InstructionFormat::Binary, // x86_fmax
    InstructionFormat::Unary, // x86_push
    InstructionFormat::NullAry, // x86_pop
    InstructionFormat::Unary, // x86_bsr
    InstructionFormat::Unary, // x86_bsf
    InstructionFormat::ExtractLane, // x86_pshufd
    InstructionFormat::Binary, // x86_pshufb
    InstructionFormat::ExtractLane, // x86_pextr
    InstructionFormat::InsertLane, // x86_pinsr
    InstructionFormat::InsertLane, // x86_insertps
    InstructionFormat::Binary, // x86_movsd
    InstructionFormat::Binary, // x86_movlhps
];

fn opcode_name(opc: Opcode) -> &'static str {
    match opc {
        Opcode::AdjustSpDown => {
            "adjust_sp_down"
        }
        Opcode::AdjustSpDownImm => {
            "adjust_sp_down_imm"
        }
        Opcode::AdjustSpUpImm => {
            "adjust_sp_up_imm"
        }
        Opcode::Band => {
            "band"
        }
        Opcode::BandImm => {
            "band_imm"
        }
        Opcode::BandNot => {
            "band_not"
        }
        Opcode::Bconst => {
            "bconst"
        }
        Opcode::Bextend => {
            "bextend"
        }
        Opcode::Bint => {
            "bint"
        }
        Opcode::Bitcast => {
            "bitcast"
        }
        Opcode::Bitrev => {
            "bitrev"
        }
        Opcode::Bmask => {
            "bmask"
        }
        Opcode::Bnot => {
            "bnot"
        }
        Opcode::Bor => {
            "bor"
        }
        Opcode::BorImm => {
            "bor_imm"
        }
        Opcode::BorNot => {
            "bor_not"
        }
        Opcode::BrIcmp => {
            "br_icmp"
        }
        Opcode::BrTable => {
            "br_table"
        }
        Opcode::Breduce => {
            "breduce"
        }
        Opcode::Brff => {
            "brff"
        }
        Opcode::Brif => {
            "brif"
        }
        Opcode::Brnz => {
            "brnz"
        }
        Opcode::Brz => {
            "brz"
        }
        Opcode::Bxor => {
            "bxor"
        }
        Opcode::BxorImm => {
            "bxor_imm"
        }
        Opcode::BxorNot => {
            "bxor_not"
        }
        Opcode::Call => {
            "call"
        }
        Opcode::CallIndirect => {
            "call_indirect"
        }
        Opcode::Ceil => {
            "ceil"
        }
        Opcode::Cls => {
            "cls"
        }
        Opcode::Clz => {
            "clz"
        }
        Opcode::Copy => {
            "copy"
        }
        Opcode::CopyNop => {
            "copy_nop"
        }
        Opcode::CopySpecial => {
            "copy_special"
        }
        Opcode::CopyToSsa => {
            "copy_to_ssa"
        }
        Opcode::Ctz => {
            "ctz"
        }
        Opcode::Debugtrap => {
            "debugtrap"
        }
        Opcode::Extractlane => {
            "extractlane"
        }
        Opcode::F32const => {
            "f32const"
        }
        Opcode::F64const => {
            "f64const"
        }
        Opcode::Fabs => {
            "fabs"
        }
        Opcode::Fadd => {
            "fadd"
        }
        Opcode::Fallthrough => {
            "fallthrough"
        }
        Opcode::FallthroughReturn => {
            "fallthrough_return"
        }
        Opcode::Fcmp => {
            "fcmp"
        }
        Opcode::Fcopysign => {
            "fcopysign"
        }
        Opcode::FcvtFromSint => {
            "fcvt_from_sint"
        }
        Opcode::FcvtFromUint => {
            "fcvt_from_uint"
        }
        Opcode::FcvtToSint => {
            "fcvt_to_sint"
        }
        Opcode::FcvtToSintSat => {
            "fcvt_to_sint_sat"
        }
        Opcode::FcvtToUint => {
            "fcvt_to_uint"
        }
        Opcode::FcvtToUintSat => {
            "fcvt_to_uint_sat"
        }
        Opcode::Fdemote => {
            "fdemote"
        }
        Opcode::Fdiv => {
            "fdiv"
        }
        Opcode::Ffcmp => {
            "ffcmp"
        }
        Opcode::Fill => {
            "fill"
        }
        Opcode::FillNop => {
            "fill_nop"
        }
        Opcode::Floor => {
            "floor"
        }
        Opcode::Fma => {
            "fma"
        }
        Opcode::Fmax => {
            "fmax"
        }
        Opcode::Fmin => {
            "fmin"
        }
        Opcode::Fmul => {
            "fmul"
        }
        Opcode::Fneg => {
            "fneg"
        }
        Opcode::Fpromote => {
            "fpromote"
        }
        Opcode::Fsub => {
            "fsub"
        }
        Opcode::FuncAddr => {
            "func_addr"
        }
        Opcode::GetPinnedReg => {
            "get_pinned_reg"
        }
        Opcode::GlobalValue => {
            "global_value"
        }
        Opcode::HeapAddr => {
            "heap_addr"
        }
        Opcode::Iadd => {
            "iadd"
        }
        Opcode::IaddCarry => {
            "iadd_carry"
        }
        Opcode::IaddCin => {
            "iadd_cin"
        }
        Opcode::IaddCout => {
            "iadd_cout"
        }
        Opcode::IaddIfcarry => {
            "iadd_ifcarry"
        }
        Opcode::IaddIfcin => {
            "iadd_ifcin"
        }
        Opcode::IaddIfcout => {
            "iadd_ifcout"
        }
        Opcode::IaddImm => {
            "iadd_imm"
        }
        Opcode::Icmp => {
            "icmp"
        }
        Opcode::IcmpImm => {
            "icmp_imm"
        }
        Opcode::Iconcat => {
            "iconcat"
        }
        Opcode::Iconst => {
            "iconst"
        }
        Opcode::Ifcmp => {
            "ifcmp"
        }
        Opcode::IfcmpImm => {
            "ifcmp_imm"
        }
        Opcode::IfcmpSp => {
            "ifcmp_sp"
        }
        Opcode::Imul => {
            "imul"
        }
        Opcode::ImulImm => {
            "imul_imm"
        }
        Opcode::IndirectJumpTableBr => {
            "indirect_jump_table_br"
        }
        Opcode::Ineg => {
            "ineg"
        }
        Opcode::Insertlane => {
            "insertlane"
        }
        Opcode::Ireduce => {
            "ireduce"
        }
        Opcode::IrsubImm => {
            "irsub_imm"
        }
        Opcode::IsNull => {
            "is_null"
        }
        Opcode::Ishl => {
            "ishl"
        }
        Opcode::IshlImm => {
            "ishl_imm"
        }
        Opcode::Isplit => {
            "isplit"
        }
        Opcode::Istore16 => {
            "istore16"
        }
        Opcode::Istore16Complex => {
            "istore16_complex"
        }
        Opcode::Istore32 => {
            "istore32"
        }
        Opcode::Istore32Complex => {
            "istore32_complex"
        }
        Opcode::Istore8 => {
            "istore8"
        }
        Opcode::Istore8Complex => {
            "istore8_complex"
        }
        Opcode::Isub => {
            "isub"
        }
        Opcode::IsubBin => {
            "isub_bin"
        }
        Opcode::IsubBorrow => {
            "isub_borrow"
        }
        Opcode::IsubBout => {
            "isub_bout"
        }
        Opcode::IsubIfbin => {
            "isub_ifbin"
        }
        Opcode::IsubIfborrow => {
            "isub_ifborrow"
        }
        Opcode::IsubIfbout => {
            "isub_ifbout"
        }
        Opcode::Jump => {
            "jump"
        }
        Opcode::JumpTableBase => {
            "jump_table_base"
        }
        Opcode::JumpTableEntry => {
            "jump_table_entry"
        }
        Opcode::Load => {
            "load"
        }
        Opcode::LoadComplex => {
            "load_complex"
        }
        Opcode::Nearest => {
            "nearest"
        }
        Opcode::Nop => {
            "nop"
        }
        Opcode::Null => {
            "null"
        }
        Opcode::Popcnt => {
            "popcnt"
        }
        Opcode::RawBitcast => {
            "raw_bitcast"
        }
        Opcode::Regfill => {
            "regfill"
        }
        Opcode::Regmove => {
            "regmove"
        }
        Opcode::Regspill => {
            "regspill"
        }
        Opcode::ResumableTrap => {
            "resumable_trap"
        }
        Opcode::Return => {
            "return"
        }
        Opcode::Rotl => {
            "rotl"
        }
        Opcode::RotlImm => {
            "rotl_imm"
        }
        Opcode::Rotr => {
            "rotr"
        }
        Opcode::RotrImm => {
            "rotr_imm"
        }
        Opcode::SaddSat => {
            "sadd_sat"
        }
        Opcode::Safepoint => {
            "safepoint"
        }
        Opcode::ScalarToVector => {
            "scalar_to_vector"
        }
        Opcode::Sdiv => {
            "sdiv"
        }
        Opcode::SdivImm => {
            "sdiv_imm"
        }
        Opcode::Select => {
            "select"
        }
        Opcode::Selectif => {
            "selectif"
        }
        Opcode::SetPinnedReg => {
            "set_pinned_reg"
        }
        Opcode::Sextend => {
            "sextend"
        }
        Opcode::Shuffle => {
            "shuffle"
        }
        Opcode::Sload16 => {
            "sload16"
        }
        Opcode::Sload16Complex => {
            "sload16_complex"
        }
        Opcode::Sload32 => {
            "sload32"
        }
        Opcode::Sload32Complex => {
            "sload32_complex"
        }
        Opcode::Sload8 => {
            "sload8"
        }
        Opcode::Sload8Complex => {
            "sload8_complex"
        }
        Opcode::Smulhi => {
            "smulhi"
        }
        Opcode::Spill => {
            "spill"
        }
        Opcode::Splat => {
            "splat"
        }
        Opcode::Sqrt => {
            "sqrt"
        }
        Opcode::Srem => {
            "srem"
        }
        Opcode::SremImm => {
            "srem_imm"
        }
        Opcode::Sshr => {
            "sshr"
        }
        Opcode::SshrImm => {
            "sshr_imm"
        }
        Opcode::SsubSat => {
            "ssub_sat"
        }
        Opcode::StackAddr => {
            "stack_addr"
        }
        Opcode::StackLoad => {
            "stack_load"
        }
        Opcode::StackStore => {
            "stack_store"
        }
        Opcode::Store => {
            "store"
        }
        Opcode::StoreComplex => {
            "store_complex"
        }
        Opcode::SymbolValue => {
            "symbol_value"
        }
        Opcode::TableAddr => {
            "table_addr"
        }
        Opcode::Trap => {
            "trap"
        }
        Opcode::Trapff => {
            "trapff"
        }
        Opcode::Trapif => {
            "trapif"
        }
        Opcode::Trapnz => {
            "trapnz"
        }
        Opcode::Trapz => {
            "trapz"
        }
        Opcode::Trueff => {
            "trueff"
        }
        Opcode::Trueif => {
            "trueif"
        }
        Opcode::Trunc => {
            "trunc"
        }
        Opcode::UaddSat => {
            "uadd_sat"
        }
        Opcode::Udiv => {
            "udiv"
        }
        Opcode::UdivImm => {
            "udiv_imm"
        }
        Opcode::Uextend => {
            "uextend"
        }
        Opcode::Uload16 => {
            "uload16"
        }
        Opcode::Uload16Complex => {
            "uload16_complex"
        }
        Opcode::Uload32 => {
            "uload32"
        }
        Opcode::Uload32Complex => {
            "uload32_complex"
        }
        Opcode::Uload8 => {
            "uload8"
        }
        Opcode::Uload8Complex => {
            "uload8_complex"
        }
        Opcode::Umulhi => {
            "umulhi"
        }
        Opcode::Urem => {
            "urem"
        }
        Opcode::UremImm => {
            "urem_imm"
        }
        Opcode::Ushr => {
            "ushr"
        }
        Opcode::UshrImm => {
            "ushr_imm"
        }
        Opcode::UsubSat => {
            "usub_sat"
        }
        Opcode::Vconcat => {
            "vconcat"
        }
        Opcode::Vconst => {
            "vconst"
        }
        Opcode::Vselect => {
            "vselect"
        }
        Opcode::Vsplit => {
            "vsplit"
        }
        Opcode::X86Bsf => {
            "x86_bsf"
        }
        Opcode::X86Bsr => {
            "x86_bsr"
        }
        Opcode::X86Cvtt2si => {
            "x86_cvtt2si"
        }
        Opcode::X86Fmax => {
            "x86_fmax"
        }
        Opcode::X86Fmin => {
            "x86_fmin"
        }
        Opcode::X86Insertps => {
            "x86_insertps"
        }
        Opcode::X86Movlhps => {
            "x86_movlhps"
        }
        Opcode::X86Movsd => {
            "x86_movsd"
        }
        Opcode::X86Pextr => {
            "x86_pextr"
        }
        Opcode::X86Pinsr => {
            "x86_pinsr"
        }
        Opcode::X86Pop => {
            "x86_pop"
        }
        Opcode::X86Pshufb => {
            "x86_pshufb"
        }
        Opcode::X86Pshufd => {
            "x86_pshufd"
        }
        Opcode::X86Push => {
            "x86_push"
        }
        Opcode::X86Sdivmodx => {
            "x86_sdivmodx"
        }
        Opcode::X86Smulx => {
            "x86_smulx"
        }
        Opcode::X86Udivmodx => {
            "x86_udivmodx"
        }
        Opcode::X86Umulx => {
            "x86_umulx"
        }
    }
}

const OPCODE_HASH_TABLE: [Option<Opcode>; 256] = [
    Some(Opcode::Regfill),
    Some(Opcode::Imul),
    None,
    Some(Opcode::Brif),
    Some(Opcode::HeapAddr),
    Some(Opcode::FcvtToSintSat),
    Some(Opcode::Fsub),
    Some(Opcode::Rotr),
    Some(Opcode::TableAddr),
    Some(Opcode::Iconst),
    Some(Opcode::Uload32Complex),
    Some(Opcode::Ifcmp),
    None,
    Some(Opcode::Store),
    Some(Opcode::Brnz),
    Some(Opcode::Fallthrough),
    Some(Opcode::Isub),
    Some(Opcode::UshrImm),
    Some(Opcode::Brff),
    Some(Opcode::Trap),
    Some(Opcode::Srem),
    Some(Opcode::SshrImm),
    Some(Opcode::Uload16Complex),
    Some(Opcode::Sdiv),
    Some(Opcode::JumpTableEntry),
    Some(Opcode::IaddIfcarry),
    Some(Opcode::IsubIfborrow),
    Some(Opcode::FcvtFromUint),
    Some(Opcode::SremImm),
    Some(Opcode::Insertlane),
    Some(Opcode::BxorNot),
    Some(Opcode::Load),
    Some(Opcode::Fneg),
    Some(Opcode::Bxor),
    Some(Opcode::Jump),
    Some(Opcode::Null),
    Some(Opcode::Shuffle),
    Some(Opcode::Copy),
    Some(Opcode::Umulhi),
    Some(Opcode::Ushr),
    Some(Opcode::Fadd),
    Some(Opcode::BxorImm),
    None,
    Some(Opcode::Fmax),
    Some(Opcode::Urem),
    Some(Opcode::Band),
    Some(Opcode::CopyNop),
    Some(Opcode::Fabs),
    Some(Opcode::Ishl),
    Some(Opcode::X86Cvtt2si),
    Some(Opcode::Vconst),
    Some(Opcode::Call),
    Some(Opcode::X86Insertps),
    Some(Opcode::Sqrt),
    Some(Opcode::Fill),
    None,
    Some(Opcode::Ceil),
    Some(Opcode::Ineg),
    Some(Opcode::FuncAddr),
    Some(Opcode::SaddSat),
    Some(Opcode::Popcnt),
    Some(Opcode::X86Bsr),
    Some(Opcode::FcvtToUint),
    Some(Opcode::Fmin),
    None,
    Some(Opcode::GlobalValue),
    Some(Opcode::Bnot),
    Some(Opcode::Sextend),
    Some(Opcode::Trueff),
    Some(Opcode::Trapff),
    Some(Opcode::Trunc),
    Some(Opcode::Bint),
    Some(Opcode::RotlImm),
    Some(Opcode::Fcmp),
    Some(Opcode::FcvtToSint),
    Some(Opcode::IaddIfcin),
    Some(Opcode::Fmul),
    Some(Opcode::AdjustSpDownImm),
    Some(Opcode::IsubBin),
    None,
    Some(Opcode::Trapif),
    Some(Opcode::Nearest),
    Some(Opcode::Fdiv),
    Some(Opcode::FcvtFromSint),
    Some(Opcode::BrIcmp),
    Some(Opcode::X86Fmax),
    Some(Opcode::UremImm),
    Some(Opcode::Trueif),
    Some(Opcode::LoadComplex),
    Some(Opcode::Trapnz),
    Some(Opcode::Uload16),
    Some(Opcode::CopyToSsa),
    Some(Opcode::Uload32),
    Some(Opcode::Safepoint),
    Some(Opcode::Bitrev),
    Some(Opcode::IaddCarry),
    Some(Opcode::Smulhi),
    Some(Opcode::IsNull),
    Some(Opcode::Vsplit),
    None,
    Some(Opcode::IaddImm),
    Some(Opcode::RawBitcast),
    Some(Opcode::X86Udivmodx),
    Some(Opcode::IfcmpSp),
    Some(Opcode::BorNot),
    Some(Opcode::X86Fmin),
    Some(Opcode::AdjustSpUpImm),
    Some(Opcode::IsubIfbout),
    Some(Opcode::Istore8Complex),
    Some(Opcode::X86Pop),
    None,
    Some(Opcode::SetPinnedReg),
    Some(Opcode::Isplit),
    None,
    None,
    None,
    Some(Opcode::X86Movsd),
    Some(Opcode::ImulImm),
    Some(Opcode::Ireduce),
    None,
    Some(Opcode::RotrImm),
    Some(Opcode::Fdemote),
    Some(Opcode::StackStore),
    None,
    Some(Opcode::Select),
    Some(Opcode::IaddCin),
    Some(Opcode::Istore32),
    Some(Opcode::Selectif),
    Some(Opcode::Istore16),
    Some(Opcode::Bconst),
    None,
    Some(Opcode::BorImm),
    Some(Opcode::Uload8Complex),
    Some(Opcode::IcmpImm),
    None,
    None,
    Some(Opcode::Sload16),
    Some(Opcode::FillNop),
    Some(Opcode::Fcopysign),
    Some(Opcode::SdivImm),
    Some(Opcode::ResumableTrap),
    Some(Opcode::Breduce),
    Some(Opcode::Sload32),
    None,
    Some(Opcode::X86Pshufb),
    Some(Opcode::Extractlane),
    Some(Opcode::StackAddr),
    Some(Opcode::X86Sdivmodx),
    Some(Opcode::BandImm),
    Some(Opcode::IsubBorrow),
    Some(Opcode::Return),
    Some(Opcode::X86Movlhps),
    Some(Opcode::X86Pshufd),
    None,
    None,
    Some(Opcode::X86Umulx),
    Some(Opcode::Sload8Complex),
    Some(Opcode::UsubSat),
    Some(Opcode::JumpTableBase),
    Some(Opcode::Sload16Complex),
    None,
    Some(Opcode::Sload32Complex),
    Some(Opcode::IsubIfbin),
    Some(Opcode::Iconcat),
    None,
    None,
    Some(Opcode::Regspill),
    Some(Opcode::X86Pextr),
    None,
    Some(Opcode::Fma),
    None,
    Some(Opcode::Istore8),
    Some(Opcode::BrTable),
    Some(Opcode::F64const),
    Some(Opcode::Nop),
    Some(Opcode::StackLoad),
    Some(Opcode::IrsubImm),
    Some(Opcode::IaddIfcout),
    Some(Opcode::StoreComplex),
    Some(Opcode::IsubBout),
    Some(Opcode::Debugtrap),
    Some(Opcode::BandNot),
    Some(Opcode::Bor),
    Some(Opcode::IshlImm),
    Some(Opcode::Clz),
    Some(Opcode::Ctz),
    Some(Opcode::AdjustSpDown),
    Some(Opcode::Ffcmp),
    Some(Opcode::Uextend),
    Some(Opcode::Brz),
    Some(Opcode::Bextend),
    Some(Opcode::X86Smulx),
    Some(Opcode::Floor),
    Some(Opcode::UaddSat),
    Some(Opcode::Bitcast),
    None,
    Some(Opcode::IndirectJumpTableBr),
    Some(Opcode::X86Push),
    Some(Opcode::Cls),
    Some(Opcode::Fpromote),
    Some(Opcode::X86Pinsr),
    Some(Opcode::X86Bsf),
    Some(Opcode::SymbolValue),
    Some(Opcode::Bmask),
    Some(Opcode::FcvtToUintSat),
    Some(Opcode::GetPinnedReg),
    Some(Opcode::SsubSat),
    Some(Opcode::Vselect),
    None,
    Some(Opcode::ScalarToVector),
    None,
    None,
    Some(Opcode::Uload8),
    Some(Opcode::FallthroughReturn),
    None,
    None,
    Some(Opcode::Trapz),
    None,
    Some(Opcode::Udiv),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(Opcode::CopySpecial),
    Some(Opcode::Rotl),
    None,
    Some(Opcode::Regmove),
    None,
    None,
    None,
    Some(Opcode::F32const),
    Some(Opcode::UdivImm),
    None,
    Some(Opcode::Spill),
    Some(Opcode::Splat),
    None,
    Some(Opcode::CallIndirect),
    Some(Opcode::Sload8),
    None,
    Some(Opcode::Istore16Complex),
    None,
    None,
    Some(Opcode::IfcmpImm),
    Some(Opcode::Icmp),
    None,
    None,
    Some(Opcode::Iadd),
    None,
    Some(Opcode::IaddCout),
    Some(Opcode::Vconcat),
    None,
    Some(Opcode::Sshr),
    None,
    Some(Opcode::Istore32Complex),
];

// Table of opcode constraints.
const OPCODE_CONSTRAINTS: [OpcodeConstraints; 205] = [
    // Jump: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Fallthrough: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Brz: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 0,
        constraint_offset: 0,
    },
    // Brnz: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 0,
        constraint_offset: 0,
    },
    // BrIcmp: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x58,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // Brif: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Concrete(ir::types::IFLAGS)']
    OpcodeConstraints {
        flags: 0x20,
        typeset_offset: 255,
        constraint_offset: 3,
    },
    // Brff: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Concrete(ir::types::FFLAGS)']
    OpcodeConstraints {
        flags: 0x20,
        typeset_offset: 255,
        constraint_offset: 4,
    },
    // BrTable: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // JumpTableEntry: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // JumpTableBase: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // IndirectJumpTableBr: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // Debugtrap: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Trap: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Trapz: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 0,
        constraint_offset: 0,
    },
    // ResumableTrap: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Trapnz: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 0,
        constraint_offset: 0,
    },
    // Trapif: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Concrete(ir::types::IFLAGS)']
    OpcodeConstraints {
        flags: 0x20,
        typeset_offset: 255,
        constraint_offset: 3,
    },
    // Trapff: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Concrete(ir::types::FFLAGS)']
    OpcodeConstraints {
        flags: 0x20,
        typeset_offset: 255,
        constraint_offset: 4,
    },
    // Return: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // FallthroughReturn: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Call: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // CallIndirect: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // FuncAddr: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // Load: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 3,
        constraint_offset: 5,
    },
    // LoadComplex: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 3,
        constraint_offset: 0,
    },
    // Store: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x58,
        typeset_offset: 3,
        constraint_offset: 5,
    },
    // StoreComplex: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 3,
        constraint_offset: 0,
    },
    // Uload8: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={16, 32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 4,
        constraint_offset: 5,
    },
    // Uload8Complex: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={16, 32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 4,
        constraint_offset: 0,
    },
    // Sload8: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={16, 32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 4,
        constraint_offset: 5,
    },
    // Sload8Complex: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={16, 32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 4,
        constraint_offset: 0,
    },
    // Istore8: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={16, 32, 64})
    OpcodeConstraints {
        flags: 0x58,
        typeset_offset: 4,
        constraint_offset: 5,
    },
    // Istore8Complex: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={16, 32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 4,
        constraint_offset: 0,
    },
    // Uload16: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 2,
        constraint_offset: 5,
    },
    // Uload16Complex: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // Sload16: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 2,
        constraint_offset: 5,
    },
    // Sload16Complex: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // Istore16: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x58,
        typeset_offset: 2,
        constraint_offset: 5,
    },
    // Istore16Complex: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // Uload32: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Concrete(ir::types::I64)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 2,
        constraint_offset: 7,
    },
    // Uload32Complex: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Concrete(ir::types::I64)']
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 255,
        constraint_offset: 7,
    },
    // Sload32: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Concrete(ir::types::I64)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 2,
        constraint_offset: 7,
    },
    // Sload32Complex: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Concrete(ir::types::I64)']
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 255,
        constraint_offset: 7,
    },
    // Istore32: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['Concrete(ir::types::I64)', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={64})
    OpcodeConstraints {
        flags: 0x58,
        typeset_offset: 5,
        constraint_offset: 9,
    },
    // Istore32Complex: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Concrete(ir::types::I64)']
    OpcodeConstraints {
        flags: 0x20,
        typeset_offset: 255,
        constraint_offset: 7,
    },
    // StackLoad: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 3,
        constraint_offset: 0,
    },
    // StackStore: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 3,
        constraint_offset: 0,
    },
    // StackAddr: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // GlobalValue: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 3,
        constraint_offset: 0,
    },
    // SymbolValue: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 3,
        constraint_offset: 0,
    },
    // HeapAddr: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 2,
        constraint_offset: 5,
    },
    // GetPinnedReg: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // SetPinnedReg: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // TableAddr: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(2)']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 2,
        constraint_offset: 5,
    },
    // Iconst: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // F32const: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Concrete(ir::types::F32)']
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 255,
        constraint_offset: 11,
    },
    // F64const: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Concrete(ir::types::F64)']
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 255,
        constraint_offset: 12,
    },
    // Bconst: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 7,
        constraint_offset: 0,
    },
    // Vconst: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 8,
        constraint_offset: 0,
    },
    // Shuffle: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={16}, ints={8}, bools={8})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 9,
        constraint_offset: 0,
    },
    // Null: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 10,
        constraint_offset: 0,
    },
    // Nop: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Select: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Free(0)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x69,
        typeset_offset: 11,
        constraint_offset: 13,
    },
    // Selectif: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Concrete(ir::types::IFLAGS)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x61,
        typeset_offset: 11,
        constraint_offset: 16,
    },
    // Copy: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // Spill: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // Fill: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // FillNop: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // Regmove: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // CopySpecial: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // CopyToSsa: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // CopyNop: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // AdjustSpDown: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // AdjustSpUpImm: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // AdjustSpDownImm: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // IfcmpSp: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Concrete(ir::types::IFLAGS)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 2,
        constraint_offset: 17,
    },
    // Regspill: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // Regfill: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 11,
        constraint_offset: 0,
    },
    // Safepoint: fixed_results=0, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=[]
    OpcodeConstraints {
        flags: 0x00,
        typeset_offset: 255,
        constraint_offset: 0,
    },
    // Vsplit: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['HalfVector', 'HalfVector', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x3a,
        typeset_offset: 8,
        constraint_offset: 20,
    },
    // Vconcat: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['DoubleVector', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x59,
        typeset_offset: 12,
        constraint_offset: 23,
    },
    // Vselect: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'AsBool', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x69,
        typeset_offset: 8,
        constraint_offset: 25,
    },
    // Splat: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'LaneOf']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 8,
        constraint_offset: 28,
    },
    // Insertlane: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'LaneOf']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 8,
        constraint_offset: 27,
    },
    // Extractlane: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['LaneOf', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 8,
        constraint_offset: 29,
    },
    // Icmp: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['AsBool', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x59,
        typeset_offset: 6,
        constraint_offset: 26,
    },
    // IcmpImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Concrete(ir::types::B1)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 1,
        constraint_offset: 31,
    },
    // Ifcmp: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['Concrete(ir::types::IFLAGS)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x59,
        typeset_offset: 1,
        constraint_offset: 17,
    },
    // IfcmpImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Concrete(ir::types::IFLAGS)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 1,
        constraint_offset: 17,
    },
    // Iadd: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // UaddSat: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // SaddSat: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Isub: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // UsubSat: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // SsubSat: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Ineg: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Imul: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Umulhi: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Smulhi: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Udiv: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Sdiv: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Urem: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Srem: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // IaddImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // ImulImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // UdivImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // SdivImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // UremImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // SremImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // IrsubImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // IaddCin: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Same', 'Same', 'Concrete(ir::types::B1)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x69,
        typeset_offset: 1,
        constraint_offset: 32,
    },
    // IaddIfcin: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Same', 'Same', 'Concrete(ir::types::IFLAGS)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x69,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // IaddCout: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Concrete(ir::types::B1)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x4a,
        typeset_offset: 1,
        constraint_offset: 30,
    },
    // IaddIfcout: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Concrete(ir::types::IFLAGS)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x4a,
        typeset_offset: 1,
        constraint_offset: 16,
    },
    // IaddCarry: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Concrete(ir::types::B1)', 'Same', 'Same', 'Concrete(ir::types::B1)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x6a,
        typeset_offset: 1,
        constraint_offset: 34,
    },
    // IaddIfcarry: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Concrete(ir::types::IFLAGS)', 'Same', 'Same', 'Concrete(ir::types::IFLAGS)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x6a,
        typeset_offset: 1,
        constraint_offset: 39,
    },
    // IsubBin: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Same', 'Same', 'Concrete(ir::types::B1)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x69,
        typeset_offset: 1,
        constraint_offset: 32,
    },
    // IsubIfbin: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Same', 'Same', 'Concrete(ir::types::IFLAGS)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x69,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // IsubBout: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Concrete(ir::types::B1)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x4a,
        typeset_offset: 1,
        constraint_offset: 30,
    },
    // IsubIfbout: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Concrete(ir::types::IFLAGS)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x4a,
        typeset_offset: 1,
        constraint_offset: 16,
    },
    // IsubBorrow: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Concrete(ir::types::B1)', 'Same', 'Same', 'Concrete(ir::types::B1)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x6a,
        typeset_offset: 1,
        constraint_offset: 34,
    },
    // IsubIfborrow: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Concrete(ir::types::IFLAGS)', 'Same', 'Same', 'Concrete(ir::types::IFLAGS)']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x6a,
        typeset_offset: 1,
        constraint_offset: 39,
    },
    // Band: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 13,
        constraint_offset: 0,
    },
    // Bor: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 13,
        constraint_offset: 0,
    },
    // Bxor: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 13,
        constraint_offset: 0,
    },
    // Bnot: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 13,
        constraint_offset: 0,
    },
    // BandNot: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 13,
        constraint_offset: 0,
    },
    // BorNot: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 13,
        constraint_offset: 0,
    },
    // BxorNot: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 13,
        constraint_offset: 0,
    },
    // BandImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // BorImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // BxorImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // Rotl: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Free(1)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 44,
    },
    // Rotr: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Free(1)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 44,
    },
    // RotlImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // RotrImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Ishl: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Free(1)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 44,
    },
    // Ushr: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Free(1)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 44,
    },
    // Sshr: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Free(1)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 6,
        constraint_offset: 44,
    },
    // IshlImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // UshrImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // SshrImm: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 6,
        constraint_offset: 0,
    },
    // Bitrev: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // Clz: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // Cls: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // Ctz: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // Popcnt: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 1,
        constraint_offset: 0,
    },
    // Fcmp: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['AsBool', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x59,
        typeset_offset: 14,
        constraint_offset: 26,
    },
    // Ffcmp: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['Concrete(ir::types::FFLAGS)', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x59,
        typeset_offset: 14,
        constraint_offset: 47,
    },
    // Fadd: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fsub: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fmul: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fdiv: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Sqrt: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fma: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x69,
        typeset_offset: 14,
        constraint_offset: 48,
    },
    // Fneg: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fabs: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fcopysign: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fmin: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Fmax: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Ceil: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Floor: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Trunc: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // Nearest: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // IsNull: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Concrete(ir::types::B1)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 10,
        constraint_offset: 31,
    },
    // Trueif: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Concrete(ir::types::B1)', 'Concrete(ir::types::IFLAGS)']
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 255,
        constraint_offset: 52,
    },
    // Trueff: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Concrete(ir::types::B1)', 'Concrete(ir::types::FFLAGS)']
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 255,
        constraint_offset: 54,
    },
    // Bitcast: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(3)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 3,
        constraint_offset: 56,
    },
    // RawBitcast: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(11)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 11,
        constraint_offset: 58,
    },
    // ScalarToVector: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'LaneOf']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 8,
        constraint_offset: 28,
    },
    // Breduce: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(7)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 7,
        constraint_offset: 60,
    },
    // Bextend: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(7)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 7,
        constraint_offset: 60,
    },
    // Bint: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(7)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 60,
    },
    // Bmask: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(7)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 60,
    },
    // Ireduce: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(6)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 62,
    },
    // Uextend: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(6)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 62,
    },
    // Sextend: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(6)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 62,
    },
    // Fpromote: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(14)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 14,
        constraint_offset: 64,
    },
    // Fdemote: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(14)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 14,
        constraint_offset: 64,
    },
    // FcvtToUint: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(14)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 64,
    },
    // FcvtToUintSat: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(14)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 64,
    },
    // FcvtToSint: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(14)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 64,
    },
    // FcvtToSintSat: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(14)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 6,
        constraint_offset: 64,
    },
    // FcvtFromUint: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(6)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 14,
        constraint_offset: 62,
    },
    // FcvtFromSint: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(6)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 14,
        constraint_offset: 62,
    },
    // Isplit: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['HalfWidth', 'HalfWidth', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x3a,
        typeset_offset: 15,
        constraint_offset: 66,
    },
    // Iconcat: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=2
    // Constraints=['DoubleWidth', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64})
    OpcodeConstraints {
        flags: 0x59,
        typeset_offset: 16,
        constraint_offset: 69,
    },
    // X86Udivmodx: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Same', 'Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x6a,
        typeset_offset: 2,
        constraint_offset: 70,
    },
    // X86Sdivmodx: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=3
    // Constraints=['Same', 'Same', 'Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x6a,
        typeset_offset: 2,
        constraint_offset: 70,
    },
    // X86Umulx: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x4a,
        typeset_offset: 2,
        constraint_offset: 48,
    },
    // X86Smulx: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x4a,
        typeset_offset: 2,
        constraint_offset: 48,
    },
    // X86Cvtt2si: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Free(14)']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x21,
        typeset_offset: 17,
        constraint_offset: 64,
    },
    // X86Fmin: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // X86Fmax: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 14,
        constraint_offset: 0,
    },
    // X86Push: fixed_results=0, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x38,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // X86Pop: fixed_results=1, use_typevar_operand=false, requires_typevar_operand=false, fixed_values=0
    // Constraints=['Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x01,
        typeset_offset: 2,
        constraint_offset: 0,
    },
    // X86Bsr: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Concrete(ir::types::IFLAGS)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x2a,
        typeset_offset: 2,
        constraint_offset: 16,
    },
    // X86Bsf: fixed_results=2, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Concrete(ir::types::IFLAGS)', 'Same']
    // Polymorphic over TypeSet(lanes={1}, ints={32, 64})
    OpcodeConstraints {
        flags: 0x2a,
        typeset_offset: 2,
        constraint_offset: 16,
    },
    // X86Pshufd: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=1
    // Constraints=['Same', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x29,
        typeset_offset: 8,
        constraint_offset: 0,
    },
    // X86Pshufb: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 8,
        constraint_offset: 0,
    },
    // X86Pextr: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=true, fixed_values=1
    // Constraints=['LaneOf', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x39,
        typeset_offset: 8,
        constraint_offset: 29,
    },
    // X86Pinsr: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'LaneOf']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, bools={1, 8, 16, 32, 64, 128})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 18,
        constraint_offset: 27,
    },
    // X86Insertps: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'LaneOf']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 19,
        constraint_offset: 27,
    },
    // X86Movsd: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 19,
        constraint_offset: 0,
    },
    // X86Movlhps: fixed_results=1, use_typevar_operand=true, requires_typevar_operand=false, fixed_values=2
    // Constraints=['Same', 'Same', 'Same']
    // Polymorphic over TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
    OpcodeConstraints {
        flags: 0x49,
        typeset_offset: 19,
        constraint_offset: 0,
    },
];

// Table of value type sets.
const TYPE_SETS: [ir::instructions::ValueTypeSet; 20] = [
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1}, ints={8, 16, 32, 64, 128}, bools={1, 8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(1),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(249),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1}, ints={8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(1),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1}, ints={32, 64})
        lanes: BitSet::<u16>(1),
        ints: BitSet::<u8>(96),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(96),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1}, ints={16, 32, 64})
        lanes: BitSet::<u16>(1),
        ints: BitSet::<u8>(112),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1}, ints={64})
        lanes: BitSet::<u16>(1),
        ints: BitSet::<u8>(64),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, bools={1, 8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(0),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(249),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(510),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(96),
        bools: BitSet::<u8>(249),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={16}, ints={8}, bools={8})
        lanes: BitSet::<u16>(16),
        ints: BitSet::<u8>(8),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(8),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1}, refs={32, 64})
        lanes: BitSet::<u16>(1),
        ints: BitSet::<u8>(0),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(96),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128}, refs={32, 64})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(96),
        bools: BitSet::<u8>(249),
        refs: BitSet::<u8>(96),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(255),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(96),
        bools: BitSet::<u8>(249),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, floats={32, 64}, bools={1, 8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(96),
        bools: BitSet::<u8>(249),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(0),
        floats: BitSet::<u8>(96),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={16, 32, 64, 128})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(240),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(120),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={1, 2, 4, 8, 16, 32, 64, 128, 256}, ints={32, 64})
        lanes: BitSet::<u16>(511),
        ints: BitSet::<u8>(96),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, ints={8, 16, 32, 64, 128}, bools={1, 8, 16, 32, 64, 128})
        lanes: BitSet::<u16>(510),
        ints: BitSet::<u8>(248),
        floats: BitSet::<u8>(0),
        bools: BitSet::<u8>(249),
        refs: BitSet::<u8>(0),
    },
    ir::instructions::ValueTypeSet {
        // TypeSet(lanes={2, 4, 8, 16, 32, 64, 128, 256}, floats={32, 64})
        lanes: BitSet::<u16>(510),
        ints: BitSet::<u8>(0),
        floats: BitSet::<u8>(96),
        bools: BitSet::<u8>(0),
        refs: BitSet::<u8>(0),
    },
];

// Table of operand constraint sequences.
const OPERAND_CONSTRAINTS: [OperandConstraint; 75] = [
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::IFLAGS),
    OperandConstraint::Concrete(ir::types::FFLAGS),
    OperandConstraint::Same,
    OperandConstraint::Free(2),
    OperandConstraint::Concrete(ir::types::I64),
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::I64),
    OperandConstraint::Free(2),
    OperandConstraint::Concrete(ir::types::F32),
    OperandConstraint::Concrete(ir::types::F64),
    OperandConstraint::Same,
    OperandConstraint::Free(0),
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::IFLAGS),
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::HalfVector,
    OperandConstraint::HalfVector,
    OperandConstraint::Same,
    OperandConstraint::DoubleVector,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::AsBool,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::LaneOf,
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::B1),
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::B1),
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::B1),
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::IFLAGS),
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::IFLAGS),
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Free(1),
    OperandConstraint::Concrete(ir::types::FFLAGS),
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Concrete(ir::types::B1),
    OperandConstraint::Concrete(ir::types::IFLAGS),
    OperandConstraint::Concrete(ir::types::B1),
    OperandConstraint::Concrete(ir::types::FFLAGS),
    OperandConstraint::Same,
    OperandConstraint::Free(3),
    OperandConstraint::Same,
    OperandConstraint::Free(11),
    OperandConstraint::Same,
    OperandConstraint::Free(7),
    OperandConstraint::Same,
    OperandConstraint::Free(6),
    OperandConstraint::Same,
    OperandConstraint::Free(14),
    OperandConstraint::HalfWidth,
    OperandConstraint::HalfWidth,
    OperandConstraint::Same,
    OperandConstraint::DoubleWidth,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Same,
    OperandConstraint::Same,
];
