use alloc::{boxed::Box,vec::Vec};
use defmt::Format;
use crate::isa_model::{MapperLocation, ValueSize};

pub type Address = u32;

#[derive(Debug, PartialEq, Clone, Format)]
pub enum AtomicVB{
    I32Const {imm: i32},
    I64Const {imm: i64},
    F32Const {imm: u32},
    F64Const {imm: u64},
    Local {index:u32},
    Global {index:u32},
    Resolved {size:ValueSize, offset: usize},
    MemorySize,
    Unreachable
}

#[derive(Debug, PartialEq, Clone, Format)]
pub enum UnaryVB{
    I32Load {offset:Address, align:u8},
    I64Load {offset:Address, align:u8},
    F32Load {offset:Address, align:u8},
    F64Load {offset:Address, align:u8},
    I32Load8s {offset:Address, align:u8},
    I32Load8u {offset:Address, align:u8},
    I32Load16s {offset:Address, align:u8},
    I32Load16u {offset:Address, align:u8},
    I64Load8s {offset:Address, align:u8},
    I64Load8u {offset:Address, align:u8},
    I64Load16s {offset:Address, align:u8},
    I64Load16u {offset:Address, align:u8},
    I64Load32s {offset:Address, align:u8},
    I64Load32u {offset:Address, align:u8},
    I32EqZ,
    I64EqZ,
    I32Clz,
    I32Ctz,
    I32PopCnt,
    I64Clz,
    I64Ctz,
    I64PopCnt,
    F32Abs,
    F32Neg,
    F32Sqrt,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F64Abs,
    F64Neg,
    F64Sqrt,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
}


#[derive(Debug, PartialEq, Clone, Format)]
pub enum BinaryVB{
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32GeS,
    I32GeU,
    I32LeS,
    I32LeU,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64GeS,
    I64GeU,
    I64LeS,
    I64LeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Ge,
    F32Gt,
    F32Le,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Ge,
    F64Gt,
    F64Le,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign
}
#[derive(Debug, PartialEq, Clone)]
pub enum VB{
    AtomicVB(AtomicVB),
    UnaryVB {vb:UnaryVB, child: Box<VB> },
    BinaryVB{vb: BinaryVB, lhs: Box<VB>, rhs:Box<VB>},
    Select{selector:Box<VB>, lhs:Box<VB>, rhs:Box<VB>, size:ValueSize}
}

/// ContextVB is used to keep track of the position of a VB in a tree using Zippers
#[derive(Debug, PartialEq, Clone)]
pub enum ContextVB{
    Top,
    ContextUnaryVB{vb:UnaryVB, context: Box<ContextVB>},
    ContextBinaryVBLhs{vb:BinaryVB, rhs:Box<VB>, context: Box<ContextVB>},
    ContextBinaryVBRhs{vb:BinaryVB, lhs:Box<VB>, context: Box<ContextVB>},
    ContextSelectSelector{lhs:Box<VB>, rhs:Box<VB>, size:ValueSize, context: Box<ContextVB>},
    ContextSelectLhs{selector:Box<VB>, rhs:Box<VB>, size:ValueSize, context: Box<ContextVB>},
    ContextSelectRhs{selector:Box<VB>, lhs:Box<VB>, size:ValueSize, context: Box<ContextVB>}
}

/// VBTreeLocation is used to keep track of the position of a VB in a tree using Zippers
#[derive(Debug, PartialEq, Clone)]
pub struct VBTreeLocation (pub VB, pub ContextVB);

impl VBTreeLocation{
    
