pub fn gv_if(width: u32, height: u32) -> String {
    format!(
        "    if (gv.x >= 0. && gv.y >= 0. &&
            gv.x <= {}. && gv.y <= {}.) {{
",
        width - 1,
        height - 1
    )
}
