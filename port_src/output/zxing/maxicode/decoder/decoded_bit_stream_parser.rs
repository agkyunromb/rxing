/*
 * Copyright 2011 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::google::zxing::maxicode::decoder;

/**
 * <p>MaxiCodes can encode text or structured information as bits in one of several modes,
 * with multiple character sets in one code. This class decodes the bits back into text.</p>
 *
 * @author mike32767
 * @author Manuel Kasten
 */

 const SHIFTA: char = '￰';

 const SHIFTB: char = '￱';

 const SHIFTC: char = '￲';

 const SHIFTD: char = '￳';

 const SHIFTE: char = '￴';

 const TWOSHIFTA: char = '￵';

 const THREESHIFTA: char = '￶';

 const LATCHA: char = '￷';

 const LATCHB: char = '￸';

 const LOCK: char = '￹';

 const ECI: char = '￺';

 const NS: char = '￻';

 const PAD: char = '￼';

 const FS: char = '';

 const GS: char = '';

 const RS: char = '';

 const COUNTRY_BYTES: vec![Vec<i8>; 10] = vec![53, 54, 43, 44, 45, 46, 47, 48, 37, 38, ]
;

 const SERVICE_CLASS_BYTES: vec![Vec<i8>; 10] = vec![55, 56, 57, 58, 59, 60, 49, 50, 51, 52, ]
;

 const POSTCODE_2_LENGTH_BYTES: vec![Vec<i8>; 6] = vec![39, 40, 41, 42, 31, 32, ]
;

 const POSTCODE_2_BYTES: vec![Vec<i8>; 30] = vec![33, 34, 35, 36, 25, 26, 27, 28, 29, 30, 19, 20, 21, 22, 23, 24, 13, 14, 15, 16, 17, 18, 7, 8, 9, 10, 11, 12, 1, 2, ]
;

 const POSTCODE_3_BYTES: vec![vec![Vec<Vec<i8>>; 6]; 6] = vec![vec![39, 40, 41, 42, 31, 32, ]
, vec![33, 34, 35, 36, 25, 26, ]
, vec![27, 28, 29, 30, 19, 20, ]
, vec![21, 22, 23, 24, 13, 14, ]
, vec![15, 16, 17, 18, 7, 8, ]
, vec![9, 10, 11, 12, 1, 2, ]
, ]
;

 const SETS: vec![Vec<String>; 5] = vec![format!("\rABCDEFGHIJKLMNOPQRSTUVWXYZ{}{}{}{}{} {}\"#$%&'()*+,-./0123456789:{}{}{}{}{}", ECI, FS, GS, RS, NS, PAD, SHIFTB, SHIFTC, SHIFTD, SHIFTE, LATCHB), format!("`abcdefghijklmnopqrstuvwxyz{}{}{}{}{}{{}}~;<=>?[\\]^_ ,./:@!|{}{}{}{}{}{}{}{}{}", ECI, FS, GS, RS, NS, PAD, PAD, TWOSHIFTA, THREESHIFTA, PAD, SHIFTA, SHIFTC, SHIFTD, SHIFTE, LATCHA), format!("ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚ{}{}{}{}{}ÛÜÝÞßª¬±²³µ¹º¼½¾{} {}{}{}{}", ECI, FS, GS, RS, NS, LATCHA, LOCK, SHIFTD, SHIFTE, LATCHB), format!("àáâãäåæçèéêëìíîïðñòóôõö÷øùú{}{}{}{}{}ûüýþÿ¡¨«¯°´·¸»¿{} {}{}{}{}", ECI, FS, GS, RS, NS, LATCHA, SHIFTC, LOCK, SHIFTE, LATCHB), format!(" 	\n\r{}{}{}{}{}{}{} ¢£¤¥¦§©­®¶{} {}{}{}{}", ECI, PAD, PAD, NS, FS, GS, RS, LATCHA, SHIFTC, SHIFTD, LOCK, LATCHB), ]
;
struct DecodedBitStreamParser {
}

impl DecodedBitStreamParser {

    fn new() -> DecodedBitStreamParser {
    }

