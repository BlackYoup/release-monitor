// TODO: use iterators and map instead of that loop
pub fn extract_next_link(header_val: String) -> Option<String>{
    let fields: Vec<&str> = header_val.split(',').collect();
    let mut splitted_fields: Vec<Vec<&str>> = Vec::new();

    for field in fields{
        let tmp: Vec<&str> = field.split(';').collect();
        splitted_fields.push(tmp);
    }

    let mut res = None;
    for field in splitted_fields{
        if field[1].trim() == "rel=\"next\"" {
            res = Some(field[0].to_string().replace("<", "").replace(">", ""));
            break;
        }
    }

    return res;
}
