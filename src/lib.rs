use std::collections::HashMap;
    
pub struct TiengViet {
    _dictionary: HashMap<char, char>
}


impl TiengViet {
    pub fn new() -> TiengViet {
        let mut dictionary: HashMap<char, char> = HashMap::new();
        for (c, c2) in "àáạảãâầấậẩẫăằắặẳẵÀÁẠẢÃÂẦẤẬẨẪĂẰẮẶẲẴèéẹẻẽêềếệểễÈÉẸẺẼÊỀẾỆỂỄòóọỏõôồốộổỗơờớợởỡÒÓỌỎÕÔỒỐỘỔỖƠỜỚỢỞỠùúụủũưừứựửữÙÚỤỦŨƯỪỨỰỬỮìíịỉĩÌÍỊỈĨđĐỳýỵỷỹỲÝỴỶỸ".chars().zip("aaaaaaaaaaaaaaaaaAAAAAAAAAAAAAAAAAeeeeeeeeeeeEEEEEEEEEEEoooooooooooooooooOOOOOOOOOOOOOOOOOuuuuuuuuuuuUUUUUUUUUUUiiiiiIIIIIdDyyyyyYYYYY".chars()) {
            dictionary.insert(c, c2);
        }
        return TiengViet {
            _dictionary: dictionary
        }
    }
    pub fn parse(&self, input: &String) -> String {

        let mut output: String = String::new();
        for c in input.chars() {
            match self._dictionary.get(&c) {
                Some(v) => output.push(*v),
                None => output.push(c)
            }
        }
        output
    }
}