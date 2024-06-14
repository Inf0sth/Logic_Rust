use rand::Rng;

pub struct Encrypt {
    message: String,
    key: u32,
    cripted: u16,
}


impl Encrypt{
    pub fn gen_key(&self) -> u32{
        let mut rng = rand::thread_rng();
        let mut matrix: [u32; 9] = [0;9];
        for val  in 0..8{
            matrix[val] = rng.gen_range(100..=999);
        }
        let n1: u32 = matrix[0] * matrix[4] * matrix[8];
        let n2: u32 = matrix[3] * matrix[7] * matrix[2];
        let n3: u32 = matrix[6] * matrix[1] * matrix[5];
        let a: u32 = n1 + n2 + n3;
        let n1: u32 = matrix[2] * matrix[4] * matrix[6];
        let n2: u32 = matrix[5] * matrix[7] * matrix[0];
        let n3: u32 = matrix[8] * matrix[1] * matrix[3];
        let b: u32 = n1 + n2 + n3;
        let r: u32 = a-b;
        &self.key = r;
    }

    fn message_to_matrix(&self) -> [u8]{
        let msg = &self.message;
        let mut matrix: [u8, msg.len()] = [0,msg.len()];
        let mut n: u8 = 0;
        for letter in msg.iter(){
            matrix[n] = msg.to_ascii_lowercase() as u8;
            n += 1;
        }
        matrix
    }

    fn get_key(&self) -> u32{
        &self.key
    }

    fn reverse_message(matrix: [u8]) -> u32{
        let mut rvs_msg: [u32,matrix.len()] = [0,matrix.len()];
        let new_mtrx = matrix.align_to::<u32>(shorts);
        for i in new_mtrx.iter(){

        }

    }

    pub fn encrypt_message(&self) -> &String{
        let ky = get_key();
        let matrix_msg = message_to_matrix();

    }

}


