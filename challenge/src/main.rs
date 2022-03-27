#[allow(unused_imports)]
use irc::{decode, encode};
use num_bigint::{BigUint, RandBigInt};
use std::collections::HashMap;
use std::time::Instant;
fn main() {
    let n = Instant::now();
    //let input =  "\"<?R&2gOkzH,((!VmYC4SaEuKa{z\"mROK{\"IPB1#)".escape_default().to_string();
    //let ans = encode(7,&input);
    // println!("{:?} || {}",input, decode(&ans));
    // println!("{:?}",doubles(1,3));
    println!("{:?}", str_read("Hi"));
    table(4, true);
    /* println!("{:?}",assembler(vec![
        "mov c 12",
        "mov b 0",
        "mov a 200",
        "dec a",
        "inc b",
        "jnz a -2",
        "dec c",
        "mov a b",
        "jnz c -5",
        "jnz 0 1",
        "mov c a",
    ]));
    */
    println!("{:?}", n.elapsed())
}

#[allow(dead_code)]
fn big_fact(n: i32) -> BigUint {
    return (1..=n)
        .map(|e| BigUint::from(e as u32))
        .reduce(|c, n| c * n)
        .unwrap();
}
#[allow(dead_code)]
fn decomp(n: i32) -> String {
    // your code
    let mut a = big_fact(n);

    //let mut factors:Vec<BigUint> = vec![];
    let zero = BigUint::from(0u64);
    let mut rng = rand::thread_rng();
    let i = rng.gen_biguint_range(&BigUint::from(2u32), &a);
    //println!("{:?}",i);
    println!("{:?}", i);
    while a.clone() % i.clone() < BigUint::from(1u32) && a > zero {
        a /= i.clone();
    }

    String::from("")
}
#[inline(always)]
#[allow(dead_code)]
fn going(n: i32) -> f64 {
    // your code
    let mut result = 1.0;
    let mut acc = 1.0;
    for i in (2..=n).rev() {
        acc = acc / (i as f64);
        result += acc;
    }
    let re = format!("{}", result);
    if re.len() > 8 {
        return (re[..8]).parse::<f64>().unwrap();
    }
    result
}

#[allow(dead_code, unused_mut, unused_variables)]

fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    program.into_iter().for_each(|e| {
        let v: Vec<&str> = e.split(' ').collect();
        // check for the instruction
        if e.contains("mov") {
            // movs
            //
            // if charact is numeric
            let item: i64 = match v[2].parse::<i64>() {
                Ok(n) => n,
                Err(_) => {
                    // get the number from hash map
                    if let Some(x) = registers.get(v[2]) {
                        *x
                    } else {
                        0
                    }
                }
            };

            registers.insert(v[1].to_string(), item);
        }
        // check for increments and decrements
        else if e.contains("inc") || e.contains("dec") {
            if let Some(x) = registers.get_mut(v[1]) {
                // increment or decrement
                if v[0].contains("inc") {
                    *x = *x + 1;
                } else {
                    *x = *x - 1;
                }
            }
        }
        //
        if e.contains("jnz") {
            // decrease
            match registers.get_mut(v[1]) {
                Some(x) => {
                    // decrease the
                    while *x > 0 {
                        *x = *x - 1;
                    }
                }
                None => {}
            }
        }
    });
    registers
}
// javascript implementation

