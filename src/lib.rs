/*
    Iced (Dis)Assembler
    C-Compatible Exports
  
    TetzkatLipHoka 2022
*/

#![allow( non_snake_case )]
extern crate libc;

use iced_x86::*;
use std::ptr::eq;
use std::{slice, str, ptr::null_mut};
use libc::{c_char, strlen};
use std::ffi::CString;
use std::mem::transmute;// Enum<->Int

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Free Memory
#[no_mangle]
pub unsafe extern "C" fn IcedFreeMemory( Pointer: *mut Decoder ) -> bool { 
    if Pointer.is_null() {
        return false;
    }

    Box::from_raw( Pointer );
    return true;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Decoder

// Creates a decoder
//
// # Errors
// Fails if `bitness` is not one of 16, 32, 64.
//
// # Arguments
// * `bitness`: 16, 32 or 64
// * `data`: Data to decode
// * `data`: ByteSize of `Data`
// * `options`: Decoder options, `0` or eg. `DecoderOptions::NO_INVALID_CHECK | DecoderOptions::AMD`
#[no_mangle]
pub extern "C" fn Decoder_Create( Bitness: u32, Data: *const u8, DataSize : usize, IP: u64, Options: u32 ) -> *mut Decoder<'static> {
    let data2 = unsafe { slice::from_raw_parts( Data, DataSize ) };
    match Decoder::try_with_ip( Bitness, data2, IP, Options ) {
        Ok( value ) => return Box::into_raw( Box::new( value ) ),
        Err( _e ) => return null_mut()
    }
}

// Returns `true` if there's at least one more byte to decode. It doesn't verify that the
// next instruction is valid, it only checks if there's at least one more byte to read.
// See also [ `position()` ] and [ `max_position()` ]
//
// It's not required to call this method. If this method returns `false`, then [ `decode_out()` ]
// and [ `decode()` ] will return an instruction whose [ `code()` ] == [ `Code::INVALID` ].
#[no_mangle]
pub unsafe extern "C" fn Decoder_CanDecode( Decoder: *mut Decoder ) -> bool {
    if Decoder.is_null() {
        return false;
    }
    let obj = Box::from_raw( Decoder );

    let value = obj.can_decode();

    Box::into_raw( obj );

    return value;
}

// Gets the current `IP`/`EIP`/`RIP` value, see also [ `position()` ]
#[no_mangle]
pub unsafe extern "C" fn Decoder_GetIP( Decoder: *mut Decoder ) -> u64 {
    if Decoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Decoder );

    let value = obj.ip();

    Box::into_raw( obj );

    return value;
}

// Sets the current `IP`/`EIP`/`RIP` value, see also [ `try_set_position()` ]
// This method only updates the IP value, it does not change the data position, use [ `try_set_position()` ] to change the position.
#[no_mangle]
pub unsafe extern "C" fn Decoder_SetIP( Decoder: *mut Decoder, Value : u64 ) -> bool {
    if Decoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Decoder );

    obj.set_ip( Value );

    Box::into_raw( obj );

    return true;
}

// Gets the bitness ( 16, 32 or 64 )
#[no_mangle]
pub unsafe extern "C" fn Decoder_GetBitness( Decoder: *mut Decoder ) -> u32 {
    if Decoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Decoder );

    let value = obj.bitness();

    Box::into_raw( obj );

    return value;
}

// Gets the max value that can be passed to [ `try_set_position()` ]. This is the size of the data that gets
// decoded to instructions and it's the length of the slice that was passed to the constructor.
#[no_mangle]
pub unsafe extern "C" fn Decoder_GetMaxPosition( Decoder: *mut Decoder ) -> usize {
    if Decoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Decoder );

    let value = obj.max_position();

    Box::into_raw( obj );
 
    return value;
}

// Gets the current data position. This value is always <= [ `max_position()` ].
// When [ `position()` ] == [ `max_position()` ], it's not possible to decode more
// instructions and [ `can_decode()` ] returns `false`.
#[no_mangle]
pub unsafe extern "C" fn Decoder_GetPosition( Decoder: *mut Decoder ) -> usize {
    if Decoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Decoder );

    let value = obj.position();

    Box::into_raw( obj );
 
    return value;
}

// Sets the current data position, which is the index into the data passed to the constructor.
// This value is always <= [ `max_position()` ]
#[no_mangle]
pub unsafe extern "C" fn Decoder_SetPosition( Decoder: *mut Decoder, Value : usize ) -> bool {
    if Decoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Decoder );

    let value = obj.set_position( Value );

    Box::into_raw( obj );

    return value.is_ok();
}

// Gets the last decoder error. Unless you need to know the reason it failed,
// it's better to check [ `instruction.is_invalid()` ].
#[no_mangle]
pub unsafe extern "C" fn Decoder_GetLastError( Decoder: *mut Decoder ) -> u32 { // FFI-Unsafe: TDecoderError
    if Decoder.is_null() {
        return 0;// TDecoderError::None;
    }
    let obj = Box::from_raw( Decoder );

    let value: u32/*TDecoderError*/ = transmute( obj.last_error() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Decodes and returns the next instruction, see also [ `decode_out( &mut Instruction )` ]
// which avoids copying the decoded instruction to the caller's return variable.
// See also [ `last_error()` ].
#[no_mangle]
pub unsafe extern "C" fn Decoder_Decode( Decoder: *mut Decoder, Instruction: *mut Instruction ) {
    if Decoder.is_null() {
        return;
    }
    let mut obj = Box::from_raw( Decoder );

    obj.decode_out( Instruction.as_mut().unwrap() );

    Box::into_raw( obj );
}

// Gets the offsets of the constants ( memory displacement and immediate ) in the decoded instruction.
// The caller can check if there are any relocations at those addresses.
//
// # Arguments
// * `instruction`: The latest instruction that was decoded by this decoder
#[no_mangle]
pub unsafe extern "C" fn Decoder_GetConstantOffsets( Decoder: *mut Decoder, Instruction: *mut Instruction, ConstantOffsets : *mut ConstantOffsets ) -> bool {
    if Decoder.is_null() {
        return false;
    }
    let obj = Box::from_raw( Decoder );
    *ConstantOffsets = obj.get_constant_offsets( Instruction.as_mut().unwrap() );

    Box::into_raw( obj );
    return true;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Callbacks

// Tries to resolve a symbol
//
// # Arguments
// - `instruction`: Instruction
// - `operand`: Operand number, 0-based. This is a formatter operand and isn't necessarily the same as an instruction operand.
// - `instruction_operand`: Instruction operand number, 0-based, or `None` if it's an operand created by the formatter.
// - `address`: Address
// - `address_size`: Size of `address` in bytes ( eg. 1, 2, 4 or 8 )
type
  TSymbolResolverCallback = unsafe extern "C" fn( Instruction: *const Instruction, Operand: u32, InstructionOperand: u32, Address: u64, Size: u32, UserData : *const usize ) -> *const c_char;

  struct TSymbolResolver {
    //map: HashMap<u64, String>,
    userData: *const usize,
    callback: Option<TSymbolResolverCallback>
}

impl SymbolResolver for TSymbolResolver {
    fn symbol( &mut self, Instruction: &Instruction, Operand: u32, InstructionOperand: Option<u32>, Address: u64, Size: u32 ) -> Option<SymbolResult> {
        unsafe {
            if !self.callback.is_none() {   
                //let value = self.callback.unwrap()( &mut u as *mut u32, &mut i as *mut i32 );// Var-Parameter-Sample

                let instructionoperand: u32;
                match InstructionOperand {
                    None => instructionoperand = 0xFFFF_FFFF,
                    _Some => instructionoperand = InstructionOperand.unwrap()
                }

                let value = self.callback.unwrap()( Instruction, Operand, instructionoperand, Address, Size, self.userData );
                let symbol_string = str::from_utf8_unchecked( slice::from_raw_parts( value as *const u8, strlen( value ) ) );
                if !symbol_string.is_empty() {
                    // The 'Address' arg is the address of the symbol and doesn't have to be identical
                    // to the 'address' arg passed to symbol(). If it's different from the input
                    // address, the formatter will add +N or -N, eg. '[ rax+symbol+123 ]'
                    return Some( SymbolResult::with_str( Address, symbol_string ) )
                }else {
                    return None
                    /*
                    if let Some( symbol_string ) = self.map.get( &addr ) {
                        // The 'address' arg is the address of the symbol and doesn't have to be identical
                        // to the 'address' arg passed to symbol(). If it's different from the input
                        // address, the formatter will add +N or -N, eg. '[ rax+symbol+123 ]'
                        return Some( SymbolResult::with_str( addr, symbol_string.as_str() ) )
                        //Some( SymbolResult::with_str( Address, symbol_string.as_str() ) )
                    }else {
                        return None
                    }
                    */                    
              }
            }else {
                return None
                /*
                if let Some( symbol_string ) = self.map.get( &address ) {
                    // The 'address' arg is the address of the symbol and doesn't have to be identical
                    // to the 'address' arg passed to symbol(). If it's different from the input
                    // address, the formatter will add +N or -N, eg. '[ rax+symbol+123 ]'
                    return Some( SymbolResult::with_str( addr, symbol_string.as_str() ) )
                    //return Some( SymbolResult::with_str( Address, symbol_string.as_str() ) )
                }else {
                    return None
                }
                */
            }
        };
    }
}

// Called by the formatter. The method can override any options before the formatter uses them.
//
// # Arguments
// - `instruction`: Instruction
// - `operand`: Operand number, 0-based. This is a formatter operand and isn't necessarily the same as an instruction operand.
// - `instruction_operand`: Instruction operand number, 0-based, or `None` if it's an operand created by the formatter.
// - `options`: Options. Only those options that will be used by the formatter are initialized.
// - `number_options`: Number formatting options
type
  TFormatterOptionsProviderCallback = unsafe extern "C" fn( Instruction: &Instruction, Operand: u32, InstructionOperand: u32, Options: &mut FormatterOperandOptions, NumberOptions: &mut TNumberFormattingOptions, UserData : *const usize );

  struct TFormatterOptionsProvider {
    userData: *const usize,
    callback: Option<TFormatterOptionsProviderCallback>
}

#[repr(C)]
pub struct TNumberFormattingOptions {
	/// Number prefix or an empty string
	pub prefix: *const c_char,
	/// Number suffix or an empty string
	pub suffix: *const c_char,
	/// Digit separator or an empty string to not use a digit separator
	pub digit_separator: *const c_char,
	/// Size of a digit group or 0 to not use a digit separator
	pub digit_group_size: u8,
	/// Number base
	pub number_base: NumberBase,
	/// Use uppercase hex digits
	pub uppercase_hex: bool,
	/// Small hex numbers ( -9 .. 9 ) are shown in decimal
	pub small_hex_numbers_in_decimal: bool,
	/// Add a leading zero to hex numbers if there's no prefix and the number starts with hex digits `A-F`
	pub add_leading_zero_to_hex_numbers: bool,
	/// If `true`, add leading zeros to numbers, eg. `1h` vs `00000001h`
	pub leading_zeros: bool,
	/// If `true`, the number is signed, and if `false` it's an unsigned number
	pub signed_number: bool,
	/// Add leading zeros to displacements
	pub displacement_leading_zeros: bool,
}

impl FormatterOptionsProvider for TFormatterOptionsProvider {
    fn operand_options( &mut self, Instruction: &Instruction, Operand: u32, InstructionOperand: Option<u32>, Options: &mut FormatterOperandOptions, NumberOptions: &mut NumberFormattingOptions<'_> ) {
        unsafe {
            if !self.callback.is_none() {   
                let tprefix = CString::new( NumberOptions.prefix ).unwrap();
                let tsuffix = CString::new( NumberOptions.suffix ).unwrap();
                let tdigit_separator = CString::new( NumberOptions.digit_separator ).unwrap();

                let pprefix = tprefix.as_ptr() as *const c_char;
                let psuffix = tsuffix.as_ptr() as *const c_char;
                let pdigit_separator = tdigit_separator.as_ptr() as *const c_char;
              
                let mut numberoptions = TNumberFormattingOptions {                    
                    prefix: pprefix,
                    suffix: psuffix,
                    digit_separator: pdigit_separator,

                    digit_group_size : NumberOptions.digit_group_size,
                    number_base : NumberOptions.number_base,
                    uppercase_hex : NumberOptions.uppercase_hex,
                    small_hex_numbers_in_decimal : NumberOptions.small_hex_numbers_in_decimal,
                    add_leading_zero_to_hex_numbers : NumberOptions.add_leading_zero_to_hex_numbers,
                    leading_zeros : NumberOptions.leading_zeros,
                    signed_number : NumberOptions.signed_number,
                    displacement_leading_zeros : NumberOptions.displacement_leading_zeros
                };

                match InstructionOperand {
                    None => self.callback.unwrap()( Instruction, Operand, 0xFFFF_FFFF as u32, Options, &mut numberoptions, self.userData ),
                    _Some => self.callback.unwrap()( Instruction, Operand, InstructionOperand.unwrap(), Options, &mut numberoptions, self.userData )
                }

                if !eq(pprefix, numberoptions.prefix) {
                    NumberOptions.prefix = str::from_utf8_unchecked( slice::from_raw_parts( numberoptions.prefix as *const u8, strlen( numberoptions.prefix ) ) );
                }

                if !eq(psuffix, numberoptions.suffix) {
                    NumberOptions.suffix = str::from_utf8_unchecked( slice::from_raw_parts( numberoptions.suffix as *const u8, strlen( numberoptions.suffix ) ) );
                }

                if !eq(pdigit_separator, numberoptions.digit_separator) {
                    NumberOptions.digit_separator = str::from_utf8_unchecked( slice::from_raw_parts( numberoptions.digit_separator as *const u8, strlen( numberoptions.digit_separator ) ) );
                }

                NumberOptions.digit_group_size = numberoptions.digit_group_size;
                NumberOptions.number_base = numberoptions.number_base;
                NumberOptions.uppercase_hex = numberoptions.uppercase_hex;
                NumberOptions.small_hex_numbers_in_decimal = numberoptions.small_hex_numbers_in_decimal;
                NumberOptions.add_leading_zero_to_hex_numbers = numberoptions.add_leading_zero_to_hex_numbers;
                NumberOptions.leading_zeros = numberoptions.leading_zeros;
                NumberOptions.signed_number = numberoptions.signed_number;
                NumberOptions.displacement_leading_zeros = numberoptions.displacement_leading_zeros;
            }
        };
    }
}

// Formatter Output Callback
// Used by a [`Formatter`] to write all text. `String` also implements this trait.
//
// The only method that must be implemented is [`write()`], all other methods call it if they're not overridden.
type
  TFormatterOutputCallback = unsafe extern "C" fn( Text: *const c_char, Kind: u8 /*FormatterTextKind*/, UserData : *const usize );

pub struct TFormatterOutput {
    userData : *const usize,
    callback: Option<TFormatterOutputCallback>    
}

impl FormatterOutput for TFormatterOutput {
    fn write(&mut self, text: &str, kind: FormatterTextKind) {
        let value = CString::new( text ).unwrap();
        unsafe {
            self.callback.unwrap()( value.to_bytes().as_ptr() as *const i8, kind as u8, self.userData );
        }
    }
}

// Creates a formatter Output Callback
#[no_mangle]
pub extern "C" fn FormatterOutput_Create( Callback : Option<TFormatterOutputCallback>, UserData : *const usize ) -> *mut TFormatterOutput {   
    if Callback.is_none() {
        return null_mut();
    }

    Box::into_raw( Box::new( TFormatterOutput { callback:Callback, userData:UserData }) )
}
    
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Masm-Formatter

// Creates a masm formatter
//
// # Arguments
// - `symbol_resolver`: Symbol resolver or `None`
// - `options_provider`: Operand options provider or `None`
#[no_mangle]
pub extern "C" fn MasmFormatter_Create( SymbolResolver : Option<TSymbolResolverCallback>, OptionsProvider : Option<TFormatterOptionsProviderCallback>, UserData : *const usize ) -> *mut MasmFormatter {   
    if !SymbolResolver.is_none() && !OptionsProvider.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( MasmFormatter::with_options( Some( symbols ), Some( options ) ) ) )
    }else if !SymbolResolver.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        Box::into_raw( Box::new( MasmFormatter::with_options( Some( symbols ), None ) ) )                
    }else if !OptionsProvider.is_none() {
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( MasmFormatter::with_options( None, Some( options ) ) ) )
    }else {
        Box::into_raw( Box::new( MasmFormatter::with_options( None, None ) ) )
    }
}

