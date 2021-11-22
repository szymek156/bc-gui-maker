#[cfg(test)]
mod test {
    use crate::{
        bc_render::render_to_bc,
        common::{
            h_layout, h_line, h_split, invalidate_dimensions, tile, v_layout, v_line, v_list,
            Dimension,
        },
        sixtyfps_render::render_to_60fps,
    };

    use super::*;

    #[test]
    fn statistics_splash() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([h_line(), h_layout([tile("Statistics")])]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

    #[test]
    fn select_stats() {
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
                v_layout([tile("Stats"), h_line(), tile("")]),
                v_list([
                    "Running",
                    "Cycling",
                    "Hiking",
                    "Ind. Cycling",
                ])
                .with_font_size(16),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

    #[test]
    fn stats_selected() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([
            h_line(),
            h_split(
                v_layout([tile("Stats for: workout type"), h_line()]),
                0.2,
                v_layout([
                    tile("All time:Run for your life for 12 mins").with_font_size(12),
                    tile("5k: 50min").with_font_size(12),
                    tile("10k: 4hrs").with_font_size(12),
                    tile("Half M: 2 days 4hrs").with_font_size(12),
                    // TODO: all steps disappear after adding another element
                    // Because for .60fps font of size 16 does not fit in a
                    // rect of height 18 and clips the text
                    // tile("Step 5"),
                ]),
            ),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);

        render_to_bc(&gui, &d);
    }

}
