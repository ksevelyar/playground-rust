pub fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut flick = true;

    list.iter()
        .map(|item| match *item {
            "flick" => {
                flick = !flick;
                flick
            }
            _ => flick,
        })
        .collect()
}