    /// Move the cursor to the parent of the current node. The current subtree is replaced by a placeholder node 
    pub fn move_up(self) -> Option<Self>{
        match self {
            VBTreeLocation(_ , ContextVB::Top) => None,
            VBTreeLocation(_, ContextVB::ContextUnaryVB{vb:unary_vb, context}) 
                => Some(VBTreeLocation(VB::UnaryVB {vb: unary_vb, child: Box::new(VB::AtomicVB(AtomicVB::Unreachable))}, *context)),
            VBTreeLocation(_, ContextVB::ContextBinaryVBLhs{vb: binary_vb, rhs, context})
            => Some(VBTreeLocation(VB::BinaryVB {vb: binary_vb, lhs: Box::new(VB::AtomicVB(AtomicVB::Unreachable)), rhs}, *context)),
            VBTreeLocation(_, ContextVB::ContextBinaryVBRhs{vb: binary_vb, lhs, context})
            => Some(VBTreeLocation(VB::BinaryVB {vb: binary_vb, lhs, rhs: Box::new(VB::AtomicVB(AtomicVB::Unreachable))}, *context)),
            VBTreeLocation(_, ContextVB::ContextSelectSelector{lhs, rhs, size, context})
            => Some(VBTreeLocation(VB::Select {selector: Box::new(VB::AtomicVB(AtomicVB::Unreachable)), lhs, rhs, size}, *context)),
            VBTreeLocation(_, ContextVB::ContextSelectLhs{selector, rhs, size, context})
            => Some(VBTreeLocation(VB::Select {selector, lhs: Box::new(VB::AtomicVB(AtomicVB::Unreachable)), rhs, size}, *context)),
            VBTreeLocation(_, ContextVB::ContextSelectRhs{selector, lhs, size, context})
            => Some(VBTreeLocation(VB::Select {selector, lhs, rhs: Box::new(VB::AtomicVB(AtomicVB::Unreachable)), size}, *context)),
        }
    }
    
    /// Move the cursor to the first child of the current node
    fn move_first_child(self) -> Option<Self>{
        match self {
            VBTreeLocation(VB::UnaryVB {vb, child}, context) => Some(VBTreeLocation(*child, ContextVB::ContextUnaryVB{vb, context: Box::new(context)})),
            VBTreeLocation(VB::BinaryVB {vb, lhs, rhs}, context) => Some(VBTreeLocation(*lhs, ContextVB::ContextBinaryVBLhs{vb, rhs, context: Box::new(context)})),
            VBTreeLocation(VB::Select {selector, lhs, rhs, size}, context) => Some(VBTreeLocation(*lhs, ContextVB::ContextSelectLhs{selector, rhs, size, context: Box::new(context)})),
            _ => None
        }
    }
    
    /// Move the cursor to the last child of the current node
    fn move_first_leaf(self) -> Self{
        let mut current = self;
        while current.0.has_child(){
            current = current.move_first_child().unwrap();
        }
        current
    }
    
    /// Move the cursor to the next sibling of the current node
    fn next_sibling(self) -> Option<Self>{
        match self {
            VBTreeLocation(_, ContextVB::Top) => None,
            VBTreeLocation(_, ContextVB::ContextUnaryVB{..}) 
                => None,
            VBTreeLocation(_, ContextVB::ContextBinaryVBLhs{vb: binary_vb, rhs, context})
            => Some(VBTreeLocation(*rhs, ContextVB::ContextBinaryVBRhs{vb: binary_vb, lhs: Box::new(VB::AtomicVB(AtomicVB::Unreachable)), context})),
            VBTreeLocation(_, ContextVB::ContextBinaryVBRhs{..})
            => None,
            VBTreeLocation(_, ContextVB::ContextSelectLhs{selector, rhs, size, context})
            => Some(VBTreeLocation(*rhs, ContextVB::ContextSelectRhs{selector, lhs: Box::new(VB::AtomicVB(AtomicVB::Unreachable)), size, context: Box::new(*context)})),
            VBTreeLocation(_, ContextVB::ContextSelectRhs{selector, lhs, size, context})
            => Some(VBTreeLocation(*selector, ContextVB::ContextSelectSelector{lhs, rhs: Box::new(VB::AtomicVB(AtomicVB::Unreachable)), size, context: Box::new(*context)})),
            VBTreeLocation(_, ContextVB::ContextSelectSelector{..})
            => None,
        }
    }
    
