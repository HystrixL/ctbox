#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn encrypt_test() {
        assert_eq!("2tHxu", encrypt(false, "ctbox"));
        assert_eq!("XWVGw", encrypt(false, "hhhil"));
        assert_eq!("jiIJthtt1SDW1sf6", encrypt(false, "20240323.hil_321"));
        assert_eq!("kIjhJ6tN", encrypt(false, "31415926"));
    }

    #[test]
    fn decrypt_test() {
        assert_eq!("DtyxH", encrypt(true, "ctbox"));
        assert_eq!("u%02R", encrypt(true, "hhhil"));
        assert_eq!("clsJp0021wyp1xxY", encrypt(true, "20240323.hil_321"));
        assert_eq!("rmIpDA0b", encrypt(true, "31415926"));
    }
}