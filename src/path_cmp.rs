use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path::Path;

#[cfg(windows)]
type OsBytes = Vec<u16>;
#[cfg(not(windows))]
type OsBytes = Vec<u8>;

macro_rules! new_elem {
    ($ret: ident, $elem: ident, $b: ident) => {
        $ret.push($elem.clone());
        if is_int($b) {
            $elem = Element::Integer(($b - 0x30).into());
        } else if is_str($b) {
            $elem = Element::Str(to_str($b).unwrap());
        } else {
            $elem = Element::Bytes(vec![$b]);
        }
    };
}

#[derive(Clone, Debug)]
enum Element {
    Bytes(OsBytes),
    Str(String),
    Integer(u32),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Element::Bytes(b1) => match other {
                Element::Bytes(b2) => b1.cmp(b2),
                Element::Str(_) => Ordering::Greater,
                Element::Integer(_) => Ordering::Greater,
            },
            Element::Str(s1) => match other {
                Element::Bytes(_) => Ordering::Less,
                Element::Str(s2) => s1.cmp(s2),
                Element::Integer(_) => Ordering::Less,
            },
            Element::Integer(i1) => match other {
                Element::Bytes(_) => Ordering::Less,
                Element::Str(_) => Ordering::Greater,
                Element::Integer(i2) => i1.cmp(i2),
            },
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Element {}

#[cfg(windows)]
fn osstr_to_osbytes(s: &OsStr) -> OsBytes {
    use std::os::windows::ffi::OsStrExt;
    s.encode_wide().collect()
}

#[cfg(not(windows))]
fn osstr_to_osbytes(s: &OsStr) -> OsBytes {
    use std::os::unix::ffi::OsStrExt;
    s.as_bytes().into()
}

#[cfg(windows)]
fn is_int(b: u16) -> bool {
    match String::from_utf16(&[b]) {
        Ok(s) => {
            let st = s.into_bytes();
            st.len() == 1 && st[0].is_ascii_digit()
        }
        Err(_) => false,
    }
}

#[cfg(not(windows))]
fn is_int(b: u8) -> bool {
    b.is_ascii_digit()
}

#[cfg(windows)]
fn is_str(b: u16) -> bool {
    to_str(b).is_ok()
}

#[cfg(not(windows))]
fn is_str(b: u8) -> bool {
    to_str(b).is_ok()
}

#[cfg(windows)]
fn to_str(b: u16) -> Result<String, ()> {
    String::from_utf16(&[b]).map_err(|_| ())
}

#[cfg(not(windows))]
fn to_str(b: u8) -> Result<String, ()> {
    String::from_utf8(vec![b]).map_err(|_| ())
}

fn str_to_chunks(s: &OsStr) -> Vec<Element> {
    let mut ret = vec![];
    let mut elem = Element::Str(String::new());
    for b in osstr_to_osbytes(s) {
        match &elem {
            Element::Bytes(bt) => {
                if !is_str(b) {
                    let mut new_vec = bt.to_owned();
                    new_vec.push(b);
                    elem = Element::Bytes(new_vec);
                } else {
                    new_elem!(ret, elem, b);
                }
            }
            Element::Str(st) => {
                if is_str(b) && !is_int(b) {
                    let new_str = st.to_owned() + &to_str(b).unwrap();
                    elem = Element::Str(new_str);
                } else {
                    new_elem!(ret, elem, b);
                }
            }
            Element::Integer(it) => {
                if is_int(b) {
                    elem = Element::Integer((it * 10) + ((b as u32) - 0x30));
                } else {
                    new_elem!(ret, elem, b);
                }
            }
        };
    }
    ret.push(elem);
    ret
}

fn path_to_chunks(p: &Path) -> Vec<Element> {
    let mut ret = vec![];
    for e in p.iter() {
        if !e.is_empty() {
            let ne = e.to_ascii_lowercase();
            let mut chunks = str_to_chunks(&ne);
            ret.append(&mut chunks);
        }
    }
    ret
}

pub fn path_cmp_name(a: &Path, b: &Path) -> Ordering {
    let a = path_to_chunks(a);
    let b = path_to_chunks(b);
    let mut a_it = a.iter();
    let mut b_it = b.iter();
    loop {
        let a = a_it.next();
        let b = b_it.next();
        if a.is_none() && b.is_none() {
            return Ordering::Equal;
        } else if a.is_none() {
            return Ordering::Less;
        } else if b.is_none() {
            return Ordering::Greater;
        }
        let ret = a.cmp(&b);
        if !ret.is_eq() {
            return ret;
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_str_eq() {
        let p1 = Path::new("./some/easy/path.txt");
        let p2 = Path::new("./some/easy/path.txt");
        assert!(path_cmp_name(p1, p2).is_eq());
    }

    #[test]
    fn test_str_eq_case() {
        let p1 = Path::new("./Some/EASY/Path.txt");
        let p2 = Path::new("./SomE/easy/PATH.txt");
        assert!(path_cmp_name(p1, p2).is_eq());
    }

    #[test]
    fn test_str() {
        let p1 = Path::new("./some/easy/path/aaaa.txt");
        let p2 = Path::new("./some/easy/path/abaa.txt");
        assert!(path_cmp_name(p1, p2).is_lt());
    }

    #[test]
    fn test_str_case() {
        let p1 = Path::new("./Some/EASY/Path/aaaa.txt");
        let p2 = Path::new("./SomE/easy/PATH/abaa.txt");
        assert!(path_cmp_name(p1, p2).is_lt());
    }

    #[test]
    fn test_int() {
        let p1 = Path::new("./some/path/1_test.txt");
        let p2 = Path::new("./some/path/02_test.txt");
        let p3 = Path::new("./some/path/5_test.txt");
        let p4 = Path::new("./some/path/10_test.txt");
        let p5 = Path::new("./some/path/21_test.txt");

        assert!(path_cmp_name(p1, p2).is_lt());
        assert!(path_cmp_name(p1, p3).is_lt());
        assert!(path_cmp_name(p1, p4).is_lt());
        assert!(path_cmp_name(p1, p5).is_lt());

        assert!(path_cmp_name(p2, p1).is_gt());
        assert!(path_cmp_name(p2, p3).is_lt());
        assert!(path_cmp_name(p2, p4).is_lt());
        assert!(path_cmp_name(p2, p5).is_lt());

        assert!(path_cmp_name(p3, p1).is_gt());
        assert!(path_cmp_name(p3, p2).is_gt());
        assert!(path_cmp_name(p3, p4).is_lt());
        assert!(path_cmp_name(p3, p5).is_lt());

        assert!(path_cmp_name(p4, p1).is_gt());
        assert!(path_cmp_name(p4, p2).is_gt());
        assert!(path_cmp_name(p4, p3).is_gt());
        assert!(path_cmp_name(p4, p5).is_lt());

        assert!(path_cmp_name(p5, p1).is_gt());
        assert!(path_cmp_name(p5, p2).is_gt());
        assert!(path_cmp_name(p5, p3).is_gt());
        assert!(path_cmp_name(p5, p4).is_gt());
    }
}
