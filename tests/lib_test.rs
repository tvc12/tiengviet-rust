#[cfg(test)]
mod tests {
    use tiengviet::TiengViet;

    #[test]
    fn it_work_none_vietnamese() {
        let tiengviet = TiengViet::new();
        let result = tiengviet.parse(&String::from("Tests can be grouped "));
        assert_eq!(result, String::from("Tests can be grouped "));
        let result = tiengviet.parse(&String::from("Each group\'s description is added to the beginning of its test\'s descriptions"));
        assert_eq!(result, String::from("Each group\'s description is added to the beginning of its test\'s descriptions"));
    }

    #[test]
    fn it_work_vietnamese_unsign() {
        let tiengviet = TiengViet::new();
        let result = tiengviet.parse(&String::from("viet nam"));
        assert_eq!(result, String::from("viet nam"));
        let result = tiengviet.parse(&String::from("Vi Chi Thien"));
        assert_eq!(result, String::from("Vi Chi Thien"));
        // Code để kiểm tra tính chính xác của module my_module
    }

    #[test]
    fn it_work_vietnamese_sign() {
        let tiengviet = TiengViet::new();
        let result = tiengviet.parse(&String::from("Văn học theo cách nói chung nhất, là bất kỳ tác phẩm nào bằng văn bản"));
        assert_eq!(result, String::from("Van hoc theo cach noi chung nhat, la bat ky tac pham nao bang van ban"));
        let result = tiengviet.parse(&String::from("VĂN HỌC THEO CÁCH NÓI CHUNG NHẤT, LÀ BẤT KỲ TÁC PHẨM NÀO BẰNG VĂN BẢN"));
        assert_eq!(result, String::from("VAN HOC THEO CACH NOI CHUNG NHAT, LA BAT KY TAC PHAM NAO BANG VAN BAN"));
        let result = tiengviet.parse(&String::from("Bạn đang có ý tưởng về 1 công cụ nào đó"));
        assert_eq!(result, String::from("Ban dang co y tuong ve 1 cong cu nao do"));
    }

    #[test]
    fn it_work_empty_string() {
        let tiengviet = TiengViet::new();
        let result = tiengviet.parse(&String::from(""));
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn it_work_one_char() {
        let tiengviet = TiengViet::new();
        let result = tiengviet.parse(&String::from("a"));
        assert_eq!(result, String::from("a"));
    }

    #[test]
    fn it_work_number() {
        let tiengviet = TiengViet::new();
        let result = tiengviet.parse(&String::from("1234567890"));
        assert_eq!(result, String::from("1234567890"));
    }

    #[test]
    fn it_work_mixed() {
        let tiengviet = TiengViet::new();
        let result = tiengviet.parse(&"360–400 million (2006);".to_string());
        assert_eq!(result, String::from("360–400 million (2006);"));
        let result = tiengviet.parse(&"B2 upper intermediate: An interview".to_string());
        assert_eq!(result, String::from("B2 upper intermediate: An interview"));

        let actual: String = tiengviet.parse(&"Cuoi the ky thu 19 tro di".to_string());
        assert_eq!(actual, "Cuoi the ky thu 19 tro di");

        let actual = tiengviet.parse(&"Sau hơn 10 thế kỷ hình thành và phát triển".to_string());
        assert_eq!(actual, "Sau hon 10 the ky hinh thanh va phat trien");
    }
}