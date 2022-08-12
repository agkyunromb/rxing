/*
 * Copyright 2010 ZXing authors
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
// package com::google::zxing::oned;

/**
 * This object renders a CODE39 code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */
pub struct Code39Writer {
    super: OneDimensionalCodeWriter;
}

impl Code39Writer {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::CODE_39);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let mut length: i32 = contents.length();
        if length > 80 {
            throw IllegalArgumentException::new(format!("Requested contents should be less than 80 digits long, but got {}", length));
        }
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let index_in_string: i32 = Code39Reader::ALPHABET_STRING::index_of(&contents.char_at(i));
                    if index_in_string < 0 {
                        contents = ::try_to_convert_to_extended_mode(&contents);
                        length = contents.length();
                        if length > 80 {
                            throw IllegalArgumentException::new(format!("Requested contents should be less than 80 digits long, but got {} (extended full ASCII mode)", length));
                        }
                        break;
                    }
                }
                i += 1;
             }
         }

         let widths: [i32; 9] = [0; 9];
         let code_width: i32 = 24 + 1 + (13 * length);
         let result: [bool; code_width] = [false; code_width];
        ::to_int_array(Code39Reader::ASTERISK_ENCODING, &widths);
         let mut pos: i32 = append_pattern(&result, 0, &widths, true);
         let narrow_white: vec![Vec<i32>; 1] = vec![1, ]
        ;
        pos += append_pattern(&result, pos, &narrow_white, false);
        //append next character to byte matrix
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let index_in_string: i32 = Code39Reader::ALPHABET_STRING::index_of(&contents.char_at(i));
                    ::to_int_array(Code39Reader::CHARACTER_ENCODINGS[index_in_string], &widths);
                    pos += append_pattern(&result, pos, &widths, true);
                    pos += append_pattern(&result, pos, &narrow_white, false);
                }
                i += 1;
             }
         }

        ::to_int_array(Code39Reader::ASTERISK_ENCODING, &widths);
        append_pattern(&result, pos, &widths, true);
        return result;
    }

    fn  to_int_array( a: i32,  to_return: &Vec<i32>)   {
         {
             let mut i: i32 = 0;
            while i < 9 {
                {
                     let temp: i32 = a & (1 << (8 - i));
                    to_return[i] =  if temp == 0 { 1 } else { 2 };
                }
                i += 1;
             }
         }

    }

    fn  try_to_convert_to_extended_mode( contents: &String) -> String  {
         let length: i32 = contents.length();
         let extended_content: StringBuilder = StringBuilder::new();
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let character: char = contents.char_at(i);
                    match character {
                          ' ' => 
                             {
                                extended_content.append("%U");
                                break;
                            }
                          ' ' => 
                             {
                            }
                          '-' => 
                             {
                            }
                          '.' => 
                             {
                                extended_content.append(character);
                                break;
                            }
                          '@' => 
                             {
                                extended_content.append("%V");
                                break;
                            }
                          '`' => 
                             {
                                extended_content.append("%W");
                                break;
                            }
                        _ => 
                             {
                                if character <= 26 {
                                    extended_content.append('$');
                                    extended_content.append(('A' + (character - 1)) as char);
                                } else if character < ' ' {
                                    extended_content.append('%');
                                    extended_content.append(('A' + (character - 27)) as char);
                                } else if character <= ',' || character == '/' || character == ':' {
                                    extended_content.append('/');
                                    extended_content.append(('A' + (character - 33)) as char);
                                } else if character <= '9' {
                                    extended_content.append(('0' + (character - 48)) as char);
                                } else if character <= '?' {
                                    extended_content.append('%');
                                    extended_content.append(('F' + (character - 59)) as char);
                                } else if character <= 'Z' {
                                    extended_content.append(('A' + (character - 65)) as char);
                                } else if character <= '_' {
                                    extended_content.append('%');
                                    extended_content.append(('K' + (character - 91)) as char);
                                } else if character <= 'z' {
                                    extended_content.append('+');
                                    extended_content.append(('A' + (character - 97)) as char);
                                } else if character <= 127 {
                                    extended_content.append('%');
                                    extended_content.append(('P' + (character - 123)) as char);
                                } else {
                                    throw IllegalArgumentException::new(format!("Requested content contains a non-encodable character: '{}'", contents.char_at(i)));
                                }
                                break;
                            }
                    }
                }
                i += 1;
             }
         }

        return extended_content.to_string();
    }
}

