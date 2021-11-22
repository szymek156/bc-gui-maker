use crate::mocks_sharp_mip_2in7::common_params::display_dimension;

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
    fn page_1() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = h_layout([
            v_layout([
                v_layout([
                    tile("pace").with_font_size(16),
                    tile("10.20").with_format("%.2f").with_font_size(20),
                ]),
                h_line(),
                v_layout([
                    // tile("stride").with_font_size(16),
                    // tile("1.23").with_format("%.2f").with_font_size(20),
                    h_layout([
                        tile("lap time").with_font_size(16),
                        tile("02:03:04").with_format("%.2f").with_font_size(16),
                    ]),
                    h_layout([
                        tile("lap dist").with_font_size(16),
                        tile("21.37").with_format("%.2f").with_font_size(20),
                    ]),
                ]),
            ]),
            v_line(),
            v_layout([
                v_layout([
                    tile("HR zone").with_font_size(16),
                    tile("2.79").with_format("%.2f").with_font_size(20),
                ]),
                h_line(),
                v_layout([
                    tile("cadence").with_font_size(16),
                    tile("158").with_format("%3d").with_font_size(20),
                ]),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        invalidate_dimensions(&mut gui, &display_dimension);

        render_to_60fps(&gui, &display_dimension);

        render_to_bc(&gui, &display_dimension);
    }

    #[test]
    fn page_2() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = h_layout([
            v_layout([
                v_layout([
                    tile("total dist").with_font_size(16),
                    tile("10.20").with_format("%.2f").with_font_size(20),
                ]),
                h_line(),
                v_layout([
                    tile("lap dist").with_font_size(16),
                    tile("5.20").with_format("%.2f").with_font_size(20),
                ]),
            ]),
            v_line(),
            v_layout([
                v_layout([
                    tile("total time").with_font_size(16),
                    tile("02:12:20").with_format("%T").with_font_size(20),
                ]),
                h_line(),
                v_layout([
                    tile("lap time").with_font_size(16),
                    tile("01:12:20").with_format("%T").with_font_size(20),
                ]),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        invalidate_dimensions(&mut gui, &display_dimension);

        render_to_60fps(&gui, &display_dimension);

        render_to_bc(&gui, &display_dimension);
    }

    #[test]
    fn workout_steps_splash() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([h_line(), h_layout([tile("Workout Steps")])]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &display_dimension);

        render_to_60fps(&gui, &display_dimension);

        render_to_bc(&gui, &display_dimension);
    }

    #[test]
    fn page_3() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = h_layout([v_list([
            "run 5.00 1/5",
            "cool down 2 minutes",
            "run 5.00 2/5",
            "cool down 2 minutes",
        ])
        .with_font_size(16)]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &display_dimension);

        render_to_60fps(&gui, &display_dimension);

        render_to_bc(&gui, &display_dimension);
    }

    #[test]
    fn page_paused() {
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
                v_layout([tile("Paused"), h_line()]),
                v_list(["Resume", "Skip Step", "Save", "Discard"]).with_font_size(16),
            ]),
        ]);

        let mut gui = h_split(status_bar, 0.101, welcome_page);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &display_dimension);

        render_to_60fps(&gui, &display_dimension);

        render_to_bc(&gui, &display_dimension);
    }
}
