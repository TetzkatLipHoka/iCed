/*
    Iced (Dis)Assembler
    C-Compatible Exports
  
    TetzkatLipHoka 2022-2024
*/

#![allow( non_snake_case )]
#![allow( dead_code )] // TFormatterType
extern crate libc;

mod FreeMemory;
mod Mnemonic;
mod Code;
mod Register;
mod OpKind;
mod RoundingControl;
mod MemorySize;
mod CodeSize;
mod Instruction;

mod OpCodeInfo;
mod MandatoryPrefix;
mod OpCodeOperandKind;
mod OpCodeTableKind;

mod CPUIdFeature;
mod ConditionCode;
mod FlowControl;
mod OpAccess;
mod InstructionInfoFactory;

mod MvexEHBit;
mod MvexTupleTypeLutKind;
mod MvexConvFn;
mod MvexRegMemConv;

mod RepPrefixKind;
mod InstructionWith;

mod EncodingKind;
mod TupleType;
mod DecoderError;
mod Decoder;

mod Encoder;

mod BlockEncoder;

mod FormatterTextKind;
mod NumberBase;
mod MemorySizeOptions;

mod SymbolResolver;
mod OptionsProvider;
mod OutputCallback;
mod VirtualAddressResolver;

mod Formatter;
mod MasmFormatter;
mod NasmFormatter;
mod GasFormatter;
mod IntelFormatter;
mod FastFormatter;
// ~100KB more
mod SpecializedFormatterTraitOptions;
mod SpecializedFormatter;