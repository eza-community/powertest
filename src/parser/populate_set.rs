#[inline]
pub fn populate_set(
    config: &crate::data::Config,
    parse: &Vec<(Option<String>, Option<String>)>,
    set: &mut Vec<String>,
) {
    for option in parse {
        if let Some((key, stuff)) = config.commands.as_ref().unwrap().get_key_value(&option) {
            match key {
                (Some(left), Some(right)) => {
                    if let Some(prefix) = stuff.prefix.as_ref() {
                        for value in stuff.values.as_ref().unwrap_or(&vec!["".to_string()]) {
                            set.push(format!("{} {} {}", prefix, left, &value));
                            set.push(format!("{} {} {}", prefix, right, &value));
                        }
                    } else {
                        for value in stuff.values.as_ref().unwrap_or(&vec!["".to_string()]) {
                            set.push(format!("{} {}", left, &value));
                            set.push(format!("{} {}", right, &value));
                        }
                    }
                }
                (Some(left), None) => {
                    if let Some(prefix) = stuff.prefix.as_ref() {
                        for value in stuff.values.as_ref().unwrap_or(&vec!["".to_string()]) {
                            set.push(format!("{} {} {}", prefix, left, &value));
                        }
                    } else {
                        for value in stuff.values.as_ref().unwrap_or(&vec!["".to_string()]) {
                            set.push(format!("{} {}", left, &value));
                        }
                    }
                }
                (None, Some(right)) => {
                    if let Some(prefix) = stuff.prefix.as_ref() {
                        for value in stuff.values.as_ref().unwrap_or(&vec!["".to_string()]) {
                            set.push(format!("{} {} {}", prefix, right, &value));
                        }
                    } else {
                        for value in stuff.values.as_ref().unwrap_or(&vec!["".to_string()]) {
                            set.push(format!("{} {}", right, &value));
                        }
                    }
                }
                (None, None) => todo!(),
            }
        } else {
            match &option {
                (Some(left), Some(right)) => {
                    set.push(left.to_string());
                    set.push(right.to_string());
                }
                (Some(left), None) => {
                    set.push(left.to_string());
                }
                (None, Some(right)) => {
                    set.push(right.to_string());
                }
                (None, None) => todo!(),
            }
        }
    }
}