#[allow(dead_code, unused_mut, unused_variables)]
fn assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers = HashMap::new();
    let mut stack_pointer: usize = 0;

    while stack_pointer < program.len() {
        let mut curr_instruction = program[stack_pointer].split(' ').collect::<Vec<&str>>();
        let code = curr_instruction[0];
        match code {
            "mov" => {
                // mov requires a source and destination
                let dest = curr_instruction[1];
                let source = curr_instruction[2];
                // source can be a constant or another register
                // check if character is alphabetic
                let alph = "abcdefghijklmnopqrstuvwxyz";
                if alph.contains(source) {
                    // move to different register
                    println!(" move {} to source: {}", dest, source);
                    registers.insert(dest.to_string(), *registers.get(source).unwrap());
                } else {
                    registers.insert(dest.to_string(), source.parse::<i64>().unwrap());
                }
                stack_pointer += 1;
            }
            "inc" => {
                let register = curr_instruction[1];

                if let Some(x) = registers.get_mut(register) {
                    // increment or decrement

                    *x = *x + 1;
                }
                stack_pointer += 1;
            }
            "dec" => {
                let register = curr_instruction[1];

                if let Some(x) = registers.get_mut(register) {
                    // increment or decrement

                    *x = *x - 1;
                }
                stack_pointer += 1;
            }
            "jnz" => {
                //println!("{}",curr_instruction[2]);
                let register = curr_instruction[1];
                let step = curr_instruction[2];
                if let Some(reg) = registers.get(register) {
                    if *reg == 0 {
                        stack_pointer += 1;
                    } else {
                        match step.parse::<i32>() {
                            Ok(val) => {
                                let res = (stack_pointer as i32 + val) as usize;
                                //println!(" res {}, stack ={} , val ={}",res,stack_pointer,val);
                                stack_pointer = (stack_pointer as i32 + val) as usize
                            }
                            Err(_) => {}
                        }
                    }
                } else {
                    break;
                }
            }
            &_ => {
                println!("{:?}", code);
            }
        }
    }

    registers
}

// function that return bytes
#[allow(dead_code)]
#[inline]
fn str_read(s: &str) -> Vec<Vec<usize>> {
    let mut bits = String::from("0100");
    //append a binary string of length
    let len_bit: String = format!("{:#08b}", s.len());

    bits.push_str(&len_bit);
    //println!("{}",len_bit);
    s.as_bytes()
        .iter()
        .for_each(|e| bits.push_str(&format!("{:08b}", e)));

    let mut bits: String = bits
        .chars()
        .map(|e| {
            // conversion in 8bint binary with b
            if e == 'b' {
                return '0';
            } else {
                e
            }
        })
        .collect();
    //println!("{}",bits.len());
    // if bits length is less than 69, add 0000, if its 70 add 00, if its 71 add 0. otherwise do nothing

    match bits.len() {
        0..=69 => {
            bits.push_str("0000");
        }
        70 => {
            bits.push_str("00");
        }
        71 => {
            bits.push_str("0");
        }
        _ => {
            // do nothing
        }
    };
    // if the length of bits is less than 72, we need to make  it up to 72 characters
    if bits.len() < 72 {
        let remaining = (72 - bits.len()) / 8;

        let mut chunks = vec!["11101100"; remaining];
        if chunks.len() > 1 {
            for i in 0..=chunks.len() - 1 {
                let after = i + 1 % chunks.len() - 1;
                let before = if i == 0 { 0 } else { i - 1 };

                if i != 0 && chunks[after] == "11101100" && chunks[before] == "11101100" {
                    chunks[i] = "00010001"
                }
                if i == chunks.len() - 1 && chunks[before] == "11101100" {
                    chunks[i] = "00010001";
                    //println!("last index");
                }
            }
        }
        bits.push_str(&chunks.join(""));
    //println!("total length {}, chunks ={:?}",bits.len(),chunks);
    } else {
        // dothing
    }
    // Error Correction

    let groups: Vec<usize> = sub_strings(&bits, 8)
        .iter()
        .map(|e| usize::from_str_radix(e, 2).unwrap())
        .collect();
    //println!("{:?}",groups);

    // message Polynomial - ax^25 + bx^24+ cx^23 + dx^22 + ex^21+ Fx^20 = gx^19 + hx^18 + ix^17
    let mut message: Vec<usize> = (17usize..=25)
        .rev()
        .enumerate()
        .map(|(ind, _)| groups[ind])
        .collect();

    // generator polynomial  = (alphapower)
    let g = vec![
        0, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136,
    ];

    let g_field = [
        1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117,
        234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181,
        119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222,
        161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231,
        211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17,
        34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59,
        118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66,
        132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115,
        230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219,
        171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141,
        7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155,
        43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139,
        11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142, 1,
    ];
    //
    let initial = message.len();
    let mut count = 1;
    // data analysis  n times

    while count <= initial {
        // add first polynormal
        #[allow(non_snake_case)]
        let Firstind = g_field.iter().position(|&e| e == message[0]).unwrap();

        let poly = g.iter().map(|e| {
            let power = e + Firstind;
            //println!("prev ={:?}", Firstind);
            if power > 254 {
                return power % 255;
            } else {
                return power;
            }
        });
        //println!("prev ={:?}",poly);
        message = poly
            .enumerate()
            .map(|(ind, e)| {
                // find the  iterator corresponding to postion
                let int = if e < 254 {
                    g_field[e]
                } else {
                    g_field[e % 255]
                };

                if let Some(x) = message.get(ind) {
                    int ^ x
                } else {
                    return int;
                }
            })
            .filter(|&e| e != 0)
            .collect();

        //Firstind = g_field.iter().position(|&e|e ==message[0]).unwrap();

        count += 1;
    }
    // add the greated errors code to the // convert to 8 bit binary and append to string
    //bits = "01000000001001001000011010010000".to_string();

    //println!("mine ={:?}",bits);
    message.iter().map(|e| format!("{:b}", e)).for_each(|t| {
        if t.len() < 8 {
            let zero = 8 - t.len();

            bits.push_str(&format!("{}{}", "0".repeat(zero), t));
        } else {
            bits.push_str(&t);
        }
    });

    // funciton that creates the matrix

    // fill the matrix

    get_raw_qr(bits)

    // Matrix placement, Format and Version information
}