// Format Instruction
#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_Format( MasmFormatter: *mut MasmFormatter, Instruction: *mut Instruction, Output : *mut u8, Size : usize ) {     
    if MasmFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if Output.is_null() {
        return;
    }
    if Size <= 0 {
        return;
    }

    let mut obj = Box::from_raw( MasmFormatter );
    let mut output = String::new();
    obj.format( Instruction.as_mut().unwrap(), &mut output );
    Box::into_raw( obj );

    let mut l = output.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Output.add( i ) ) = output.as_bytes()[ i ];
        
        }
    }
    *( Output.add( l ) ) = 0;
}

#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_FormatCallback( MasmFormatter: *mut MasmFormatter, Instruction: *mut Instruction, FormatterOutput : *mut TFormatterOutput ) {     
    if MasmFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if FormatterOutput.is_null() {
        return;
    }

    let mut obj = Box::from_raw( MasmFormatter );
    let mut output = Box::from_raw( FormatterOutput );
    obj.format( Instruction.as_mut().unwrap(), output.as_mut() );
    Box::into_raw( output );
    Box::into_raw( obj );
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Nasm-Formatter

// Creates a Nasm formatter
//
// # Arguments
// - `symbol_resolver`: Symbol resolver or `None`
// - `options_provider`: Operand options provider or `None`
#[no_mangle]
pub extern "C" fn NasmFormatter_Create( SymbolResolver : Option<TSymbolResolverCallback>, OptionsProvider : Option<TFormatterOptionsProviderCallback>, UserData : *const usize ) -> *mut NasmFormatter {   
    if !SymbolResolver.is_none() && !OptionsProvider.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( NasmFormatter::with_options( Some( symbols ), Some( options ) ) ) )        
    }else if !SymbolResolver.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        Box::into_raw( Box::new( NasmFormatter::with_options( Some( symbols ), None ) ) )                
    }else if !OptionsProvider.is_none() {
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( NasmFormatter::with_options( None, Some( options ) ) ) )
    }else {
        Box::into_raw( Box::new( NasmFormatter::with_options( None, None ) ) )
    }
}

// Format Instruction
#[no_mangle]
pub unsafe extern "C" fn NasmFormatter_Format( NasmFormatter: *mut NasmFormatter, Instruction: *mut Instruction, Output : *mut u8, Size : usize ) {     
    if NasmFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if Output.is_null() {
        return;
    }
    if Size <= 0 {
        return;
    }

    let mut obj = Box::from_raw( NasmFormatter );
    let mut output = String::new();
    obj.format( Instruction.as_mut().unwrap(), &mut output );
    Box::into_raw( obj );

    let mut l = output.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Output.add( i ) ) = output.as_bytes()[ i ];
        
        }
    }
    *( Output.add( l ) ) = 0;
}

#[no_mangle]
pub unsafe extern "C" fn NasmFormatter_FormatCallback( NasmFormatter: *mut NasmFormatter, Instruction: *mut Instruction, FormatterOutput : *mut TFormatterOutput ) {     
    if NasmFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if FormatterOutput.is_null() {
        return;
    }

    let mut obj = Box::from_raw( NasmFormatter );
    let mut output = Box::from_raw( FormatterOutput );
    obj.format( Instruction.as_mut().unwrap(), output.as_mut() );
    Box::into_raw( output );
    Box::into_raw( obj );
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Gas-Formatter

// Creates a Gas formatter
//
// # Arguments
// - `symbol_resolver`: Symbol resolver or `None`
// - `options_provider`: Operand options provider or `None`
#[no_mangle]
pub extern "C" fn GasFormatter_Create( SymbolResolver : Option<TSymbolResolverCallback>, OptionsProvider : Option<TFormatterOptionsProviderCallback>, UserData : *const usize ) -> *mut GasFormatter {   
    if !SymbolResolver.is_none() && !OptionsProvider.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( GasFormatter::with_options( Some( symbols ), Some( options ) ) ) )        
    }else if !SymbolResolver.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        Box::into_raw( Box::new( GasFormatter::with_options( Some( symbols ), None ) ) )                
    }else if !OptionsProvider.is_none() {
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( GasFormatter::with_options( None, Some( options ) ) ) )
    }else {
        Box::into_raw( Box::new( GasFormatter::with_options( None, None ) ) )
    }
}

// Format Instruction
#[no_mangle]
pub unsafe extern "C" fn GasFormatter_Format( GasFormatter: *mut GasFormatter, Instruction: *mut Instruction, Output : *mut u8, Size : usize ) {     
    if GasFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if Output.is_null() {
        return;
    }
    if Size <= 0 {
        return;
    }

    let mut obj = Box::from_raw( GasFormatter );
    let mut output = String::new();
    obj.format( Instruction.as_mut().unwrap(), &mut output );
    Box::into_raw( obj );

    let mut l = output.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Output.add( i ) ) = output.as_bytes()[ i ];
        
        }
    }
    *( Output.add( l ) ) = 0;
}

#[no_mangle]
pub unsafe extern "C" fn GasFormatter_FormatCallback( GasFormatter: *mut GasFormatter, Instruction: *mut Instruction, FormatterOutput : *mut TFormatterOutput ) {     
    if GasFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if FormatterOutput.is_null() {
        return;
    }

    let mut obj = Box::from_raw( GasFormatter );
    let mut output = Box::from_raw( FormatterOutput );
    obj.format( Instruction.as_mut().unwrap(), output.as_mut() );
    Box::into_raw( output );
    Box::into_raw( obj );
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Intel-Formatter

// Creates a Intel formatter
//
// # Arguments
// - `symbol_resolver`: Symbol resolver or `None`
// - `options_provider`: Operand options provider or `None`
#[no_mangle]
pub extern "C" fn IntelFormatter_Create( SymbolResolver : Option<TSymbolResolverCallback>, OptionsProvider : Option<TFormatterOptionsProviderCallback>, UserData : *const usize ) -> *mut IntelFormatter {   
    if !SymbolResolver.is_none() && !OptionsProvider.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( IntelFormatter::with_options( Some( symbols ), Some( options ) ) ) )        
    }else if !SymbolResolver.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });
        Box::into_raw( Box::new( IntelFormatter::with_options( Some( symbols ), None ) ) )                
    }else if !OptionsProvider.is_none() {
        let options = Box::new( TFormatterOptionsProvider { callback:OptionsProvider, userData:UserData });
        Box::into_raw( Box::new( IntelFormatter::with_options( None, Some( options ) ) ) )
    }else {
        Box::into_raw( Box::new( IntelFormatter::with_options( None, None ) ) )
    }
}

// Format Instruction
#[no_mangle]
pub unsafe extern "C" fn IntelFormatter_Format( IntelFormatter: *mut IntelFormatter, Instruction: *mut Instruction, Output : *mut u8, Size : usize ) {     
    if IntelFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if Output.is_null() {
        return;
    }
    if Size <= 0 {
        return;
    }

    let mut obj = Box::from_raw( IntelFormatter );
    let mut output = String::new();
    obj.format( Instruction.as_mut().unwrap(), &mut output );
    Box::into_raw( obj );

    let mut l = output.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Output.add( i ) ) = output.as_bytes()[ i ];
        
        }
    }
    *( Output.add( l ) ) = 0;
}

#[no_mangle]
pub unsafe extern "C" fn IntelFormatter_FormatCallback( IntelFormatter: *mut IntelFormatter, Instruction: *mut Instruction, FormatterOutput : *mut TFormatterOutput ) {     
    if IntelFormatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if FormatterOutput.is_null() {
        return;
    }

    let mut obj = Box::from_raw( IntelFormatter );
    let mut output = Box::from_raw( FormatterOutput );
    obj.format( Instruction.as_mut().unwrap(), output.as_mut() );
    Box::into_raw( output );
    Box::into_raw( obj );
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Fast-Formatter

type
  TFastFormatter = SpecializedFormatter<DefaultFastFormatterTraitOptions>;

// Creates a Fast formatter
#[no_mangle]
pub extern "C" fn FastFormatter_Create( SymbolResolver : Option<TSymbolResolverCallback>, UserData : *const usize ) -> *mut TFastFormatter {   
    if !SymbolResolver.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });

        match TFastFormatter::try_with_options( Some( symbols ) ) {
            Ok( value ) => return Box::into_raw( Box::new( value ) ),
            Err( _e ) => return null_mut()
        }
    }else {
        match TFastFormatter::try_with_options( None ) {
            Ok( value ) => return Box::into_raw( Box::new( value ) ),
            Err( _e ) => return null_mut()
        }
    }
}

// Format Instruction
#[no_mangle]
pub unsafe extern "C" fn FastFormatter_Format( Formatter: *mut TFastFormatter, Instruction: *mut Instruction, Output : *mut u8, Size : usize ) {     
    if Formatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if Output.is_null() {
        return;
    }
    if Size <= 0 {
        return;
    }

    let mut obj = Box::from_raw( Formatter );
    let mut output = String::new();
    obj.format( Instruction.as_mut().unwrap(), &mut output );
    Box::into_raw( obj );

    let mut l = output.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Output.add( i ) ) = output.as_bytes()[ i ];
        
        }
    }
    *( Output.add( l ) ) = 0;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Specialized-Formatter

type
  TSpecializedFormatter = SpecializedFormatter<DefaultSpecializedFormatterTraitOptions>;

// Creates a Specialized formatter
#[no_mangle]
pub extern "C" fn SpecializedFormatter_Create( SymbolResolver : Option<TSymbolResolverCallback>, UserData : *const usize ) -> *mut TSpecializedFormatter {   
    if !SymbolResolver.is_none() {
        let symbols = Box::new( TSymbolResolver { callback:SymbolResolver, userData:UserData });

        match TSpecializedFormatter::try_with_options( Some( symbols ) ) {
            Ok( value ) => return Box::into_raw( Box::new( value ) ),
            Err( _e ) => return null_mut()
        }
    }else {
        match TSpecializedFormatter::try_with_options( None ) {
            Ok( value ) => return Box::into_raw( Box::new( value ) ),
            Err( _e ) => return null_mut()
        }
    }
}

// Format Instruction
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_Format( Formatter: *mut TSpecializedFormatter, Instruction: *mut Instruction, Output : *mut u8, Size : usize ) {     
    if Formatter.is_null() {
        return;
    }
    if Instruction.is_null() {
        return;
    }
    if Output.is_null() {
        return;
    }
    if Size <= 0 {
        return;
    }

    let mut obj = Box::from_raw( Formatter );
    let mut output = String::new();
    obj.format( Instruction.as_mut().unwrap(), &mut output );
    Box::into_raw( obj );

    let mut l = output.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Output.add( i ) ) = output.as_bytes()[ i ];
        
        }
    }
    *( Output.add( l ) ) = 0;
}

// NOTE: Specialized Formatter only supports the following Options
// Options

// Always show the size of memory operands
//
// Default | Value | Example | Example
// --------|-------|---------|--------
// _ | `true` | `mov eax,dword ptr [ ebx ]` | `add byte ptr [ eax ],0x12`
// 👍 | `false` | `mov eax,[ ebx ]` | `add byte ptr [ eax ],0x12`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetAlwaysShowMemorySize( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().always_show_memory_size();

    Box::into_raw( obj );
 
    return value;
}

// Always show the size of memory operands
//
// Default | Value | Example | Example
// --------|-------|---------|--------
// _ | `true` | `mov eax,dword ptr [ ebx ]` | `add byte ptr [ eax ],0x12`
// 👍 | `false` | `mov eax,[ ebx ]` | `add byte ptr [ eax ],0x12`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetAlwaysShowMemorySize( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_always_show_memory_size( Value );

    Box::into_raw( obj );

    return true;
}

// Always show the effective segment register. If the option is `false`, only show the segment register if
// there's a segment override prefix.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,ds:[ ecx ]`
// 👍 | `false` | `mov eax,[ ecx ]`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetAlwaysShowSegmentRegister( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().always_show_segment_register();

    Box::into_raw( obj );
 
    return value;
}

// Always show the effective segment register. If the option is `false`, only show the segment register if
// there's a segment override prefix.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,ds:[ ecx ]`
// 👍 | `false` | `mov eax,[ ecx ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetAlwaysShowSegmentRegister( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_always_show_segment_register( Value );

    Box::into_raw( obj );

    return true;
}

// Use a hex prefix ( `0x` ) or a hex suffix ( `h` )
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `0x5A`
// X | `false` | `5Ah`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetUseHexPrefix( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().use_hex_prefix();

    Box::into_raw( obj );
 
    return value;
}

// Use a hex prefix ( `0x` ) or a hex suffix ( `h` )
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `0x5A`
// X | `false` | `5Ah`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetUseHexPrefix( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_use_hex_prefix( Value );

    Box::into_raw( obj );

    return true;
}

// Use pseudo instructions
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `vcmpnltsd xmm2,xmm6,xmm3`
// _ | `false` | `vcmpsd xmm2,xmm6,xmm3,5h`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetUsePseudoOps( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().use_pseudo_ops();

    Box::into_raw( obj );
 
    return value;
}

// Use pseudo instructions
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `vcmpnltsd xmm2,xmm6,xmm3`
// _ | `false` | `vcmpsd xmm2,xmm6,xmm3,5h`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetUsePseudoOps( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_use_pseudo_ops( Value );

    Box::into_raw( obj );

    return true;
}

// Show `RIP+displ` or the virtual address
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rip+12345678h ]`
// 👍 | `false` | `mov eax,[ 1029384756AFBECDh ]`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetRipRelativeAddresses( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().rip_relative_addresses();

    Box::into_raw( obj );
 
    return value;
}

// Show `RIP+displ` or the virtual address
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rip+12345678h ]`
// 👍 | `false` | `mov eax,[ 1029384756AFBECDh ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetRipRelativeAddresses( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_rip_relative_addresses( Value );

    Box::into_raw( obj );

    return true;
}

// Show the original value after the symbol name
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ myfield ( 12345678 ) ]`
// 👍 | `false` | `mov eax,[ myfield ]`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetShowSymbolAddress( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().show_symbol_address();

    Box::into_raw( obj );
 
    return value;
}

// Show the original value after the symbol name
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ myfield ( 12345678 ) ]`
// 👍 | `false` | `mov eax,[ myfield ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetShowSymbolAddress( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_show_symbol_address( Value );

    Box::into_raw( obj );

    return true;
}

// Add a space after the operand separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov rax, rcx`
// 👍 | `false` | `mov rax,rcx`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetSpaceAfterOperandSeparator( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().space_after_operand_separator();

    Box::into_raw( obj );
 
    return value;
}

// Add a space after the operand separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov rax, rcx`
// 👍 | `false` | `mov rax,rcx`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetSpaceAfterOperandSeparator( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_space_after_operand_separator( Value );

    Box::into_raw( obj );

    return true;
}

// Use uppercase hex digits
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `0xFF`
// _ | `false` | `0xff`
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_GetUpperCaseHex( Formatter: *mut TSpecializedFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_hex();

    Box::into_raw( obj );
 
    return value;
}

// Use uppercase hex digits
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `0xFF`
// _ | `false` | `0xff`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn SpecializedFormatter_SetUpperCaseHex( Formatter: *mut TSpecializedFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_hex( Value );

    Box::into_raw( obj );

    return true;
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Formatter Options

// Prefixes are uppercased
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `REP stosd`
// 👍 | `false` | `rep stosd`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUpperCasePrefixes( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_prefixes();

    Box::into_raw( obj );
 
    return value;
}

// Prefixes are uppercased
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `REP stosd`
// 👍 | `false` | `rep stosd`
//
// # Arguments
//
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUpperCasePrefixes( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_prefixes( Value );

    Box::into_raw( obj );

    return true;
}

// Mnemonics are uppercased
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `MOV rcx,rax`
// 👍 | `false` | `mov rcx,rax`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUpperCaseMnemonics( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_mnemonics();

    Box::into_raw( obj );
 
    return value;
}

// Mnemonics are uppercased
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `MOV rcx,rax`
// 👍 | `false` | `mov rcx,rax`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUpperCaseMnemonics( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_mnemonics( Value );

    Box::into_raw( obj );

    return true;
}

