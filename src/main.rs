use crypto::aes::Aes;

fn main() {
    let input: Vec<Vec<u8>> = vec![
        vec![0x32, 0x88, 0x31, 0xe0],
        vec![0x43, 0x5a, 0x31, 0x37],
        vec![0xf6, 0x30, 0x98, 0x07],
        vec![0xa8, 0x8d, 0xa2, 0x34],
    ];
    let key: Vec<u32> = vec![
        0x2b7e_1516,
        0x28ae_d2a6,
        0xabf7_1588,
        0x09cf_4f3c,
    ];
    let mut aes = Aes::new(input, &key, 128);
    print_state(&aes.state);
    aes.cipher();
    print_state(&aes.state);
    aes.inv_cipher();
    print_state(&aes.state);
}

fn print_state(state: &Vec<Vec<u8>>) {
    println!("== state ==");
    for i in 0..state.len() {
        for j in 0..state[0].len() {
            print!("{:02x} ", state[i][j]);
        }
        print!("\n");
    }
    println!("===========");
}
