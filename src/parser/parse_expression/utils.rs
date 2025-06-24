#[macro_export]
macro_rules! define_parsing_step {
    ( $func:ident, $tokens:expr, $pos:expr, $lhs_opt:expr ) => {{
        let (flag, new_lhs) = $func($tokens, $pos, $lhs_opt)?;
        $lhs_opt = new_lhs;

        match flag {
            LoopFlags::Break => break,
            LoopFlags::Continue => {
                continue;
            }
            LoopFlags::None => {}
        }
    }};

    ( $func:ident, $tokens:expr, $pos:expr, $min_bp:expr, $lhs_opt:expr ) => {{
        let (flag, new_lhs) = $func($tokens, $pos, $min_bp, $lhs_opt)?;
        $lhs_opt = new_lhs;

        match flag {
            LoopFlags::Break => break,
            LoopFlags::Continue => {
                continue;
            }
            LoopFlags::None => {}
        }
    }};
}
