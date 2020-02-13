pub fn bitmap_gv_if(width: u32, height: u32) -> String {
    format!(
        "    if (gv.x >= 0. && gv.y >= 0. &&
            gv.x <= {}. && gv.y <= {}.) {{
",
        width - 1,
        height - 1
    )
}

pub fn define_q_fn(width: u32) -> String {
    let mut out = String::from("float Q(float m, int y, int i, ");
    for arg in 0..width {
        out.push_str(&format!("float r{}, ", arg))
    }

    out.push_str(
        ") {
        if (y == i) {
            return ",
    );

    for arg in 0..width {
        out.push_str(&format!("(r{}+4.*", arg))
    }

    out.push_str("1.");
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

const COLORS: &[&str] = &["_", "B", "D", "O"];
pub fn bitmap_q_calls(width: u32, height: u32) -> String {
    let mut out = String::new();
    for row in 0..height {
        out.push_str(&format!("      m = Q(m,y,{},", row));
        let mut wheelie = 0;
        for _ in 0..width {
            out.push_str(&format!("{},", COLORS[wheelie]));
            wheelie = (wheelie + 1) % 4;
        }
        out.push_str(
            ";
",
        )
    }
    out
}
