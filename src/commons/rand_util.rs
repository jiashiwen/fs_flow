use rand::Rng;

const CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~.,;:<>?/\\|}{[]`-=_+";
const PATH_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

//生成定长随机字符串
pub fn rand_string(len: usize) -> String {
    let mut rng = rand::rng();
    let rest: String = (0..len)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    rest
}

pub fn rand_path(len: usize) -> String {
    let mut rng = rand::rng();
    let rest: String = (0..len)
        .map(|_| {
            let idx = rng.random_range(0..PATH_CHARSET.len());
            PATH_CHARSET[idx] as char
        })
        .collect();
    rest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //cargo test commons::rand_util::test::test_rand_string -- --nocapture
    fn test_rand_string() {
        let len = 10;
        let s = rand_string(len);

        // 检查长度是否正确
        assert_eq!(s.len(), len);

        // 检查首字符是否为字母或数字
        let first_char = s.chars().next().unwrap();
        assert!(first_char.is_alphanumeric());
    }
}