// Registers are uppercased
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov RCX,[ RAX+RDX*8 ]`
// 👍 | `false` | `mov rcx,[ rax+rdx*8 ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUpperCaseRegisters( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_registers();

    Box::into_raw( obj );
 
    return value;
}

// Registers are uppercased
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov RCX,[ RAX+RDX*8 ]`
// 👍 | `false` | `mov rcx,[ rax+rdx*8 ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUpperCaseRegisters( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_registers( Value );

    Box::into_raw( obj );

    return true;
}

// Keywords are uppercased ( eg. `BYTE PTR`, `SHORT` )
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov BYTE PTR [ rcx ],12h`
// 👍 | `false` | `mov byte ptr [ rcx ],12h`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUpperCaseKeyWords( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_keywords();

    Box::into_raw( obj );
 
    return value;
}

// Keywords are uppercased ( eg. `BYTE PTR`, `SHORT` )
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov BYTE PTR [ rcx ],12h`
// 👍 | `false` | `mov byte ptr [ rcx ],12h`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUpperCaseKeyWords( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_keywords( Value );

    Box::into_raw( obj );

    return true;
}

// Uppercase decorators, eg. `{z}`, `{sae}`, `{rd-sae}` ( but not opmask registers: `{k1}` )
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `vunpcklps xmm2{k5}{Z},xmm6,dword bcst [ rax+4 ]`
// 👍 | `false` | `vunpcklps xmm2{k5}{z},xmm6,dword bcst [ rax+4 ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUpperCaseDecorators( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_decorators();

    Box::into_raw( obj );
 
    return value;
}

// Uppercase decorators, eg. `{z}`, `{sae}`, `{rd-sae}` ( but not opmask registers: `{k1}` )
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `vunpcklps xmm2{k5}{Z},xmm6,dword bcst [ rax+4 ]`
// 👍 | `false` | `vunpcklps xmm2{k5}{z},xmm6,dword bcst [ rax+4 ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUpperCaseDecorators( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_decorators( Value );

    Box::into_raw( obj );

    return true;
}

// Everything is uppercased, except numbers and their prefixes/suffixes
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `MOV EAX,GS:[ RCX*4+0ffh ]`
// 👍 | `false` | `mov eax,gs:[ rcx*4+0ffh ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUpperCaseEverything( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_all();

    Box::into_raw( obj );
 
    return value;
}

// Everything is uppercased, except numbers and their prefixes/suffixes
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `MOV EAX,GS:[ RCX*4+0ffh ]`
// 👍 | `false` | `mov eax,gs:[ rcx*4+0ffh ]`
//
// # Arguments
//
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUpperCaseEverything( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_all( Value );

    Box::into_raw( obj );

    return true;
}

// Character index ( 0-based ) where the first operand is formatted. Can be set to 0 to format it immediately after the mnemonic.
// At least one space or tab is always added between the mnemonic and the first operand.
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `0` | `mov•rcx,rbp`
// _ | `8` | `mov•••••rcx,rbp`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetFirstOperandCharIndex( Formatter: *mut MasmFormatter ) -> u32 {
    if Formatter.is_null() {
        return 0;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().first_operand_char_index();

    Box::into_raw( obj );
 
    return value;
}

// Character index ( 0-based ) where the first operand is formatted. Can be set to 0 to format it immediately after the mnemonic.
// At least one space or tab is always added between the mnemonic and the first operand.
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `0` | `mov•rcx,rbp`
// _ | `8` | `mov•••••rcx,rbp`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetFirstOperandCharIndex( Formatter: *mut MasmFormatter, Value : u32 ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_first_operand_char_index( Value );

    Box::into_raw( obj );

    return true;
}

// Size of a tab character or 0 to use spaces
//
// - Default: `0`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetTabSize( Formatter: *mut MasmFormatter ) -> u32 {
    if Formatter.is_null() {
        return 0;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().tab_size();

    Box::into_raw( obj );
 
    return value;
}

// Size of a tab character or 0 to use spaces
//
// - Default: `0`
//
// # Arguments
//
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetTabSize( Formatter: *mut MasmFormatter, Value : u32 ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_tab_size( Value );

    Box::into_raw( obj );

    return true;
}

// Add a space after the operand separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov rax, rcx`
// 👍 | `false` | `mov rax,rcx`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetSpaceAfterOperandSeparator( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().space_after_operand_separator();

    Box::into_raw( obj );
 
    return value;
}

// Add a space after the operand separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov rax, rcx`
// 👍 | `false` | `mov rax,rcx`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetSpaceAfterOperandSeparator( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_space_after_operand_separator( Value );

    Box::into_raw( obj );

    return true;
}

// Add a space between the memory expression and the brackets
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[rcx+rdx ]`
// 👍 | `false` | `mov eax,[ rcx+rdx ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetSpaceAfterMemoryBracket( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().space_after_memory_bracket();

    Box::into_raw( obj );
 
    return value;
}

// Add a space between the memory expression and the brackets
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[rcx+rdx ]`
// 👍 | `false` | `mov eax,[ rcx+rdx ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetSpaceAfterMemoryBracket( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_space_after_memory_bracket( Value );

    Box::into_raw( obj );

    return true;
}

// Add spaces between memory operand `+` and `-` operators
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rcx + rdx*8 - 80h ]`
// 👍 | `false` | `mov eax,[ rcx+rdx*8-80h ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetSpaceBetweenMemoryAddOperators( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().space_between_memory_add_operators();

    Box::into_raw( obj );
 
    return value;
}

// Add spaces between memory operand `+` and `-` operators
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rcx + rdx*8 - 80h ]`
// 👍 | `false` | `mov eax,[ rcx+rdx*8-80h ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetSpaceBetweenMemoryAddOperators( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_space_between_memory_add_operators( Value );

    Box::into_raw( obj );

    return true;
}

// Add spaces between memory operand `*` operator
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rcx+rdx * 8-80h ]`
// 👍 | `false` | `mov eax,[ rcx+rdx*8-80h ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetSpaceBetweenMemoryMulOperators( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().space_between_memory_mul_operators();

    Box::into_raw( obj );
 
    return value;
}

// Add spaces between memory operand `*` operator
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rcx+rdx * 8-80h ]`
// 👍 | `false` | `mov eax,[ rcx+rdx*8-80h ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetSpaceBetweenMemoryMulOperators( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_space_between_memory_mul_operators( Value );

    Box::into_raw( obj );

    return true;
}

// Show memory operand scale value before the index register
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ 8*rdx ]`
// 👍 | `false` | `mov eax,[ rdx*8 ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetScaleBeforeIndex( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().scale_before_index();

    Box::into_raw( obj );
 
    return value;
}

// Show memory operand scale value before the index register
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ 8*rdx ]`
// 👍 | `false` | `mov eax,[ rdx*8 ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetScaleBeforeIndex( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_scale_before_index( Value );

    Box::into_raw( obj );

    return true;
}

// Always show the scale value even if it's `*1`
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rbx+rcx*1 ]`
// 👍 | `false` | `mov eax,[ rbx+rcx ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetAlwaysShowScale( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().always_show_scale();

    Box::into_raw( obj );
 
    return value;
}

// Always show the scale value even if it's `*1`
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rbx+rcx*1 ]`
// 👍 | `false` | `mov eax,[ rbx+rcx ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetAlwaysShowScale( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_always_show_scale( Value );

    Box::into_raw( obj );

    return true;
}

// Always show the effective segment register. If the option is `false`, only show the segment register if
// there's a segment override prefix.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,ds:[ ecx ]`
// 👍 | `false` | `mov eax,[ ecx ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetAlwaysShowSegmentRegister( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().always_show_segment_register();

    Box::into_raw( obj );
 
    return value;
}

// Always show the effective segment register. If the option is `false`, only show the segment register if
// there's a segment override prefix.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,ds:[ ecx ]`
// 👍 | `false` | `mov eax,[ ecx ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetAlwaysShowSegmentRegister( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_always_show_segment_register( Value );

    Box::into_raw( obj );

    return true;
}

// Show zero displacements
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rcx*2+0 ]`
// 👍 | `false` | `mov eax,[ rcx*2 ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetShowZeroDisplacements( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().show_zero_displacements();

    Box::into_raw( obj );
 
    return value;
}

// Show zero displacements
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rcx*2+0 ]`
// 👍 | `false` | `mov eax,[ rcx*2 ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetShowZeroDisplacements( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_show_zero_displacements( Value );

    Box::into_raw( obj );

    return true;
}

// Hex number prefix or an empty string, eg. `"0x"`
//
// - Default: `""` ( masm/nasm/intel ), `"0x"` ( gas )
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetHexPrefix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {    
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().hex_prefix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Hex number prefix or an empty string, eg. `"0x"`
//
// - Default: `""` ( masm/nasm/intel ), `"0x"` ( gas )
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetHexPrefix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()        
    };

    obj.options_mut().set_hex_prefix_string( value );

    Box::into_raw( obj );

    return true;
}

// Hex number suffix or an empty string, eg. `"h"`
//
// - Default: `"h"` ( masm/nasm/intel ), `""` ( gas )
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetHexSuffix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().hex_suffix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Hex number suffix or an empty string, eg. `"h"`
//
// - Default: `"h"` ( masm/nasm/intel ), `""` ( gas )
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetHexSuffix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_hex_suffix_string( value );

    Box::into_raw( obj );

    return true;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `0x12345678`
// 👍 | `4` | `0x1234_5678`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetHexDigitGroupSize( Formatter: *mut MasmFormatter ) -> u32 {
    if Formatter.is_null() {
        return 0;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().hex_digit_group_size();

    Box::into_raw( obj );
 
    return value;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `0x12345678`
// 👍 | `4` | `0x1234_5678`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetHexDigitGroupSize( Formatter: *mut MasmFormatter, Value : u32 ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_hex_digit_group_size( Value );

    Box::into_raw( obj );

    return true;
}

// Decimal number prefix or an empty string
//
// - Default: `""`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetDecimalPrefix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().decimal_prefix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Decimal number prefix or an empty string
//
// - Default: `""`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetDecimalPrefix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_decimal_prefix_string( value );

    Box::into_raw( obj );

    return true;
}

// Decimal number suffix or an empty string
//
// - Default: `""`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetDecimalSuffix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().decimal_suffix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Decimal number suffix or an empty string
//
// - Default: `""`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetDecimalSuffix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_decimal_suffix_string( value );

    Box::into_raw( obj );

    return true;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `12345678`
// 👍 | `3` | `12_345_678`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetDecimalDigitGroupSize( Formatter: *mut MasmFormatter ) -> u32 {
    if Formatter.is_null() {
        return 0;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().decimal_digit_group_size();

    Box::into_raw( obj );
 
    return value;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `12345678`
// 👍 | `3` | `12_345_678`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetDecimalDigitGroupSize( Formatter: *mut MasmFormatter, Value : u32 ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_decimal_digit_group_size( Value );

    Box::into_raw( obj );

    return true;
}

// Octal number prefix or an empty string
//
// - Default: `""` ( masm/nasm/intel ), `"0"` ( gas )
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetOctalPrefix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().octal_prefix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Octal number prefix or an empty string
//
// - Default: `""` ( masm/nasm/intel ), `"0"` ( gas )
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetOctalPrefix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_octal_prefix_string( value );

    Box::into_raw( obj );

    return true;
}

// Octal number suffix or an empty string
//
// - Default: `"o"` ( masm/nasm/intel ), `""` ( gas )
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetOctalSuffix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().octal_suffix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Octal number suffix or an empty string
//
// - Default: `"o"` ( masm/nasm/intel ), `""` ( gas )
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetOctalSuffix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_octal_suffix_string( value );

    Box::into_raw( obj );

    return true;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `12345670`
// 👍 | `4` | `1234_5670`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetOctalDigitGroupSize( Formatter: *mut MasmFormatter ) -> u32 {
    if Formatter.is_null() {
        return 0;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().octal_digit_group_size();

    Box::into_raw( obj );
 
    return value;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `12345670`
// 👍 | `4` | `1234_5670`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetOctalDigitGroupSize( Formatter: *mut MasmFormatter, Value : u32 ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_octal_digit_group_size( Value );

    Box::into_raw( obj );

    return true;
}

// Binary number prefix or an empty string
//
// - Default: `""` ( masm/nasm/intel ), `"0b"` ( gas )
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetBinaryPrefix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().binary_prefix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Binary number prefix or an empty string
//
// - Default: `""` ( masm/nasm/intel ), `"0b"` ( gas )
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetBinaryPrefix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_binary_prefix_string( value );

    Box::into_raw( obj );

    return true;
}

// Binary number suffix or an empty string
//
// - Default: `"b"` ( masm/nasm/intel ), `""` ( gas )
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetBinarySuffix( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().binary_suffix();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Binary number suffix or an empty string
//
// - Default: `"b"` ( masm/nasm/intel ), `""` ( gas )
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetBinarySuffix( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_binary_suffix_string( value );

    Box::into_raw( obj );

    return true;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `11010111`
// 👍 | `4` | `1101_0111`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetBinaryDigitGroupSize( Formatter: *mut MasmFormatter ) -> u32 {
    if Formatter.is_null() {
        return 0;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().binary_digit_group_size();

    Box::into_raw( obj );
 
    return value;
}

// Size of a digit group, see also [ `digit_separator()` ]
//
// [ `digit_separator()` ]: #method.digit_separator
//
// Default | Value | Example
// --------|-------|--------
// _ | `0` | `11010111`
// 👍 | `4` | `1101_0111`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetBinaryDigitGroupSize( Formatter: *mut MasmFormatter, Value : u32 ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_binary_digit_group_size( Value );

    Box::into_raw( obj );

    return true;
}

// Digit separator or an empty string. See also eg. [ `hex_digit_group_size()` ]
//
// [ `hex_digit_group_size()` ]: #method.hex_digit_group_size
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `""` | `0x12345678`
// _ | `"_"` | `0x1234_5678`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetDigitSeparator( Formatter: *mut MasmFormatter, Value : *mut u8, Size : usize ) -> usize {
    if Formatter.is_null() {
        return 0;
    }
    if Value.is_null() {
        return 0;
    }

    if Size <= 0 {
        return 0
    }

    let mut obj = Box::from_raw( Formatter );
    
    let tmp = obj.options_mut().digit_separator();
    let mut l = tmp.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Value.add( i ) ) = tmp.as_bytes()[ i ];
        
        }
    }
    *( Value.add( l ) ) = 0;

    Box::into_raw( obj );

    return l;
}

// Digit separator or an empty string. See also eg. [ `hex_digit_group_size()` ]
//
// [ `hex_digit_group_size()` ]: #method.hex_digit_group_size
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `""` | `0x12345678`
// _ | `"_"` | `0x1234_5678`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetDigitSeparator( Formatter: *mut MasmFormatter, Value : *const c_char ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = {
        let c_s = Value;
        str::from_utf8_unchecked( slice::from_raw_parts( c_s as *const u8, strlen( c_s ) ) ).to_owned()
    };

    obj.options_mut().set_digit_separator_string( value );

    Box::into_raw( obj );

    return true;
}

// Add leading zeros to hexadecimal/octal/binary numbers.
// This option has no effect on branch targets and displacements, use [ `branch_leading_zeros` ]
// and [ `displacement_leading_zeros` ].
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `0x0000000A`/`0000000Ah`
// 👍 | `false` | `0xA`/`0Ah`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetLeadingZeros( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().leading_zeros();

    Box::into_raw( obj );
 
    return value;
}

// Add leading zeros to hexadecimal/octal/binary numbers.
// This option has no effect on branch targets and displacements, use [ `branch_leading_zeros` ]
// and [ `displacement_leading_zeros` ].
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `0x0000000A`/`0000000Ah`
// 👍 | `false` | `0xA`/`0Ah`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetLeadingZeros( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_leading_zeros( Value );

    Box::into_raw( obj );

    return true;
}

// Use uppercase hex digits
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `0xFF`
// _ | `false` | `0xff`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUppercaseHex( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().uppercase_hex();

    Box::into_raw( obj );
 
    return value;
}

// Use uppercase hex digits
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `0xFF`
// _ | `false` | `0xff`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUppercaseHex( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_uppercase_hex( Value );

    Box::into_raw( obj );

    return true;
}

// Small hex numbers ( -9 .. 9 ) are shown in decimal
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `9`
// _ | `false` | `0x9`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetSmallHexNumbersInDecimal( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().small_hex_numbers_in_decimal();

    Box::into_raw( obj );
 
    return value;
}

// Small hex numbers ( -9 .. 9 ) are shown in decimal
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `9`
// _ | `false` | `0x9`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetSmallHexNumbersInDecimal( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_small_hex_numbers_in_decimal( Value );

    Box::into_raw( obj );

    return true;
}

// Add a leading zero to hex numbers if there's no prefix and the number starts with hex digits `A-F`
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `0FFh`
// _ | `false` | `FFh`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetAddLeadingZeroToHexNumbers( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().add_leading_zero_to_hex_numbers();

    Box::into_raw( obj );
 
    return value;
}