use itertools::Itertools;

fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    source
        .chars()
        .chunks(sub_size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
fn sub_strings2(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}

// function getMatrix

fn fill_area(
    matrix: &mut Vec<Vec<usize>>,
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    fill: usize,
) {
    for i in row..(row + height) {
        // mutate the  vector
        for e in col..(col + width) {
            matrix[i][e] = fill;
        }
    }
}

fn masks(index: usize, row: usize, col: usize) -> bool {
    let mask_array: Vec<fn(usize, usize) -> bool> = vec![
        |row, col| (row + col) % 2 == 0,
        |row, _| row % 2 == 0,
        |_, col| col % 3 == 0,
        |row, col| (row + col) % 3 == 0,
        |row, col| (row / 2 + col / 3) % 3 == 0,
        |row, col| (row * col) % 2 + (row * col) % 3 == 0,
        |row, col| ((row * col) % 2 + (row * col) % 3) % 2 == 0,
        |row, col| ((row + col) % 2 + (row * col) % 3) % 2 == 0,
    ];

    return mask_array[index](row, col);
}
fn get_module_seq(version: usize) -> Vec<[usize; 2]> {
    let mut matrix = get_matrix(version);
    let size = get_size(version);
    //
    //println!("{:?}, size = {}",matrix,size);
    // Finder patterns + divisors
    fill_area(&mut matrix, 0, 0, 9, 9, 1);
    fill_area(&mut matrix, 0, size - 8, 8, 9, 1);
    fill_area(&mut matrix, size - 8, 0, 9, 8, 1);
    // Alignment pattern -yes, we just place one for the general
    // implementation, wait for the next parts in the series
    fill_area(&mut matrix, size - 9, size - 9, 5, 5, 1);
    // Timing patterns
    fill_area(&mut matrix, 6, 9, version * 4, 1, 1);
    fill_area(&mut matrix, 9, 6, 1, version * 4, 1);

    // Dark module
    matrix[size - 8][8] = 1;

    let mut row_step = -1;
    let mut row = (size - 1) as isize;
    let mut column = size - 1;
    let mut sequence = vec![];
    let mut index = 0;
    while column as isize >= 0 {
        if matrix[row as usize][column] == 0 {
            sequence.push([row as usize, column])
        }
        // Checking the parity of the index of the current module

        if index & 1 == 1 {
            row += row_step;
            if row == -1 || row == size as isize {
                row_step = -row_step;
                row += row_step;
                column -= if column == 7 { 2 } else { 1 };
            } else {
                column += 1;
            }
        } else {
            column -= 1;
        }

        index += 1;
    }

    sequence
}
// get
fn table(ind: usize, exp_t: bool) -> usize {
    // log tables
    let mut log = vec![0; 256];
    let mut exp = vec![0; 256];
    let mut value = 1;
    for exponent in 1..256 {
        value = if value > 127 {
            (value << 1) ^ 285
        } else {
            value << 1
        };
        log[value] = exponent % 255;
        exp[exponent % 255] = value;
    }
    //println!("log ={:?}, exp= {:?}",log.len(),exp.len());
    if exp_t {
        return exp[ind];
    } else {
        log[ind]
    }
}
#[allow(dead_code)]
fn mul(a: usize, b: usize) -> usize {
    table((table(a, false) + table(b, false)) % 255, true)
}
#[allow(dead_code)]
fn div(a: usize, b: usize) -> usize {
    table((table(a, false) + table(b, false) * 254) % 255, true)
}
#[allow(dead_code, non_snake_case)]
fn polyMul(polya: Vec<usize>, polyb: Vec<usize>) -> Vec<usize> {
    // This is going to be the product polynomial, that we pre-allocate.
    // We know it's going to be `poly1.length + poly2.length - 1` long.
    let mut coeffs = vec![0; polya.len() + polyb.len() - 1];
    // Instead of executing all the steps in the example, we can jump to
    // computing the coefficients of the result
    for index in 0..coeffs.len() {
        let mut coeff = 0;
        for p1 in 0..=index {
            let mut p2 = index - p1;
            p2 = if p2 >= polyb.len() { 0 } else { p2 };
            // We *should* do better here, as `p1index` and `p2index` could
            // be out of range, but `mul` defined above will handle that case.
            // Just beware of that when implementing in other languages.
            coeff ^= mul(polya[p1], polyb[p2]);
        }
        coeffs[index] = coeff;
    }

    coeffs
}
#[allow(dead_code, non_snake_case)]
fn polyRest(dividend: Vec<usize>, divisor: Vec<usize>) -> Vec<usize> {
    let quotientLength = dividend.len() - divisor.len() + 1;
    let mut rest = dividend;
    for _ in 0..quotientLength {
        if rest.get(0) != None {
            let factor = div(rest[0], divisor[0]);
            let mut subtr = vec![0; rest.len()];
            let ans = polyMul(divisor.clone(), vec![factor]);
            for i in 0..=ans.len() - 1 {
                subtr[i] = ans[i];
            }
            rest = rest
                .into_iter()
                .enumerate()
                .map(|(index, value)| value ^ subtr[index])
                .collect::<Vec<usize>>()[1..]
                .to_vec();
        } else {
            rest = rest[1..].to_vec();
        }
    }

    rest
}

//
#[allow(non_snake_case, dead_code, unused_variables)]
fn get_formated_modules(error_level: char, maskIndex: usize) -> Vec<usize> {
    let mut formatPoly = vec![0; 15];
    const EDC_ORDER: [char; 4] = ['M', 'L', 'H', 'Q'];
    let FORMAT_DIVISOR: Vec<usize> = vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1];
    let FORMAT_MASK: [usize; 15] = [1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0];
    let error_index: usize = EDC_ORDER.iter().position(|&e| error_level == e).unwrap();
    formatPoly[0] = error_index >> 1;
    formatPoly[1] = error_index & 1;
    formatPoly[2] = maskIndex >> 2;
    formatPoly[3] = (maskIndex >> 1) & 1;
    formatPoly[4] = maskIndex & 1;
    let rest = polyRest(formatPoly.clone(), FORMAT_DIVISOR);
    let mut count = 0;
    for i in 5..=5 + rest.len() - 1 {
        if count < rest.len() {
            formatPoly[i] = rest[count];
        }
        count += 1;
    }
    formatPoly
        .iter()
        .enumerate()
        .map(|(index, bit)| bit ^ FORMAT_MASK[index])
        .collect()
}
// adding masking
#[allow(non_snake_case, dead_code)]
fn get_masked_matrix(version: usize, codewars: Vec<usize>, maskIndex: usize) -> Vec<Vec<usize>> {
    let sequence = get_module_seq(version);
    let mut matrix = get_matrix(version);
    sequence.iter().enumerate().for_each(|(index, [row, col])| {
        // Each codeword contains 8 modules, so shifting the index to the
        // right by 3 gives the codeword's index
        let codewar = codewars[index >> 3];

        let bit_shift = 7 - (index & 7);
        let module_bit = (codewar >> bit_shift) & 1;
        matrix[*row][*col] = module_bit ^ if masks(maskIndex, *row, *col) { 1 } else { 0 };
    });

    matrix
}
#[allow(dead_code, non_snake_case)]
fn get_masked_qr(
    version: usize,
    codewars: Vec<usize>,
    error_level: char,
    maskIndex: usize,
) -> Vec<Vec<usize>> {
    let mut matrix = get_masked_matrix(version, codewars, maskIndex);
    placeFormatModules(&mut matrix, error_level, maskIndex);
    placeFixedPatterns(&mut matrix);
    matrix
}
#[allow(dead_code, non_snake_case)]
fn placeFormatModules(matrix: &mut Vec<Vec<usize>>, error_level: char, maskIndex: usize) {
    let format_modules = get_formated_modules(error_level, maskIndex);
    let length = matrix.len();
    set(&mut matrix[8], &format_modules[0..6], 0);
    set(&mut matrix[8], &format_modules[6..8], 7);
    set(&mut matrix[8], &format_modules[0..=7], length - 8);
    matrix[7][8] = format_modules[8];
    format_modules[0..7]
        .iter()
        .enumerate()
        .for_each(|(index, cell)| {
            matrix[length - index - 1][8] = *cell;
        });
    format_modules[..=9]
        .iter()
        .enumerate()
        .for_each(|(index, cell)| {
            let ind = if index > 5 { index % 5 } else { 5 - index };
            matrix[ind][8] = *cell;
        });
}

