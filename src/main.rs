use crate::{
    bc_render::render_to_bc,
    common::{h_layout, h_line, h_split, invalidate_dimensions, tile, v_layout, v_line, Dimension},
    sixtyfps_render::render_to_60fps,
};

// sixtyfps::sixtyfps!(
//     import { MainWindow } from "./ui/main.60";
// );

mod bc_render;
mod common;
mod sixtyfps_render;
mod mocks_sharp_mip_2in7;
mod mocks_waveshare2in9;

fn main() {
    bc_test_page();

    // let main_window = MainWindow::new();
    // main_window.run();
}

/// Generate a mockup by declaring a GUI tree.
///
/// Having a tree, call invalidate_dimensions,
/// to automatically calculate widths and heights
/// of each element.
///
/// Call render_to_60fps to generate .60 markup
/// file allowing to quick peek on how mockup looks like.
///
/// Call render_to_bc to generate C++ code so you don't
/// have to write it anymore!
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
                tile("02/09/21").with_format("%d/%m/%y"),
                tile("19:34:20").with_format("%T"),
            ]),
            v_layout([
                tile("in view / tracked"),
                tile("13 / 11").with_format("%d / %d").with_font_size(19),
            ]),
        ]),
        // Current implementation of invalidate_dimensions
        // makes {h,v}_line split tiles defined after them
        // so they 'stick' to the bottom
        h_line(),
        v_line(),
        h_layout([
            v_layout([
                tile("23.19[*C]").with_format("%5.2f[*C]"),
                tile("133.94[m]").with_format("%5.2f[m]"),
            ]),
            v_layout([
                tile("Hit button below"),
                tile("to calculate your"),
                tile("BMI"),
            ]),
        ]),
    ]);

    let mut gui = h_split(status_bar, 0.101, welcome_page);

    // Dimensions of BC display
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

/// Rest of mockups are in form of tests, so you can quickly
/// Look at them by clicking Run Test button above the test case
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
    fn sample_gui() {
        let mut gui = v_layout([
            h_layout([
                h_layout([tile("A"), tile("B")]),
                v_layout([tile("C"), tile("D")]),
            ]),
            h_layout([
                tile("E"),
                h_layout([tile("F"), v_layout([tile("G"), tile("H")])]),
            ]),
        ]);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);
    }

    #[test]
    fn v_layout_with_h_layouts_splits_them_via_v_lines() {
        // V:
        //   H: A | B
        //   H: C | D
        let mut gui = v_layout([
            v_line(), // splits A | B
            h_layout([tile("A"), tile("B")]),
            v_line(), // splits C | D
            h_layout([tile("C"), tile("D")]),
        ]);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);
    }

    #[test]
    fn h_layout_with_v_layouts_splits_them_via_h_lines() {
        // H: V: A  V: C
        //       --    --
        //       B ,   D
        let mut gui = h_layout([
            h_line(),
            // splits
            // A
            // --
            // B
            v_layout([tile("A"), tile("B")]),
            h_line(),
            // splits
            // C
            // --
            // D
            v_layout([tile("C"), tile("D")]),
        ]);

        let d = Dimension {
            x: 0,
            y: 0,
            width: 296,
            height: 128,
        };

        invalidate_dimensions(&mut gui, &d);

        render_to_60fps(&gui, &d);
    }


}