// Add a leading zero to hex numbers if there's no prefix and the number starts with hex digits `A-F`
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `0FFh`
// _ | `false` | `FFh`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetAddLeadingZeroToHexNumbers( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_add_leading_zero_to_hex_numbers( Value );

    Box::into_raw( obj );

    return true;
}

// Number base
//
// - Default: [ `Hexadecimal` ]
//
// [ `Hexadecimal` ]: enum.NumberBase.html#variant.Hexadecimal
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetNumberBase( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: NumberBase
    if Formatter.is_null() {
        return 0;// NumberBase::Hexadecimal;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32/*TNumberBase*/ = transmute( obj.options_mut().number_base() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Number base
//
// - Default: [ `Hexadecimal` ]
//
// [ `Hexadecimal` ]: enum.NumberBase.html#variant.Hexadecimal
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetNumberBase( Formatter: *mut MasmFormatter, Value : u32 /*NumberBase*/ ) -> bool { // FFI-Unsafe:
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_number_base( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Add leading zeros to branch offsets. Used by `CALL NEAR`, `CALL FAR`, `JMP NEAR`, `JMP FAR`, `Jcc`, `LOOP`, `LOOPcc`, `XBEGIN`
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `je 00000123h`
// _ | `false` | `je 123h`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetBranchLeadingZeros( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().branch_leading_zeros();

    Box::into_raw( obj );
 
    return value;
}

// Add leading zeros to branch offsets. Used by `CALL NEAR`, `CALL FAR`, `JMP NEAR`, `JMP FAR`, `Jcc`, `LOOP`, `LOOPcc`, `XBEGIN`
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `je 00000123h`
// _ | `false` | `je 123h`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetBranchLeadingZeros( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_branch_leading_zeros( Value );

    Box::into_raw( obj );

    return true;
}

// Show immediate operands as signed numbers
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,-1`
// 👍 | `false` | `mov eax,FFFFFFFF`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetSignedImmediateOperands( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().signed_immediate_operands();

    Box::into_raw( obj );
 
    return value;
}

// Show immediate operands as signed numbers
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,-1`
// 👍 | `false` | `mov eax,FFFFFFFF`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetSignedImmediateOperands( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_signed_immediate_operands( Value );

    Box::into_raw( obj );

    return true;
}

// Displacements are signed numbers
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `mov al,[ eax-2000h ]`
// _ | `false` | `mov al,[ eax+0FFFFE000h ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetSignedMemoryDisplacements( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().signed_memory_displacements();

    Box::into_raw( obj );
 
    return value;
}

// Displacements are signed numbers
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `mov al,[ eax-2000h ]`
// _ | `false` | `mov al,[ eax+0FFFFE000h ]`
//
// # Arguments
//
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetSignedMemoryDisplacements( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_signed_memory_displacements( Value );

    Box::into_raw( obj );

    return true;
}

// Add leading zeros to displacements
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov al,[ eax+00000012h ]`
// 👍 | `false` | `mov al,[ eax+12h ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetDisplacementLeadingZeros( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().displacement_leading_zeros();

    Box::into_raw( obj );
 
    return value;
}

// Add leading zeros to displacements
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov al,[ eax+00000012h ]`
// 👍 | `false` | `mov al,[ eax+12h ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetDisplacementLeadingZeros( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_displacement_leading_zeros( Value );

    Box::into_raw( obj );

    return true;
}

// Options that control if the memory size ( eg. `DWORD PTR` ) is shown or not.
// This is ignored by the gas ( AT&T ) formatter.
//
// - Default: [ `Default` ]
//
// [ `Default` ]: enum.MemorySizeOptions.html#variant.Default
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetMemorySizeOptions( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: MemorySizeOptions
    if Formatter.is_null() {
        return 0;// MemorySizeOptions::Default;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().memory_size_options() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Options that control if the memory size ( eg. `DWORD PTR` ) is shown or not.
// This is ignored by the gas ( AT&T ) formatter.
//
// - Default: [ `Default` ]
//
// [ `Default` ]: enum.MemorySizeOptions.html#variant.Default
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetMemorySizeOptions( Formatter: *mut MasmFormatter, Value : u32 /*MemorySizeOptions*/ ) -> bool { // FFI-Unsafe: MemorySizeOptions
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_memory_size_options( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Show `RIP+displ` or the virtual address
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rip+12345678h ]`
// 👍 | `false` | `mov eax,[ 1029384756AFBECDh ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetRipRelativeAddresses( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().rip_relative_addresses();

    Box::into_raw( obj );
 
    return value;
}

// Show `RIP+displ` or the virtual address
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ rip+12345678h ]`
// 👍 | `false` | `mov eax,[ 1029384756AFBECDh ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetRipRelativeAddresses( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_rip_relative_addresses( Value );

    Box::into_raw( obj );

    return true;
}

// Show `NEAR`, `SHORT`, etc if it's a branch instruction
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `je short 1234h`
// _ | `false` | `je 1234h`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetShowBranchSize( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().show_branch_size();

    Box::into_raw( obj );
 
    return value;
}

// Show `NEAR`, `SHORT`, etc if it's a branch instruction
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `je short 1234h`
// _ | `false` | `je 1234h`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetShowBranchSize( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_show_branch_size( Value );

    Box::into_raw( obj );

    return true;
}

// Use pseudo instructions
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `vcmpnltsd xmm2,xmm6,xmm3`
// _ | `false` | `vcmpsd xmm2,xmm6,xmm3,5`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetUsePseudoOps( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().use_pseudo_ops();

    Box::into_raw( obj );
 
    return value;
}

// Use pseudo instructions
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `vcmpnltsd xmm2,xmm6,xmm3`
// _ | `false` | `vcmpsd xmm2,xmm6,xmm3,5`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetUsePseudoOps( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_use_pseudo_ops( Value );

    Box::into_raw( obj );

    return true;
}

// Show the original value after the symbol name
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ myfield ( 12345678 ) ]`
// 👍 | `false` | `mov eax,[ myfield ]`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetShowSymbolAddress( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().show_symbol_address();

    Box::into_raw( obj );
 
    return value;
}

// Show the original value after the symbol name
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,[ myfield ( 12345678 ) ]`
// 👍 | `false` | `mov eax,[ myfield ]`
//
// # Arguments
//
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetShowSymbolAddress( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_show_symbol_address( Value );

    Box::into_raw( obj );

    return true;
}

// ( gas only ): If `true`, the formatter doesn't add `%` to registers
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,ecx`
// 👍 | `false` | `mov %eax,%ecx`
#[no_mangle]
pub unsafe extern "C" fn GasFormatter_GetNakedRegisters( Formatter: *mut GasFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().gas_naked_registers();

    Box::into_raw( obj );
 
    return value;
}

// ( gas only ): If `true`, the formatter doesn't add `%` to registers
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `mov eax,ecx`
// 👍 | `false` | `mov %eax,%ecx`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn GasFormatter_SetNakedRegisters( Formatter: *mut GasFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_gas_naked_registers( Value );

    Box::into_raw( obj );

    return true;
}

// ( gas only ): Shows the mnemonic size suffix even when not needed
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `movl %eax,%ecx`
// 👍 | `false` | `mov %eax,%ecx`
#[no_mangle]
pub unsafe extern "C" fn GasFormatter_GetShowMnemonicSizeSuffix( Formatter: *mut GasFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().gas_show_mnemonic_size_suffix();

    Box::into_raw( obj );
 
    return value;
}

// ( gas only ): Shows the mnemonic size suffix even when not needed
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `movl %eax,%ecx`
// 👍 | `false` | `mov %eax,%ecx`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn GasFormatter_SetShowMnemonicSizeSuffix( Formatter: *mut GasFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_gas_show_mnemonic_size_suffix( Value );

    Box::into_raw( obj );

    return true;
}

// ( gas only ): Add a space after the comma if it's a memory operand
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `( %eax, %ecx, 2 )`
// 👍 | `false` | `( %eax,%ecx,2 )`
#[no_mangle]
pub unsafe extern "C" fn GasFormatter_GetSpaceAfterMemoryOperandComma( Formatter: *mut GasFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().gas_space_after_memory_operand_comma();

    Box::into_raw( obj );
 
    return value;
}

// ( gas only ): Add a space after the comma if it's a memory operand
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `( %eax, %ecx, 2 )`
// 👍 | `false` | `( %eax,%ecx,2 )`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn GasFormatter_SetSpaceAfterMemoryOperandComma( Formatter: *mut GasFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_gas_space_after_memory_operand_comma( Value );

    Box::into_raw( obj );

    return true;
}

// ( masm only ): Add a `DS` segment override even if it's not present. Used if it's 16/32-bit code and mem op is a displ
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `mov eax,ds:[ 12345678 ]`
// _ | `false` | `mov eax,[ 12345678 ]`
#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_GetAddDsPrefix32( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().masm_add_ds_prefix32();

    Box::into_raw( obj );
 
    return value;
}

// ( masm only ): Add a `DS` segment override even if it's not present. Used if it's 16/32-bit code and mem op is a displ
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `mov eax,ds:[ 12345678 ]`
// _ | `false` | `mov eax,[ 12345678 ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_SetAddDsPrefix32( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_masm_add_ds_prefix32( Value );

    Box::into_raw( obj );

    return true;
}

// ( masm only ): Show symbols in brackets
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `[ ecx+symbol ]` / `[ symbol ]`
// _ | `false` | `symbol[ ecx ]` / `symbol`
#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_GetSymbolDisplacementInBrackets( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().masm_symbol_displ_in_brackets();

    Box::into_raw( obj );
 
    return value;
}

// ( masm only ): Show symbols in brackets
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `[ ecx+symbol ]` / `[ symbol ]`
// _ | `false` | `symbol[ ecx ]` / `symbol`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_SetSymbolDisplacementInBrackets( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_masm_symbol_displ_in_brackets( Value );

    Box::into_raw( obj );

    return true;
}

// ( masm only ): Show displacements in brackets
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `[ ecx+1234h ]`
// _ | `false` | `1234h[ ecx ]`
#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_GetDisplacementInBrackets( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().masm_displ_in_brackets();

    Box::into_raw( obj );
 
    return value;
}

// ( masm only ): Show displacements in brackets
//
// Default | Value | Example
// --------|-------|--------
// 👍 | `true` | `[ ecx+1234h ]`
// _ | `false` | `1234h[ ecx ]`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn MasmFormatter_SetDisplacementInBrackets( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_masm_displ_in_brackets( Value );

    Box::into_raw( obj );

    return true;
}

// ( nasm only ): Shows `BYTE`, `WORD`, `DWORD` or `QWORD` if it's a sign extended immediate operand value
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `or rcx,byte -1`
// 👍 | `false` | `or rcx,-1`
#[no_mangle]
pub unsafe extern "C" fn NasmFormatter_GetShowSignExtendedImmediateSize( Formatter: *mut NasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().nasm_show_sign_extended_immediate_size();

    Box::into_raw( obj );
 
    return value;
}

// ( nasm only ): Shows `BYTE`, `WORD`, `DWORD` or `QWORD` if it's a sign extended immediate operand value
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `or rcx,byte -1`
// 👍 | `false` | `or rcx,-1`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn NasmFormatter_SetShowSignExtendedImmediateSize( Formatter: *mut NasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_nasm_show_sign_extended_immediate_size( Value );

    Box::into_raw( obj );

    return true;
}

// Use `st( 0 )` instead of `st` if `st` can be used. Ignored by the nasm formatter.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `fadd st( 0 ),st( 3 )`
// 👍 | `false` | `fadd st,st( 3 )`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetPreferST0( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().prefer_st0();

    Box::into_raw( obj );
 
    return value;
}

// Use `st( 0 )` instead of `st` if `st` can be used. Ignored by the nasm formatter.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `fadd st( 0 ),st( 3 )`
// 👍 | `false` | `fadd st,st( 3 )`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetPreferST0( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_prefer_st0( Value );

    Box::into_raw( obj );

    return true;
}

// Show useless prefixes. If it has useless prefixes, it could be data and not code.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `es rep add eax,ecx`
// 👍 | `false` | `add eax,ecx`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetShowUselessPrefixes( Formatter: *mut MasmFormatter ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    let value = obj.options_mut().show_useless_prefixes();

    Box::into_raw( obj );
 
    return value;
}

// Show useless prefixes. If it has useless prefixes, it could be data and not code.
//
// Default | Value | Example
// --------|-------|--------
// _ | `true` | `es rep add eax,ecx`
// 👍 | `false` | `add eax,ecx`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetShowUselessPrefixes( Formatter: *mut MasmFormatter, Value : bool ) -> bool {
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_show_useless_prefixes( Value );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JB` / `JC` / `JNAE` )
//
// Default: `JB`, `CMOVB`, `SETB`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_b( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_b 
    if Formatter.is_null() {
        return 0;// CC_b::b;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_b() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JB` / `JC` / `JNAE` )
//
// Default: `JB`, `CMOVB`, `SETB`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_b( Formatter: *mut MasmFormatter, Value : u32/*CC_b*/ ) -> bool { // FFI-Unsafe: CC_b
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_b( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JAE` / `JNB` / `JNC` )
//
// Default: `JAE`, `CMOVAE`, `SETAE`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_ae( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_ae
    if Formatter.is_null() {
        return 0;// CC_ae::ae;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_ae() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JAE` / `JNB` / `JNC` )
//
// Default: `JAE`, `CMOVAE`, `SETAE`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_ae( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_ae
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_ae( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JE` / `JZ` )
//
// Default: `JE`, `CMOVE`, `SETE`, `LOOPE`, `REPE`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_e( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_e
    if Formatter.is_null() {
        return 0;// CC_e::e;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_e() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JE` / `JZ` )
//
// Default: `JE`, `CMOVE`, `SETE`, `LOOPE`, `REPE`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_e( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_e
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_e( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JNE` / `JNZ` )
//
// Default: `JNE`, `CMOVNE`, `SETNE`, `LOOPNE`, `REPNE`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_ne( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_ne
    if Formatter.is_null() {
        return 0;// CC_ne::ne;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_ne() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JNE` / `JNZ` )
//
// Default: `JNE`, `CMOVNE`, `SETNE`, `LOOPNE`, `REPNE`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_ne( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_ne
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_ne( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JBE` / `JNA` )
//
// Default: `JBE`, `CMOVBE`, `SETBE`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_be( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_be
    if Formatter.is_null() {
        return 0;// CC_be::be;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_be() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JBE` / `JNA` )
//
// Default: `JBE`, `CMOVBE`, `SETBE`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_be( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_be
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_be( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JA` / `JNBE` )
//
// Default: `JA`, `CMOVA`, `SETA`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_a( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_a
    if Formatter.is_null() {
        return 0;// CC_a::a;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_a() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JA` / `JNBE` )
//
// Default: `JA`, `CMOVA`, `SETA`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_a( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_a
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_a( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JP` / `JPE` )
//
// Default: `JP`, `CMOVP`, `SETP`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_p( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_p
    if Formatter.is_null() {
        return 0;// CC_p::p;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_p() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JP` / `JPE` )
//
// Default: `JP`, `CMOVP`, `SETP`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_p( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_p
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_p( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JNP` / `JPO` )
//
// Default: `JNP`, `CMOVNP`, `SETNP`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_np( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_np
    if Formatter.is_null() {
        return 0;// CC_np::np;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_np() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JNP` / `JPO` )
//
// Default: `JNP`, `CMOVNP`, `SETNP`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_np( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_np
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_np( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JL` / `JNGE` )
//
// Default: `JL`, `CMOVL`, `SETL`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_l( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_l
    if Formatter.is_null() {
        return 0;// CC_l::l;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_l() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JL` / `JNGE` )
//
// Default: `JL`, `CMOVL`, `SETL`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_l( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_l
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_l( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JGE` / `JNL` )
//
// Default: `JGE`, `CMOVGE`, `SETGE`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_ge( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_ge
    if Formatter.is_null() {
        return 0;// CC_ge::ge;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_ge() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JGE` / `JNL` )
//
// Default: `JGE`, `CMOVGE`, `SETGE`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_ge( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_ge
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_ge( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JLE` / `JNG` )
//
// Default: `JLE`, `CMOVLE`, `SETLE`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_le( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_le
    if Formatter.is_null() {
        return 0;// CC_le::le;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_le() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JLE` / `JNG` )
//
// Default: `JLE`, `CMOVLE`, `SETLE`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_le( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_le
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_le( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}