fn set(matrix: &mut [usize], slice: &[usize], start: usize) {
    let mut count = 0;
    for i in start..=slice.len() - 1 {
        if count < slice.len() {
            matrix[i] = slice[count];
        }
        count += 1;
    }
}

#[allow(dead_code, non_snake_case)]
fn placeFixedPatterns(mut qrcode: &mut Vec<Vec<usize>>) {
    let size = qrcode.len();
    // placing the fixed patterns
    [[0, 0], [size - 7, 0], [0, size - 7]]
        .iter()
        .for_each(|[row, col]| {
            fill_area(&mut qrcode, *row, *col, 7, 7, 1);
            fill_area(&mut qrcode, *row + 1, *col + 1, 5, 5, 0);
            fill_area(&mut qrcode, *row + 2, *col + 2, 3, 3, 1);
        });

    // Separators
    fill_area(&mut qrcode, 7, 0, 8, 1, 0);
    fill_area(&mut qrcode, 0, 7, 1, 7, 0);
    fill_area(&mut qrcode, size - 8, 0, 8, 1, 0);
    fill_area(&mut qrcode, 0, size - 8, 1, 7, 0);
    fill_area(&mut qrcode, 7, size - 8, 8, 1, 0);
    fill_area(&mut qrcode, size - 7, 7, 1, 7, 0);
    // Alignment
    fill_area(&mut qrcode, size - 9, size - 9, 5, 5, 1);
    fill_area(&mut qrcode, size - 8, size - 8, 3, 3, 0);
    qrcode[size - 7][size - 7] = 1;
    // Timing patterns
    for pos in (8..(size - 9)).step_by(2) {
        qrcode[6][pos] = 1;
        qrcode[6][pos + 1] = 0;
        qrcode[pos][6] = 1;
        qrcode[pos + 1][6] = 0;
    }
    qrcode[6][size - 7] = 1;
    qrcode[size - 7][6] = 1;
    // Dark module
    qrcode[size - 8][8] = 1;
}
fn get_matrix(version: usize) -> Vec<Vec<usize>> {
    let size = get_size(version);
    vec![vec![0; size]; size]
}
fn get_size(version: usize) -> usize {
    version * 4 + 17
}
// function
#[allow(dead_code)]
fn get_raw_qr(message: String) -> Vec<Vec<usize>> {
    const VERSION: usize = 1;

    let bit_length = 8;

    // get the codewars
    let codewars: Vec<usize> = sub_strings(&message, bit_length)
        .iter()
        .map(|e| usize::from_str_radix(e, 2).unwrap())
        .collect();

    let qrcode = get_masked_qr(VERSION, codewars, 'H', 0);

    qrcode
}

