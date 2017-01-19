use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct IPv7 {
    pub super_nets: Vec<String>,
    pub hyper_nets: Vec<String>,
}

impl IPv7 {
    /// Creates a new `IPv7`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libday_7::IPv7;
    ///
    /// let ipv7 = IPv7::new("abba[mnop]qrst".to_string());
    /// ```
    pub fn new(ipv7_address: String) -> IPv7 {
        let hyper_net_re: Regex = Regex::new(r"\[\w+\]").unwrap();


        let hyper_net_data: Vec<String> =
            hyper_net_re.find_iter(&ipv7_address).map(|found| found.as_str().to_string()).collect();
        let super_net_data: Vec<String> =
            hyper_net_re.split(&ipv7_address).map(|found| found.to_string()).collect();

        IPv7 {
            super_nets: super_net_data,
            hyper_nets: hyper_net_data,
        }
    }


    /// Returns `true` if the Ipv7 address supports TLS.
    ///
    /// # Examples
    ///
    /// ```
    /// # use libday_7::IPv7;
    /// let ipv7_1 = IPv7::new("abba[mnop]qrst".to_string());
    /// let ipv7_2 = IPv7::new("abcd[bddb]xyyx".to_string());
    ///
    /// assert!(ipv7_1.supports_tls());
    /// assert!(!ipv7_2.supports_tls());
    /// ```
    pub fn supports_tls(&self) -> bool {
        for hyper_net in &self.hyper_nets {
            if has_abba(&hyper_net) {
                return false;
            }
        }

        for super_net in &self.super_nets {
            if has_abba(&super_net) {
                return true;
            }
        }

        false
    }
}

pub fn has_abba(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();

    if chars.len() < 4 {
        return false;
    }

    for index in 0..(chars.len() - 3) {
        if (chars[index] == chars[index + 3]) && (chars[index + 1] == chars[index + 2]) &&
           (chars[index] != chars[index + 1]) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let test_ipv7: IPv7 = IPv7::new("zaz[dfef]bz[bzb]cdb".to_string());
        let test_hyper_nets: Vec<String> = vec!["[dfef]".to_string(), "[bzb]".to_string()];
        let test_super_nets: Vec<String> =
            vec!["zaz".to_string(), "bz".to_string(), "cdb".to_string()];

        assert_eq!(test_ipv7.hyper_nets, test_hyper_nets);
        assert_eq!(test_ipv7.super_nets, test_super_nets);
    }


    #[test]
    fn test_has_abba() {
        assert!(has_abba("ioxxoj"));
        assert!(has_abba("vzylvdgblzozzonhcr"));

        assert!(!has_abba("xxo"));
        assert!(!has_abba("vzylvdgblzoznhcr"));
    }


    #[test]
    fn test_supports_tls() {
        let test_data: Vec<IPv7> = vec![IPv7::new("abba[mnop]qrst".to_string()),
                                        IPv7::new("abcd[bddb]xyyx".to_string()),
                                        IPv7::new("aaaa[qwer]tyui".to_string()),
                                        IPv7::new("ioxxoj[asdfgh]zxcvbn".to_string()),
                                        IPv7::new("aba[bab]xyz".to_string()),
                                        IPv7::new("xyx[xyx]xyx".to_string()),
                                        IPv7::new("aaa[kek]eke".to_string()),
                                        IPv7::new("zazbz[bzb]cdb".to_string())];

        assert!(test_data[0].supports_tls());
        assert!(!test_data[1].supports_tls());
        assert!(!test_data[2].supports_tls());
        assert!(test_data[3].supports_tls());
        assert!(!test_data[4].supports_tls());
        assert!(!test_data[5].supports_tls());
        assert!(!test_data[6].supports_tls());
        assert!(!test_data[7].supports_tls());

    }
}