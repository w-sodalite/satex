use std::borrow::Cow;

const SEP: &str = "/";

const EMPTY: &str = "";

pub fn canonicalize(path: &str) -> Cow<'_, str> {
    if path.is_empty() || path == SEP {
        return Cow::from(SEP);
    }
    let mut legal = true;
    let components = path
        .split(SEP)
        .enumerate()
        .filter(|(index, component)| {
            if component.is_empty() {
                // 如果以/开头则会产生一个空字符串
                legal = *index == 0;
                false
            } else if *component == "." {
                legal = false;
                false
            } else if *component == ".." {
                legal = false;
                true
            } else {
                true
            }
        })
        .map(|(_, component)| component)
        .collect::<Vec<_>>();
    match legal {
        true => {
            if path.starts_with('/') {
                path.into()
            } else {
                let mut value = String::with_capacity(path.len() + 1);
                value.push_str(SEP);
                value.push_str(path);
                value.into()
            }
        }
        false => {
            let mut parts = Vec::with_capacity(components.len());
            for component in components.into_iter() {
                if component == ".." {
                    parts.pop();
                } else {
                    parts.push(component);
                }
            }
            if parts.is_empty() {
                Cow::from(SEP)
            } else {
                parts.insert(0, EMPTY);
                parts.join(SEP).into()
            }
        }
    }
}

pub fn remove_start_sep(path: &str) -> &str {
    if path.is_empty() {
        return path;
    }
    let index = path
        .chars()
        .enumerate()
        .find(|(_, c)| *c != '/')
        .map(|(index, _)| index);
    match index {
        Some(index) => &path[index..],
        None => EMPTY,
    }
}

pub fn remove_end_sep(path: &str) -> &str {
    if path.is_empty() {
        return path;
    }
    let index = path
        .chars()
        .rev()
        .enumerate()
        .find(|(_, c)| *c != '/')
        .map(|(index, _)| index);
    match index {
        Some(index) => &path[..path.len() - index],
        None => EMPTY,
    }
}