// Mnemonic condition code selector ( eg. `JG` / `JNLE` )
//
// Default: `JG`, `CMOVG`, `SETG`
#[no_mangle]
pub unsafe extern "C" fn Formatter_GetCC_g( Formatter: *mut MasmFormatter ) -> u32 { // FFI-Unsafe: CC_g
    if Formatter.is_null() {
        return 0;// CC_g::g;
    }
    let mut obj = Box::from_raw( Formatter );

    let value: u32 = transmute( obj.options_mut().cc_g() as u32 );

    Box::into_raw( obj );
   
    return value;
}

// Mnemonic condition code selector ( eg. `JG` / `JNLE` )
//
// Default: `JG`, `CMOVG`, `SETG`
//
// # Arguments
// * `value`: New value
#[no_mangle]
pub unsafe extern "C" fn Formatter_SetCC_g( Formatter: *mut MasmFormatter, Value : u32 ) -> bool { // FFI-Unsafe: CC_h
    if Formatter.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Formatter );

    obj.options_mut().set_cc_g( transmute( Value as u8 ) );

    Box::into_raw( obj );

    return true;
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Encoder

// Creates an encoder
//
// Returns NULL if `bitness` is not one of 16, 32, 64.
//
// # Arguments
// * `bitness`: 16, 32 or 64
#[no_mangle]
pub extern "C" fn Encoder_Create( Bitness: u32, Capacity: usize ) -> *mut Encoder { 
    if Capacity > 0 {
        match Encoder::try_with_capacity( Bitness, Capacity ) {
            Ok( value ) => return Box::into_raw( Box::new( value ) ),
            Err( _e ) => return null_mut()
        }
    }else {
        match Encoder::try_new( Bitness ) {
            Ok( value ) => return Box::into_raw( Box::new( value ) ),
            Err( _e ) => return null_mut()
        }
   }
}

// Encodes an instruction and returns the size of the encoded instruction
//
// # Result
// * Returns written amount of encoded Bytes
//
// # Arguments
// * `instruction`: Instruction to encode
// * `rip`: `RIP` of the encoded instruction
#[no_mangle]
pub unsafe extern "C" fn Encoder_Encode( Encoder: *mut Encoder, Instruction: *mut Instruction, RIP: u64 ) -> usize {
    if Encoder.is_null() {
        return 0;
    }
    let mut obj = Box::from_raw( Encoder );

    let value = obj.encode( Instruction.as_mut().unwrap(), RIP );
    
    Box::into_raw( obj );

    match value {
        Ok( v ) => return v,
        Err( _e ) => return 0,
    }
}

// Writes a byte to the output buffer
//
// # Arguments
//
// `value`: Value to write
#[no_mangle]
pub unsafe extern "C" fn Encoder_WriteByte( Encoder: *mut Encoder, Value : u8 ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Encoder );

    obj.write_u8( Value );

    Box::into_raw( obj );

    return true;
}

// Returns the buffer and initializes the internal buffer to an empty vector. Should be called when
// you've encoded all instructions and need the raw instruction bytes. See also [ `Encoder_SetBuffer()` ].
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetBuffer( Encoder: *mut Encoder, Buffer : *mut u8, Size : usize ) -> bool {     
    if Encoder.is_null() {
        return false;
    }

    if Buffer.is_null() {
        return false;
    }

    if Size <= 0 {
        return false;
    }

    let mut obj = Box::from_raw( Encoder );

    let value = obj.take_buffer();
    Box::into_raw( obj );

    let mut l = value.len();
    if l > Size {
        l = Size;
    }
    
    if l > 0 {
        for i in 0..l {
            *( Buffer.add( i ) ) = value[ i ];
        
        }
    }
    *( Buffer.add( l ) ) = 0;

    return true;
}

// Overwrites the buffer with a new vector. The old buffer is dropped. See also [ `Encoder_GetBuffer()` ].
// NOTE: Monitor the result of [`Encoder_Encode`] (Encoded Bytes).
// DO NOT Encode more Bytes than fitting your provided Buffer as this would cause a realloc - which will lead to an access violation.
// Disabled: Unsetting the Buffer seems impossible as Rust wants to deallocate the Vector .. 
/*
#[no_mangle]
pub unsafe extern "C" fn Encoder_SetBuffer( Encoder: *mut Encoder, Buffer : *mut u8, Size : usize ) -> bool {     
    if Encoder.is_null() {
        return false;
    }

    if !Buffer.is_null() && ( Size <= 0 ) {
        return false;
    }

    let mut obj = Box::from_raw( Encoder );

    if Buffer.is_null() { 
        obj.set_buffer( Vec::new() );
    }else {
        obj.set_buffer( Vec::from_raw_parts( Buffer, 0/*Used*/, Size/*TotalSize*/ ) );
    }
    
    Box::into_raw( obj );

    return true;
}
*/

// Gets the offsets of the constants ( memory displacement and immediate ) in the encoded instruction.
// The caller can use this information to add relocations if needed.
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetConstantOffsets( Encoder: *mut Encoder, ConstantOffsets : *mut ConstantOffsets ) {
    if Encoder.is_null() {
        return;
    }
    let obj = Box::from_raw( Encoder );
    *ConstantOffsets = obj.get_constant_offsets();

    Box::into_raw( obj );
}

// Disables 2-byte VEX encoding and encodes all VEX instructions with the 3-byte VEX encoding
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetPreventVex2( Encoder: *mut Encoder ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let obj = Box::from_raw( Encoder );

    let value = obj.prevent_vex2();

    Box::into_raw( obj );
 
    return value;
}

// Disables 2-byte VEX encoding and encodes all VEX instructions with the 3-byte VEX encoding
//
// # Arguments
// * `new_value`: new value
#[no_mangle]
pub unsafe extern "C" fn Encoder_SetPreventVex2( Encoder: *mut Encoder, Value : bool ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Encoder );

    obj.set_prevent_vex2( Value );

    Box::into_raw( obj );

    return true;
}

// Value of the `VEX.W` bit to use if it's an instruction that ignores the bit. Default is 0.
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetVexWig( Encoder: *mut Encoder ) -> u32 {
    if Encoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Encoder );

    let value = obj.vex_wig();

    Box::into_raw( obj );
 
    return value;
}

// Value of the `VEX.W` bit to use if it's an instruction that ignores the bit. Default is 0.
//
// # Arguments
// * `new_value`: new value ( 0 or 1 )
#[no_mangle]
pub unsafe extern "C" fn Encoder_SetVexWig( Encoder: *mut Encoder, Value : u32 ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Encoder );

    obj.set_vex_wig( Value );

    Box::into_raw( obj );

    return true;
}

// Value of the `VEX.L` bit to use if it's an instruction that ignores the bit. Default is 0.
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetVexLig( Encoder: *mut Encoder ) -> u32 {
    if Encoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Encoder );

    let value = obj.vex_lig();

    Box::into_raw( obj );
 
    return value;
}

// Value of the `VEX.L` bit to use if it's an instruction that ignores the bit. Default is 0.
//
// # Arguments
// * `new_value`: new value ( 0 or 1 )
#[no_mangle]
pub unsafe extern "C" fn Encoder_SetVexLig( Encoder: *mut Encoder, Value : u32 ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Encoder );

    obj.set_vex_lig( Value );

    Box::into_raw( obj );

    return true;
}

// Value of the `EVEX.W` bit to use if it's an instruction that ignores the bit. Default is 0.
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetEvexWig( Encoder: *mut Encoder ) -> u32 {
    if Encoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Encoder );

    let value = obj.evex_wig();

    Box::into_raw( obj );
 
    return value;
}

// Value of the `EVEX.W` bit to use if it's an instruction that ignores the bit. Default is 0.
//
// # Arguments
// * `new_value`: new value ( 0 or 1 )
#[no_mangle]
pub unsafe extern "C" fn Encoder_SetEvexWig( Encoder: *mut Encoder, Value : u32 ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Encoder );

    obj.set_evex_wig( Value );

    Box::into_raw( obj );

    return true;
}

// Value of the `EVEX.L'L` bits to use if it's an instruction that ignores the bits. Default is 0.
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetEvexLig( Encoder: *mut Encoder ) -> u32 {
    if Encoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Encoder );

    let value = obj.evex_lig();

    Box::into_raw( obj );
 
    return value;
}

// Value of the `EVEX.L'L` bits to use if it's an instruction that ignores the bits. Default is 0.
//
// # Arguments
// * `new_value`: new value ( 0 or 3 )
#[no_mangle]
pub unsafe extern "C" fn Encoder_SetEvexLig( Encoder: *mut Encoder, Value : u32 ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Encoder );

    obj.set_evex_lig( Value );

    Box::into_raw( obj );

    return true;
}

// Value of the `MVEX.W` bit to use if it's an instruction that ignores the bit. Default is 0.
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetMvexWig( Encoder: *mut Encoder ) -> u32 {
    if Encoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Encoder );

    let value = obj.mvex_wig();

    Box::into_raw( obj );
 
    return value;
}

// Value of the `MVEX.W` bit to use if it's an instruction that ignores the bit. Default is 0.
//
// # Arguments
// * `new_value`: new value ( 0 or 1 )
#[no_mangle]
pub unsafe extern "C" fn Encoder_SetMvexWig( Encoder: *mut Encoder, Value : u32 ) -> bool {
    if Encoder.is_null() {
        return false;
    }
    let mut obj = Box::from_raw( Encoder );

    obj.set_mvex_wig( Value );

    Box::into_raw( obj );

    return true;
}

// Gets the bitness ( 16, 32 or 64 )
#[no_mangle]
pub unsafe extern "C" fn Encoder_GetBitness( Encoder: *mut Encoder ) -> u32 {
    if Encoder.is_null() {
        return 0;
    }
    let obj = Box::from_raw( Encoder );

    let value = obj.bitness();

    Box::into_raw( obj );

    return value;
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// BlockEncoder

// Encodes instructions. Any number of branches can be part of this block.
// You can use this function to move instructions from one location to another location.
// If the target of a branch is too far away, it'll be rewritten to a longer branch.
// You can disable this by passing in [`BlockEncoderOptions::DONT_FIX_BRANCHES`].
// If the block has any `RIP`-relative memory operands, make sure the data isn't too
// far away from the new location of the encoded instructions. Every OS should have
// some API to allocate memory close (+/-2GB) to the original code location.
//
// # Errors
// Returns 0-Data if it failed to encode one or more instructions.
//
// # Arguments 
// * `bitness`: 16, 32, or 64
// * `Intructions`: First Instruction to encode
// * `Count`: Instruction-Count
// * `Options`: Encoder options, see [`TBlockEncoderOptions`]
// * `Results`: Result-Structure
//
// # Result
// * Pointer to Result-Data. Musst be free'd using RustFreeMemory()
#[repr(C)]
pub struct TBlockEncoderResult {
	/// Base IP of all encoded instructions
	pub rip: u64,

	/// The bytes of all encoded instructions
	pub code_buffer: *const u8,
    pub code_buffer_len: usize,

	/// If [`BlockEncoderOptions::RETURN_RELOC_INFOS`] option was enabled:
	///
	/// All [`RelocInfo`]s.
	///
	/// [`BlockEncoderOptions::RETURN_RELOC_INFOS`]: struct.BlockEncoderOptions.html#associatedconstant.RETURN_RELOC_INFOS
	/// [`RelocInfo`]: struct.RelocInfo.html
	pub reloc_infos: *const RelocInfo,
    pub reloc_infos_len: usize,

	/// If [`BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS`] option was enabled:
	///
	/// Offsets of the instructions relative to the base IP. If the instruction was rewritten to a new instruction
	/// (eg. `JE TARGET_TOO_FAR_AWAY` -> `JNE SHORT SKIP ;JMP QWORD PTR [MEM]`), the value `u32::MAX` is stored in that element.
	///
	/// [`BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS`]: struct.BlockEncoderOptions.html#associatedconstant.RETURN_NEW_INSTRUCTION_OFFSETS
	pub new_instruction_offsets: *const u32,
    pub new_instruction_offsets_len: usize,    

	/// If [`BlockEncoderOptions::RETURN_CONSTANT_OFFSETS`] option was enabled:
	///
	/// Offsets of all constants in the new encoded instructions. If the instruction was rewritten,
	/// the `default()` value is stored in the corresponding element.
	///
	/// [`BlockEncoderOptions::RETURN_CONSTANT_OFFSETS`]: struct.BlockEncoderOptions.html#associatedconstant.RETURN_CONSTANT_OFFSETS
	pub constant_offsets: *const ConstantOffsets,
    pub constant_offsets_len: usize,    
}

#[no_mangle]
pub unsafe extern "C" fn BlockEncoder( Bitness: u32, RIP : u64, Instructions: *mut Instruction, Count: usize, Result: *mut TBlockEncoderResult, Options: u32 ) -> *mut BlockEncoderResult { 
    if Instructions.is_null() {
        return null_mut();
    }
    if Count <= 0 {
        return null_mut();
    }
    if Result.is_null() {
        return null_mut();
    }

    let instructions = slice::from_raw_parts( Instructions, Count );
    let block = InstructionBlock::new( &instructions, RIP );
    match BlockEncoder::encode( Bitness, block, Options ) {
        Ok( value ) => {
            (*Result).rip = value.rip;

            if value.code_buffer.len() > 0 {
                (*Result).code_buffer = value.code_buffer.as_ptr();
            }else {
                (*Result).code_buffer = null_mut();
            }
            (*Result).code_buffer_len = value.code_buffer.len();

            if value.reloc_infos.len() > 0 {
                (*Result).reloc_infos = value.reloc_infos.as_ptr();
            }else {
                (*Result).reloc_infos = null_mut();
            }
            (*Result).reloc_infos_len = value.reloc_infos.len();

            if value.new_instruction_offsets.len() > 0 {
                (*Result).new_instruction_offsets = value.new_instruction_offsets.as_ptr();
            }else {
                (*Result).new_instruction_offsets = null_mut();
            }
            (*Result).new_instruction_offsets_len = value.new_instruction_offsets.len();

            if value.constant_offsets.len() > 0 {
                (*Result).constant_offsets = value.constant_offsets.as_ptr();
            }else {
                (*Result).constant_offsets = null_mut();
            }
            (*Result).constant_offsets_len = value.constant_offsets.len();
            return Box::into_raw( Box::new( value ) );
        },
        Err( _e ) => {
            (*Result).rip = 0 as u64;
            (*Result).code_buffer = null_mut();
            (*Result).code_buffer_len = 0;
            (*Result).reloc_infos = null_mut();
            (*Result).reloc_infos_len = 0;
            (*Result).new_instruction_offsets = null_mut();
            (*Result).new_instruction_offsets_len = 0;
            (*Result).constant_offsets = null_mut();
            (*Result).constant_offsets_len = 0;
            return null_mut();
        }
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Instruction

// Gets the FPU status word's `TOP` increment and whether it's a conditional or unconditional push/pop
// and whether `TOP` is written.
#[no_mangle]
pub unsafe extern "C" fn Instruction_FPU_StackIncrementInfo( Instruction: *mut Instruction, Info: *mut FpuStackIncrementInfo ) -> bool { 
    if Instruction.is_null() {
        return false;
    }
    if Info.is_null() {
        return false;
    }
    *Info = (*Instruction).fpu_stack_increment_info();

    return true;
}

// Instruction encoding, eg. Legacy, 3DNow!, VEX, EVEX, XOP
#[no_mangle]
pub unsafe extern "C" fn Instruction_Encoding( Instruction: *mut Instruction ) -> u32 { // FFI-Unsafe: EncodingKind 
    if Instruction.is_null() {
        return 0;// EncodingKind::Legacy;
    }
    
    return transmute( (*Instruction).encoding() as u32 );
}

// Gets the mnemonic, see also [`code()`]
#[no_mangle]
pub unsafe extern "C" fn Instruction_Mnemonic( Instruction: *mut Instruction ) -> u32 { // FFI-Unsafe: Mnemonic
    if Instruction.is_null() {
        return 0;// Mnemonic::INVALID;
    }
    
    return transmute( (*Instruction).mnemonic() as u32 );
}

// `true` if this is an instruction that implicitly uses the stack pointer (`SP`/`ESP`/`RSP`), eg. `CALL`, `PUSH`, `POP`, `RET`, etc.
// See also [`stack_pointer_increment()`]
//
// [`stack_pointer_increment()`]: #method.stack_pointer_increment
#[no_mangle]
pub unsafe extern "C" fn Instruction_IsStackInstruction( Instruction: *mut Instruction ) -> bool { 
    if Instruction.is_null() {
        return false;
    }

    return (*Instruction).is_stack_instruction();
}

// Gets the number of bytes added to `SP`/`ESP`/`RSP` or 0 if it's not an instruction that pushes or pops data. This method assumes
// the instruction doesn't change the privilege level (eg. `IRET/D/Q`). If it's the `LEAVE` instruction, this method returns 0.
#[no_mangle]
pub unsafe extern "C" fn Instruction_StackPointerIncrement( Instruction: *mut Instruction ) -> i32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).stack_pointer_increment();
}

// Gets the condition code if it's `Jcc`, `SETcc`, `CMOVcc`, `LOOPcc` else [`ConditionCode::None`] is returned
//
// [`ConditionCode::None`]: enum.ConditionCode.html#variant.None
#[no_mangle]
pub unsafe extern "C" fn Instruction_ConditionCode( Instruction: *mut Instruction ) -> u32 { // FFI-Unsafe: ConditionCode
    if Instruction.is_null() {
        return 0;// ConditionCode::None;
    }

    return transmute( (*Instruction).condition_code() as u32 );
}

// All flags that are read by the CPU when executing the instruction.
// This method returns an [`RflagsBits`] value. See also [`rflags_modified()`].
#[no_mangle]
pub unsafe extern "C" fn Instruction_RFlagsRead( Instruction: *mut Instruction ) -> u32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).rflags_read();
}