    /// Returns true if the current node has a next sibling
    fn has_next_sibling(&self) -> bool{
        match self {
            VBTreeLocation(_, ContextVB::Top) => false,
            VBTreeLocation(_, ContextVB::ContextUnaryVB{..}) => false,
            VBTreeLocation(_, ContextVB::ContextBinaryVBLhs{..}) => true,
            VBTreeLocation(_, ContextVB::ContextBinaryVBRhs{..}) => false,
            VBTreeLocation(_, ContextVB::ContextSelectSelector{..}) => false,
            VBTreeLocation(_, ContextVB::ContextSelectLhs{..}) => true,
            VBTreeLocation(_, ContextVB::ContextSelectRhs{..}) => true,
        }
    }
    
}

impl VB{
    
    /// Returns the size of thr value represented by the VB
    ///
    /// # Arguments
    /// * `locals_map` - the local variables map used during translation to represent the physical location of the local variables
    /// * `global_variable_map` - the global variables map used during translation to represent the physical location of the global variables
    /// 
    /// # Returns
    /// The size of the value represented by the VB
    pub fn val_size (&self, locals_map: &Vec<MapperLocation>, global_variable_map:&Vec<(u32, ValueSize)>) -> ValueSize{
        match self {
            VB::AtomicVB(AtomicVB::I32Const{..}) => ValueSize::Word,
            VB::AtomicVB(AtomicVB::I64Const{..}) => ValueSize::DoubleWord,
            VB::AtomicVB(AtomicVB::F32Const{..}) => ValueSize::Word,
            VB::AtomicVB(AtomicVB::F64Const{..}) => ValueSize::DoubleWord,
            VB::AtomicVB(AtomicVB::Local{ index}) => locals_map[*index as usize].get_size(),
            VB::AtomicVB(AtomicVB::Global{ index}) => global_variable_map[*index as usize].1,
            VB::AtomicVB(AtomicVB::Resolved{size, ..}) => *size,
            VB::AtomicVB(AtomicVB::Unreachable) => ValueSize::Word,
            VB::AtomicVB(AtomicVB::MemorySize) => ValueSize::Word,
            VB::UnaryVB{vb, ..} => match vb {
                    UnaryVB::I32EqZ => ValueSize::Word,
                    UnaryVB::I64EqZ => ValueSize::Word,
                    UnaryVB::I32Clz => ValueSize::Word,
                    UnaryVB::I32Ctz => ValueSize::Word,
                    UnaryVB::I32PopCnt => ValueSize::Word,
                    UnaryVB::I64Clz => ValueSize::DoubleWord,
                    UnaryVB::I64Ctz => ValueSize::DoubleWord,
                    UnaryVB::I64PopCnt => ValueSize::DoubleWord,
                    UnaryVB::F32Abs => ValueSize::Word,
                    UnaryVB::F32Neg => ValueSize::Word,
                    UnaryVB::F32Sqrt => ValueSize::Word,
                    UnaryVB::F32Ceil => ValueSize::Word,
                    UnaryVB::F32Floor => ValueSize::Word,
                    UnaryVB::F32Trunc => ValueSize::Word,
                    UnaryVB::F32Nearest => ValueSize::Word,
                    UnaryVB::F64Abs => ValueSize::DoubleWord,
                    UnaryVB::F64Neg => ValueSize::DoubleWord,
                    UnaryVB::F64Sqrt => ValueSize::DoubleWord,
                    UnaryVB::F64Ceil => ValueSize::DoubleWord,
                    UnaryVB::F64Floor => ValueSize::DoubleWord,
                    UnaryVB::F64Trunc => ValueSize::DoubleWord,
                    UnaryVB::F64Nearest => ValueSize::DoubleWord,
                    UnaryVB::I32WrapI64 => ValueSize::Word,
                    UnaryVB::I32TruncF32S => ValueSize::Word,
                    UnaryVB::I32TruncF32U => ValueSize::Word,
                    UnaryVB::I32TruncF64S => ValueSize::Word,
                    UnaryVB::I32TruncF64U => ValueSize::Word,
                    UnaryVB::I64ExtendI32S => ValueSize::DoubleWord,
                    UnaryVB::I64ExtendI32U => ValueSize::DoubleWord,
                    UnaryVB::I64TruncF32S => ValueSize::DoubleWord,
                    UnaryVB::I64TruncF32U => ValueSize::DoubleWord,
                    UnaryVB::I64TruncF64S => ValueSize::DoubleWord,
                    UnaryVB::I64TruncF64U => ValueSize::DoubleWord,
                    UnaryVB::F32ConvertI32S => ValueSize::Word,
                    UnaryVB::F32ConvertI32U => ValueSize::Word,
                    UnaryVB::F32ConvertI64S => ValueSize::Word,
                    UnaryVB::F32ConvertI64U => ValueSize::Word,
                    UnaryVB::F32DemoteF64 => ValueSize::Word,
                    UnaryVB::F64ConvertI32S => ValueSize::DoubleWord,
                    UnaryVB::F64ConvertI32U => ValueSize::DoubleWord,
                    UnaryVB::F64ConvertI64S => ValueSize::DoubleWord,
                    UnaryVB::F64ConvertI64U => ValueSize::DoubleWord,
                    UnaryVB::F64PromoteF32 => ValueSize::DoubleWord,
                    UnaryVB::I32Load { ..} => ValueSize::Word,
                    UnaryVB::I64Load { ..} => ValueSize::DoubleWord,
                    UnaryVB::F32Load { ..} => ValueSize::Word,
                    UnaryVB::F64Load { ..} => ValueSize::DoubleWord,
                    UnaryVB::I32Load8s { ..} => ValueSize::Word,
                    UnaryVB::I32Load8u { ..} => ValueSize::Word,
                    UnaryVB::I32Load16s { ..} => ValueSize::Word,
                    UnaryVB::I32Load16u { ..} => ValueSize::Word,
                    UnaryVB::I64Load8s { ..} => ValueSize::DoubleWord,
                    UnaryVB::I64Load8u { ..} => ValueSize::DoubleWord,
                    UnaryVB::I64Load16s { ..} => ValueSize::DoubleWord,
                    UnaryVB::I64Load16u { ..} => ValueSize::DoubleWord,
                    UnaryVB::I64Load32s { ..} => ValueSize::DoubleWord,
                    UnaryVB::I64Load32u { ..} => ValueSize::DoubleWord,
                },
                VB::BinaryVB {vb, ..} => match vb {
                    BinaryVB::I32Eq => ValueSize::Word,
                    BinaryVB::I32Ne => ValueSize::Word,
                    BinaryVB::I32LtS => ValueSize::Word,
                    BinaryVB::I32LtU => ValueSize::Word,
                    BinaryVB::I32GtS => ValueSize::Word,
                    BinaryVB::I32GtU => ValueSize::Word,
                    BinaryVB::I32GeS => ValueSize::Word,
                    BinaryVB::I32GeU => ValueSize::Word,
                    BinaryVB::I32LeS => ValueSize::Word,
                    BinaryVB::I32LeU => ValueSize::Word,
                    BinaryVB::I64Eq => ValueSize::DoubleWord,
                    BinaryVB::I64Ne => ValueSize::DoubleWord,
                    BinaryVB::I64LtS => ValueSize::DoubleWord,
                    BinaryVB::I64LtU => ValueSize::DoubleWord,
                    BinaryVB::I64GtS => ValueSize::DoubleWord,
                    BinaryVB::I64GtU => ValueSize::DoubleWord,
                    BinaryVB::I64GeS => ValueSize::DoubleWord,
                    BinaryVB::I64GeU => ValueSize::DoubleWord,
                    BinaryVB::I64LeS => ValueSize::DoubleWord,
                    BinaryVB::I64LeU => ValueSize::DoubleWord,
                    BinaryVB::F32Eq => ValueSize::Word,
                    BinaryVB::F32Ne => ValueSize::Word,
                    BinaryVB::F32Lt => ValueSize::Word,
                    BinaryVB::F32Ge => ValueSize::Word,
                    BinaryVB::F32Gt => ValueSize::Word,
                    BinaryVB::F32Le => ValueSize::Word,
                    BinaryVB::F64Eq => ValueSize::DoubleWord,
                    BinaryVB::F64Ne => ValueSize::DoubleWord,
                    BinaryVB::F64Lt => ValueSize::DoubleWord,
                    BinaryVB::F64Ge => ValueSize::DoubleWord,
                    BinaryVB::F64Gt => ValueSize::DoubleWord,
                    BinaryVB::F64Le => ValueSize::DoubleWord,
                    BinaryVB::I32Add => ValueSize::Word,
                    BinaryVB::I32Sub => ValueSize::Word,
                    BinaryVB::I32Mul => ValueSize::Word,
                    BinaryVB::I32DivS => ValueSize::Word,
                    BinaryVB::I32DivU => ValueSize::Word,
                    BinaryVB::I32RemS => ValueSize::Word,
                    BinaryVB::I32RemU => ValueSize::Word,
                    BinaryVB::I32And => ValueSize::Word,
                    BinaryVB::I32Or => ValueSize::Word,
                    BinaryVB::I32Xor => ValueSize::Word,
                    BinaryVB::I32Shl => ValueSize::Word,
                    BinaryVB::I32ShrS => ValueSize::Word,
                    BinaryVB::I32ShrU => ValueSize::Word,
                    BinaryVB::I32Rotl => ValueSize::Word,
                    BinaryVB::I32Rotr => ValueSize::Word,
                    BinaryVB::I64Add => ValueSize::DoubleWord,
                    BinaryVB::I64Sub => ValueSize::DoubleWord,
                    BinaryVB::I64Mul => ValueSize::DoubleWord,
                    BinaryVB::I64DivS => ValueSize::DoubleWord,
                    BinaryVB::I64DivU => ValueSize::DoubleWord,
                    BinaryVB::I64RemS => ValueSize::DoubleWord,
                    BinaryVB::I64RemU => ValueSize::DoubleWord,
                    BinaryVB::I64And => ValueSize::DoubleWord,
                    BinaryVB::I64Or => ValueSize::DoubleWord,
                    BinaryVB::I64Xor => ValueSize::DoubleWord,
                    BinaryVB::I64Shl => ValueSize::DoubleWord,
                    BinaryVB::I64ShrS => ValueSize::DoubleWord,
                    BinaryVB::I64ShrU => ValueSize::DoubleWord,
                    BinaryVB::I64Rotl => ValueSize::DoubleWord,
                    BinaryVB::I64Rotr => ValueSize::DoubleWord,
                    BinaryVB::F32Add => ValueSize::Word,
                    BinaryVB::F32Sub => ValueSize::Word,
                    BinaryVB::F32Mul => ValueSize::Word,
                    BinaryVB::F32Div => ValueSize::Word,
                    BinaryVB::F32Min => ValueSize::Word,
                    BinaryVB::F32Max => ValueSize::Word,
                    BinaryVB::F32CopySign => ValueSize::Word,
                    BinaryVB::F64Add => ValueSize::DoubleWord,
                    BinaryVB::F64Sub => ValueSize::DoubleWord,
                    BinaryVB::F64Mul => ValueSize::DoubleWord,
                    BinaryVB::F64Div => ValueSize::DoubleWord,
                    BinaryVB::F64Min => ValueSize::DoubleWord,
                    BinaryVB::F64Max => ValueSize::DoubleWord,
                    BinaryVB::F64CopySign => ValueSize::DoubleWord,
                },
                VB::Select{size, ..} => *size
            }
    }
    
