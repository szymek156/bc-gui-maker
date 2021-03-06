
#[cfg(test)]
mod test {
    use crate::{bc_render::render_to_bc, common::{
            h_layout, h_line, h_split, invalidate_dimensions, tile, v_layout, v_line, v_list,
            Dimension,
        }, mocks_sharp_mip_2in7::common_params::display_dimension, sixtyfps_render::render_to_60fps};

    use super::*;

    #[test]
    fn activity_paused() {
        let status_bar = h_layout([
            tile("21:12").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            v_line(),
            h_layout([
                v_layout([tile("Paused").with_font_size(42), h_line(), tile("")]),
                v_list(["Resume", "Save", "Discard"]).with_font_size(24),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        invalidate_dimensions(&mut gui, &display_dimension);

        render_to_60fps(&gui, &display_dimension);

        render_to_bc(&gui, &display_dimension);
    }
}
