# bc-gui-maker
Quick And Dirty GUI generator for my [embedded project](https://github.com/szymek156/bike_computer_esp32)
1) You declare GUI layout
```
let welcome_page = v_layout([
            h_line(),
            v_line(),
            h_layout([
                v_layout([tile("Paused").with_font_size(42), h_line(), tile("")]),
                v_list(["Resume", "Save", "Discard"]).with_font_size(24),
            ]),
        ]);
```
2) Application calculates font sizes, coordinates of GUI elements and dumps it to the C++ code (so you don't have to write it anymore!)
```
// Following code is generated automagically,
// don't bother understand it.
// Paused
display_->enqueueDraw(
    [&](Paint &paint) {
        const int msg_size = 128;
        char message[msg_size];

        snprintf(message, msg_size, "Paused");
        paint.DrawStringAt(28, 57, message, &Font42, COLORED);
},
{1, 25, 199, 131});

// 
display_->enqueueDraw(
    [&](Paint &paint) {
        const int msg_size = 128;
        char message[msg_size];

        snprintf(message, msg_size, "");
        paint.DrawStringAt(100, 158, message, &Font56, COLORED);
},
{1, 133, 199, 239});

// CTOR
VListWidget(display, {"Resume", "Save", "Discard"}, Font24, {201, 25, 399, 239})
```
3) You can peek how GUI will look like using the preview window:
<img src="https://user-images.githubusercontent.com/1136779/144651288-b17849de-5aaf-48ee-b40a-ec7a287858b0.jpg" alt="drawing" width="300"/>