// All flags that are written by the CPU, except those flags that are known to be undefined, always set or always cleared.
// This method returns an [`RflagsBits`] value. See also [`rflags_modified()`].
#[no_mangle]
pub unsafe extern "C" fn Instruction_RFlagsWritten( Instruction: *mut Instruction ) -> u32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).rflags_written();
}

// All flags that are always cleared by the CPU.
// This method returns an [`RflagsBits`] value. See also [`rflags_modified()`].
#[no_mangle]
pub unsafe extern "C" fn Instruction_RFlagsCleared( Instruction: *mut Instruction ) -> u32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).rflags_cleared();
}

// All flags that are always set by the CPU.
// This method returns an [`RflagsBits`] value. See also [`rflags_modified()`].
#[no_mangle]
pub unsafe extern "C" fn Instruction_RFlagsSet( Instruction: *mut Instruction ) -> u32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).rflags_set();
}

// All flags that are undefined after executing the instruction.
// This method returns an [`RflagsBits`] value. See also [`rflags_modified()`].
#[no_mangle]
pub unsafe extern "C" fn Instruction_RFlagsUndefined( Instruction: *mut Instruction ) -> u32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).rflags_undefined();
}

// All flags that are modified by the CPU. This is `rflags_written() + rflags_cleared() + rflags_set() + rflags_undefined()`. This method returns an [`RflagsBits`] value.
#[no_mangle]
pub unsafe extern "C" fn Instruction_RFlagsModified( Instruction: *mut Instruction ) -> u32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).rflags_modified();
}

// Control flow info
#[no_mangle]
pub unsafe extern "C" fn Instruction_FlowControl( Instruction: *mut Instruction ) -> u32 { // FFI-Unsafe: FlowControl
    if Instruction.is_null() {
        return 0;// FlowControl::Next;
    }

    return transmute( (*Instruction).flow_control() as u32 );
}

// Gets the CPU or CPUID feature flags
#[allow( non_upper_case_globals )]
const CPUIDFeaturesMaxEntries : usize = 5;
#[repr(C)]
pub struct TCPUIDFeaturesArray { 
    Entries : [CpuidFeature;CPUIDFeaturesMaxEntries], 
    Count : u8
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_CPUIDFeatures( Instruction: *mut Instruction, CPUIDFeatures : *mut TCPUIDFeaturesArray ) -> bool { 
    if Instruction.is_null() {
        return false;
    }
    if CPUIDFeatures.is_null() {
        return false;
    }

    let cpuidfeaturesA = (*Instruction).cpuid_features();

    (*CPUIDFeatures).Count = cpuidfeaturesA.len() as u8;
    for ( i, x ) in cpuidfeaturesA.iter().enumerate() {
        if i < (*CPUIDFeatures).Entries.len() {
            (*CPUIDFeatures).Entries[ i ] = *x;
        }
    }

    return true;
}

// Gets all op kinds ([`op_count()`] values)
#[allow( non_upper_case_globals )]
const OPKindsMaxEntries : usize = 5;
#[repr(C)]
pub struct TOPKindsArray { 
    Entries : [OpKind;OPKindsMaxEntries], 
    Count : u8
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_OPKinds( Instruction: *mut Instruction, OPKinds : *mut TOPKindsArray ) -> bool { 
    if Instruction.is_null() {
        return false;
    }
    if OPKinds.is_null() {
        return false;
    }

    let opkindsA = (*Instruction).op_kinds();

    (*OPKinds).Count = opkindsA.len() as u8;
    for ( i, x ) in opkindsA.enumerate() {
        if i < (*OPKinds).Entries.len() {
            (*OPKinds).Entries[ i ] = x;
        }
    }

    return true;
}

// Gets the size of the memory location that is referenced by the operand. See also [`is_broadcast()`].
// Use this method if the operand has kind [`OpKind::Memory`],
#[no_mangle]
pub unsafe extern "C" fn Instruction_MemorySize( Instruction: *mut Instruction ) -> u8 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).memory_size().size() as u8;
}

// Gets the operand count. An instruction can have 0-5 operands.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OPCount( Instruction: *mut Instruction ) -> u32 { 
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).op_count();
}

// Gets the code
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Code( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: Code
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).op_code().code() as u32;
}

// Gets the mnemonic
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Mnemonic( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: Mnemonic
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).op_code().mnemonic() as u32;
}

// Gets the encoding
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Encoding( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: Encoding
    if Instruction.is_null() {
        return 0;
    }

    return (*Instruction).op_code().encoding() as u32;
}

// `true` if it's an instruction, `false` if it's eg. [`Code::INVALID`], [`db`], [`dw`], [`dd`], [`dq`], [`zero_bytes`]
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsInstruction( Instruction: *mut Instruction ) -> bool { 
    if Instruction.is_null() {
        return false;
    }

    return (*Instruction).op_code().is_instruction();
}

// `true` if it's an instruction available in 16-bit mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Mode16( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().mode16();
}

// `true` if it's an instruction available in 32-bit mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Mode32( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().mode32();
}

// `true` if it's an instruction available in 64-bit mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Mode64( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().mode64();
}

// `true` if an `FWAIT` (`9B`) instruction is added before the instruction
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Fwait( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().fwait();
}

// (Legacy encoding) Gets the required operand size (16,32,64) or 0
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OperandSize( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().operand_size();
}

// (Legacy encoding) Gets the required address size (16,32,64) or 0
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_AddressSize( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().address_size();
}

// (VEX/XOP/EVEX) `L` / `L'L` value or default value if [`is_lig()`] is `true`
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_L( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().l();
}

// (VEX/XOP/EVEX/MVEX) `W` value or default value if [`is_wig()`] or [`is_wig32()`] is `true`
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_W( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().w();
}

// (VEX/XOP/EVEX) `true` if the `L` / `L'L` fields are ignored.
//
// EVEX: if reg-only ops and `{er}` (`EVEX.b` is set), `L'L` is the rounding control and not ignored.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsLig( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_lig();
}

// (VEX/XOP/EVEX/MVEX) `true` if the `W` field is ignored in 16/32/64-bit modes
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsWig( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_wig();
}

// (VEX/XOP/EVEX/MVEX) `true` if the `W` field is ignored in 16/32-bit modes (but not 64-bit mode)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsWig32( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_wig32();
}

// (EVEX/MVEX) Gets the tuple type
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_TupleType( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: TupleType
  if Instruction.is_null() {
      return 0;// TupleType::N1;
  }

  return (*Instruction).op_code().tuple_type() as u32;
}

// (MVEX) Gets the `EH` bit that's required to encode this instruction
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexEhBit( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: MvexEHBit
  if Instruction.is_null() {
      return 0;// MvexEHBit::None;
  }

  return (*Instruction).op_code().mvex_eh_bit() as u32;
}

// (MVEX) `true` if the instruction supports eviction hint (if it has a memory operand)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexCanUseEvictionHint( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().mvex_can_use_eviction_hint();
}

// (MVEX) `true` if the instruction's rounding control bits are stored in `imm8[1:0]`
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexCanUseImmRoundingControl( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().mvex_can_use_imm_rounding_control();
}

// (MVEX) `true` if the instruction ignores op mask registers (eg. `{k1}`)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexIgnoresOpMaskRegister( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().mvex_ignores_op_mask_register();
}

// (MVEX) `true` if the instruction must have `MVEX.SSS=000` if `MVEX.EH=1`
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexNoSaeRc( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().mvex_no_sae_rc();
}

// (MVEX) Gets the tuple type / conv lut kind
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexTupleTypeLutKind( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: MvexTupleTypeLutKind
  if Instruction.is_null() {
      return 0;// MvexTupleTypeLutKind::Int32;
  }

  return (*Instruction).op_code().mvex_tuple_type_lut_kind() as u32;
}

// (MVEX) Gets the conversion function, eg. `Sf32`
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexConversionFunc( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: MvexConvFn
  if Instruction.is_null() {
      return 0;// MvexConvFn::None;
  }

  return (*Instruction).op_code().mvex_conversion_func() as u32;
}

// (MVEX) Gets flags indicating which conversion functions are valid (bit 0 == func 0)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexValidConversionFuncsMask( Instruction: *mut Instruction ) -> u8 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().mvex_valid_conversion_funcs_mask();
}

// (MVEX) Gets flags indicating which swizzle functions are valid (bit 0 == func 0)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MvexValidSwizzleFuncsMask( Instruction: *mut Instruction ) -> u8 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().mvex_valid_swizzle_funcs_mask();
}

// If it has a memory operand, gets the [`MemorySize`] (non-broadcast memory type)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MemorySize( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: MemorySize
  if Instruction.is_null() {
      return 0;// MemorySize::Unknown;
  }

  return (*Instruction).op_code().memory_size() as u32;
}

// If it has a memory operand, gets the [`MemorySize`] (broadcast memory type)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_BroadcastMemorySize( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: MemorySize
  if Instruction.is_null() {
      return 0;// MemorySize::Unknown;
  }

  return (*Instruction).op_code().broadcast_memory_size() as u32;
}

// (EVEX) `true` if the instruction supports broadcasting (`EVEX.b` bit) (if it has a memory operand)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanBroadcast( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_broadcast();
}

// (EVEX/MVEX) `true` if the instruction supports rounding control
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseRoundingControl( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_rounding_control();
}

// (EVEX/MVEX) `true` if the instruction supports suppress all exceptions
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanSuppressAllExceptions( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_suppress_all_exceptions();
}

// (EVEX/MVEX) `true` if an opmask register can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseOpMaskRegister( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_op_mask_register();
}

// (EVEX/MVEX) `true` if a non-zero opmask register must be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_RequireOpMaskRegister( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().require_op_mask_register();
}

// (EVEX) `true` if the instruction supports zeroing masking (if one of the opmask registers `K1`-`K7` is used and destination operand is not a memory operand)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseZeroingMasking( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_zeroing_masking();
}

// `true` if the `LOCK` (`F0`) prefix can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseLockPrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_lock_prefix();
}

// `true` if the `XACQUIRE` (`F2`) prefix can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseXacquirePrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_xacquire_prefix();
}

// `true` if the `XRELEASE` (`F3`) prefix can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseXreleasePrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_xrelease_prefix();
}

// `true` if the `REP` / `REPE` (`F3`) prefixes can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseRepPrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_rep_prefix();
}

// `true` if the `REPNE` (`F2`) prefix can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseRepnePrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_repne_prefix();
}

// `true` if the `BND` (`F2`) prefix can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseBndPrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_bnd_prefix();
}

// `true` if the `HINT-TAKEN` (`3E`) and `HINT-NOT-TAKEN` (`2E`) prefixes can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseHintTakenPrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_hint_taken_prefix();
}

// `true` if the `NOTRACK` (`3E`) prefix can be used
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CanUseNotrackPrefix( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().can_use_notrack_prefix();
}

// `true` if rounding control is ignored (#UD is not generated)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IgnoresRoundingControl( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().ignores_rounding_control();
}

// `true` if the `LOCK` prefix can be used as an extra register bit (bit 3) to access registers 8-15 without a `REX` prefix (eg. in 32-bit mode)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_AmdLockRegBit( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().amd_lock_reg_bit();
}

// `true` if the default operand size is 64 in 64-bit mode. A `66` prefix can switch to 16-bit operand size.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_DefaultOpSize64( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().default_op_size64();
}

// `true` if the operand size is always 64 in 64-bit mode. A `66` prefix is ignored.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_ForceOpSize64( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().force_op_size64();
}

// `true` if the Intel decoder forces 64-bit operand size. A `66` prefix is ignored.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IntelForceOpSize64( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().intel_force_op_size64();
}

// `true` if it can only be executed when CPL=0
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MustBeCpl0( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().must_be_cpl0();
}

// `true` if it can be executed when CPL=0
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Cpl0( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().cpl0();
}

// `true` if it can be executed when CPL=1
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Cpl1( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().cpl1();
}

// `true` if it can be executed when CPL=2
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Cpl2( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().cpl2();
}

// `true` if it can be executed when CPL=3
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Cpl3( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().cpl3();
}

// `true` if the instruction accesses the I/O address space (eg. `IN`, `OUT`, `INS`, `OUTS`)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsInputOutput( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_input_output();
}

// `true` if it's one of the many nop instructions (does not include FPU nop instructions, eg. `FNOP`)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsNop( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_nop();
}

// `true` if it's one of the many reserved nop instructions (eg. `0F0D`, `0F18-0F1F`)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsReservedNop( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_reserved_nop();
}

// `true` if it's a serializing instruction (Intel CPUs)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsSerializingIntel( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_serializing_intel();
}

// `true` if it's a serializing instruction (AMD CPUs)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsSerializingAmd( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_serializing_amd();
}

// `true` if the instruction requires either CPL=0 or CPL<=3 depending on some CPU option (eg. `CR4.TSD`, `CR4.PCE`, `CR4.UMIP`)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MayRequireCpl0( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().may_require_cpl0();
}

// `true` if it's a tracked `JMP`/`CALL` indirect instruction (CET)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsCetTracked( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_cet_tracked();
}

// `true` if it's a non-temporal hint memory access (eg. `MOVNTDQ`)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsNonTemporal( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_non_temporal();
}

// `true` if it's a no-wait FPU instruction, eg. `FNINIT`
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsFpuNoWait( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_fpu_no_wait();
}

// `true` if the mod bits are ignored and it's assumed `modrm[7:6] == 11b`
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IgnoresModBits( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().ignores_mod_bits();
}

// `true` if the `66` prefix is not allowed (it will #UD)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_No66( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().no66();
}

// `true` if the `F2`/`F3` prefixes aren't allowed
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Nfx( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().nfx();
}

// `true` if the index reg's reg-num (vsib op) (if any) and register ops' reg-nums must be unique,
// eg. `MNEMONIC XMM1,YMM1,[RAX+ZMM1*2]` is invalid. Registers = `XMM`/`YMM`/`ZMM`/`TMM`.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_RequiresUniqueRegNums( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().requires_unique_reg_nums();
}

// `true` if the destination register's reg-num must not be present in any other operand, eg. `MNEMONIC XMM1,YMM1,[RAX+ZMM1*2]`
// is invalid. Registers = `XMM`/`YMM`/`ZMM`/`TMM`.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_RequiresUniqueDestRegNum( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().requires_unique_dest_reg_num();
}

// `true` if it's a privileged instruction (all CPL=0 instructions (except `VMCALL`) and IOPL instructions `IN`, `INS`, `OUT`, `OUTS`, `CLI`, `STI`)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsPrivileged( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_privileged();
}

// `true` if it reads/writes too many registers
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsSaveRestore( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_save_restore();
}

// `true` if it's an instruction that implicitly uses the stack register, eg. `CALL`, `POP`, etc
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsStackInstruction( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_stack_instruction();
}

// `true` if the instruction doesn't read the segment register if it uses a memory operand
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IgnoresSegment( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().ignores_segment();
}

// `true` if the opmask register is read and written (instead of just read). This also implies that it can't be `K0`.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsOpMaskReadWrite( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_op_mask_read_write();
}

// `true` if it can be executed in real mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_RealMode( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().real_mode();
}

// `true` if it can be executed in protected mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_ProtectedMode( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().protected_mode();
}

// `true` if it can be executed in virtual 8086 mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Virtual8086Mode( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().virtual8086_mode();
}

