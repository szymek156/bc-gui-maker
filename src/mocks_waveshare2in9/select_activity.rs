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
    fn activity_splash() {
        let status_bar = h_layout([
            tile("21:37").with_format("%T"),
            v_line(),
            tile("GPS 3D").with_format("GPS %1d"),
            v_line(),
            tile("02/09/21").with_format("%d/%m/%y"),
        ]);
        let welcome_page = v_layout([h_line(), h_layout([tile("Activities")])]);

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
    fn select_activity() {
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
                v_layout([tile("Activity"), h_line(), tile("")]),
                v_list([
                    "Running",
                    "Cycling",
                    "Hiking",
                    "Ind. Cycling",
                    "Yoga",
                    "Swimming",
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
    fn select_running_workouts() {
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
                v_layout([tile("Workouts"), h_line(), tile("Running")]),
                v_list(["5k", "10k", "Half Marathon", "Marathon", "Cooper Test"])
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
    fn activity_running_cooper_test() {
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
                v_layout([tile("Running"), h_line(), tile("Cooper Test")]),
                v_list(["Do It", "View"]).with_font_size(16),
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
    fn activity_running_cooper_test_view() {
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
                v_layout([tile("Cooper Test"), h_line()]),
                0.2,
                v_layout([
                    tile("Step 1: Warmup").with_font_size(12),
                    tile("Step 2: Run for your life for 12 mins").with_font_size(12),
                    tile("Step 3: Note the distance").with_font_size(12),
                    tile("Step 4: Look at the table").with_font_size(12),
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

    #[test]
    fn activity_running_do_it() {
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
                v_layout([tile("Running"), h_line(), tile("5k")]),
                v_layout([
                    tile("GPS 3D").with_font_size(16),
                    h_line(),
                    tile("Press OK").with_font_size(16),
                    tile("to start").with_font_size(16),
                ]),
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
}