    /// Returns true if the VB depends on a local variable
    /// 
    /// # Arguments
    /// * `local_index` - the index of the local variable
    pub fn depends_on_local(&self, local_index: u32) -> bool{
        match self {
            VB::AtomicVB(AtomicVB::Local{index}) => *index == local_index,
            VB::AtomicVB(..) => false,
            VB::UnaryVB{child, ..} => child.depends_on_local(local_index),
            VB::BinaryVB{lhs, rhs, ..} => lhs.depends_on_local(local_index) || rhs.depends_on_local(local_index),
            VB::Select{selector, lhs, rhs, ..} => selector.depends_on_local(local_index) || lhs.depends_on_local(local_index) || rhs.depends_on_local(local_index),
        }
    }
    
    /// Returns true if the VB depends on a memory location
    
    pub fn depends_on_memory(&self) -> bool{
        match self {
            VB::AtomicVB(..) => false,
            VB::UnaryVB{child, vb} => match vb {
                UnaryVB::I32Load {..} | UnaryVB::I64Load {..} | UnaryVB::F32Load {..} | UnaryVB::F64Load {..} | UnaryVB::I32Load8s {..} | UnaryVB::I32Load8u {..} | UnaryVB::I32Load16s {..} | UnaryVB::I32Load16u {..} | UnaryVB::I64Load8s {..} | UnaryVB::I64Load8u {..} | UnaryVB::I64Load16s {..} | UnaryVB::I64Load16u {..} | UnaryVB::I64Load32s {..} | UnaryVB::I64Load32u {..} => true,
                _ => child.depends_on_memory()
            }
            VB::BinaryVB{lhs, rhs, ..} => lhs.depends_on_memory() || rhs.depends_on_memory(),
            VB::Select{selector, lhs, rhs, ..} => selector.depends_on_memory() || lhs.depends_on_memory() || rhs.depends_on_memory(),
        }
    }