// `true` if it can be executed in compatibility mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_CompatibilityMode( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().compatibility_mode();
}

// `true` if it can be executed in 64-bit mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_LongMode( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().long_mode();
}

// `true` if it can be used outside SMM
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseOutsideSmm( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_outside_smm();
}

// `true` if it can be used in SMM
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseInSmm( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_in_smm();
}

// `true` if it can be used outside an enclave (SGX)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseOutsideEnclaveSgx( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_outside_enclave_sgx();
}

// `true` if it can be used inside an enclave (SGX1)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseInEnclaveSgx1( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_in_enclave_sgx1();
}

// `true` if it can be used inside an enclave (SGX2)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseInEnclaveSgx2( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_in_enclave_sgx2();
}

// `true` if it can be used outside VMX operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseOutsideVmxOp( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_outside_vmx_op();
}

// `true` if it can be used in VMX root operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseInVmxRootOp( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_in_vmx_root_op();
}

// `true` if it can be used in VMX non-root operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseInVmxNonRootOp( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_in_vmx_non_root_op();
}

// `true` if it can be used outside SEAM
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseOutsideSeam( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_outside_seam();
}

// `true` if it can be used in SEAM
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_UseInSeam( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().use_in_seam();
}

// `true` if #UD is generated in TDX non-root operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_TdxNonRootGenUd( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().tdx_non_root_gen_ud();
}

// `true` if #VE is generated in TDX non-root operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_TdxNonRootGenVe( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().tdx_non_root_gen_ve();
}

// `true` if an exception (eg. #GP(0), #VE) may be generated in TDX non-root operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_TdxNonRootMayGenEx( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().tdx_non_root_may_gen_ex();
}

// (Intel VMX) `true` if it causes a VM exit in VMX non-root operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IntelVMExit( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().intel_vm_exit();
}

// (Intel VMX) `true` if it may cause a VM exit in VMX non-root operation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IntelMayVMExit( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().intel_may_vm_exit();
}

// (Intel VMX) `true` if it causes an SMM VM exit in VMX root operation (if dual-monitor treatment is activated)
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IntelSmmVMExit( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().intel_smm_vm_exit();
}

// (AMD SVM) `true` if it causes a #VMEXIT in guest mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_AmdVMExit( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().amd_vm_exit();
}

// (AMD SVM) `true` if it may cause a #VMEXIT in guest mode
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_AmdMayVMExit( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().amd_may_vm_exit();
}

// `true` if it causes a TSX abort inside a TSX transaction
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_TsxAbort( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().tsx_abort();
}

// `true` if it causes a TSX abort inside a TSX transaction depending on the implementation
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_TsxImplAbort( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().tsx_impl_abort();
}

// `true` if it may cause a TSX abort inside a TSX transaction depending on some condition
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_TsxMayAbort( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().tsx_may_abort();
}

// `true` if it's decoded by iced's 16-bit Intel decoder
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IntelDecoder16( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().intel_decoder16();
}

// `true` if it's decoded by iced's 32-bit Intel decoder
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IntelDecoder32( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().intel_decoder32();
}

// `true` if it's decoded by iced's 64-bit Intel decoder
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IntelDecoder64( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().intel_decoder64();
}

// `true` if it's decoded by iced's 16-bit AMD decoder
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_AmdDecoder16( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().amd_decoder16();
}

// `true` if it's decoded by iced's 32-bit AMD decoder
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_AmdDecoder32( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().amd_decoder32();
}

// `true` if it's decoded by iced's 64-bit AMD decoder
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_AmdDecoder64( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().amd_decoder64();
}

// Gets the decoder option that's needed to decode the instruction or [`DecoderOptions::NONE`].
// The return value is a [`DecoderOptions`] value.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_DecoderOption( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().decoder_option();
}

// Gets the opcode table
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_Table( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: OpCodeTableKind 
  if Instruction.is_null() {
      return 0;// OpCodeTableKind::Normal;
  }

  return (*Instruction).op_code().table() as u32;
}

// Gets the mandatory prefix
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_MandatoryPrefix( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: MandatoryPrefix 
  if Instruction.is_null() {
      return 0;// MandatoryPrefix::None;
  }

  return (*Instruction).op_code().mandatory_prefix() as u32;
}

// Gets the opcode byte(s). The low byte(s) of this value is the opcode. The length is in [`op_code_len()`].
// It doesn't include the table value, see [`table()`].
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OpCode( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().op_code();
}

// Gets the length of the opcode bytes ([`op_code()`]). The low bytes is the opcode value.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OpCodeLen( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().op_code_len();
}

// `true` if it's part of a group
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsGroup( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_group();
}

// Group index (0-7) or -1. If it's 0-7, it's stored in the `reg` field of the `modrm` byte.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_GroupIndex( Instruction: *mut Instruction ) -> i32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().group_index();
}

// `true` if it's part of a modrm.rm group
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsRMGroup( Instruction: *mut Instruction ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_rm_group();
}

// Group index (0-7) or -1. If it's 0-7, it's stored in the `rm` field of the `modrm` byte.
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_RMGroupIndex( Instruction: *mut Instruction ) -> i32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().rm_group_index();
}

// Gets the number of operands
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OPCount( Instruction: *mut Instruction ) -> u32 {
  if Instruction.is_null() {
      return 0;
  }

  return (*Instruction).op_code().op_count();
}

// Gets operand #0's opkind
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OP0Kind( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: OpCodeOperandKind 
  if Instruction.is_null() {
      return 0;// OpCodeOperandKind::None;
  }

  return (*Instruction).op_code().op0_kind() as u32;
}

// Gets operand #1's opkind
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OP1Kind( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: OpCodeOperandKind 
  if Instruction.is_null() {
      return 0;// OpCodeOperandKind::None;
  }

  return (*Instruction).op_code().op1_kind() as u32;
}

// Gets operand #2's opkind
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OP2Kind( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: OpCodeOperandKind 
  if Instruction.is_null() {
      return 0;// OpCodeOperandKind::None;
  }

  return (*Instruction).op_code().op2_kind() as u32;
}

// Gets operand #3's opkind
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OP3Kind( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: OpCodeOperandKind 
  if Instruction.is_null() {
      return 0;// OpCodeOperandKind::None;
  }

  return (*Instruction).op_code().op3_kind() as u32;
}

// Gets operand #4's opkind
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OP4Kind( Instruction: *mut Instruction ) -> u32 { // FFI Unsafe: OpCodeOperandKind 
  if Instruction.is_null() {
      return 0;// OpCodeOperandKind::None;
  }

  return (*Instruction).op_code().op4_kind() as u32;
}

// Gets an operand's opkind
//
// # Arguments
//
// * `operand`: Operand number, 0-4
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OPKind( Instruction: *mut Instruction, operand: u32 ) -> u32 { // FFI Unsafe: OpCodeOperandKind
  if Instruction.is_null() {
      return 0;// OpCodeOperandKind::None;
  }

  return (*Instruction).op_code().op_kind( operand ) as u32;
}

