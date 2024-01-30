
use std::cell::Cell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::ast::{ValueType, NumberType, VectorType, ReferenceType, Type};

pub struct Reader {
    data: Vec<u8>,
    pos: Cell<usize>,
}

impl Reader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            pos: Cell::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dword(&self) -> u32 {
        let prev = self.pos.replace(self.pos.get() + 4);
        u32::from_le_bytes(self.data[prev..self.pos.get()].try_into().unwrap())
    }

    pub fn bytes(&self, num: usize) -> &[u8] {
        let prev = self.pos.replace(self.pos.get() + num);
        &self.data[prev..self.pos.get()]
    }

    pub fn byte(&self) -> u8 {
        let prev = self.pos.replace(self.pos.get() + 1);
        self.data[prev]
    }
}

fn check_header(wasm: &Reader) -> Result<(), RuntimeError> {
    if wasm.len() < 8 {
        return Err(RuntimeError::InvalidModuleLength);
    }

    if wasm.bytes(4) != *b"\0asm" {
        return Err(RuntimeError::InvalidMagicNumber);
    }

    if wasm.dword() != 1 {
        return Err(RuntimeError::InvalidVersionNumber);
    }
    Ok(())
}

fn parse_section(wasm: &Reader) -> Result<(), RuntimeError> {
    let section_code = wasm.byte();
    let size = wasm.dword();
    let prev = wasm.pos.replace(wasm.pos.get() + size as usize);
    match section_code {
        section::TYPE => parse_type_section(wasm),
        section::IMPORT => parse_import_section(wasm),
        section::FUNCTION => parse_function_section(wasm),
        section::TABLE => parse_table_section(wasm),
        section::MEMORY => parse_memory_section(wasm),
        section::GLOBAL => parse_global_section(wasm),
        section::EXPORT => parse_export_section(wasm),
        section::START => parse_start_section(wasm),
        section::ELEMENT => parse_element_section(wasm),
        section::CODE => parse_code_section(wasm),
        section::DATA => parse_data_section(wasm),
        _ => Err(RuntimeError::InvalidSectionCode),
    }?;
    wasm.pos.replace(prev);
    Ok(())
}

fn parse_type_section(wasm: &Reader) -> Result<Vec<Type>, RuntimeError> {
    if wasm.byte() != section::TYPE {
        return Err(RuntimeError::InvalidSectionCode);
    }

    let size = wasm.byte();
    let num_types = wasm.byte();
    let mut types = vec![];


}

fn parse_valuetype(wasm: &Reader) -> Result<ValueType, RuntimeError> {
    match wasm.byte() {
        0x7F => Ok(ValueType::NumberType(NumberType::I32)),
        0x7E => Ok(ValueType::NumberType(NumberType::I64)),
        0x7D => Ok(ValueType::NumberType(NumberType::F32)),
        0x7C => Ok(ValueType::NumberType(NumberType::F64)),
        0x7B => Ok(ValueType::VectorType(VectorType::V128)),
        0x70 => Ok(ValueType::ReferenceType(ReferenceType::FuncRef)),
        0x6F => Ok(ValueType::ReferenceType(ReferenceType::ExternRef)),
        _ => Err(RuntimeError::InvlaidValueType),
    }
}

pub enum RuntimeError {
    InvalidModuleLength,
    InvalidMagicNumber,
    InvalidVersionNumber,
    InvalidSectionCode,
    InvlaidValueType,
    InvalidExportType,
    InvalidExportName,
    InvalidInstruction,
    ExportNotFound,
    InvalidArgNumber,
}

impl RuntimeError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidModuleLength => "Invalid module length",
            Self::InvalidMagicNumber => "Invalid magic number",
            Self::InvalidVersionNumber => "Invalid version number",
            Self::InvalidSectionCode => "Invalid section code",
            Self::InvlaidValueType => "Invalid value type",
            Self::InvalidExportType => "Invalid export type",
            Self::InvalidExportName => "Invalid export name",
            Self::InvalidInstruction => "Invalid instruction",
            Self::ExportNotFound => "Export not found",
            Self::InvalidArgNumber => "Invalid argument number",
        }
    }
}

impl Error for RuntimeError {}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}