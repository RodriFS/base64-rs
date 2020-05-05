pub fn decode_faster(file: &mut Vec<u8>) -> String {
  let dec_table: [u8; 128] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, //
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, //
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, //
    62,  // -> +
    255, 255, 255, //
    63,  // -> /
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, // -> 0-9
    255, 255, 255, //
    64,  // -> =
    255, 255, 255, //
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, // -> A-Z
    255, 255, 255, 255, 255, 255, //
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, // -> a-z
    255, 255, 255, 255, 255,
  ];

  let array_length = file.len();
  let mut result: Vec<char> = Vec::new();

  for item in (0..array_length - 1).step_by(4) {
    let byte1 = dec_table[file[item] as usize];
    let byte2 = dec_table[file[item + 1] as usize];
    let byte3 = dec_table[file[item + 2] as usize];
    let byte4 = dec_table[file[item + 3] as usize];
    let group1 = (byte1 << 2) | ((byte2 & 0xf0) >> 4);
    let group2 = ((byte2 & 0x0f) << 4) | ((byte3 & 0x3c) >> 2);
    let group3 = ((byte3 & 0x03) << 6) | (byte4 & 0x3f);
    result.push(group1 as char);
    result.push(group2 as char);
    result.push(group3 as char);
  }
  let joined_result = result.into_iter().collect();
  joined_result
}