    /// Returns the offset of the highest value on the runtime stack that the VB depends on. This function is useful e.g. to determine where the stack pointer should be placed for stack unwinding 
    pub fn get_runtime_stack_offset(&self) -> Option<usize>{
        match self {
            VB::AtomicVB(AtomicVB::Resolved {offset, ..}) => Some(*offset),
            VB::AtomicVB(..) => None,
            VB::UnaryVB{child, ..} => child.get_runtime_stack_offset(),
            VB::BinaryVB{lhs, rhs, ..} => rhs.get_runtime_stack_offset().or_else(|| lhs.get_runtime_stack_offset()),
            VB::Select {selector, lhs , rhs, ..} => selector.get_runtime_stack_offset().or_else(|| rhs.get_runtime_stack_offset().or_else(|| lhs.get_runtime_stack_offset()))
        }
    }

    /// Performs a post-order depth-first search on the VB tree. This is used to resolve the VB tree into a sequence of instructions.
    pub fn post_order_dfs(self, mut f: impl FnMut(&VB, bool) ){
        let mut tree_location = VBTreeLocation(self, ContextVB::Top).move_first_leaf();
        loop {
            f(&tree_location.0, tree_location.1 == ContextVB::Top);
            if tree_location.has_next_sibling(){
                tree_location = tree_location.next_sibling().unwrap().move_first_leaf();
            } else {
                    if let Some(parent) = tree_location.move_up() {
                        tree_location = parent;
                    } else {
                        return;
                    }
            }
        }
       
    }

