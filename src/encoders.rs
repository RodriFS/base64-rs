pub fn encode(file: &mut Vec<u8>) -> String {
  let base_64_table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
  file.reverse();
  let mut result: Vec<char> = Vec::new();
  let mut padding = 0;
  while file.len() > 0 {
    let byte1 = file.pop().unwrap();
    let byte2 = match file.pop() {
      Some(b) => b,
      None => {
        padding += 1;
        0
      }
    };
    let byte3 = match file.pop() {
      Some(b) => b,
      None => {
        padding += 1;
        0
      }
    };
    let group1 = byte1 >> 2;
    let group2 = ((byte1 & 0x03) << 4) | ((byte2 & 0xf0) >> 4);
    let group3 = ((byte2 & 0x0f) << 2) | ((byte3 & 0xc0) >> 6);
    let group4 = byte3 & 0x3f;
    result.push(base_64_table[group1 as usize] as char);
    result.push(base_64_table[group2 as usize] as char);
    result.push(base_64_table[group3 as usize] as char);
    result.push(base_64_table[group4 as usize] as char);
  }
  let mut joined_result: String = result.into_iter().collect();
  let result_length = joined_result.len();
  joined_result.replace_range(result_length - padding..result_length, &"=".repeat(padding));
  joined_result
}

pub fn encode_faster(file: &mut Vec<u8>) -> Vec<u8> {
  let base_64_table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
  let array_length = file.len();
  let mut result: Vec<u8> = Vec::new();
  let completed_triples = (array_length / 3) * 3;

  for item in (0..completed_triples).step_by(3) {
    let byte1 = file[item];
    let byte2 = file[item + 1];
    let byte3 = file[item + 2];

    let group1 = byte1 >> 2;
    let group2 = ((byte1 & 0x03) << 4) | ((byte2 & 0xf0) >> 4);
    let group3 = ((byte2 & 0x0f) << 2) | ((byte3 & 0xc0) >> 6);
    let group4 = byte3 & 0x3f;
    result.push(base_64_table[group1 as usize]);
    result.push(base_64_table[group2 as usize]);
    result.push(base_64_table[group3 as usize]);
    result.push(base_64_table[group4 as usize]);
  }

  let rest = array_length - completed_triples;
  if rest > 0 {
    let byte1 = file[completed_triples];
    let group1 = byte1 >> 2;
    result.push(base_64_table[group1 as usize]);
    if rest > 1 {
      let byte2 = file[completed_triples + 1];
      let group2 = ((byte1 & 0x03) << 4) | ((byte2 & 0xf0) >> 4);
      let group3 = ((byte2 & 0x0f) << 2) | ((0 & 0xc0) >> 6);
      result.push(base_64_table[group2 as usize]);
      result.push(base_64_table[group3 as usize]);
    } else {
      let group2 = ((byte1 & 0x03) << 4) | ((0 & 0xf0) >> 4);
      result.push(base_64_table[group2 as usize]);
      result.push('=' as u8)
    }
    result.push('=' as u8);
  }

  result
}