    fn  decode( bytes: &Vec<i8>,  mode: i32) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
         let result: StringBuilder = StringBuilder::new(144);
        match mode {
              2 => 
                 {
                }
              3 => 
                 {
                     let mut postcode: String;
                    if mode == 2 {
                         let pc: i32 = ::get_post_code2(&bytes);
                         let ps2_length: i32 = ::get_post_code2_length(&bytes);
                        if ps2_length > 10 {
                            throw FormatException::get_format_instance();
                        }
                         let df: NumberFormat = DecimalFormat::new(&"0000000000".substring(0, ps2_length));
                        postcode = df.format(pc);
                    } else {
                        postcode = ::get_post_code3(&bytes);
                    }
                     let three_digits: NumberFormat = DecimalFormat::new("000");
                     let country: String = three_digits.format(&::get_country(&bytes));
                     let service: String = three_digits.format(&::get_service_class(&bytes));
                    result.append(&::get_message(&bytes, 10, 84));
                    if result.to_string().starts_with(format!("[)>{}01{}", RS, GS)) {
                        result.insert(9, format!("{}{}{}{}{}{}", postcode, GS, country, GS, service, GS));
                    } else {
                        result.insert(0, format!("{}{}{}{}{}{}", postcode, GS, country, GS, service, GS));
                    }
                    break;
                }
              4 => 
                 {
                    result.append(&::get_message(&bytes, 1, 93));
                    break;
                }
              5 => 
                 {
                    result.append(&::get_message(&bytes, 1, 77));
                    break;
                }
        }
        return Ok(DecoderResult::new(&bytes, &result.to_string(), null, &String::value_of(mode)));
    }

    fn  get_bit( bit: i32,  bytes: &Vec<i8>) -> i32  {
        bit -= 1;
        return  if (bytes[bit / 6] & (1 << (5 - (bit % 6)))) == 0 { 0 } else { 1 };
    }

    fn  get_int( bytes: &Vec<i8>,  x: &Vec<i8>) -> i32  {
         let mut val: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < x.len() {
                {
                    val += ::get_bit(x[i], &bytes) << (x.len() - i - 1);
                }
                i += 1;
             }
         }

        return val;
    }

    fn  get_country( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &COUNTRY_BYTES);
    }

    fn  get_service_class( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &SERVICE_CLASS_BYTES);
    }

    fn  get_post_code2_length( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &POSTCODE_2_LENGTH_BYTES);
    }

    fn  get_post_code2( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &POSTCODE_2_BYTES);
    }

    fn  get_post_code3( bytes: &Vec<i8>) -> String  {
         let sb: StringBuilder = StringBuilder::new(POSTCODE_3_BYTES.len());
        for  let p3bytes: Vec<i8> in POSTCODE_3_BYTES {
            sb.append(&SETS[0]::char_at(&::get_int(&bytes, &p3bytes)));
        }
        return sb.to_string();
    }

    fn  get_message( bytes: &Vec<i8>,  start: i32,  len: i32) -> String  {
         let sb: StringBuilder = StringBuilder::new();
         let mut shift: i32 = -1;
         let mut set: i32 = 0;
         let mut lastset: i32 = 0;
         {
             let mut i: i32 = start;
            while i < start + len {
                {
                     let c: char = SETS[set]::char_at(bytes[i]);
                    match c {
                          LATCHA => 
                             {
                                set = 0;
                                shift = -1;
                                break;
                            }
                          LATCHB => 
                             {
                                set = 1;
                                shift = -1;
                                break;
                            }
                          SHIFTA => 
                             {
                            }
                          SHIFTB => 
                             {
                            }
                          SHIFTC => 
                             {
                            }
                          SHIFTD => 
                             {
                            }
                          SHIFTE => 
                             {
                                lastset = set;
                                set = c - SHIFTA;
                                shift = 1;
                                break;
                            }
                          TWOSHIFTA => 
                             {
                                lastset = set;
                                set = 0;
                                shift = 2;
                                break;
                            }
                          THREESHIFTA => 
                             {
                                lastset = set;
                                set = 0;
                                shift = 3;
                                break;
                            }
                          NS => 
                             {
                                 let nsval: i32 = (bytes[i += 1] << 24) + (bytes[i += 1] << 18) + (bytes[i += 1] << 12) + (bytes[i += 1] << 6) + bytes[i += 1];
                                sb.append(&DecimalFormat::new("000000000").format(nsval));
                                break;
                            }
                          LOCK => 
                             {
                                shift = -1;
                                break;
                            }
                        _ => 
                             {
                                sb.append(c);
                            }
                    }
                    if shift -= 1 !!!check!!! post decrement == 0 {
                        set = lastset;
                    }
                }
                i += 1;
             }
         }

        while sb.length() > 0 && sb.char_at(sb.length() - 1) == PAD {
            sb.set_length(sb.length() - 1);
        }
        return sb.to_string();
    }
}

