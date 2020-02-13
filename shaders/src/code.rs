pub fn gv_if(width: u32, height: u32) -> String {
    format!(
        "    if (gv.x >= 0. && gv.y >= 0. &&
            gv.x <= {}. && gv.y <= {}.) {{
",
        width - 1,
        height - 1
    )
}

pub fn q_fn(width: u32) -> String {
    let mut out = String::from("float Q(float m, int y, int i, ");
    for arg in 0..width {
        out.push_str(&format!("float r{}", arg))
    }

    out.push_str(
        ") {
        if (y == i) {
            return ",
    );

    for arg in 0..width {
        out.push_str(&format!("({}+4.*", arg))
    }

    out.push_str("*1.");
    for _ in 0..width {
        out.push_str(")")
    }

    out.push_str(
        ";
} else {
    return m;
}
}

",
    );

    out
}
