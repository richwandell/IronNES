use std::collections::HashMap;
use graphics::{Context, text, Transformed};
use graphics::types::Color;
use opengl_graphics::{GlGraphics, GlyphCache};
use crate::{COLOR_BLUE, COLOR_WHITE, COLOR_GREEN, COLOR_RED, Cpu, State};
use crate::bus::cpu_read;
use crate::display::IColor;
use crate::cpu::Flags;
use crate::display::{EMU_HEIGHT, EMU_WIDTH};



pub(crate) fn draw_debug(state: &State,
                         cpu: &Cpu,
                         context: Context,
                         mut glyphs: &mut GlyphCache,
                         disassembly: &HashMap<u16, String>,
                         gl: &mut GlGraphics
) {
    let size = context.get_view_size();
    let x_scaler = size[0] / EMU_WIDTH as f64;
    let y_scaler = size[1] / EMU_HEIGHT as f64;
    let average_scaler = (x_scaler + y_scaler) / 2.;

    let mut text_size = 12;

    let mut text_color = COLOR_WHITE;

    macro_rules! draw_string {
        ($text:expr, $x:expr, $y:expr) => {
            let transform = context.transform.trans($x as f64 * x_scaler, $y as f64 * y_scaler);
            let color: Color = IColor::from(text_color).into();
            text(color, (text_size as f64 * average_scaler) as u32, $text, glyphs, transform, gl);
        };

        ($text:expr, $x:expr, $y:expr, $color:expr) => {
            let transform = context.transform.trans($x as f64 * x_scaler, $y as f64 * y_scaler);
            let color: Color = IColor::from($color).into();
            text(color, (text_size as f64 * average_scaler) as u32, $text, glyphs, transform, gl);
        };

        ($text:expr, $x:expr, $y:expr, $size:expr, $color:expr) => {
            let transform = context.transform.trans($x as f64 * x_scaler, $y as f64 * y_scaler);
            let color: Color = IColor::from($color).into();
            text(color, ($size as f64 * average_scaler) as u32, $text, glyphs, transform, gl);
        };
    }

    text_color = COLOR_BLUE;
    draw_string!("Hey There", 10, 100);

    let mut x = 180;
    let mut y = 10;
    text_size = 3;

    draw_string!("STATUS:", x, y, COLOR_WHITE);

    draw_string!("N", x + text_size * 6, y, if cpu.status & Flags::N as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    draw_string!("V", x + text_size * 7, y, if cpu.status & Flags::V as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    draw_string!("-", x + text_size * 8, y, if cpu.status & Flags::U as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    draw_string!("B", x + text_size * 9, y, if cpu.status & Flags::B as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    draw_string!("D", x + text_size * 10, y, if cpu.status & Flags::D as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    draw_string!("I", x + text_size * 11, y, if cpu.status & Flags::I as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    draw_string!("Z", x + text_size * 12, y, if cpu.status & Flags::Z as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    draw_string!("C", x + text_size * 13, y, if cpu.status & Flags::C as u8 > 0 {
        COLOR_GREEN
    } else {
        COLOR_RED
    });

    let pc_string = hex::encode(&cpu.pc.to_be_bytes());
    draw_string!(&format!("PC: ${}", pc_string), x, y + text_size, COLOR_WHITE);

    let a_reg_string = hex::encode(&cpu.a.to_be_bytes());
    let a_string = cpu.a.to_string();
    draw_string!(&format!("A: ${} [{}]", a_reg_string, a_string), x, y + text_size * 2, COLOR_WHITE);

    let x_reg_string = hex::encode(&cpu.x.to_be_bytes());
    let x_string = cpu.x.to_string();
    draw_string!(&format!("X: ${} [{}]", x_reg_string, x_string), x, y + text_size * 3, COLOR_WHITE);

    let y_reg_string = hex::encode(&cpu.y.to_be_bytes());
    let y_string = cpu.y.to_string();
    draw_string!(&format!("Y: ${} [{}]", y_reg_string, y_string), x, y + text_size * 4, COLOR_WHITE);

    let stack_string = hex::encode(&cpu.stkp.to_be_bytes());
    draw_string!(&format!("Stack P: ${}", stack_string), x, y + text_size * 5, COLOR_WHITE);

    text_size = 3;
    x = 180;
    y = y + text_size * 7;
    let n_lines = 20;
    let mut c_line = 0;
    let mut c_addr = 0;
    while c_line < n_lines {
        let instruction = cpu.pc + c_addr;
        if let Some(dis_inst) = disassembly.get(&instruction) {
            draw_string!(&dis_inst, x, y, COLOR_WHITE);
            y += text_size;
            c_line += 1
        }
        c_addr += 1;
    }

    x = 3;
    y = 10;
    text_size = 3;
    let mut addr: u16 = 0x0000;
    for _ in 0..16 {
        let mut write_string = format!("${}:", hex::encode(&addr.to_be_bytes()));
        for _ in 0..16 {
            let value = hex::encode(&cpu_read(&state, addr, true).to_be_bytes());
            write_string = format!("{} {}", write_string, &value[value.len()-2..]);
            addr += 1;
        }
        draw_string!(&write_string, x, y, COLOR_WHITE);
        y += text_size;
    }

    x = 3;
    y += text_size;
    text_size = 3;
    let mut addr: u16 = 0x8000;
    for _ in 0..16 {
        let mut write_string = format!("${}:", hex::encode(&addr.to_be_bytes()));
        for _ in 0..16 {
            let value = hex::encode(&cpu_read(&state, addr, true).to_be_bytes());
            write_string = format!("{} {}", write_string, &value[value.len()-2..]);
            addr += 1;
        }
        draw_string!(&write_string, x, y, COLOR_WHITE);
        y += text_size;
    }
}