#[allow(dead_code)]
#[inline(always)]
fn checkers(size: usize) -> Vec<Vec<usize>> {
    let mut board = vec![vec![0; size]; size];

    for i in 0..=size - 1 {
        for x in 0..=size - 1 {
            let after = i + 1 % size - 1;
            let before = if i == 0 { 0 } else { i - 1 };

            if i != 0 && board[x][after] == 0 && board[x][before] == 0 {
                board[x][i] = 1;
            }
            if i == board.len() - 1 && board[x][before] == 0 {
                board[x][i] = 1;
                //println!("last index");
            }
        }
    }

    //  complete the grid
    for i in 0..=size - 1 {
        let after = i + 1 % size - 1;
        let before = if i == 0 { 0 } else { i - 1 };

        if i != 0 && board[after] == board[i] && board[before] == board[i] {
            board[i].reverse()
        }
        if i == size - 1 && board[before] == board[i] {
            board[i].reverse()
            //println!("last index");
        }
    }

    board
}
#[allow(dead_code)]
#[inline]
fn hamming(n: usize) -> usize {
    let mut seq = vec![1];
    let (mut i2, mut i3, mut i5) = (0, 0, 0);
    for _ in 1..n {
        let (left, next, right) = (2 * seq[i2], 3 * seq[i3], 5 * seq[i5]);
        let x = *[left, next, right]
            .iter()
            .reduce(|a, b| std::cmp::min(a, b))
            .unwrap();
        seq.push(x);
        if left <= x {
            i2 += 1;
        }
        if next <= x {
            i3 += 1;
        }
        if right <= x {
            i5 += 1;
        }
    }
    //println!("{:?}",seq);
    seq.pop().unwrap()
}
mod irc {
    use itertools::Itertools;
    #[allow(dead_code, unused_variables)]
    #[inline]
    pub fn get_shifted_str_right(sum: &str, right: usize, to_right: bool) -> String {
        // get an ar
        let mut arr: Vec<_> = sum.chars().collect();

        let right = if right > arr.len() {
            right % arr.len()
        } else {
            right
        };
        if to_right {
            arr.rotate_right(right);
        } else {
            arr.rotate_left(right);
        }

        arr.iter().collect()
    }
    #[allow(dead_code)]
    #[inline]
    pub fn encode(n: u32, qoute: &str) -> String {
        let mut message = if qoute.contains('"') {
            qoute.escape_default().to_string()
        } else {
            qoute.to_owned()
        };

        let spaces: Vec<(usize, char)> =
            message.char_indices().filter(|(_, c)| c == &' ').collect();
        // get no_space

        for _ in 1..=n {
            // remove spaces
            let mut no_spaces: Vec<char> = message.replace(" ", "").chars().collect();
            //println!("{:?}",no_spaces);
            // rotate by to right  n
            let rotate = if n as usize > no_spaces.len() {
                n as usize % no_spaces.len()
            } else {
                n as usize
            };
            no_spaces.rotate_right(rotate);
            // place back the spaces in their original spaces
            spaces
                .iter()
                .for_each(|(ind, s)| no_spaces.insert(*ind, *s));
            // shift every substring by to the right by 10
            let after: String = no_spaces.iter().collect();
            //

            message = after
                .split(" ")
                .map(|e| get_shifted_str_right(e, rotate, true))
                .join(" ");
            //println!("{}",message);
        }
        format!("{} {}", n, message)
    }
    #[allow(dead_code, unused_variables)]
    #[inline]
    pub fn decode(encoded: &str) -> String {
        // extract n from str
        let mut arr = encoded.split(' ');
        // num
        let n = arr.nth(0).unwrap().parse::<usize>().unwrap();
        // message

        let mut message = arr.join(" ");
        let mut count = 1;
        while count <= n {
            message = message
                .split(' ')
                .map(|e| get_shifted_str_right(e, n, false))
                .join(" ");
            // get the space and indices
            let spaces = message.char_indices().filter(|(_, c)| c == &' ');
            // chars without spaces
            // remove spaces
            let mut no_spaces: Vec<char> = message.replace(" ", "").chars().collect();
            // rotate no_spaces to the left by n
            let rotate = if n as usize > no_spaces.len() {
                n as usize % no_spaces.len()
            } else {
                n as usize
            };
            no_spaces.rotate_left(rotate);
            // replace the spaces back

            spaces.for_each(|(ind, s)| no_spaces.insert(ind, s));
            message = no_spaces.iter().collect();
            //println!("{}",message);
            //
            count += 1;
        }
        message
    }
}