    /// Returns true if the top operation of the VB tree may produce a non-canonical NaN from non-NaN operands. This is used to adjust the VB tree to avoid producing non-canonical NaNs
    pub fn produces_non_canonical_nan(&self) -> bool {
        match self {
            VB::BinaryVB { vb, rhs,.. } => match vb {
                BinaryVB::F32Add => match **rhs {
                    VB::AtomicVB(AtomicVB::F32Const{imm}) => f32::from_bits(imm) != 0.0,
                    _ => true

                }
                BinaryVB::F32Mul| BinaryVB::F32Sub| BinaryVB::F32Div  => true,
                _ => false
            }
            VB::UnaryVB { vb,.. } => match vb {
                UnaryVB::F32Sqrt => true,
                _ => false
            }
            _ => false
        }
    }

    /// Adjusts the VB tree to avoid producing non-canonical NaNs by adding an addition of -0.0 to the top operation
    pub fn adjust_for_snan(self) -> Self {
        VB::BinaryVB { vb: BinaryVB::F32Add, lhs: Box::new(self) , rhs: Box::new(VB::AtomicVB(AtomicVB::F32Const { imm: (-0.0_f32).to_bits() })) }
    }

    /// Returns true if the VB has a child
    pub fn has_child(&self) -> bool {
        match self {
            VB::AtomicVB(..) => false,
            _ => true
        }
    }
}
