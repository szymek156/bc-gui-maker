#[cfg(test)]
mod test {
    use crate::{bc_render::render_to_bc, common::{
            h_layout, h_line, h_split, invalidate_dimensions, tile, v_layout, v_line, v_list,
            Dimension,
        }, mocks_sharp_mip_2in7::common_params::display_dimension, sixtyfps_render::render_to_60fps};

    use super::*;

    #[test]
    fn bc_test_page() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            v_line(),
            h_layout([
                v_layout([
                    tile("02/09/21").with_format("%d/%m/%y").with_font_size(24),
                    tile("19:34:19").with_format("%T").with_font_size(24),
                ]),
                v_layout([
                    tile("in view: 13")
                        .with_format("in view: %d")
                        .with_font_size(24),
                    tile("tracked: 11")
                        .with_format("tracked: %d")
                        .with_font_size(24),
                ]),
            ]),
            // Current implementation of invalidate_dimensions
            // makes {h,v}_line split tiles defined after them
            // so they 'stick' to the bottom
            h_line(),
            v_line(),
            h_layout([
                v_layout([
                    tile("23.19[*C]").with_format("%5.2f[*C]").with_font_size(24),
                    tile("8848.94[m]").with_format("%07.2f[m]").with_font_size(24),
                ]),
                v_layout([tile("")]),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        invalidate_dimensions(&mut gui, &display_dimension);

        render_to_60fps(&gui, &display_dimension);

        render_to_bc(&gui, &display_dimension);
    }
}
