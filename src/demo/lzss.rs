use std::convert::TryInto;

pub fn decompress(input: &[u8], output: &mut Vec<u8>) {
    decompress_(input, output);
}

/// inner fn that returns an option so we can use ? for short circuiting return
fn decompress_(input: &[u8], output: &mut Vec<u8>) -> Option<()> {
    let target_len = u32::from_le_bytes(input[0..4].try_into().unwrap()) as usize;

    let mut read_pos = 4;
    let mut read_byte = move || {
        let byte = *input.get(read_pos)?;
        read_pos += 1;
        Some(byte)
    };

    loop {
        let mut cmd_byte = read_byte()?;

        for _ in 0..8 {
            if cmd_byte & 0x01 == 0x01 {
                let pos = (read_byte()? as usize) << 4;
                let mixed = read_byte()? as usize;
                let pos = pos | (mixed >> 4);
                let count = (mixed & 0x0F) + 1;
                if count == 1 {
                    return None;
                }

                if output.len() + count > target_len {
                    return None;
                }

                let start = output.len() - pos - 1;
                // can't do extend_from_within since it start + count can be larger than output.len
                for i in 0..count {
                    output.push(output[start + i]);
                }
            } else {
                output.push(read_byte()?);
            }
            cmd_byte >>= 1;
        }
    }
}
