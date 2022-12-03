use std::{io, env};

#[allow(non_snake_case, dead_code, unused_variables)]
fn main() {
    // get string and set variables
    println!("enter something -> ");
    let stdin = io::stdin();
    let input = &mut String::new();
    stdin.read_line(input).err();

    // convert to binary
    let mut input_string_binary = "".to_string();
    for character in input.clone().into_bytes(){
        input_string_binary += &format!("{:032b}", character);
    }
    let input_string_binary_length = input_string_binary.len();

    // append a single 1 and pad with 0's until data is a multiple of 16
    let mut string_binary_plus_one = input_string_binary + "1";
    let mut string_binary_padded = "".to_string();
    loop {
        let string_length = string_binary_plus_one.len();
        if string_length % 64 != 0 {
            string_binary_plus_one += "0";
        }
        else {
            string_binary_padded = string_binary_plus_one;
            break;
        }
    }

    // append 64 bits to the end, where the 64 bits are a big-endian integer representing the length of the original input in binary
    let input_binary_length = format!("{:032b}", input_string_binary_length);
    let mut trailing_zeroes = "".to_string();
    let mut input_binary_lengh_appended = input_binary_length;
    loop{
        if input_binary_lengh_appended.len() + trailing_zeroes.len() != 32 {
            trailing_zeroes += "0"
        }else{
            input_binary_lengh_appended = string_binary_padded.clone() + &trailing_zeroes + &input_binary_lengh_appended;
            break;
        }
    }

    // initialize 2 hash values (h), hard-coded constants that represent the first 32 bits of the fractional parts of the square roots of the first 2 primes, 2, 3
    let mut H0: String = "10011010010011100111101111001100".to_string(); // 2
    let mut H1: String = "10001000010110101110011100010101".to_string(); // 3
    
    // initialize 8  round constants (k) of the first 32 bits of the fractional parts of the cube roots of the first 8 primes
    let K0: String = "00000001100011001001101110101101".to_string();
    let K1: String = "00000010101000101101000110111001".to_string();
    let K2: String = "00000100001110110101011001011011".to_string();
    let K3: String = "00000101011100010000010010111110".to_string();
    let K4: String = "00000001010101011100010000111001".to_string();
    let K5: String = "00000010000110100001100000011101".to_string();
    let K6: String = "00000011011001111011010011011111".to_string();
    let K7: String = "00010011111110111110011001100101".to_string();
    let K8: String = "10000011111110111110011001100101".to_string();
    let K9: String = "10000011111110111110011001100101".to_string();
    let K10: String = "00100011111110111110011001100101".to_string();
    let K11: String = "00110011000110111110011001100111".to_string();
    let K12: String = "01000010101000101101000110111101".to_string();
    let K12: String = "10010100001110110101011001011011".to_string();
    let K13: String = "00100101011100010010010110111110".to_string();
    let K14: String = "00011001010101011101010000111001".to_string();
    let K15: String = "00010010000110000101100000011101".to_string();
    let K16: String = "00001011011001111011010011011111".to_string();
    let K17: String = "00100011111110110110011001100101".to_string();
    let K18: String = "00000011111110101110011001100101".to_string();
    let K19: String = "00011011111110011110011001100101".to_string();
    let K20: String = "00000011111010111110011001100101".to_string();
    let K21: String = "00000010111000101101000110111101".to_string();
    let K22: String = "00110010010110110101011001011011".to_string();
    let K23: String = "00000101011101010000010110111110".to_string();
    let K24: String = "00100001010101011100010000111001".to_string();
    let K25: String = "01000010000110001001100000011101".to_string();
    let K26: String = "00000011011001111011010011011111".to_string();
    let K27: String = "01000011101110100110011001100101".to_string();
    let K28: String = "00110001101110111110011001100101".to_string();
    let K29: String = "01001011111110111110011001100101".to_string();
    let K30: String = "00000110110110111110011001100101".to_string();
    let K31: String = "01001010101000101101000110111101".to_string();
    let K32: String = "01100100001110110111011001011011".to_string();
    let K33: String = "00010101011100010010010110111110".to_string();
    let K34: String = "00010001010101011100010000111001".to_string();
    let K35: String = "10101010000110000001100000011101".to_string();
    let K36: String = "10010011011001111011010011011111".to_string();
    let K37: String = "10101011111110111110011001100101".to_string();
    let K38: String = "00000011101010111110011001100101".to_string();
    let K39: String = "00100011111110110110011001100101".to_string();
    let K40: String = "01000011111110111110011001100101".to_string();
    let K41: String = "10000010101000101101000110111101".to_string();
    let K42: String = "00000100001110111101011001011011".to_string();
    let K43: String = "00000101011101010100010110111110".to_string();
    let K44: String = "01010100010101011100010000111001".to_string();
    let K45: String = "01000010000110000101100000011101".to_string();
    let K46: String = "00100110111001111011010011011111".to_string();
    let K47: String = "00010000111110111110011001100101".to_string();
    let K48: String = "10000001011010110110001001100101".to_string();
    let K: Vec<String> = vec![K0.clone(), K1.clone(), K2.clone(), K3.clone(), K4.clone(), K5.clone(), K6.clone(), K7.clone(), K8.clone(), K9.clone(), K10.clone(), K11.clone(), K12.clone(), K13.clone(), K14.clone(), K15.clone(), K16.clone(), K17.clone(), K18.clone(), K19.clone(), K20.clone(), K21.clone(), K22.clone(), K23.clone(), K24.clone(), K25.clone(), K26.clone(), K27.clone(), K28.clone(), K29.clone(), K30.clone(), K31.clone(), K32.clone(), K33.clone(), K34.clone(), K35.clone(), K36.clone(), K37.clone(), K38.clone(), K39.clone(), K40.clone(), K41.clone(), K42.clone(), K43.clone(), K44.clone(), K45.clone(), K46.clone(), K47.clone(), K48.clone()] ;
    

    // following steps will happen for each 16 bit chunk of data from our input. At each iteration of the loop, we will be mutating the has values H0 H1, which will be our final output
    // Copy input of input_binary_lengh_appended into a new array (w) where each element is a 16 bit word
    let mut word_array: Vec<String> = vec![String::new(); 0];
    let mut current_word = "".to_string();
    let char1: Vec<char> = input_binary_lengh_appended.clone().chars().collect();
    for binary_num in 0..input_binary_lengh_appended.len() {
        if current_word.len() == 32 {
            word_array.push(current_word);
            current_word="".to_string();
            current_word += &char1[binary_num].to_string();
        } 
        else {
            current_word += &char1[binary_num].to_string();
        }
    }

    // add 48 more 16 bit words initialized to 0, such that we have w[0..48]
    for _n in word_array.len()..48 {
        word_array.push("00000000000000000000000000000000".to_string());
    }

    /*  Modify the zero-ed indexes at the end of the array using the following algorithm:
        For i from w[16…63]:                                                                    || actually, don't ask ||
        s0 = (w[i-15] rightrotate 7) xor (w[i-15] rightrotate 18) xor (w[i-15] rightshift 3)
        s1 = (w[i- 2] rightrotate 17) xor (w[i- 2] rightrotate 19) xor (w[i- 2] rightshift 10)
        w[i] = w[i-16] + s0 + w[i-7] + s1                                                   */
        env::set_var("RUST_BACKTRACE", "1");

        // function that rotaterights strings
        pub enum Direction {
            Left,
            Right,
        }
        pub fn rotate(str: &str, direction: Direction, count: usize) -> String {
            let mut str_vec: Vec<char> = str.chars().collect();
            match direction {
                Direction::Left => { str_vec.rotate_left(count) }
                Direction::Right => { str_vec.rotate_right(count) }
            }
            str_vec.iter().collect()
        }

        // function that xors strings
        pub fn xor_string(str: &str, str2: &str) -> Vec<String> {
            let mut current_iteration = 0;
            let mut xord_string = "".to_string();
            let mut returned_value = Vec::new();
            let char1: Vec<char> = str.chars().collect();
            let char2: Vec<char> = str2.chars().collect();

            for char in char1 {               
                if !(char == char2[current_iteration]){
                    xord_string += &char.to_string();
                }
                else {
                    xord_string += "0";
                }
                current_iteration += 1; 
            }
            returned_value.push(xord_string.to_string());
            return returned_value;
        }

        let mut s0 = String::new();
        let mut s1 = String::new();
        let mut intermediary_s0 = String::new();
        let mut intermediary_s1 = String::new();
        for i in 6..48 {
            intermediary_s0 = xor_string(&rotate(&word_array[i-6], Direction::Right, 7),&rotate(&word_array[i-6], Direction::Right, 3)).into_iter().collect();
            s0 = xor_string(&intermediary_s0, &rotate(&word_array[i-6], Direction::Right, 3)).into_iter().collect();

            intermediary_s1 = xor_string(&rotate(&word_array[i-1], Direction::Right, 14),&rotate(&word_array[i-1], Direction::Right, 15)).into_iter().collect();
            s1 = xor_string(&intermediary_s1, &rotate(&word_array[i-1], Direction::Right, 10)).into_iter().collect();
            
            word_array[i] = format!("{:032b}",(i128::from_str_radix(&word_array[i-6].to_string(), 2).expect("Not a binary number") + i128::from_str_radix(&s0, 2).expect("Not a binary number") + i128::from_str_radix(&word_array[i-3].to_string(), 2).expect("Not a binary number") + i128::from_str_radix(&s1, 2).expect("Not a binary number")) % 32).to_string();
            
        }


        //  yea I'm not doing this, I don't have that many hashes
        /*  Initialize variables a, b, c, d, e, f, g, h and set them equal to the current hash values respectively. h0, h1, h2, h3, h4, h5, h6, h7
            Run the compression loop. The compression loop will mutate the values of a…h. The compression loop is as follows:
            for i from 0 to 63
            S1 = (e rightrotate 6) xor (e rightrotate 11) xor (e rightrotate 25)
            ch = (e and f) xor ((not e) and g)
            temp1 = h + S1 + ch + k[i] + w[i]
            S0 = (a rightrotate 2) xor (a rightrotate 13) xor (a rightrotate 22)
            maj = (a and b) xor (a and c) xor (b and c)
            temp2 := S0 + maj
            h = g
            g = f
            f = e
            e = d + temp1
            d = c
            c = b
            b = a
            a = temp1 + temp2 */

                    // function that xors strings
        pub fn and_string(str: &str, str2: &str) -> Vec<String> {
            let mut current_iteration = 0;
            let mut xord_string = "".to_string();
            let mut returned_value = Vec::new();
            let char1: Vec<char> = str.chars().collect();
            let char2: Vec<char> = str2.chars().collect();

            for char in char1 {               
                if char == char2[current_iteration] {
                    xord_string += &char.to_string();
                }
                else {
                    xord_string += "0";
                }
                current_iteration += 1; 
            }
            returned_value.push(xord_string.to_string());
            return returned_value;
        }

        let mut a = word_array[3].clone();
        let mut b = word_array[2].clone();
        for i in 0..48 {
            let s1: String = xor_string(&b, &a).into_iter().collect();
            
            let a_and_b: String = and_string(&a, &b).into_iter().collect();
            let s0: String = xor_string(&a_and_b, &b).into_iter().collect();

            let ch: String = xor_string(&s0, &a).into_iter().collect();

            let temp1 = format!("{:032b}", (i128::from_str_radix(&ch.to_string(), 2).expect("not a binary number") % 32 + i128::from_str_radix(&s1, 2).expect("not a binary number") + i128::from_str_radix(&ch, 2).expect("not a binary number") % 32 + i128::from_str_radix(&K[i], 2).expect("not a binary number") % 32 + i128::from_str_radix(&word_array[i], 2).expect("not a binary number") % 32)).to_string();
            let maj = format!("{:032b}", i128::from_str_radix(&a_and_b, 2).expect("not a binary number") % 32 + i128::from_str_radix(&K[i], 2).expect("Not a binary numberr :)") % 32);
            let temp2 = format!("{:032b}", (i128::from_str_radix(&s0.to_string(), 2).expect("not a binary number") % 32 + i128::from_str_radix(&maj, 2).expect("not a binary number") % 32)).to_string();
            a = rotate(&maj, Direction::Right, 6);
            b = rotate(&temp1, Direction::Right, 11);
            a = xor_string(&a, &temp1).clone().into_iter().collect();
            b = xor_string(&b, &temp2).into_iter().collect();
            

        }
        H0 += &a;
        H1 += &b;
        let H0Final = format!("{:032b}", u128::from_str_radix(&H0, 2).expect("not a binary number"));
        let H1Final = format!("{:032b}", u128::from_str_radix(&H1, 2).expect("not a binary number"));

        let digest_bits = H0Final + &H1Final;
        let output_binary = u128::from_str_radix(&digest_bits, 2);
        let digest_string = output_binary;

    println!("input: {} digest bits: {:?}\ndigest decimal: {:?}", input, digest_bits, digest_string);
}