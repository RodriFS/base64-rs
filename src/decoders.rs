pub fn decode_faster(file: &mut Vec<u8>) -> Vec<u8> {
  let dec_table: [u8; 128] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, // 0-14
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, // 14-29
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, // 30-42
    62,  // 43 -> +
    255, 255, 255, // 44-46
    63,  // 47 -> /
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, // 48-57 -> 0-9
    255, 255, 255, 255, // 58-61
    255, 255, 255, // 62-64
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    25, // 65-90 -> A-Z
    255, 255, 255, 255, 255, 255, // 91-96
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, // 97-122 -> a-z
    255, 255, 255, 255, 255, // 123-127
  ];

  let array_length = file.len();
  let mut result: Vec<u8> = Vec::with_capacity(array_length * 3 / 4);
  let q1 = file[array_length - 1] == '=' as u8;
  let q2 = file[array_length - 2] == '=' as u8;

  for item in (0..if q1 { array_length - 4 } else { array_length }).step_by(4) {
    let byte1 = dec_table[file[item] as usize];
    let byte2 = dec_table[file[item + 1] as usize];
    let byte3 = dec_table[file[item + 2] as usize];
    let byte4 = dec_table[file[item + 3] as usize];
    let group1 = (byte1 << 2) | ((byte2 & 0xf0) >> 4);
    let group2 = ((byte2 & 0x0f) << 4) | ((byte3 & 0x3c) >> 2);
    let group3 = ((byte3 & 0x03) << 6) | (byte4 & 0x3f);
    result.push(group1);
    result.push(group2);
    result.push(group3);
  }

  if q1 {
    let starting_index = array_length - 4;
    let byte1 = dec_table[file[starting_index] as usize];
    let byte2 = dec_table[file[starting_index + 1] as usize];
    let group1 = (byte1 << 2) | ((byte2 & 0xf0) >> 4);
    result.push(group1);
    if !q2 {
      let byte3 = dec_table[file[starting_index + 2] as usize];
      let group2 = ((byte2 & 0x0f) << 4) | ((byte3 & 0x3c) >> 2);
      result.push(group2);
    }
  }
  result
}
