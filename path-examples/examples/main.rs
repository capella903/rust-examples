use std::path::PathBuf;

fn main() {
    let mut path_buf = PathBuf::from("foo/bar");
    dbg!(path_buf.join("hoge"));
    dbg!(path_buf.join("fuga"));
    dbg!(path_buf.clone());

    path_buf.push("homu");
    dbg!(path_buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_join() {
        let path = Path::new("foo/bar");

        // Pathはimmutableなものなので新たなPathを生成するためにはjoinする、pushとかはない
        assert_eq!(path.join("hoge"), Path::new("foo/bar/hoge"));
        assert_eq!(path, Path::new("foo/bar"));

        let mut path_buf = PathBuf::from("foo/bar");

        assert_eq!(path_buf.join("hoge"), PathBuf::from("foo/bar/hoge"));
        // joinでは元のpath_bufは弄られない
        assert_eq!(path_buf, PathBuf::from("foo/bar"));
        // pushでは元のpath_bufが弄られる
        path_buf.push("fuga");
        assert_eq!(path_buf, PathBuf::from("foo/bar/fuga"));
    }
}