// Gets all operand kinds
#[allow( non_upper_case_globals )]
const TOPCodeOperandKindArrayMaxEntries : usize = 5;
#[repr(C)]
pub struct TOPCodeOperandKindArray { 
    Entries : [OpCodeOperandKind;TOPCodeOperandKindArrayMaxEntries], 
    Count : u8
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OPKinds( Instruction: *mut Instruction, OPKinds : *mut TOPCodeOperandKindArray ) -> bool { 
    if Instruction.is_null() {
        return false;
    }
    if OPKinds.is_null() {
        return false;
    }

    let opkindsA = (*Instruction).op_code().op_kinds();

    (*OPKinds).Count = opkindsA.len() as u8;
    for ( i, x ) in opkindsA.iter().enumerate() {
        if i < (*OPKinds).Entries.len() {
            (*OPKinds).Entries[ i ] = *x;
        }
    }

    return true;
}

// Checks if the instruction is available in 16-bit mode, 32-bit mode or 64-bit mode
//
// # Arguments
//
// * `bitness`: 16, 32 or 64
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_IsAvailableInMode( Instruction: *mut Instruction, Bitness: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  return (*Instruction).op_code().is_available_in_mode( Bitness );
}

// Gets the opcode string, eg. `VEX.128.66.0F38.W0 78 /r`, see also [`instruction_string()`]
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_OpCodeString( Instruction: *mut Instruction, Output : *mut u8, Size : usize ) -> bool {
  if Instruction.is_null() {
      return false;
  }
  if Output.is_null() {
      return false;
  }
  if Size <= 0 {
      return false;
  }

  let output = (*Instruction).op_code().op_code_string();

  let mut l = output.len();
  if l > Size {
      l = Size;
  }

  if l > 0 {
      for i in 0..l {
          *( Output.add( i ) ) = output.as_bytes()[ i ];
      
      }
  }
  *( Output.add( l ) ) = 0;

  return true;
}

// Gets the instruction string, eg. `VPBROADCASTB xmm1, xmm2/m8`, see also [`op_code_string()`]
#[no_mangle]
pub unsafe extern "C" fn Instruction_OpCodeInfo_InstructionString( Instruction: *mut Instruction, Output : *mut u8, Size : usize ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  if Output.is_null() {
    return false;
  }
  if Size <= 0 {
    return false;
  }

  let output = (*Instruction).op_code().instruction_string();

  let mut l = output.len();
  if l > Size {
    l = Size;
  }

  if l > 0 {
    for i in 0..l {
        *( Output.add( i ) ) = output.as_bytes()[ i ];
    
    }
  }
  *( Output.add( l ) ) = 0;

  return true;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Instruction 'WITH'
// Creates an instruction with no operands
#[no_mangle]
pub unsafe extern "C" fn Instruction_With( Instruction : *mut Instruction, Code : u16 ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  (*Instruction) = Instruction::with( transmute( Code as u16 ) );
  return true;
}

// Creates an instruction with 1 operand
//
// # Errors
// Fails if one of the operands is invalid (basic checks)
#[no_mangle]
pub unsafe extern "C" fn Instruction_With1_Register( Instruction : *mut Instruction, Code : u16, Register : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register );
  match Instruction::with1( transmute( Code as u16 ), register ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With1_i32( Instruction : *mut Instruction, Code : u16, Immediate : i32 ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with1( transmute( Code as u16 ), Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With1_u32( Instruction : *mut Instruction, Code : u16, Immediate : u32 ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with1( transmute( Code as u16 ), Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[repr(C)]
pub struct TMemoryOperand {
	/// Segment override or [`Register::None`]
	///
	/// [`Register::None`]: enum.Register.html#variant.None
	pub segment_prefix: u8, // FFI-Unsafe: Register

	/// Base register or [`Register::None`]
	///
	/// [`Register::None`]: enum.Register.html#variant.None
	pub base: u8, // FFI-Unsafe: Register

	/// Index register or [`Register::None`]
	///
	/// [`Register::None`]: enum.Register.html#variant.None
	pub index: u8, // FFI-Unsafe: Register

	/// Index register scale (1, 2, 4, or 8)
	pub scale: u32,

	/// Memory displacement
	pub displacement: i64,

	/// 0 (no displ), 1 (16/32/64-bit, but use 2/4/8 if it doesn't fit in a `i8`), 2 (16-bit), 4 (32-bit) or 8 (64-bit)
	pub displ_size: u32,

	/// `true` if it's broadcast memory (EVEX instructions)
	pub is_broadcast: bool,
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With1_Memory( Instruction : *mut Instruction, Code : u16, Memory : *mut TMemoryOperand ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };

  match Instruction::with1( transmute( Code as u16 ), memory ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_Register_Register( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  match Instruction::with2( transmute( Code as u16 ), register1, register2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_Register_i32( Instruction : *mut Instruction, Code : u16, Register : u8, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register );
  match Instruction::with2( transmute( Code as u16 ), register, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_Register_u32( Instruction : *mut Instruction, Code : u16, Register : u8, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register );
  match Instruction::with2( transmute( Code as u16 ), register, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_Register_i64( Instruction : *mut Instruction, Code : u16, Register : u8, Immediate : i64 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register );
  match Instruction::with2( transmute( Code as u16 ), register, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_Register_u64( Instruction : *mut Instruction, Code : u16, Register : u8, Immediate : u64 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register );
  match Instruction::with2( transmute( Code as u16 ), register, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_Register_MemoryOperand( Instruction : *mut Instruction, Code : u16, Register : u8, Memory : *mut TMemoryOperand ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register as u8 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };

  match Instruction::with2( transmute( Code as u16 ), register, memory ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_i32_Register( Instruction : *mut Instruction, Code : u16, Immediate : i32, Register : u8 ) -> bool { // FFI-Unsafe: Code, Register
    if Instruction.is_null() {
        return false;
    }
  
    let register: Register = transmute( Register );
    match Instruction::with2( transmute( Code as u16 ), Immediate, register ) {
      Err( _e ) => return false,
      Ok( instruction ) => { 
          (*Instruction) = instruction;
          return true
      }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_u32_Register( Instruction : *mut Instruction, Code : u16, Immediate : u32, Register : u8 ) -> bool { // FFI-Unsafe: Code, Register
    if Instruction.is_null() {
        return false;
    }
  
    let register: Register = transmute( Register );
    match Instruction::with2( transmute( Code as u16 ), Immediate, register ) {
      Err( _e ) => return false,
      Ok( instruction ) => { 
          (*Instruction) = instruction;
          return true
      }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_i32_i32( Instruction : *mut Instruction, Code : u16, Immediate1 : i32, Immediate2 : i32 ) -> bool { // FFI-Unsafe: Code
    if Instruction.is_null() {
        return false;
    }
  
    match Instruction::with2( transmute( Code as u16 ), Immediate1, Immediate2 ) {
      Err( _e ) => return false,
      Ok( instruction ) => { 
          (*Instruction) = instruction;
          return true
      }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_u32_u32( Instruction : *mut Instruction, Code : u16, Immediate1 : u32, Immediate2 : u32 ) -> bool { // FFI-Unsafe: Code
    if Instruction.is_null() {
        return false;
    }
  
    match Instruction::with2( transmute( Code as u16 ), Immediate1, Immediate2 ) {
      Err( _e ) => return false,
      Ok( instruction ) => { 
          (*Instruction) = instruction;
          return true
      }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_MemoryOperand_Register( Instruction : *mut Instruction, Code : u16, Memory : *mut TMemoryOperand, Register : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
  let register: Register = transmute( Register as u8 );

  match Instruction::with2( transmute( Code as u16 ), memory, register ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_MemoryOperand_i32( Instruction : *mut Instruction, Code : u16, Memory : *mut TMemoryOperand, Immediate : i32 ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };

  match Instruction::with2( transmute( Code as u16 ), memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With2_MemoryOperand_u32( Instruction : *mut Instruction, Code : u16, Memory : *mut TMemoryOperand, Immediate : u32 ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };

  match Instruction::with2( transmute( Code as u16 ), memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_Register_Register( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
  
  match Instruction::with3( transmute( Code as u16 ), register1, register2, register3 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_Register_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  
  match Instruction::with3( transmute( Code as u16 ), register1, register2, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_Register_u32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  
  match Instruction::with3( transmute( Code as u16 ), register1, register2, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_Register_MemoryOperand( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Memory : *mut TMemoryOperand ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
  
  match Instruction::with3( transmute( Code as u16 ), register1, register2, memory ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_i32_i32( Instruction : *mut Instruction, Code : u16, Register : u8, Immediate1 : i32, Immediate2 : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register );

  match Instruction::with3( transmute( Code as u16 ), register, Immediate1, Immediate2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_u32_u32( Instruction : *mut Instruction, Code : u16, Register : u8, Immediate1 : u32, Immediate2 : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register as u8 );
    
  match Instruction::with3( transmute( Code as u16 ), register, Immediate1, Immediate2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_MemoryOperand_Register( Instruction : *mut Instruction, Code : u16, Register1 : u8, Memory : *mut TMemoryOperand, Register2 : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
  let register2: Register = transmute( Register2 );
    
  match Instruction::with3( transmute( Code as u16 ), register1, memory, register2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_MemoryOperand_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Memory : *mut TMemoryOperand, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
    
  match Instruction::with3( transmute( Code as u16 ), register1, memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_Register_MemoryOperand_u32( Instruction : *mut Instruction, Code : u16, Register : u8, Memory : *mut TMemoryOperand, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register: Register = transmute( Register as u8 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
    
  match Instruction::with3( transmute( Code as u16 ), register, memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_MemoryOperand_Register_Register( Instruction : *mut Instruction, Code : u16, Memory : *mut TMemoryOperand, Register1 : u8, Register2 : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
   let register1: Register = transmute( Register1 );
   let register2: Register = transmute( Register2 );
    
  match Instruction::with3( transmute( Code as u16 ), memory, register1, register2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_MemoryOperand_Register_i32( Instruction : *mut Instruction, Code : u16, Memory : *mut TMemoryOperand, Register : u8, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
   let register: Register = transmute( Register );
    
  match Instruction::with3( transmute( Code as u16 ), memory, register, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With3_MemoryOperand_Register_u32( Instruction : *mut Instruction, Code : u16, Memory : *mut TMemoryOperand, Register : u8, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
   let register: Register = transmute( Register );
    
  match Instruction::with3( transmute( Code as u16 ), memory, register, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_Register_Register( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Register4 : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
  let register4: Register = transmute( Register4 );
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, register3, register4 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_Register_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, register3, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_Register_u32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, register3, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_Register_MemoryOperand( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Memory : *mut TMemoryOperand ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, register3, memory ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_i32_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Immediate1 : i32, Immediate2 : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, Immediate1, Immediate2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_u32_u32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Immediate1 : u32, Immediate2 : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, Immediate1, Immediate2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_MemoryOperand_Register( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Memory : *mut TMemoryOperand, Register3 : u8 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
   let register3: Register = transmute( Register3 );
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, memory, register3 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_MemoryOperand_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Memory : *mut TMemoryOperand, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With4_Register_Register_MemoryOperand_u32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Memory : *mut TMemoryOperand, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
    
  match Instruction::with4( transmute( Code as u16 ), register1, register2, memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With5_Register_Register_Register_Register_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Register4 : u8, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
  let register4: Register = transmute( Register4 );

  match Instruction::with5( transmute( Code as u16 ), register1, register2, register3, register4, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With5_Register_Register_Register_Register_u32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Register4 : u8, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
  let register4: Register = transmute( Register4 );

  match Instruction::with5( transmute( Code as u16 ), register1, register2, register3, register4, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With5_Register_Register_Register_MemoryOperand_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Memory : *mut TMemoryOperand, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };

  match Instruction::with5( transmute( Code as u16 ), register1, register2, register3, memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With5_Register_Register_Register_MemoryOperand_u32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Register3 : u8, Memory : *mut TMemoryOperand, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let register3: Register = transmute( Register3 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };

  match Instruction::with5( transmute( Code as u16 ), register1, register2, register3, memory, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With5_Register_Register_MemoryOperand_Register_i32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Memory : *mut TMemoryOperand, Register3 : u8, Immediate : i32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
   let register3: Register = transmute( Register3 );

  match Instruction::with5( transmute( Code as u16 ), register1, register2, memory, register3, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With5_Register_Register_MemoryOperand_Register_u32( Instruction : *mut Instruction, Code : u16, Register1 : u8, Register2 : u8, Memory : *mut TMemoryOperand, Register3 : u8, Immediate : u32 ) -> bool { // FFI-Unsafe: Code, Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let memory = MemoryOperand { 
    segment_prefix: transmute( (*Memory).segment_prefix ),
    base: transmute( (*Memory).base ),
    index: transmute( (*Memory).index ),
    scale: (*Memory).scale,
    displacement: (*Memory).displacement,
    displ_size: (*Memory).displ_size,
    is_broadcast: (*Memory).is_broadcast
   };
   let register3: Register = transmute( Register3 );

  match Instruction::with5( transmute( Code as u16 ), register1, register2, memory, register3, Immediate ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Branch( Instruction : *mut Instruction, Code : u16, Target : u64 ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_branch( transmute( Code as u16 ), Target ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Far_Branch( Instruction : *mut Instruction, Code : u16, Selector : u16, Offset : u32 ) -> bool { // FFI-Unsafe: Code
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_far_branch( transmute( Code as u16 ), Selector, Offset ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_xbegin( Instruction : *mut Instruction, Bitness : u32, Target : u64 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_xbegin( Bitness, Target ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_outsb( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_outsb( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_outsb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_outsb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_outsw( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_outsw( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_outsw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_outsw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_outsd( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_outsd( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_outsd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_outsd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_lodsb( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_lodsb( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_lodsb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_lodsb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_lodsw( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_lodsw( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_lodsw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_lodsw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_lodsd( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_lodsd( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_lodsd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_lodsd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_lodsq( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_lodsq( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_lodsq( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_lodsq( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_scasb( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_scasb( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_scasb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_scasb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_scasb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_scasb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_scasw( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_scasw( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_scasw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_scasw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_scasw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_scasw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_scasd( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_scasd( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_scasd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_scasd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_scasd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_scasd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_scasq( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_scasq( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_scasq( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_scasq( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_scasq( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_scasq( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_insb( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_insb( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_insb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_insb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_insw( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_insw( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_insw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_insw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_insd( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_insd( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_insd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_insd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_stosb( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_stosb( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_stosb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_stosb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_stosw( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_stosw( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_stosw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_stosw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_stosd( Instruction : *mut Instruction, AddressSize: u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_stosd( AddressSize, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_stosd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_stosd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_stosq( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_stosq( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_cmpsb( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_cmpsb( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_cmpsb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_cmpsb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_cmpsb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_cmpsb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_cmpsw( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_cmpsw( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_cmpsw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_cmpsw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_cmpsw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_cmpsw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_cmpsd( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_cmpsd( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_cmpsd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_cmpsd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_cmpsd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_cmpsd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_cmpsq( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_cmpsq( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repe_cmpsq( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repe_cmpsq( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Repne_cmpsq( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_repne_cmpsq( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_movsb( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_movsb( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_movsb( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_movsb( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_movsw( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_movsw( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_movsw( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_movsw( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_movsd( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_movsd( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_movsd( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_movsd( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_movsq( Instruction : *mut Instruction, AddressSize: u32, SegmentPrefix : u32, RepPrefix: u32 ) -> bool { // FFI-Unsafe: Register, RepPrefixKind
  if Instruction.is_null() {
      return false;
  }

  let segmentprefix: Register = transmute( SegmentPrefix as u8 );
  let repprefix: RepPrefixKind = transmute( RepPrefix as u8 );

  match Instruction::with_movsq( AddressSize, segmentprefix, repprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Rep_movsq( Instruction : *mut Instruction, AddressSize: u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::with_rep_movsq( AddressSize ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_maskmovq( Instruction : *mut Instruction, AddressSize: u32, Register1 : u8, Register2 : u8, SegmentPrefix : u32 ) -> bool { // FFI-Unsafe: Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let segmentprefix: Register = transmute( SegmentPrefix as u8 );

  match Instruction::with_maskmovq( AddressSize, register1, register2, segmentprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_maskmovdqu( Instruction : *mut Instruction, AddressSize: u32, Register1 : u8, Register2 : u8, SegmentPrefix : u32 ) -> bool { // FFI-Unsafe: Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let segmentprefix: Register = transmute( SegmentPrefix as u8 );

  match Instruction::with_maskmovdqu( AddressSize, register1, register2, segmentprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_vmaskmovdqu( Instruction : *mut Instruction, AddressSize: u32, Register1 : u8, Register2 : u8, SegmentPrefix : u32 ) -> bool { // FFI-Unsafe: Register
  if Instruction.is_null() {
      return false;
  }

  let register1: Register = transmute( Register1 );
  let register2: Register = transmute( Register2 );
  let segmentprefix: Register = transmute( SegmentPrefix as u8 );

  match Instruction::with_vmaskmovdqu( AddressSize, register1, register2, segmentprefix ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_1( Instruction : *mut Instruction, B0 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_1( B0 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_2( Instruction : *mut Instruction, B0 : u8, B1 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_2( B0, B1 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_3( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_3( B0, B1, B2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_4( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_4( B0, B1, B2, B3 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_5( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_5( B0, B1, B2, B3, B4 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_6( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_6( B0, B1, B2, B3, B4, B5 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_7( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_7( B0, B1, B2, B3, B4, B5, B6 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_8( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_8( B0, B1, B2, B3, B4, B5, B6, B7 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_9( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_9( B0, B1, B2, B3, B4, B5, B6, B7, B8 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_10( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8, B9 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_10( B0, B1, B2, B3, B4, B5, B6, B7, B8, B9 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_11( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8, B9 : u8, B10 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_11( B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_12( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8, B9 : u8, B10 : u8, B11 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_12( B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_13( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8, B9 : u8, B10 : u8, B11 : u8, B12 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_13( B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_14( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8, B9 : u8, B10 : u8, B11 : u8, B12 : u8, B13 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_14( B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_15( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8, B9 : u8, B10 : u8, B11 : u8, B12 : u8, B13 : u8, B14 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_15( B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Byte_16( Instruction : *mut Instruction, B0 : u8, B1 : u8, B2 : u8, B3 : u8, B4 : u8, B5 : u8, B6 : u8, B7 : u8, B8 : u8, B9 : u8, B10 : u8, B11 : u8, B12 : u8, B13 : u8, B14 : u8, B15 : u8 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_byte_16( B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_1( Instruction : *mut Instruction, W0 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_1( W0 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_2( Instruction : *mut Instruction, W0 : u16, W1 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_2( W0, W1 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_3( Instruction : *mut Instruction, W0 : u16, W1 : u16, W2 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_3( W0, W1, W2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_4( Instruction : *mut Instruction, W0 : u16, W1 : u16, W2 : u16, W3 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_4( W0, W1, W2, W3 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_5( Instruction : *mut Instruction, W0 : u16, W1 : u16, W2 : u16, W3 : u16, W4 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_5( W0, W1, W2, W3, W4 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_6( Instruction : *mut Instruction, W0 : u16, W1 : u16, W2 : u16, W3 : u16, W4 : u16, W5 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_6( W0, W1, W2, W3, W4, W5 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_7( Instruction : *mut Instruction, W0 : u16, W1 : u16, W2 : u16, W3 : u16, W4 : u16, W5 : u16, W6 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_7( W0, W1, W2, W3, W4, W5, W6 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_Word_8( Instruction : *mut Instruction, W0 : u16, W1 : u16, W2 : u16, W3 : u16, W4 : u16, W5 : u16, W6 : u16, W7 : u16 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_word_8( W0, W1, W2, W3, W4, W5, W6, W7 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_DWord_1( Instruction : *mut Instruction, D0 : u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_dword_1( D0 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_DWord_2( Instruction : *mut Instruction, D0 : u32, D1 : u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_dword_2( D0, D1 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_DWord_3( Instruction : *mut Instruction, D0 : u32, D1 : u32, D2 : u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_dword_3( D0, D1, D2 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_DWord_4( Instruction : *mut Instruction, D0 : u32, D1 : u32, D2 : u32, D3 : u32 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_dword_4( D0, D1, D2, D3 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_QWord_1( Instruction : *mut Instruction, Q0 : u64 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_qword_1( Q0 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

#[no_mangle]
pub unsafe extern "C" fn Instruction_With_Declare_QWord_2( Instruction : *mut Instruction, Q0 : u64, Q1 : u64 ) -> bool {
  if Instruction.is_null() {
      return false;
  }

  match Instruction::try_with_declare_qword_2( Q0, Q1 ) {
    Err( _e ) => return false,
    Ok( instruction ) => { 
        (*Instruction) = instruction;
        return true
    }
  };
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Virtual-Address Resolver
// Gets the virtual address of a memory operand
//
// # Arguments
// * `operand`: Operand number, 0-4, must be a memory operand
// * `element_index`: Only used if it's a vsib memory operand. This is the element index of the vector index register.
// * `get_register_value`: Function that returns the value of a register or the base address of a segment register, or `None` for unsupported
//    registers.
//
// # Call-back function args
// * Arg 1: `register`: Register (GPR8, GPR16, GPR32, GPR64, XMM, YMM, ZMM, seg). If it's a segment register, the call-back function should return the segment's base address, not the segment's register value.
// * Arg 2: `element_index`: Only used if it's a vsib memory operand. This is the element index of the vector index register.
// * Arg 3: `element_size`: Only used if it's a vsib memory operand. Size in bytes of elements in vector index register (4 or 8).
type
  TVirtualAddressResolverCallback = unsafe extern "C" fn( Register: u8/*Register*/, Index: usize, Size: usize, Address : *mut u64, UserData : *const usize ) -> bool;// FFI-Unsafe: Register

#[no_mangle]
pub unsafe extern "C" fn Instruction_VirtualAddress( Instruction: *mut Instruction, Callback : Option<TVirtualAddressResolverCallback>, Operand : u32, Index : usize, UserData : *const usize ) -> u64 {
    if Instruction.is_null() {
       return 0
    }
    if Callback.is_none() {
        return 0
    }

    let va = Instruction.as_mut().unwrap().virtual_address(Operand, Index, 
        |Register, Index, Size| {
            match Register {                
                Register::ES | Register::CS | Register::SS | Register::DS => Some( 0 ), // The base address of ES, CS, SS and DS is always 0 in 64-bit mode
                _ => {
                    let mut value : u64 = 0;
                    if Callback.unwrap()( Register as u8, Index, Size, &mut value as *mut u64, UserData ) {
                        Some( value )
                    }else {
                        None
                    }
                }
            }
        }
    );

    let res: u64;
    match va {
        None => res = 0x0,
        _ => res = va.unwrap()
    }
    return res;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// InstructionInfoFactory

// Creates a new instance.
//
// If you don't need to know register and memory usage, it's faster to call [`Instruction`] and
// [`Code`] methods such as [`Instruction::flow_control()`] instead of getting that info from this struct.
//
// [`Instruction`]: struct.Instruction.html
// [`Code`]: enum.Code.html
// [`Instruction::flow_control()`]: struct.Instruction.html#method.flow_control
#[no_mangle]
pub extern "C" fn InstructionInfoFactory_Create() -> *mut InstructionInfoFactory {
    return Box::into_raw( Box::new( InstructionInfoFactory::new() ) );
}

// Creates a new [`InstructionInfo`], see also [`info()`].
//
// If you don't need to know register and memory usage, it's faster to call [`Instruction`] and
// [`Code`] methods such as [`Instruction::flow_control()`] instead of getting that info from this struct.
#[allow( non_upper_case_globals )]
const UsedRegisterMaxEntries : usize = 100;
#[repr(C)]
pub struct TUsedRegisterArray { 
    Entries : [UsedRegister;UsedRegisterMaxEntries], 
    Count : u8
}

#[allow( non_upper_case_globals )]
const UsedMemoryMaxEntries : usize = 255;
#[repr(C)]
pub struct TUsedMemoryArray { 
    Entries : [UsedMemory;UsedMemoryMaxEntries], 
    Count : u8
}

#[repr(C)]
pub struct TInstructionInfo {
	used_registers: TUsedRegisterArray,
	used_memory_locations: TUsedMemoryArray,
	op_accesses: [OpAccess;5/*IcedConstants::MAX_OP_COUNT*/]
}

#[no_mangle]
pub unsafe extern "C" fn InstructionInfoFactory_Info( InstructionInfoFactory: *mut InstructionInfoFactory, Instruction: *mut Instruction, InstructionInfo: *mut TInstructionInfo, Options: u32/*InstructionInfoOptions*/ ) -> bool { 
    if InstructionInfoFactory.is_null() {
        return false;
    }
    if Instruction.is_null() {
        return false;
    }
    if InstructionInfo.is_null() {
        return false;
    }

    let value = &*(*InstructionInfoFactory).info_options( &(*Instruction), Options );

    let usedregistersA = value.used_registers();
    (*InstructionInfo).used_registers.Count = usedregistersA.len() as u8;
    for ( i, x ) in usedregistersA.iter().enumerate() {
        if i < (*InstructionInfo).used_registers.Entries.len() {
            (*InstructionInfo).used_registers.Entries[ i ] = *x;
        }
    }

    let usedmemoryA = value.used_memory();
    (*InstructionInfo).used_memory_locations.Count = usedmemoryA.len() as u8;
    for ( i, x ) in usedmemoryA.iter().enumerate() {
        if i < (*InstructionInfo).used_memory_locations.Entries.len() { 
            (*InstructionInfo).used_memory_locations.Entries[ i ] = *x;
        }
    }
    
    (*InstructionInfo).op_accesses[ 0 ] = value.op0_access();
    (*InstructionInfo).op_accesses[ 1 ] = value.op1_access();
    (*InstructionInfo).op_accesses[ 2 ] = value.op2_access();
    (*InstructionInfo).op_accesses[ 3 ] = value.op3_access();
    (*InstructionInfo).op_accesses[ 4 ] = value.op4_access();

    return true;
}