#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn example_tests() {
        let encode_examples: Vec<(u32,&str)> = vec![
             (10,"If you wish to make an apple pie from scratch, you must first invent the universe."),
             (14,"True evil is a mundane bureaucracy."),
             (22,"There is nothing more atrociously cruel than an adored child."),
             (36,"As I was going up the stair\nI met a man who wasn't there!\nHe wasn't there again today,\nOh how I wish he'd go away!"),
             (29,"I avoid that bleak first hour of the working day during which my still sluggish senses and body make every chore a penance.\nI find that in arriving later, the work which I do perform is of a much higher quality.")
         ];
        let decode_examples = vec![
             "10 hu fmo a,ys vi utie mr snehn rni tvte .ysushou teI fwea pmapi apfrok rei tnocsclet",
             "14 daue ilev is a munbune Traurecracy.",
             "22 tareu oo iucnaTr dled oldthser.hg hiarm nhcn se rliyet oincoa",
             "36 ws h weA dgIaa ug owh n!asrit git \n msm phw teaI'e tanantwhe reos\ns ther! aHeae 'gwadin\nt haw n htoo ,I'i sy aohOy",
             "29 a r.lht niou gwryd aoshg gIsi mk lei adwhfci isd seensn rdohy mo kleie oltbyhes a\naneu p.n rndr tehh irnne yifav t eo,raclhtc frpw IIti im gwkaidhv aicufh ima doea eruhi y io qshhcoa kr ef l btah gtrrse otnvugrt"
         ];

        for ((n, s1), s2) in encode_examples.into_iter().zip(decode_examples.into_iter()) {
            assert_eq!(&irc::encode(n, s1), s2);
            assert_eq!(&irc::decode(s2), s1);
        }
    }
}
#[allow(dead_code, unused_variables)]
#[inline]
fn decompose(n: i64) -> Option<Vec<i64>> {
    // your code
    let mut res = vec![n];
    while res.get(0) != None {
        let mut area = n.pow(2);
        let lim = res[res.len() - 1];
        res.pop();
        res.iter().for_each(|x| area -= x.pow(2));
        for i in (0..=lim - 1).rev() {
            if area - i.pow(2) >= 0 {
                res.push(i);
                area -= i.pow(2);

                if area == 0 {
                    res.reverse();
                    return Some(res);
                }
            }
        }
    }
    None
}
#[allow(dead_code, unused_variables)]
#[inline]
fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {
    testing(50, Some(vec![1, 3, 5, 8, 49]));
    testing(44, Some(vec![2, 3, 5, 7, 43]));
    testing(625, Some(vec![2, 5, 8, 34, 624]));
    testing(5, Some(vec![3, 4]));
}

#[allow(dead_code, unused_variables)]
fn doubles(maxk: i32, maxn: i32) -> f64 {
    let mut result = 0.0;

    for k in 1..=maxk {
        let twok = 2 * k;
        for n in 1..=maxn {
            result += 1.0 / (k as f64 * ((n + 1) as f64).powi(twok));
        }
    }
    result
}
